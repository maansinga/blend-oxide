#[repr(C)]
pub struct MVert {
    co: [f32; 3],
    no: [i16; 3],

    flag: u8,
    bweight: i8
}

pub enum MVertFlag {
    // SELECT = (1 << 0)
    ME_VERT_TMP_TAG = (1 << 2),
    ME_HIDE = (1 << 4),
    ME_VERT_FACEDOT = (1 << 5),
    //  ME_VERT_MERGED = (1 << 6)
    ME_VERT_PBVH_UPDATE = (1 << 7),
}

/**
 * Mesh Edges.
 *
 * Typically accessed from #Mesh.medge
 */
#[repr(C)]
pub struct MEdge {
    /** Un-ordered vertex indices (cannot match). */
    v1: u32,
    v2: u32,
    crease: i8,
    bweight: i8,
    flag: u16
}

pub enum MEdgeFlag{
    //  SELECT = (1 << 0)
    ME_EDGEDRAW = (1 << 1),
    ME_SEAM = (1 << 2),
    //  ME_HIDE = (1 << 4)
    ME_EDGERENDER = (1 << 5),
    ME_LOOSEEDGE = (1 << 7),
    ME_EDGE_TMP_TAG = (1 << 8),
    ME_SHARP = (1 << 9), /* only reason this flag remains a 'short' */
}

/**
 * Mesh Faces
 * This only stores the polygon size & flags, the vertex & edge indices are stored in the #MLoop.
 *
 * Typically accessed from #Mesh.mpoly.
 */
#[repr(C)]
pub struct MPoly {
    /** Offset into loop array and number of loops in the face. */
    loopstart: i32,
    /** Keep signed since we need to subtract when getting the previous loop. */
    totloop: i32,
    mat_nr: i16,
    flag: u8,
    _pad: u8
}

/** #MPoly.flag */
pub enum MPolyFlag{
    ME_SMOOTH = (1 << 0),
    ME_FACE_SEL = (1 << 1),
    /* ME_HIDE = (1 << 4), */
}

/**
 * Mesh Loops.
 * Each loop represents the corner of a polygon (#MPoly).
 *
 * Typically accessed from #Mesh.mloop.
 */
#[repr(C)]
pub struct MLoop {
    /** Vertex index. */
    v: u32,

    /**
    * Edge index.
    *
    * \note The e here is because we want to move away from relying on edge hashes.
    */
    e: u32
}

/* -------------------------------------------------------------------- */
/** \name Ordered Selection Storage
 * \{ */

/**
 * Optionally store the order of selected elements.
 * This wont always be set since only some selection operations have an order.
 *
 * Typically accessed from #Mesh.mselect
 */
#[repr(C)]
pub struct MSelect {
    /** Index in the vertex, edge or polygon array. */
    index: i32,
    /** #ME_VSEL, #ME_ESEL, #ME_FSEL. */
    type: u32
}

/** #MSelect.type */
#[repr(C)]
pub enum MSelectType{
    ME_VSEL = 0,
    ME_ESEL = 1,
    ME_FSEL = 2,
};


/* -------------------------------------------------------------------- */
/** \name Loop Tesselation Runtime Data
 * \{ */

/**
 * #MLoopTri's are lightweight triangulation data,
 * for functionality that doesn't support ngons (#MPoly).
 * This is cache data created from (#MPoly, #MLoop & #MVert arrays).
 * There is no attempt to maintain this data's validity over time,
 * any changes to the underlying mesh invalidate the #MLoopTri array,
 * which will need to be re-calculated.
 *
 * Users normally access this via #BKE_mesh_runtime_looptri_ensure.
 * In rare cases its calculated directly, with #BKE_mesh_recalc_looptri.
 *
 * Typical usage includes:
 * - OpenGL drawing.
 * - #BVHTree creation.
 * - Physics/collision detection.
 *
 * Storing loop indices (instead of vertex indices) allows us to
 * directly access UV's, vertex-colors as well as vertices.
 * The index of the source polygon is stored as well,
 * giving access to materials and polygon normals.
 *
 * \note This data is runtime only, never written to disk.
 *
 * Usage examples:
 * \code{.c}
 * // access original material.
 * short mat_nr = mpoly[lt->poly].mat_nr;
 *
 * // access vertex locations.
 * float *vtri_co[3] = {
 *     mvert[mloop[lt->tri[0]].v].co,
 *     mvert[mloop[lt->tri[1]].v].co,
 *     mvert[mloop[lt->tri[2]].v].co,
 * };
 *
 * // access UV coordinates (works for all loop data, vertex colors... etc).
 * float *uvtri_co[3] = {
 *     mloopuv[lt->tri[0]].uv,
 *     mloopuv[lt->tri[1]].uv,
 *     mloopuv[lt->tri[2]].uv,
 * };
 * \endcode
 *
 * #MLoopTri's are allocated in an array, where each polygon's #MLoopTri's are stored contiguously,
 * the number of triangles for each polygon is guaranteed to be (#MPoly.totloop - 2),
 * even for degenerate geometry. See #ME_POLY_TRI_TOT macro.
 *
 * It's also possible to perform a reverse lookup (find all #MLoopTri's for any given #MPoly).
 *
 * \code{.c}
 * // loop over all looptri's for a given polygon: i
 * MPoly *mp = &mpoly[i];
 * MLoopTri *lt = &looptri[poly_to_tri_count(i, mp->loopstart)];
 * int j, lt_tot = ME_POLY_TRI_TOT(mp);
 *
 * for (j = 0; j < lt_tot; j++, lt++) {
 *     unsigned int vtri[3] = {
 *         mloop[lt->tri[0]].v,
 *         mloop[lt->tri[1]].v,
 *         mloop[lt->tri[2]].v,
 *     };
 *     printf("tri %u %u %u\n", vtri[0], vtri[1], vtri[2]);
 * };
 * \endcode
 *
 * It may also be useful to check whether or not two vertices of a triangle
 * form an edge in the underlying mesh.
 *
 * This can be done by checking the edge of the referenced loop (#MLoop.e),
 * the winding of the #MLoopTri and the #MLoop's will always match,
 * however the order of vertices in the edge is undefined.
 *
 * \code{.c}
 * // print real edges from an MLoopTri: lt
 * int j, j_next;
 * for (j = 2, j_next = 0; j_next < 3; j = j_next++) {
 *     MEdge *ed = &medge[mloop[lt->tri[j]].e];
 *     unsigned int tri_edge[2]  = {mloop[lt->tri[j]].v, mloop[lt->tri[j_next]].v};
 *
 *     if (((ed->v1 == tri_edge[0]) && (ed->v2 == tri_edge[1])) ||
 *         ((ed->v1 == tri_edge[1]) && (ed->v2 == tri_edge[0])))
 *     {
 *         printf("real edge found %u %u\n", tri_edge[0], tri_edge[1]);
 *     }
 * }
 * \endcode
 *
 * See #BKE_mesh_looptri_get_real_edges for a utility that does this.
 *
 * \note A #MLoopTri may be in the middle of an ngon and not reference **any** edges.
 */
 #[repr(C)]
pub struct MLoopTri {
    tri: [u32; 3];
    poly: u32;
}

#[repr(C)]
pub struct MVertTri {
    tri: [u32; 3];
}


/* -------------------------------------------------------------------- */
/** \name Custom Data (Generic)
 * \{ */

/** Custom Data Properties */
#[repr(C)]
pub struct MFloatProperty {
    f: f32;
}

#[repr(C)]
pub struct MIntProperty {
    i: i32
}

#[repr(C)]
pub struct MStringProperty {
    s: [char; 255],
    s_len: u8
}
  
/* -------------------------------------------------------------------- */
/** \name Custom Data (Vertex)
 * \{ */

/**
 * Vertex group index and weight for #MDeformVert.dw
 */
 #[repr(C)]
 pub struct MDeformWeight {
    /** The index for the vertex group, must *always* be unique when in an array. */
    def_nr: u32,
    /** Weight between 0.0 and 1.0. */
    weight: f32
}

#[repr(C)]
pub struct MDeformVert {
    dw: *mut MDeformWeight,
    totweight: i32,
    /** Flag is only in use as a run-time tag at the moment. */
    flag: i32
} MDeformVert;
  
#[repr(C)]
pub struct MVertSkin {
    /**
    * Radii of the skin, define how big the generated frames are.
    * Currently only the first two elements are used.
    */
    radius: [f32; 3],

    /** #eMVertSkinFlag */
    flag: i32
}

enum eMVertSkinFlag {
    /** Marks a vertex as the edge-graph root, used for calculating rotations for all connected
        * edges (recursively). Also used to choose a root when generating an armature.
        */
    MVERT_SKIN_ROOT = 1,

    /** Marks a branch vertex (vertex with more than two connected edges), so that it's neighbors
        * are directly hulled together, rather than the default of generating intermediate frames.
        */
    MVERT_SKIN_LOOSE = 2,
}


/* -------------------------------------------------------------------- */
/** \name Custom Data (Loop)
 * \{ */

/**
 * UV coordinate for a polygon face & flag for selection & other options.
 */
 #[repr(C)]
 struct MLoopUV {
    uv: [f32; 2],
    flag: i32
 }
  
  /** #MLoopUV.flag */
enum MLoopUVFlag{
    /* MLOOPUV_DEPRECATED = (1 << 0), MLOOPUV_EDGESEL removed */
    MLOOPUV_VERTSEL = (1 << 1),
    MLOOPUV_PINNED = (1 << 2)
}

/**
 * \note While alpha is currently is not in the view-port,
 * this may eventually be added back, keep this value set to 255.
 */
#[repr(C)]
pub struct MLoopCol {
    r: u8,
    g: u8,
    b: u8,
    a: u8
 }
  
 #[repr(C)]
 pub struct MPropCol {
    color: [f32; 4]
 }
  
/** Multi-Resolution loop data. */
#[repr(C)]
pub struct MDisps {
    /* Strange bug in SDNA: if disps pointer comes first, it fails to see totdisp */
    totdisp: i32,
    level: i32,
    disps: [*mut f32; 3],
  
    /**
     * Used for hiding parts of a multires mesh.
     * Essentially the multires equivalent of #MVert.flag's ME_HIDE bit.
     *
     * \note This is a bitmap, keep in sync with type used in BLI_bitmap.h
     */
    hidden: *mut u32;
}

/** Multi-Resolution grid loop data. */
#[repr(C)]
pub struct GridPaintMask {
    /**
     * The data array contains `grid_size * grid_size` elements.
     * Where `grid_size = (1 << (level - 1)) + 1`.
     */
    data: *mut f32,
  
    /** The maximum multires level associated with this grid. */
    level: u32,
  
    _pad: [char; 4];
}
  
/** \} */

/* -------------------------------------------------------------------- */
/** \name Custom Data (Original Space for Poly, Face)
* \{ */

/**
* Original space within a face (similar to UV coordinates),
* however they are used to determine the original position in a face.
*
* Unlike UV's these are not user editable and always start out using a fixed 0-1 range.
* Currently only used for particle placement.
*/
#[repr(C)]
pub struct OrigSpaceFace {
    uv: [[f32; 4]; 2]
}
  
#[repr(C)]
pub struct OrigSpaceLoop {
    uv: [f32; 2]
}
  
  /** \} */
  
/* -------------------------------------------------------------------- */
/** \name Custom Data (FreeStyle for Edge, Face)
* \{ */
#[repr(C)]
pub struct FreestyleEdge {
    flag: char,
    _pad: [char; 3]
}
  
/** #FreestyleEdge.flag */
pub enum FreestyleEdgeFlag{
    FREESTYLE_EDGE_MARK = 1,
}

#[repr(C)]
pub struct FreestyleFace {
    flag: char
    _pad: [char; 3];
}
  
/** #FreestyleFace.flag */
pub enum FreestyleFaceFlag {
    FREESTYLE_FACE_MARK = 1,
};

/**
 * Used in Blender pre 2.63, See #MLoop, #MPoly for face data stored in the blend file.
 * Use for reading old files and in a handful of cases which should be removed eventually.
 */
#[repr(C)]
pub struct MFace {
    v1: u32,
    v2: u32,
    v3: u32,
    v4: u32
    mat_nr: i16,
    /** We keep edcode, for conversion to edges draw flags in old files. */
    edcode: u8, 
    flag: u8
}
  
/** #MFace.edcode */
enum MFaceEdcode{
    ME_V1V2 = (1 << 0),
    ME_V2V3 = (1 << 1),
    ME_V3V1 = (1 << 2),
    ME_V3V4 = ME_V3V1,
    ME_V4V1 = (1 << 3),
};

/** Tessellation uv face data. */
#[repr(C)]
pub struct MTFace {
  uv: [[float; 4]; 2];
}

/**
 * Tessellation vertex color data.
 *
 * \note The red and blue are swapped for historical reasons.
 */
 #[repr(C)]
struct MCol {
  a: u8,
  r: u8,
  g: u8, 
  b: u8
}

#[inline]
fn MESH_MLOOPCOL_FROM_MCOL(_mloopcol: &mut MLoopCol, _mcol: &MCol) {
    _mloopcol.r = _mcol.b;
    _mloopcol.g = _mcol.g;
    _mloopcol.b = _mcol.r;
    _mloopcol.a = _mcol.a;
}

#[inline]
fn MESH_MLOOPCOL_TO_MCOL(_mloopcol: &MLoopCol, _mcol: &mut MCol) {
    _mcol.b = _mloopcol.r;
    _mcol.g = _mloopcol.g;
    _mcol.r = _mloopcol.b;
    _mcol.a = _mloopcol.a;
}


/** Old game engine recast navigation data, while unused 2.7x files may contain this. */
struct MRecast {
    i: i32
}
  
/** Multires structs kept for compatibility with old files. */
struct MultiresCol {
    a: f32,
    r: f32,
    g: f32,
    b: f32
}
  
struct MultiresColFace {
    /* vertex colors */
    col: [MultiresCol; 4]
}
  
struct MultiresFace {
    v: [u32; 4],
    mid: u32;
    flag: u8, 
    mat_nr: u8, 
    _pad: [u8; 2]
}
  
struct MultiresEdge {
    v: [u32; 2],
    mid: u32;
}
  
struct MultiresLevel {
    next: *mut MultiresLevel,
    prev: *mut MultiresLevel;
  
    faces: *mut MultiresFace;
    colfaces: *mut MultiresColFace;
    edges: *mut MultiresEdge;
  
    totvert: u32,
    totface: u32,
    totedge: u32;
    _pad: [char; 4],
  
    /* Kept for compatibility with even older files */
    verts: *mut MVert
}
  
// struct Multires {
//     ListBase levels;
//     MVert *verts;
  
//     unsigned char level_count, current, newlvl, edgelvl, pinlvl, renderlvl;
//     unsigned char use_col, flag;
  
//     /* Special level 1 data that cannot be modified from other levels */
//     CustomData vdata;
//     CustomData fdata;
//     short *edge_flags;
//     char *edge_creases;
// }
/* End multi-res structs. */
  
  /** \} */
  
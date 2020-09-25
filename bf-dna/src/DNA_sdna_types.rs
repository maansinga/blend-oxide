use bf_dna_blenlib::bli_ghash::GHash;
use bf_dna_blenlib::bli_memarena::MemArena;

#[repr(C)]
pub struct _Alias {
    /** Aligned with #SDNA.names, same pointers when unchanged. */
    names: *mut *const char,
    /** Aligned with #SDNA.types, same pointers when unchanged. */
    types: *mut *const char,
    /** A version of #SDNA.structs_map that uses #SDNA.alias.types for it's keys. */
    structs_map: *mut GHash,
}

#[repr(C)]
pub struct SDNA {
    /** Full copy of 'encoded' data (when data_alloc is set, otherwise borrowed). */
    data: *const char,
    /** Length of data. */
    data_len: i32,
    data_alloc: bool,

    /** Total number of struct members. */
    names_len: i32,
    names_len_alloc: i32,

    /** Struct member names. */
    names: *mut *const char,

    /** Result of #DNA_elem_array_size (aligned with #names). */
    names_array_len: *mut i16,

    /** Size of a pointer in bytes. */
    pointer_size: i32,

    /** Type names. */
    types: *mut *const char,

    /** Number of basic types + struct types. */
    types_len: i32,

    /** Type lengths. */
    types_size: *mut i16,

    /**
     * sp = structs[a] is the address of a struct definition
     * sp[0] is struct type number, sp[1] amount of members
     *
     * (sp[2], sp[3]), (sp[4], sp[5]), .. are the member
     * type and name numbers respectively.
     */
    structs: *mut *const i16,

    /** Number of struct types. */
    structs_len: i32,

    // /** #GHash for faster lookups, requires WITH_DNA_GHASH to be used for now. */
    // structs_map: *mut GHash,

    // /** Temporary memory currently only used for version patching DNA. */
    mem_arena: *mut MemArena,

    // /** Runtime versions of data stored in DNA, lazy initialized,
    // * only different when renaming is done. */
    alias: _Alias,
}

pub mod bli_memarena {
    #[repr(C)]
    pub struct MemBuf {
        next: *mut MemBuf,
        data: [u8; 0],
    }

    #[repr(C)]
    pub struct MemArena {
        curbuf: *mut u8,
        name: *const char,
        bufs: *mut MemBuf,

        bufsize: u64,
        cursize: u64,
        align: u64,

        use_calloc: bool,
    }
}

pub mod bli_mempool {
    #[repr(C)]
    pub struct BLI_freenode {
        next: *mut BLI_freenode,
        /** Used to identify this as a freed node. */
        freeword: i64,
    }

    #[repr(C)]
    pub struct BLI_mempool_chunk {
        next: *mut BLI_mempool_chunk,
    }

    #[repr(C)]
    pub struct BLI_mempool {
        chunks: *mut BLI_mempool_chunk,
        chunk_tail: *mut BLI_mempool_chunk,

        esize: u32,
        csize: u32,
        pchunk: u32,
        flag: u32,

        free: BLI_freenode,

        maxchunks: u32,

        totused: u32,
    }
}

pub mod bli_ghash {
    type GHashHashFP = fn(*const usize) -> u32;
    type GHashCmpFP = fn(*const usize) -> bool;

    #[repr(C)]
    struct Entry {
        next: *mut Entry,

        key: *mut usize,
    }

    #[repr(C)]
    pub struct GHash {
        hashfp: GHashHashFP,
        cmpfp: GHashCmpFP,
        buckets: *mut *mut Entry,
        entrypool: *mut crate::bli_mempool::BLI_mempool,
        nbuckets: u32,
        limit_grow: u32,
        limit_shrink: u32,
        cursize: u32,
        size_min: u32,

        nentries: u32,
        flag: u32,
    }
}

#[repr(C)]
struct BlendFileReadParams {
    skip_flags: u32,
    is_startup: u32,

    undo_direction: i32,
}

impl Default for BlendFileReadParams {
    fn default() -> Self {
        BlendFileReadParams {
            skip_flags: 3u32,
            is_startup: 1u32,

            undo_direction: 2i32,
        }
    }
}

pub mod DNA_sdna_types {
    pub struct BHead {
        code: i32,
        len: i32,
        old: *const usize,
        SDNAnr: i32,
        nr: i32,
    }

    pub struct BHead4 {
        code: i32,
        len: i32,
        old: i32,
        SDNAnr: i32,
        nr: i32,
    }

    pub struct BHead8 {
        code: i32,
        len: i32,
        old: i64,
        SDNAnr: i32,
        nr: i32,
    }
}

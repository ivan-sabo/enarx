bitflags::bitflags! {
    /// Section 38.8.1
    pub struct Flags: u64 {
        const DBGOPTIN = 1 << 0;
    }
}

/// Thread Control Structure (TCS) is an enclave page visible in its address
/// space that defines an entry point inside the enclave. A thread enters inside
/// an enclave by supplying address of TCS to ENCLU(EENTER). A TCS can be entered
/// by only one thread at a time.
///
/// Section 38.8
#[derive(Debug)]
#[repr(C, align(4096))]
pub struct Tcs {
    pub state: u64,         // used to mark an entered TCS
    pub flags: Flags,       // execution flags (cleared by EADD)
    pub ssa_offset: u64,    // SSA stack offset relative to the enclave base
    pub ssa_index: u32,     // the current SSA frame index (cleard by EADD)
    pub nr_ssa_frames: u32, // the number of frames in the SSA stack
    pub entry_offset: u64,  // entry point offset relative to the enclave base
    pub exit_addr: u64,     // address outside enclave to exit on an exception or interrupt
    pub fs_offset: u64, // offset relative to enclave base to become FS segment inside the enclave
    pub gs_offset: u64, // offset relative to enclave base to become GS segment inside the enclave
    pub fs_limit: u32,  // size to become a new FS-limit (only 32-bit enclaves)
    pub gs_limit: u32,  // size to become a new GS-limit (only 32-bit enclaves)
}

impl Tcs {
    pub const fn new(entry: u64, ssa: u64, nssa: u32) -> Self {
        Self {
            state: 0,
            flags: Flags::empty(),
            ssa_offset: ssa,
            ssa_index: 0,
            nr_ssa_frames: nssa,
            entry_offset: entry,
            exit_addr: 0,
            fs_offset: 0,
            gs_offset: 0,
            fs_limit: 0,
            gs_limit: 0,
        }
    }
}

impl AsRef<[u8]> for Tcs {
    fn as_ref(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(
                self as *const Self as *const u8,
                core::mem::size_of_val(self),
            )
        }
    }
}

testaso! {
    struct Tcs: 4096, 4096 => {
        state: 0,
        flags: 8,
        ssa_offset: 16,
        ssa_index: 24,
        nr_ssa_frames: 28,
        entry_offset: 32,
        exit_addr: 40,
        fs_offset: 48,
        gs_offset: 56,
        fs_limit: 64,
        gs_limit: 68
    }
}

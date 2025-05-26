#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    ier: IER,
    _reserved1: [u8; 0x1c],
    m: [M; 2],
}
impl RegisterBlock {
    ///0x00 - RAMECC interrupt enable register
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    ///0x20..0x60 - Cluster M%s, containing M?CR, M?SR, M?FAR, M?FDRL, M?FDRH, M?FECR
    ///
    ///<div class="warning">`n` is the index of cluster in the array. `n == 0` corresponds to `M1` cluster.</div>
    #[inline(always)]
    pub const fn m(&self, n: usize) -> &M {
        &self.m[n]
    }
    ///Iterator for array of:
    ///0x20..0x60 - Cluster M%s, containing M?CR, M?SR, M?FAR, M?FDRL, M?FDRH, M?FECR
    #[inline(always)]
    pub fn m_iter(&self) -> impl Iterator<Item = &M> {
        self.m.iter()
    }
    ///0x20..0x40 - Cluster M1, containing M?CR, M?SR, M?FAR, M?FDRL, M?FDRH, M?FECR
    #[inline(always)]
    pub const fn m1(&self) -> &M {
        self.m(0)
    }
    ///0x40..0x60 - Cluster M2, containing M?CR, M?SR, M?FAR, M?FDRL, M?FDRH, M?FECR
    #[inline(always)]
    pub const fn m2(&self) -> &M {
        self.m(1)
    }
}
pub use crate::stm32h733::ramecc1::ier;
pub use crate::stm32h733::ramecc1::m;
pub use crate::stm32h733::ramecc1::IER;
///Cluster M%s, containing M?CR, M?SR, M?FAR, M?FDRL, M?FDRH, M?FECR
pub use crate::stm32h733::ramecc1::M;

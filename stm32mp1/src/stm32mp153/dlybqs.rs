#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    cfgr: CFGR,
    _reserved2: [u8; 0x03ec],
    verr: VERR,
    ipidr: IPIDR,
    sidr: SIDR,
}
impl RegisterBlock {
    ///0x00 - DLYB control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - DLYB configuration register
    #[inline(always)]
    pub const fn cfgr(&self) -> &CFGR {
        &self.cfgr
    }
    ///0x3f4 - DLYB IP version register
    #[inline(always)]
    pub const fn verr(&self) -> &VERR {
        &self.verr
    }
    ///0x3f8 - DLYB IP identification register
    #[inline(always)]
    pub const fn ipidr(&self) -> &IPIDR {
        &self.ipidr
    }
    ///0x3fc - DLYB size ID register
    #[inline(always)]
    pub const fn sidr(&self) -> &SIDR {
        &self.sidr
    }
}
/**CR (rw) register accessor: DLYB control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DLYBQS:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///DLYB control register
pub mod cr;
/**CFGR (rw) register accessor: DLYB configuration register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DLYBQS:CFGR)

For information about available fields see [`mod@cfgr`] module*/
pub type CFGR = crate::Reg<cfgr::CFGRrs>;
///DLYB configuration register
pub mod cfgr;
/**VERR (r) register accessor: DLYB IP version register

You can [`read`](crate::Reg::read) this register and get [`verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DLYBQS:VERR)

For information about available fields see [`mod@verr`] module*/
pub type VERR = crate::Reg<verr::VERRrs>;
///DLYB IP version register
pub mod verr;
/**IPIDR (r) register accessor: DLYB IP identification register

You can [`read`](crate::Reg::read) this register and get [`ipidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DLYBQS:IPIDR)

For information about available fields see [`mod@ipidr`] module*/
pub type IPIDR = crate::Reg<ipidr::IPIDRrs>;
///DLYB IP identification register
pub mod ipidr;
/**SIDR (r) register accessor: DLYB size ID register

You can [`read`](crate::Reg::read) this register and get [`sidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DLYBQS:SIDR)

For information about available fields see [`mod@sidr`] module*/
pub type SIDR = crate::Reg<sidr::SIDRrs>;
///DLYB size ID register
pub mod sidr;

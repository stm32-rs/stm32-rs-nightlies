#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    dlyb_cr: DLYB_CR,
    dlyb_cfgr: DLYB_CFGR,
    _reserved2: [u8; 0x03ec],
    dlyb_verr: DLYB_VERR,
    dlyb_ipidr: DLYB_IPIDR,
    dlyb_sidr: DLYB_SIDR,
}
impl RegisterBlock {
    ///0x00 - DLYB control register
    #[inline(always)]
    pub const fn dlyb_cr(&self) -> &DLYB_CR {
        &self.dlyb_cr
    }
    ///0x04 - DLYB configuration register
    #[inline(always)]
    pub const fn dlyb_cfgr(&self) -> &DLYB_CFGR {
        &self.dlyb_cfgr
    }
    ///0x3f4 - DLYB IP version register
    #[inline(always)]
    pub const fn dlyb_verr(&self) -> &DLYB_VERR {
        &self.dlyb_verr
    }
    ///0x3f8 - DLYB IP identification register
    #[inline(always)]
    pub const fn dlyb_ipidr(&self) -> &DLYB_IPIDR {
        &self.dlyb_ipidr
    }
    ///0x3fc - DLYB size ID register
    #[inline(always)]
    pub const fn dlyb_sidr(&self) -> &DLYB_SIDR {
        &self.dlyb_sidr
    }
}
/**DLYB_CR (rw) register accessor: DLYB control register

You can [`read`](crate::Reg::read) this register and get [`dlyb_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dlyb_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DLYBSD1:DLYB_CR)

For information about available fields see [`mod@dlyb_cr`]
module*/
pub type DLYB_CR = crate::Reg<dlyb_cr::DLYB_CRrs>;
///DLYB control register
pub mod dlyb_cr;
/**DLYB_CFGR (rw) register accessor: DLYB configuration register

You can [`read`](crate::Reg::read) this register and get [`dlyb_cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dlyb_cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DLYBSD1:DLYB_CFGR)

For information about available fields see [`mod@dlyb_cfgr`]
module*/
pub type DLYB_CFGR = crate::Reg<dlyb_cfgr::DLYB_CFGRrs>;
///DLYB configuration register
pub mod dlyb_cfgr;
/**DLYB_VERR (r) register accessor: DLYB IP version register

You can [`read`](crate::Reg::read) this register and get [`dlyb_verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DLYBSD1:DLYB_VERR)

For information about available fields see [`mod@dlyb_verr`]
module*/
pub type DLYB_VERR = crate::Reg<dlyb_verr::DLYB_VERRrs>;
///DLYB IP version register
pub mod dlyb_verr;
/**DLYB_IPIDR (r) register accessor: DLYB IP identification register

You can [`read`](crate::Reg::read) this register and get [`dlyb_ipidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DLYBSD1:DLYB_IPIDR)

For information about available fields see [`mod@dlyb_ipidr`]
module*/
pub type DLYB_IPIDR = crate::Reg<dlyb_ipidr::DLYB_IPIDRrs>;
///DLYB IP identification register
pub mod dlyb_ipidr;
/**DLYB_SIDR (r) register accessor: DLYB size ID register

You can [`read`](crate::Reg::read) this register and get [`dlyb_sidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DLYBSD1:DLYB_SIDR)

For information about available fields see [`mod@dlyb_sidr`]
module*/
pub type DLYB_SIDR = crate::Reg<dlyb_sidr::DLYB_SIDRrs>;
///DLYB size ID register
pub mod dlyb_sidr;

#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    dlyb_cr: DLYB_CR,
    dlyb_cfgr: DLYB_CFGR,
}
impl RegisterBlock {
    ///0x00 - control register
    #[inline(always)]
    pub const fn dlyb_cr(&self) -> &DLYB_CR {
        &self.dlyb_cr
    }
    ///0x04 - configuration register
    #[inline(always)]
    pub const fn dlyb_cfgr(&self) -> &DLYB_CFGR {
        &self.dlyb_cfgr
    }
}
/**DLYB_CR (rw) register accessor: control register

You can [`read`](crate::Reg::read) this register and get [`dlyb_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dlyb_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#DLYBOS:DLYB_CR)

For information about available fields see [`mod@dlyb_cr`]
module*/
pub type DLYB_CR = crate::Reg<dlyb_cr::DLYB_CRrs>;
///control register
pub mod dlyb_cr;
/**DLYB_CFGR (rw) register accessor: configuration register

You can [`read`](crate::Reg::read) this register and get [`dlyb_cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dlyb_cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#DLYBOS:DLYB_CFGR)

For information about available fields see [`mod@dlyb_cfgr`]
module*/
pub type DLYB_CFGR = crate::Reg<dlyb_cfgr::DLYB_CFGRrs>;
///configuration register
pub mod dlyb_cfgr;

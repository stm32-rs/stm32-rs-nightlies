#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pll1: PLL1,
    _reserved1: [u8; 0x08],
    tune: TUNE,
    _reserved2: [u8; 0x08],
    ldo: LDO,
}
impl RegisterBlock {
    #[doc = "0x00 - USBPHYC PLL1 control register"]
    #[inline(always)]
    pub const fn pll1(&self) -> &PLL1 {
        &self.pll1
    }
    #[doc = "0x0c - USBPHYC tuning control register"]
    #[inline(always)]
    pub const fn tune(&self) -> &TUNE {
        &self.tune
    }
    #[doc = "0x18 - USBPHYC LDO control and status register"]
    #[inline(always)]
    pub const fn ldo(&self) -> &LDO {
        &self.ldo
    }
}
#[doc = "PLL1 (rw) register accessor: USBPHYC PLL1 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll1`]
module"]
pub type PLL1 = crate::Reg<pll1::PLL1rs>;
#[doc = "USBPHYC PLL1 control register"]
pub mod pll1;
#[doc = "TUNE (rw) register accessor: USBPHYC tuning control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tune::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tune::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tune`]
module"]
pub type TUNE = crate::Reg<tune::TUNErs>;
#[doc = "USBPHYC tuning control register"]
pub mod tune;
#[doc = "LDO (rw) register accessor: USBPHYC LDO control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ldo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ldo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ldo`]
module"]
pub type LDO = crate::Reg<ldo::LDOrs>;
#[doc = "USBPHYC LDO control and status register"]
pub mod ldo;

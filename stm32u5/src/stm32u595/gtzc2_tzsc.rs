#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tzsc_cr: TZSC_CR,
    _reserved1: [u8; 0x0c],
    tzsc_seccfgr1: TZSC_SECCFGR1,
    _reserved2: [u8; 0x0c],
    tzsc_privcfgr1: TZSC_PRIVCFGR1,
}
impl RegisterBlock {
    #[doc = "0x00 - TZSC control register"]
    #[inline(always)]
    pub const fn tzsc_cr(&self) -> &TZSC_CR {
        &self.tzsc_cr
    }
    #[doc = "0x10 - TZSC secure configuration register 1"]
    #[inline(always)]
    pub const fn tzsc_seccfgr1(&self) -> &TZSC_SECCFGR1 {
        &self.tzsc_seccfgr1
    }
    #[doc = "0x20 - TZSC privilege configuration register 1"]
    #[inline(always)]
    pub const fn tzsc_privcfgr1(&self) -> &TZSC_PRIVCFGR1 {
        &self.tzsc_privcfgr1
    }
}
#[doc = "TZSC_CR (rw) register accessor: TZSC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzsc_cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzsc_cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzsc_cr`]
module"]
pub type TZSC_CR = crate::Reg<tzsc_cr::TZSC_CRrs>;
#[doc = "TZSC control register"]
pub mod tzsc_cr;
#[doc = "TZSC_SECCFGR1 (rw) register accessor: TZSC secure configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzsc_seccfgr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzsc_seccfgr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzsc_seccfgr1`]
module"]
pub type TZSC_SECCFGR1 = crate::Reg<tzsc_seccfgr1::TZSC_SECCFGR1rs>;
#[doc = "TZSC secure configuration register 1"]
pub mod tzsc_seccfgr1;
#[doc = "TZSC_PRIVCFGR1 (rw) register accessor: TZSC privilege configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzsc_privcfgr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzsc_privcfgr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzsc_privcfgr1`]
module"]
pub type TZSC_PRIVCFGR1 = crate::Reg<tzsc_privcfgr1::TZSC_PRIVCFGR1rs>;
#[doc = "TZSC privilege configuration register 1"]
pub mod tzsc_privcfgr1;

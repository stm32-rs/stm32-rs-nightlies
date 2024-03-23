#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mpcbb4_cr: MPCBB4_CR,
    _reserved1: [u8; 0x0c],
    mpcbb4_cfglock: MPCBB4_CFGLOCK,
    _reserved2: [u8; 0xec],
    mpcbb4_seccfgr0: MPCBB4_SECCFGR0,
    _reserved3: [u8; 0xfc],
    mpcbb4_privcfgr0: MPCBB4_PRIVCFGR0,
}
impl RegisterBlock {
    #[doc = "0x00 - MPCBB control register"]
    #[inline(always)]
    pub const fn mpcbb4_cr(&self) -> &MPCBB4_CR {
        &self.mpcbb4_cr
    }
    #[doc = "0x10 - GTZC2 SRAM4 MPCBB configuration lock register"]
    #[inline(always)]
    pub const fn mpcbb4_cfglock(&self) -> &MPCBB4_CFGLOCK {
        &self.mpcbb4_cfglock
    }
    #[doc = "0x100 - MPCBB security configuration for super-block 0 register"]
    #[inline(always)]
    pub const fn mpcbb4_seccfgr0(&self) -> &MPCBB4_SECCFGR0 {
        &self.mpcbb4_seccfgr0
    }
    #[doc = "0x200 - MPCBB privileged configuration for super-block 0 register"]
    #[inline(always)]
    pub const fn mpcbb4_privcfgr0(&self) -> &MPCBB4_PRIVCFGR0 {
        &self.mpcbb4_privcfgr0
    }
}
#[doc = "MPCBB4_CR (rw) register accessor: MPCBB control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb4_cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb4_cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb4_cr`]
module"]
pub type MPCBB4_CR = crate::Reg<mpcbb4_cr::MPCBB4_CRrs>;
#[doc = "MPCBB control register"]
pub mod mpcbb4_cr;
#[doc = "MPCBB4_CFGLOCK (rw) register accessor: GTZC2 SRAM4 MPCBB configuration lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb4_cfglock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb4_cfglock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb4_cfglock`]
module"]
pub type MPCBB4_CFGLOCK = crate::Reg<mpcbb4_cfglock::MPCBB4_CFGLOCKrs>;
#[doc = "GTZC2 SRAM4 MPCBB configuration lock register"]
pub mod mpcbb4_cfglock;
#[doc = "MPCBB4_SECCFGR0 (rw) register accessor: MPCBB security configuration for super-block 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb4_seccfgr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb4_seccfgr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb4_seccfgr0`]
module"]
pub type MPCBB4_SECCFGR0 = crate::Reg<mpcbb4_seccfgr0::MPCBB4_SECCFGR0rs>;
#[doc = "MPCBB security configuration for super-block 0 register"]
pub mod mpcbb4_seccfgr0;
#[doc = "MPCBB4_PRIVCFGR0 (rw) register accessor: MPCBB privileged configuration for super-block 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb4_privcfgr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb4_privcfgr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb4_privcfgr0`]
module"]
pub type MPCBB4_PRIVCFGR0 = crate::Reg<mpcbb4_privcfgr0::MPCBB4_PRIVCFGR0rs>;
#[doc = "MPCBB privileged configuration for super-block 0 register"]
pub mod mpcbb4_privcfgr0;

#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rng_cr: RNG_CR,
    rng_sr: RNG_SR,
    rng_dr: RNG_DR,
    _reserved3: [u8; 0x03e4],
    rng_hwcfgr: RNG_HWCFGR,
    rng_verr: RNG_VERR,
    rng_ipidr: RNG_IPIDR,
    rng_sidr: RNG_SIDR,
}
impl RegisterBlock {
    #[doc = "0x00 - RNG control register"]
    #[inline(always)]
    pub const fn rng_cr(&self) -> &RNG_CR {
        &self.rng_cr
    }
    #[doc = "0x04 - RNG status register"]
    #[inline(always)]
    pub const fn rng_sr(&self) -> &RNG_SR {
        &self.rng_sr
    }
    #[doc = "0x08 - The RNG_DR register is a read-only register."]
    #[inline(always)]
    pub const fn rng_dr(&self) -> &RNG_DR {
        &self.rng_dr
    }
    #[doc = "0x3f0 - RNG hardware configuration register"]
    #[inline(always)]
    pub const fn rng_hwcfgr(&self) -> &RNG_HWCFGR {
        &self.rng_hwcfgr
    }
    #[doc = "0x3f4 - RNG version register"]
    #[inline(always)]
    pub const fn rng_verr(&self) -> &RNG_VERR {
        &self.rng_verr
    }
    #[doc = "0x3f8 - RNG identification register"]
    #[inline(always)]
    pub const fn rng_ipidr(&self) -> &RNG_IPIDR {
        &self.rng_ipidr
    }
    #[doc = "0x3fc - RNG size ID register"]
    #[inline(always)]
    pub const fn rng_sidr(&self) -> &RNG_SIDR {
        &self.rng_sidr
    }
}
#[doc = "RNG_CR (rw) register accessor: RNG control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rng_cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rng_cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_cr`]
module"]
pub type RNG_CR = crate::Reg<rng_cr::RNG_CRrs>;
#[doc = "RNG control register"]
pub mod rng_cr;
#[doc = "RNG_SR (rw) register accessor: RNG status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rng_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rng_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_sr`]
module"]
pub type RNG_SR = crate::Reg<rng_sr::RNG_SRrs>;
#[doc = "RNG status register"]
pub mod rng_sr;
#[doc = "RNG_DR (r) register accessor: The RNG_DR register is a read-only register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rng_dr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_dr`]
module"]
pub type RNG_DR = crate::Reg<rng_dr::RNG_DRrs>;
#[doc = "The RNG_DR register is a read-only register."]
pub mod rng_dr;
#[doc = "RNG_HWCFGR (r) register accessor: RNG hardware configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rng_hwcfgr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_hwcfgr`]
module"]
pub type RNG_HWCFGR = crate::Reg<rng_hwcfgr::RNG_HWCFGRrs>;
#[doc = "RNG hardware configuration register"]
pub mod rng_hwcfgr;
#[doc = "RNG_VERR (r) register accessor: RNG version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rng_verr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_verr`]
module"]
pub type RNG_VERR = crate::Reg<rng_verr::RNG_VERRrs>;
#[doc = "RNG version register"]
pub mod rng_verr;
#[doc = "RNG_IPIDR (r) register accessor: RNG identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rng_ipidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_ipidr`]
module"]
pub type RNG_IPIDR = crate::Reg<rng_ipidr::RNG_IPIDRrs>;
#[doc = "RNG identification register"]
pub mod rng_ipidr;
#[doc = "RNG_SIDR (r) register accessor: RNG size ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rng_sidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_sidr`]
module"]
pub type RNG_SIDR = crate::Reg<rng_sidr::RNG_SIDRrs>;
#[doc = "RNG size ID register"]
pub mod rng_sidr;

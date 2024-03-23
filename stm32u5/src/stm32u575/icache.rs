#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    icache_cr: ICACHE_CR,
    icache_sr: ICACHE_SR,
    icache_ier: ICACHE_IER,
    icache_fcr: ICACHE_FCR,
    icache_hmonr: ICACHE_HMONR,
    icache_mmonr: ICACHE_MMONR,
    _reserved6: [u8; 0x08],
    icache_crr0: ICACHE_CRR0,
    icache_crr1: ICACHE_CRR1,
    icache_crr2: ICACHE_CRR2,
    icache_crr3: ICACHE_CRR3,
}
impl RegisterBlock {
    #[doc = "0x00 - ICACHE control register"]
    #[inline(always)]
    pub const fn icache_cr(&self) -> &ICACHE_CR {
        &self.icache_cr
    }
    #[doc = "0x04 - ICACHE status register"]
    #[inline(always)]
    pub const fn icache_sr(&self) -> &ICACHE_SR {
        &self.icache_sr
    }
    #[doc = "0x08 - ICACHE interrupt enable register"]
    #[inline(always)]
    pub const fn icache_ier(&self) -> &ICACHE_IER {
        &self.icache_ier
    }
    #[doc = "0x0c - ICACHE flag clear register"]
    #[inline(always)]
    pub const fn icache_fcr(&self) -> &ICACHE_FCR {
        &self.icache_fcr
    }
    #[doc = "0x10 - ICACHE hit monitor register"]
    #[inline(always)]
    pub const fn icache_hmonr(&self) -> &ICACHE_HMONR {
        &self.icache_hmonr
    }
    #[doc = "0x14 - ICACHE miss monitor register"]
    #[inline(always)]
    pub const fn icache_mmonr(&self) -> &ICACHE_MMONR {
        &self.icache_mmonr
    }
    #[doc = "0x20 - ICACHE region configuration register"]
    #[inline(always)]
    pub const fn icache_crr0(&self) -> &ICACHE_CRR0 {
        &self.icache_crr0
    }
    #[doc = "0x24 - ICACHE region configuration register"]
    #[inline(always)]
    pub const fn icache_crr1(&self) -> &ICACHE_CRR1 {
        &self.icache_crr1
    }
    #[doc = "0x28 - ICACHE region configuration register"]
    #[inline(always)]
    pub const fn icache_crr2(&self) -> &ICACHE_CRR2 {
        &self.icache_crr2
    }
    #[doc = "0x2c - ICACHE region configuration register"]
    #[inline(always)]
    pub const fn icache_crr3(&self) -> &ICACHE_CRR3 {
        &self.icache_crr3
    }
}
#[doc = "ICACHE_CR (rw) register accessor: ICACHE control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icache_cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icache_cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache_cr`]
module"]
pub type ICACHE_CR = crate::Reg<icache_cr::ICACHE_CRrs>;
#[doc = "ICACHE control register"]
pub mod icache_cr;
#[doc = "ICACHE_SR (r) register accessor: ICACHE status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icache_sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache_sr`]
module"]
pub type ICACHE_SR = crate::Reg<icache_sr::ICACHE_SRrs>;
#[doc = "ICACHE status register"]
pub mod icache_sr;
#[doc = "ICACHE_IER (rw) register accessor: ICACHE interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icache_ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icache_ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache_ier`]
module"]
pub type ICACHE_IER = crate::Reg<icache_ier::ICACHE_IERrs>;
#[doc = "ICACHE interrupt enable register"]
pub mod icache_ier;
#[doc = "ICACHE_FCR (w) register accessor: ICACHE flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icache_fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache_fcr`]
module"]
pub type ICACHE_FCR = crate::Reg<icache_fcr::ICACHE_FCRrs>;
#[doc = "ICACHE flag clear register"]
pub mod icache_fcr;
#[doc = "ICACHE_HMONR (r) register accessor: ICACHE hit monitor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icache_hmonr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache_hmonr`]
module"]
pub type ICACHE_HMONR = crate::Reg<icache_hmonr::ICACHE_HMONRrs>;
#[doc = "ICACHE hit monitor register"]
pub mod icache_hmonr;
#[doc = "ICACHE_MMONR (r) register accessor: ICACHE miss monitor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icache_mmonr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache_mmonr`]
module"]
pub type ICACHE_MMONR = crate::Reg<icache_mmonr::ICACHE_MMONRrs>;
#[doc = "ICACHE miss monitor register"]
pub mod icache_mmonr;
#[doc = "ICACHE_CRR0 (rw) register accessor: ICACHE region configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icache_crr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icache_crr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache_crr0`]
module"]
pub type ICACHE_CRR0 = crate::Reg<icache_crr0::ICACHE_CRR0rs>;
#[doc = "ICACHE region configuration register"]
pub mod icache_crr0;
#[doc = "ICACHE_CRR1 (rw) register accessor: ICACHE region configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icache_crr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icache_crr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache_crr1`]
module"]
pub type ICACHE_CRR1 = crate::Reg<icache_crr1::ICACHE_CRR1rs>;
#[doc = "ICACHE region configuration register"]
pub mod icache_crr1;
#[doc = "ICACHE_CRR2 (rw) register accessor: ICACHE region configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icache_crr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icache_crr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache_crr2`]
module"]
pub type ICACHE_CRR2 = crate::Reg<icache_crr2::ICACHE_CRR2rs>;
#[doc = "ICACHE region configuration register"]
pub mod icache_crr2;
#[doc = "ICACHE_CRR3 (rw) register accessor: ICACHE region configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icache_crr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icache_crr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache_crr3`]
module"]
pub type ICACHE_CRR3 = crate::Reg<icache_crr3::ICACHE_CRR3rs>;
#[doc = "ICACHE region configuration register"]
pub mod icache_crr3;

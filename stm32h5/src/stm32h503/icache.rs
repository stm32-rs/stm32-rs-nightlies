#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: CR,
    sr: SR,
    ier: IER,
    fcr: FCR,
    hmonr: HMONR,
    mmonr: MMONR,
}
impl RegisterBlock {
    #[doc = "0x00 - ICACHE control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x04 - ICACHE status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x08 - ICACHE interrupt enable register"]
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    #[doc = "0x0c - ICACHE flag clear register"]
    #[inline(always)]
    pub const fn fcr(&self) -> &FCR {
        &self.fcr
    }
    #[doc = "0x10 - ICACHE hit monitor register"]
    #[inline(always)]
    pub const fn hmonr(&self) -> &HMONR {
        &self.hmonr
    }
    #[doc = "0x14 - ICACHE miss monitor register"]
    #[inline(always)]
    pub const fn mmonr(&self) -> &MMONR {
        &self.mmonr
    }
}
#[doc = "CR (rw) register accessor: ICACHE control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "ICACHE control register"]
pub mod cr;
#[doc = "SR (r) register accessor: ICACHE status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SRrs>;
#[doc = "ICACHE status register"]
pub mod sr;
#[doc = "IER (rw) register accessor: ICACHE interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
pub type IER = crate::Reg<ier::IERrs>;
#[doc = "ICACHE interrupt enable register"]
pub mod ier;
#[doc = "FCR (w) register accessor: ICACHE flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcr`]
module"]
pub type FCR = crate::Reg<fcr::FCRrs>;
#[doc = "ICACHE flag clear register"]
pub mod fcr;
#[doc = "HMONR (r) register accessor: ICACHE hit monitor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hmonr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hmonr`]
module"]
pub type HMONR = crate::Reg<hmonr::HMONRrs>;
#[doc = "ICACHE hit monitor register"]
pub mod hmonr;
#[doc = "MMONR (r) register accessor: ICACHE miss monitor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmonr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmonr`]
module"]
pub type MMONR = crate::Reg<mmonr::MMONRrs>;
#[doc = "ICACHE miss monitor register"]
pub mod mmonr;

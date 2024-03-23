#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: CR,
    sr: SR,
    ier: IER,
    fcr: FCR,
    rhmonr: RHMONR,
    rmmonr: RMMONR,
    _reserved6: [u8; 0x08],
    whmonr: WHMONR,
    wmmonr: WMMONR,
    cmdrsaddrr: CMDRSADDRR,
    cmdreaddrr: CMDREADDRR,
}
impl RegisterBlock {
    #[doc = "0x00 - DCACHE control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x04 - DCACHE status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x08 - DCACHE interrupt enable register"]
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    #[doc = "0x0c - DCACHE flag clear register"]
    #[inline(always)]
    pub const fn fcr(&self) -> &FCR {
        &self.fcr
    }
    #[doc = "0x10 - DCACHE read-hit monitor register"]
    #[inline(always)]
    pub const fn rhmonr(&self) -> &RHMONR {
        &self.rhmonr
    }
    #[doc = "0x14 - DCACHE read-miss monitor register"]
    #[inline(always)]
    pub const fn rmmonr(&self) -> &RMMONR {
        &self.rmmonr
    }
    #[doc = "0x20 - DCACHE write-hit monitor register"]
    #[inline(always)]
    pub const fn whmonr(&self) -> &WHMONR {
        &self.whmonr
    }
    #[doc = "0x24 - DCACHE write-miss monitor register"]
    #[inline(always)]
    pub const fn wmmonr(&self) -> &WMMONR {
        &self.wmmonr
    }
    #[doc = "0x28 - DCACHE command range start address register"]
    #[inline(always)]
    pub const fn cmdrsaddrr(&self) -> &CMDRSADDRR {
        &self.cmdrsaddrr
    }
    #[doc = "0x2c - DCACHE command range end address register"]
    #[inline(always)]
    pub const fn cmdreaddrr(&self) -> &CMDREADDRR {
        &self.cmdreaddrr
    }
}
#[doc = "CR (rw) register accessor: DCACHE control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "DCACHE control register"]
pub mod cr;
#[doc = "SR (r) register accessor: DCACHE status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SRrs>;
#[doc = "DCACHE status register"]
pub mod sr;
#[doc = "IER (rw) register accessor: DCACHE interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
pub type IER = crate::Reg<ier::IERrs>;
#[doc = "DCACHE interrupt enable register"]
pub mod ier;
#[doc = "FCR (w) register accessor: DCACHE flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcr`]
module"]
pub type FCR = crate::Reg<fcr::FCRrs>;
#[doc = "DCACHE flag clear register"]
pub mod fcr;
#[doc = "RHMONR (r) register accessor: DCACHE read-hit monitor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rhmonr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rhmonr`]
module"]
pub type RHMONR = crate::Reg<rhmonr::RHMONRrs>;
#[doc = "DCACHE read-hit monitor register"]
pub mod rhmonr;
#[doc = "RMMONR (r) register accessor: DCACHE read-miss monitor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rmmonr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rmmonr`]
module"]
pub type RMMONR = crate::Reg<rmmonr::RMMONRrs>;
#[doc = "DCACHE read-miss monitor register"]
pub mod rmmonr;
#[doc = "WHMONR (r) register accessor: DCACHE write-hit monitor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`whmonr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@whmonr`]
module"]
pub type WHMONR = crate::Reg<whmonr::WHMONRrs>;
#[doc = "DCACHE write-hit monitor register"]
pub mod whmonr;
#[doc = "WMMONR (r) register accessor: DCACHE write-miss monitor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wmmonr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wmmonr`]
module"]
pub type WMMONR = crate::Reg<wmmonr::WMMONRrs>;
#[doc = "DCACHE write-miss monitor register"]
pub mod wmmonr;
#[doc = "CMDRSADDRR (rw) register accessor: DCACHE command range start address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdrsaddrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdrsaddrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdrsaddrr`]
module"]
pub type CMDRSADDRR = crate::Reg<cmdrsaddrr::CMDRSADDRRrs>;
#[doc = "DCACHE command range start address register"]
pub mod cmdrsaddrr;
#[doc = "CMDREADDRR (rw) register accessor: DCACHE command range end address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdreaddrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdreaddrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdreaddrr`]
module"]
pub type CMDREADDRR = crate::Reg<cmdreaddrr::CMDREADDRRrs>;
#[doc = "DCACHE command range end address register"]
pub mod cmdreaddrr;

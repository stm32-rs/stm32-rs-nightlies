#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: CR,
    sr: SR,
    ris: RIS,
    ier: IER,
    mis: MIS,
    icr: ICR,
    escr: ESCR,
    esur: ESUR,
    cwstrt: CWSTRT,
    cwsize: CWSIZE,
    dr: DR,
}
impl RegisterBlock {
    #[doc = "0x00 - control register 1"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x04 - status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x08 - raw interrupt status register"]
    #[inline(always)]
    pub const fn ris(&self) -> &RIS {
        &self.ris
    }
    #[doc = "0x0c - interrupt enable register"]
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    #[doc = "0x10 - masked interrupt status register"]
    #[inline(always)]
    pub const fn mis(&self) -> &MIS {
        &self.mis
    }
    #[doc = "0x14 - interrupt clear register"]
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    #[doc = "0x18 - embedded synchronization code register"]
    #[inline(always)]
    pub const fn escr(&self) -> &ESCR {
        &self.escr
    }
    #[doc = "0x1c - embedded synchronization unmask register"]
    #[inline(always)]
    pub const fn esur(&self) -> &ESUR {
        &self.esur
    }
    #[doc = "0x20 - crop window start"]
    #[inline(always)]
    pub const fn cwstrt(&self) -> &CWSTRT {
        &self.cwstrt
    }
    #[doc = "0x24 - crop window size"]
    #[inline(always)]
    pub const fn cwsize(&self) -> &CWSIZE {
        &self.cwsize
    }
    #[doc = "0x28 - data register"]
    #[inline(always)]
    pub const fn dr(&self) -> &DR {
        &self.dr
    }
}
#[doc = "CR (rw) register accessor: control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "control register 1"]
pub mod cr;
#[doc = "SR (r) register accessor: status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SRrs>;
#[doc = "status register"]
pub mod sr;
#[doc = "RIS (r) register accessor: raw interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ris`]
module"]
pub type RIS = crate::Reg<ris::RISrs>;
#[doc = "raw interrupt status register"]
pub mod ris;
#[doc = "IER (rw) register accessor: interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
pub type IER = crate::Reg<ier::IERrs>;
#[doc = "interrupt enable register"]
pub mod ier;
#[doc = "MIS (r) register accessor: masked interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mis`]
module"]
pub type MIS = crate::Reg<mis::MISrs>;
#[doc = "masked interrupt status register"]
pub mod mis;
#[doc = "ICR (w) register accessor: interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`]
module"]
pub type ICR = crate::Reg<icr::ICRrs>;
#[doc = "interrupt clear register"]
pub mod icr;
#[doc = "ESCR (rw) register accessor: embedded synchronization code register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`escr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`escr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@escr`]
module"]
pub type ESCR = crate::Reg<escr::ESCRrs>;
#[doc = "embedded synchronization code register"]
pub mod escr;
#[doc = "ESUR (rw) register accessor: embedded synchronization unmask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`esur::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`esur::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esur`]
module"]
pub type ESUR = crate::Reg<esur::ESURrs>;
#[doc = "embedded synchronization unmask register"]
pub mod esur;
#[doc = "CWSTRT (rw) register accessor: crop window start\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cwstrt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cwstrt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cwstrt`]
module"]
pub type CWSTRT = crate::Reg<cwstrt::CWSTRTrs>;
#[doc = "crop window start"]
pub mod cwstrt;
#[doc = "CWSIZE (rw) register accessor: crop window size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cwsize::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cwsize::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cwsize`]
module"]
pub type CWSIZE = crate::Reg<cwsize::CWSIZErs>;
#[doc = "crop window size"]
pub mod cwsize;
#[doc = "DR (r) register accessor: data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`]
module"]
pub type DR = crate::Reg<dr::DRrs>;
#[doc = "data register"]
pub mod dr;

#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: CR,
    sr: SR,
    ris: RIS,
    ier: IER,
    mis: MIS,
    icr: ICR,
    _reserved6: [u8; 0x10],
    dr: DR,
}
impl RegisterBlock {
    #[doc = "0x00 - PSSI control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x04 - PSSI status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x08 - PSSI raw interrupt status register"]
    #[inline(always)]
    pub const fn ris(&self) -> &RIS {
        &self.ris
    }
    #[doc = "0x0c - PSSI interrupt enable register"]
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    #[doc = "0x10 - PSSI masked interrupt status register"]
    #[inline(always)]
    pub const fn mis(&self) -> &MIS {
        &self.mis
    }
    #[doc = "0x14 - PSSI interrupt clear register"]
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    #[doc = "0x28 - PSSI data register"]
    #[inline(always)]
    pub const fn dr(&self) -> &DR {
        &self.dr
    }
}
#[doc = "CR (rw) register accessor: PSSI control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "PSSI control register"]
pub mod cr;
#[doc = "SR (r) register accessor: PSSI status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SRrs>;
#[doc = "PSSI status register"]
pub mod sr;
#[doc = "RIS (r) register accessor: PSSI raw interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ris`]
module"]
pub type RIS = crate::Reg<ris::RISrs>;
#[doc = "PSSI raw interrupt status register"]
pub mod ris;
#[doc = "IER (rw) register accessor: PSSI interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
pub type IER = crate::Reg<ier::IERrs>;
#[doc = "PSSI interrupt enable register"]
pub mod ier;
#[doc = "MIS (r) register accessor: PSSI masked interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mis`]
module"]
pub type MIS = crate::Reg<mis::MISrs>;
#[doc = "PSSI masked interrupt status register"]
pub mod mis;
#[doc = "ICR (w) register accessor: PSSI interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`]
module"]
pub type ICR = crate::Reg<icr::ICRrs>;
#[doc = "PSSI interrupt clear register"]
pub mod icr;
#[doc = "DR (rw) register accessor: PSSI data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`]
module"]
pub type DR = crate::Reg<dr::DRrs>;
#[doc = "PSSI data register"]
pub mod dr;

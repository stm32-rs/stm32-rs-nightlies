#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: CR,
    brr: BRR,
    _reserved2: [u8; 0x04],
    isr: ISR,
    icr: ICR,
    ier: IER,
    rfl: RFL,
    tdr: TDR,
    rdr: RDR,
    or: OR,
}
impl RegisterBlock {
    #[doc = "0x00 - SWPMI Configuration/Control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x04 - SWPMI Bitrate register"]
    #[inline(always)]
    pub const fn brr(&self) -> &BRR {
        &self.brr
    }
    #[doc = "0x0c - SWPMI Interrupt and Status register"]
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    #[doc = "0x10 - SWPMI Interrupt Flag Clear register"]
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    #[doc = "0x14 - SWPMI Interrupt Enable register"]
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    #[doc = "0x18 - SWPMI Receive Frame Length register"]
    #[inline(always)]
    pub const fn rfl(&self) -> &RFL {
        &self.rfl
    }
    #[doc = "0x1c - SWPMI Transmit data register"]
    #[inline(always)]
    pub const fn tdr(&self) -> &TDR {
        &self.tdr
    }
    #[doc = "0x20 - SWPMI Receive data register"]
    #[inline(always)]
    pub const fn rdr(&self) -> &RDR {
        &self.rdr
    }
    #[doc = "0x24 - SWPMI Option register"]
    #[inline(always)]
    pub const fn or(&self) -> &OR {
        &self.or
    }
}
#[doc = "CR (rw) register accessor: SWPMI Configuration/Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "SWPMI Configuration/Control register"]
pub mod cr;
#[doc = "BRR (rw) register accessor: SWPMI Bitrate register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brr`]
module"]
pub type BRR = crate::Reg<brr::BRRrs>;
#[doc = "SWPMI Bitrate register"]
pub mod brr;
#[doc = "ISR (r) register accessor: SWPMI Interrupt and Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
pub type ISR = crate::Reg<isr::ISRrs>;
#[doc = "SWPMI Interrupt and Status register"]
pub mod isr;
#[doc = "ICR (w) register accessor: SWPMI Interrupt Flag Clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`]
module"]
pub type ICR = crate::Reg<icr::ICRrs>;
#[doc = "SWPMI Interrupt Flag Clear register"]
pub mod icr;
#[doc = "IER (rw) register accessor: SWPMI Interrupt Enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
pub type IER = crate::Reg<ier::IERrs>;
#[doc = "SWPMI Interrupt Enable register"]
pub mod ier;
#[doc = "RFL (r) register accessor: SWPMI Receive Frame Length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfl::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfl`]
module"]
pub type RFL = crate::Reg<rfl::RFLrs>;
#[doc = "SWPMI Receive Frame Length register"]
pub mod rfl;
#[doc = "TDR (w) register accessor: SWPMI Transmit data register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdr`]
module"]
pub type TDR = crate::Reg<tdr::TDRrs>;
#[doc = "SWPMI Transmit data register"]
pub mod tdr;
#[doc = "RDR (r) register accessor: SWPMI Receive data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdr`]
module"]
pub type RDR = crate::Reg<rdr::RDRrs>;
#[doc = "SWPMI Receive data register"]
pub mod rdr;
#[doc = "OR (rw) register accessor: SWPMI Option register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`or::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`or::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@or`]
module"]
pub type OR = crate::Reg<or::ORrs>;
#[doc = "SWPMI Option register"]
pub mod or;

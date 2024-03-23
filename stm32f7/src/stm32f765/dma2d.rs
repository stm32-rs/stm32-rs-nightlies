#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: CR,
    isr: ISR,
    ifcr: IFCR,
    fgmar: FGMAR,
    fgor: FGOR,
    bgmar: BGMAR,
    bgor: BGOR,
    fgpfccr: FGPFCCR,
    fgcolr: FGCOLR,
    bgpfccr: BGPFCCR,
    bgcolr: BGCOLR,
    fgcmar: FGCMAR,
    bgcmar: BGCMAR,
    opfccr: OPFCCR,
    ocolr: OCOLR,
    omar: OMAR,
    oor: OOR,
    nlr: NLR,
    lwr: LWR,
    amtcr: AMTCR,
    _reserved20: [u8; 0x03b0],
    fgclut: FGCLUT,
    _reserved21: [u8; 0x03fc],
    bgclut: BGCLUT,
}
impl RegisterBlock {
    #[doc = "0x00 - control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x04 - Interrupt Status Register"]
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    #[doc = "0x08 - interrupt flag clear register"]
    #[inline(always)]
    pub const fn ifcr(&self) -> &IFCR {
        &self.ifcr
    }
    #[doc = "0x0c - foreground memory address register"]
    #[inline(always)]
    pub const fn fgmar(&self) -> &FGMAR {
        &self.fgmar
    }
    #[doc = "0x10 - foreground offset register"]
    #[inline(always)]
    pub const fn fgor(&self) -> &FGOR {
        &self.fgor
    }
    #[doc = "0x14 - background memory address register"]
    #[inline(always)]
    pub const fn bgmar(&self) -> &BGMAR {
        &self.bgmar
    }
    #[doc = "0x18 - background offset register"]
    #[inline(always)]
    pub const fn bgor(&self) -> &BGOR {
        &self.bgor
    }
    #[doc = "0x1c - foreground PFC control register"]
    #[inline(always)]
    pub const fn fgpfccr(&self) -> &FGPFCCR {
        &self.fgpfccr
    }
    #[doc = "0x20 - foreground color register"]
    #[inline(always)]
    pub const fn fgcolr(&self) -> &FGCOLR {
        &self.fgcolr
    }
    #[doc = "0x24 - background PFC control register"]
    #[inline(always)]
    pub const fn bgpfccr(&self) -> &BGPFCCR {
        &self.bgpfccr
    }
    #[doc = "0x28 - background color register"]
    #[inline(always)]
    pub const fn bgcolr(&self) -> &BGCOLR {
        &self.bgcolr
    }
    #[doc = "0x2c - foreground CLUT memory address register"]
    #[inline(always)]
    pub const fn fgcmar(&self) -> &FGCMAR {
        &self.fgcmar
    }
    #[doc = "0x30 - background CLUT memory address register"]
    #[inline(always)]
    pub const fn bgcmar(&self) -> &BGCMAR {
        &self.bgcmar
    }
    #[doc = "0x34 - output PFC control register"]
    #[inline(always)]
    pub const fn opfccr(&self) -> &OPFCCR {
        &self.opfccr
    }
    #[doc = "0x38 - output color register"]
    #[inline(always)]
    pub const fn ocolr(&self) -> &OCOLR {
        &self.ocolr
    }
    #[doc = "0x3c - output memory address register"]
    #[inline(always)]
    pub const fn omar(&self) -> &OMAR {
        &self.omar
    }
    #[doc = "0x40 - output offset register"]
    #[inline(always)]
    pub const fn oor(&self) -> &OOR {
        &self.oor
    }
    #[doc = "0x44 - number of line register"]
    #[inline(always)]
    pub const fn nlr(&self) -> &NLR {
        &self.nlr
    }
    #[doc = "0x48 - line watermark register"]
    #[inline(always)]
    pub const fn lwr(&self) -> &LWR {
        &self.lwr
    }
    #[doc = "0x4c - AHB master timer configuration register"]
    #[inline(always)]
    pub const fn amtcr(&self) -> &AMTCR {
        &self.amtcr
    }
    #[doc = "0x400 - FGCLUT"]
    #[inline(always)]
    pub const fn fgclut(&self) -> &FGCLUT {
        &self.fgclut
    }
    #[doc = "0x800 - BGCLUT"]
    #[inline(always)]
    pub const fn bgclut(&self) -> &BGCLUT {
        &self.bgclut
    }
}
#[doc = "CR (rw) register accessor: control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "control register"]
pub mod cr;
#[doc = "ISR (r) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
pub type ISR = crate::Reg<isr::ISRrs>;
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "IFCR (rw) register accessor: interrupt flag clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ifcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifcr`]
module"]
pub type IFCR = crate::Reg<ifcr::IFCRrs>;
#[doc = "interrupt flag clear register"]
pub mod ifcr;
#[doc = "FGMAR (rw) register accessor: foreground memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fgmar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fgmar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fgmar`]
module"]
pub type FGMAR = crate::Reg<fgmar::FGMARrs>;
#[doc = "foreground memory address register"]
pub mod fgmar;
#[doc = "FGOR (rw) register accessor: foreground offset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fgor::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fgor::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fgor`]
module"]
pub type FGOR = crate::Reg<fgor::FGORrs>;
#[doc = "foreground offset register"]
pub mod fgor;
#[doc = "BGMAR (rw) register accessor: background memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bgmar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bgmar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bgmar`]
module"]
pub type BGMAR = crate::Reg<bgmar::BGMARrs>;
#[doc = "background memory address register"]
pub mod bgmar;
#[doc = "BGOR (rw) register accessor: background offset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bgor::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bgor::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bgor`]
module"]
pub type BGOR = crate::Reg<bgor::BGORrs>;
#[doc = "background offset register"]
pub mod bgor;
#[doc = "FGPFCCR (rw) register accessor: foreground PFC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fgpfccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fgpfccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fgpfccr`]
module"]
pub type FGPFCCR = crate::Reg<fgpfccr::FGPFCCRrs>;
#[doc = "foreground PFC control register"]
pub mod fgpfccr;
#[doc = "FGCOLR (rw) register accessor: foreground color register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fgcolr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fgcolr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fgcolr`]
module"]
pub type FGCOLR = crate::Reg<fgcolr::FGCOLRrs>;
#[doc = "foreground color register"]
pub mod fgcolr;
#[doc = "BGPFCCR (rw) register accessor: background PFC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bgpfccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bgpfccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bgpfccr`]
module"]
pub type BGPFCCR = crate::Reg<bgpfccr::BGPFCCRrs>;
#[doc = "background PFC control register"]
pub mod bgpfccr;
#[doc = "BGCOLR (rw) register accessor: background color register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bgcolr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bgcolr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bgcolr`]
module"]
pub type BGCOLR = crate::Reg<bgcolr::BGCOLRrs>;
#[doc = "background color register"]
pub mod bgcolr;
#[doc = "FGCMAR (rw) register accessor: foreground CLUT memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fgcmar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fgcmar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fgcmar`]
module"]
pub type FGCMAR = crate::Reg<fgcmar::FGCMARrs>;
#[doc = "foreground CLUT memory address register"]
pub mod fgcmar;
#[doc = "BGCMAR (rw) register accessor: background CLUT memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bgcmar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bgcmar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bgcmar`]
module"]
pub type BGCMAR = crate::Reg<bgcmar::BGCMARrs>;
#[doc = "background CLUT memory address register"]
pub mod bgcmar;
#[doc = "OPFCCR (rw) register accessor: output PFC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opfccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opfccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opfccr`]
module"]
pub type OPFCCR = crate::Reg<opfccr::OPFCCRrs>;
#[doc = "output PFC control register"]
pub mod opfccr;
#[doc = "OCOLR (rw) register accessor: output color register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocolr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocolr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocolr`]
module"]
pub type OCOLR = crate::Reg<ocolr::OCOLRrs>;
#[doc = "output color register"]
pub mod ocolr;
#[doc = "OMAR (rw) register accessor: output memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`omar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`omar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@omar`]
module"]
pub type OMAR = crate::Reg<omar::OMARrs>;
#[doc = "output memory address register"]
pub mod omar;
#[doc = "OOR (rw) register accessor: output offset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oor::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oor::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oor`]
module"]
pub type OOR = crate::Reg<oor::OORrs>;
#[doc = "output offset register"]
pub mod oor;
#[doc = "NLR (rw) register accessor: number of line register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nlr`]
module"]
pub type NLR = crate::Reg<nlr::NLRrs>;
#[doc = "number of line register"]
pub mod nlr;
#[doc = "LWR (rw) register accessor: line watermark register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lwr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lwr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lwr`]
module"]
pub type LWR = crate::Reg<lwr::LWRrs>;
#[doc = "line watermark register"]
pub mod lwr;
#[doc = "AMTCR (rw) register accessor: AHB master timer configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`amtcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`amtcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@amtcr`]
module"]
pub type AMTCR = crate::Reg<amtcr::AMTCRrs>;
#[doc = "AHB master timer configuration register"]
pub mod amtcr;
#[doc = "FGCLUT (rw) register accessor: FGCLUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fgclut::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fgclut::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fgclut`]
module"]
pub type FGCLUT = crate::Reg<fgclut::FGCLUTrs>;
#[doc = "FGCLUT"]
pub mod fgclut;
#[doc = "BGCLUT (rw) register accessor: BGCLUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bgclut::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bgclut::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bgclut`]
module"]
pub type BGCLUT = crate::Reg<bgclut::BGCLUTrs>;
#[doc = "BGCLUT"]
pub mod bgclut;

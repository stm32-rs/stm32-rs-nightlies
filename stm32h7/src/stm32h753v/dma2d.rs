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
}
impl RegisterBlock {
    #[doc = "0x00 - DMA2D control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x04 - DMA2D Interrupt Status Register"]
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    #[doc = "0x08 - DMA2D interrupt flag clear register"]
    #[inline(always)]
    pub const fn ifcr(&self) -> &IFCR {
        &self.ifcr
    }
    #[doc = "0x0c - DMA2D foreground memory address register"]
    #[inline(always)]
    pub const fn fgmar(&self) -> &FGMAR {
        &self.fgmar
    }
    #[doc = "0x10 - DMA2D foreground offset register"]
    #[inline(always)]
    pub const fn fgor(&self) -> &FGOR {
        &self.fgor
    }
    #[doc = "0x14 - DMA2D background memory address register"]
    #[inline(always)]
    pub const fn bgmar(&self) -> &BGMAR {
        &self.bgmar
    }
    #[doc = "0x18 - DMA2D background offset register"]
    #[inline(always)]
    pub const fn bgor(&self) -> &BGOR {
        &self.bgor
    }
    #[doc = "0x1c - DMA2D foreground PFC control register"]
    #[inline(always)]
    pub const fn fgpfccr(&self) -> &FGPFCCR {
        &self.fgpfccr
    }
    #[doc = "0x20 - DMA2D foreground color register"]
    #[inline(always)]
    pub const fn fgcolr(&self) -> &FGCOLR {
        &self.fgcolr
    }
    #[doc = "0x24 - DMA2D background PFC control register"]
    #[inline(always)]
    pub const fn bgpfccr(&self) -> &BGPFCCR {
        &self.bgpfccr
    }
    #[doc = "0x28 - DMA2D background color register"]
    #[inline(always)]
    pub const fn bgcolr(&self) -> &BGCOLR {
        &self.bgcolr
    }
    #[doc = "0x2c - DMA2D foreground CLUT memory address register"]
    #[inline(always)]
    pub const fn fgcmar(&self) -> &FGCMAR {
        &self.fgcmar
    }
    #[doc = "0x30 - DMA2D background CLUT memory address register"]
    #[inline(always)]
    pub const fn bgcmar(&self) -> &BGCMAR {
        &self.bgcmar
    }
    #[doc = "0x34 - DMA2D output PFC control register"]
    #[inline(always)]
    pub const fn opfccr(&self) -> &OPFCCR {
        &self.opfccr
    }
    #[doc = "0x38 - DMA2D output color register"]
    #[inline(always)]
    pub const fn ocolr(&self) -> &OCOLR {
        &self.ocolr
    }
    #[doc = "0x3c - DMA2D output memory address register"]
    #[inline(always)]
    pub const fn omar(&self) -> &OMAR {
        &self.omar
    }
    #[doc = "0x40 - DMA2D output offset register"]
    #[inline(always)]
    pub const fn oor(&self) -> &OOR {
        &self.oor
    }
    #[doc = "0x44 - DMA2D number of line register"]
    #[inline(always)]
    pub const fn nlr(&self) -> &NLR {
        &self.nlr
    }
    #[doc = "0x48 - DMA2D line watermark register"]
    #[inline(always)]
    pub const fn lwr(&self) -> &LWR {
        &self.lwr
    }
    #[doc = "0x4c - DMA2D AXI master timer configuration register"]
    #[inline(always)]
    pub const fn amtcr(&self) -> &AMTCR {
        &self.amtcr
    }
}
#[doc = "CR (rw) register accessor: DMA2D control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "DMA2D control register"]
pub mod cr;
#[doc = "ISR (r) register accessor: DMA2D Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
pub type ISR = crate::Reg<isr::ISRrs>;
#[doc = "DMA2D Interrupt Status Register"]
pub mod isr;
#[doc = "IFCR (rw) register accessor: DMA2D interrupt flag clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ifcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifcr`]
module"]
pub type IFCR = crate::Reg<ifcr::IFCRrs>;
#[doc = "DMA2D interrupt flag clear register"]
pub mod ifcr;
#[doc = "FGMAR (rw) register accessor: DMA2D foreground memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fgmar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fgmar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fgmar`]
module"]
pub type FGMAR = crate::Reg<fgmar::FGMARrs>;
#[doc = "DMA2D foreground memory address register"]
pub mod fgmar;
#[doc = "FGOR (rw) register accessor: DMA2D foreground offset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fgor::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fgor::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fgor`]
module"]
pub type FGOR = crate::Reg<fgor::FGORrs>;
#[doc = "DMA2D foreground offset register"]
pub mod fgor;
#[doc = "BGMAR (rw) register accessor: DMA2D background memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bgmar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bgmar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bgmar`]
module"]
pub type BGMAR = crate::Reg<bgmar::BGMARrs>;
#[doc = "DMA2D background memory address register"]
pub mod bgmar;
#[doc = "BGOR (rw) register accessor: DMA2D background offset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bgor::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bgor::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bgor`]
module"]
pub type BGOR = crate::Reg<bgor::BGORrs>;
#[doc = "DMA2D background offset register"]
pub mod bgor;
#[doc = "FGPFCCR (rw) register accessor: DMA2D foreground PFC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fgpfccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fgpfccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fgpfccr`]
module"]
pub type FGPFCCR = crate::Reg<fgpfccr::FGPFCCRrs>;
#[doc = "DMA2D foreground PFC control register"]
pub mod fgpfccr;
#[doc = "FGCOLR (rw) register accessor: DMA2D foreground color register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fgcolr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fgcolr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fgcolr`]
module"]
pub type FGCOLR = crate::Reg<fgcolr::FGCOLRrs>;
#[doc = "DMA2D foreground color register"]
pub mod fgcolr;
#[doc = "BGPFCCR (rw) register accessor: DMA2D background PFC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bgpfccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bgpfccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bgpfccr`]
module"]
pub type BGPFCCR = crate::Reg<bgpfccr::BGPFCCRrs>;
#[doc = "DMA2D background PFC control register"]
pub mod bgpfccr;
#[doc = "BGCOLR (rw) register accessor: DMA2D background color register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bgcolr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bgcolr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bgcolr`]
module"]
pub type BGCOLR = crate::Reg<bgcolr::BGCOLRrs>;
#[doc = "DMA2D background color register"]
pub mod bgcolr;
#[doc = "FGCMAR (rw) register accessor: DMA2D foreground CLUT memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fgcmar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fgcmar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fgcmar`]
module"]
pub type FGCMAR = crate::Reg<fgcmar::FGCMARrs>;
#[doc = "DMA2D foreground CLUT memory address register"]
pub mod fgcmar;
#[doc = "BGCMAR (rw) register accessor: DMA2D background CLUT memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bgcmar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bgcmar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bgcmar`]
module"]
pub type BGCMAR = crate::Reg<bgcmar::BGCMARrs>;
#[doc = "DMA2D background CLUT memory address register"]
pub mod bgcmar;
#[doc = "OPFCCR (rw) register accessor: DMA2D output PFC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opfccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opfccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opfccr`]
module"]
pub type OPFCCR = crate::Reg<opfccr::OPFCCRrs>;
#[doc = "DMA2D output PFC control register"]
pub mod opfccr;
#[doc = "OCOLR (rw) register accessor: DMA2D output color register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocolr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocolr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocolr`]
module"]
pub type OCOLR = crate::Reg<ocolr::OCOLRrs>;
#[doc = "DMA2D output color register"]
pub mod ocolr;
#[doc = "OMAR (rw) register accessor: DMA2D output memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`omar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`omar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@omar`]
module"]
pub type OMAR = crate::Reg<omar::OMARrs>;
#[doc = "DMA2D output memory address register"]
pub mod omar;
#[doc = "OOR (rw) register accessor: DMA2D output offset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oor::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oor::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oor`]
module"]
pub type OOR = crate::Reg<oor::OORrs>;
#[doc = "DMA2D output offset register"]
pub mod oor;
#[doc = "NLR (rw) register accessor: DMA2D number of line register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nlr`]
module"]
pub type NLR = crate::Reg<nlr::NLRrs>;
#[doc = "DMA2D number of line register"]
pub mod nlr;
#[doc = "LWR (rw) register accessor: DMA2D line watermark register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lwr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lwr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lwr`]
module"]
pub type LWR = crate::Reg<lwr::LWRrs>;
#[doc = "DMA2D line watermark register"]
pub mod lwr;
#[doc = "AMTCR (rw) register accessor: DMA2D AXI master timer configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`amtcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`amtcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@amtcr`]
module"]
pub type AMTCR = crate::Reg<amtcr::AMTCRrs>;
#[doc = "DMA2D AXI master timer configuration register"]
pub mod amtcr;

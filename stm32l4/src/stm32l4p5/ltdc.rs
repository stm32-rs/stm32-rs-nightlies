#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    sscr: SSCR,
    bpcr: BPCR,
    awcr: AWCR,
    twcr: TWCR,
    gcr: GCR,
    _reserved5: [u8; 0x08],
    srcr: SRCR,
    _reserved6: [u8; 0x04],
    bccr: BCCR,
    _reserved7: [u8; 0x04],
    ier: IER,
    isr: ISR,
    icr: ICR,
    lipcr: LIPCR,
    cpsr: CPSR,
    cdsr: CDSR,
    _reserved13: [u8; 0x38],
    l1cr: L1CR,
    l1whpcr: L1WHPCR,
    l1wvpcr: L1WVPCR,
    l1ckcr: L1CKCR,
    l1pfcr: L1PFCR,
    l1cacr: L1CACR,
    l1dccr: L1DCCR,
    l1bfcr: L1BFCR,
    _reserved21: [u8; 0x08],
    l1cfbar: L1CFBAR,
    l1cfblr: L1CFBLR,
    l1cfblnr: L1CFBLNR,
    _reserved24: [u8; 0x0c],
    l1clutwr: L1CLUTWR,
    _reserved25: [u8; 0x3c],
    l2cr: L2CR,
    l2whpcr: L2WHPCR,
    l2wvpcr: L2WVPCR,
    l2ckcr: L2CKCR,
    l2pfcr: L2PFCR,
    l2cacr: L2CACR,
    l2dccr: L2DCCR,
    _reserved32: [u8; 0x04],
    l2bfcr: L2BFCR,
    _reserved33: [u8; 0x04],
    l2cfbar: L2CFBAR,
    l2cfblr: L2CFBLR,
    l2cfblnr: L2CFBLNR,
    _reserved36: [u8; 0x0c],
    l2clutwr: L2CLUTWR,
}
impl RegisterBlock {
    #[doc = "0x08 - LTDC Synchronization Size Configuration Register"]
    #[inline(always)]
    pub const fn sscr(&self) -> &SSCR {
        &self.sscr
    }
    #[doc = "0x0c - LTDC Back Porch Configuration Register"]
    #[inline(always)]
    pub const fn bpcr(&self) -> &BPCR {
        &self.bpcr
    }
    #[doc = "0x10 - LTDC Active Width Configuration Register"]
    #[inline(always)]
    pub const fn awcr(&self) -> &AWCR {
        &self.awcr
    }
    #[doc = "0x14 - LTDC Total Width Configuration Register"]
    #[inline(always)]
    pub const fn twcr(&self) -> &TWCR {
        &self.twcr
    }
    #[doc = "0x18 - LTDC Global Control Register"]
    #[inline(always)]
    pub const fn gcr(&self) -> &GCR {
        &self.gcr
    }
    #[doc = "0x24 - LTDC Shadow Reload Configuration Register"]
    #[inline(always)]
    pub const fn srcr(&self) -> &SRCR {
        &self.srcr
    }
    #[doc = "0x2c - LTDC Background Color Configuration Register"]
    #[inline(always)]
    pub const fn bccr(&self) -> &BCCR {
        &self.bccr
    }
    #[doc = "0x34 - LTDC Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    #[doc = "0x38 - LTDC Interrupt Status Register"]
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    #[doc = "0x3c - LTDC Interrupt Clear Register"]
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    #[doc = "0x40 - LTDC Line Interrupt Position Configuration Register"]
    #[inline(always)]
    pub const fn lipcr(&self) -> &LIPCR {
        &self.lipcr
    }
    #[doc = "0x44 - LTDC Current Position Status Register"]
    #[inline(always)]
    pub const fn cpsr(&self) -> &CPSR {
        &self.cpsr
    }
    #[doc = "0x48 - LTDC Current Display Status Register"]
    #[inline(always)]
    pub const fn cdsr(&self) -> &CDSR {
        &self.cdsr
    }
    #[doc = "0x84 - LTDC Layer Control Register"]
    #[inline(always)]
    pub const fn l1cr(&self) -> &L1CR {
        &self.l1cr
    }
    #[doc = "0x88 - LTDC Layer Window Horizontal Position Configuration Register"]
    #[inline(always)]
    pub const fn l1whpcr(&self) -> &L1WHPCR {
        &self.l1whpcr
    }
    #[doc = "0x8c - LTDC Layer Window Vertical Position Configuration Register"]
    #[inline(always)]
    pub const fn l1wvpcr(&self) -> &L1WVPCR {
        &self.l1wvpcr
    }
    #[doc = "0x90 - LTDC Layer Color Keying Configuration Register"]
    #[inline(always)]
    pub const fn l1ckcr(&self) -> &L1CKCR {
        &self.l1ckcr
    }
    #[doc = "0x94 - LTDC Layer Pixel Format Configuration Register"]
    #[inline(always)]
    pub const fn l1pfcr(&self) -> &L1PFCR {
        &self.l1pfcr
    }
    #[doc = "0x98 - LTDC Layer Constant Alpha Configuration Register"]
    #[inline(always)]
    pub const fn l1cacr(&self) -> &L1CACR {
        &self.l1cacr
    }
    #[doc = "0x9c - LTDC Layer Default Color Configuration Register"]
    #[inline(always)]
    pub const fn l1dccr(&self) -> &L1DCCR {
        &self.l1dccr
    }
    #[doc = "0xa0 - LTDC Layer Blending Factors Configuration Register"]
    #[inline(always)]
    pub const fn l1bfcr(&self) -> &L1BFCR {
        &self.l1bfcr
    }
    #[doc = "0xac - LTDC Layer Color Frame Buffer Address Register"]
    #[inline(always)]
    pub const fn l1cfbar(&self) -> &L1CFBAR {
        &self.l1cfbar
    }
    #[doc = "0xb0 - LTDC Layer Color Frame Buffer Length Register"]
    #[inline(always)]
    pub const fn l1cfblr(&self) -> &L1CFBLR {
        &self.l1cfblr
    }
    #[doc = "0xb4 - LTDC Layer ColorFrame Buffer Line Number Register"]
    #[inline(always)]
    pub const fn l1cfblnr(&self) -> &L1CFBLNR {
        &self.l1cfblnr
    }
    #[doc = "0xc4 - LTDC Layerx CLUT Write Register"]
    #[inline(always)]
    pub const fn l1clutwr(&self) -> &L1CLUTWR {
        &self.l1clutwr
    }
    #[doc = "0x104 - LTDC Layer Control Register"]
    #[inline(always)]
    pub const fn l2cr(&self) -> &L2CR {
        &self.l2cr
    }
    #[doc = "0x108 - LTDC Layerx Window Horizontal Position Configuration Register"]
    #[inline(always)]
    pub const fn l2whpcr(&self) -> &L2WHPCR {
        &self.l2whpcr
    }
    #[doc = "0x10c - LTDC Layer Window Vertical Position Configuration Register"]
    #[inline(always)]
    pub const fn l2wvpcr(&self) -> &L2WVPCR {
        &self.l2wvpcr
    }
    #[doc = "0x110 - LTDC Layer Color Keying Configuration Register"]
    #[inline(always)]
    pub const fn l2ckcr(&self) -> &L2CKCR {
        &self.l2ckcr
    }
    #[doc = "0x114 - LTDC Layer Pixel Format Configuration Register"]
    #[inline(always)]
    pub const fn l2pfcr(&self) -> &L2PFCR {
        &self.l2pfcr
    }
    #[doc = "0x118 - LTDC Layer Constant Alpha Configuration Register"]
    #[inline(always)]
    pub const fn l2cacr(&self) -> &L2CACR {
        &self.l2cacr
    }
    #[doc = "0x11c - LTDC Layer Default Color Configuration Register"]
    #[inline(always)]
    pub const fn l2dccr(&self) -> &L2DCCR {
        &self.l2dccr
    }
    #[doc = "0x124 - LTDC Layer Blending Factors Configuration Register"]
    #[inline(always)]
    pub const fn l2bfcr(&self) -> &L2BFCR {
        &self.l2bfcr
    }
    #[doc = "0x12c - LTDC Layer Color Frame Buffer Address Register"]
    #[inline(always)]
    pub const fn l2cfbar(&self) -> &L2CFBAR {
        &self.l2cfbar
    }
    #[doc = "0x130 - LTDC Layer Color Frame Buffer Length Register"]
    #[inline(always)]
    pub const fn l2cfblr(&self) -> &L2CFBLR {
        &self.l2cfblr
    }
    #[doc = "0x134 - LTDC Layer ColorFrame Buffer Line Number Register"]
    #[inline(always)]
    pub const fn l2cfblnr(&self) -> &L2CFBLNR {
        &self.l2cfblnr
    }
    #[doc = "0x144 - LTDC Layerx CLUT Write Register"]
    #[inline(always)]
    pub const fn l2clutwr(&self) -> &L2CLUTWR {
        &self.l2clutwr
    }
}
#[doc = "SSCR (rw) register accessor: LTDC Synchronization Size Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sscr`]
module"]
pub type SSCR = crate::Reg<sscr::SSCRrs>;
#[doc = "LTDC Synchronization Size Configuration Register"]
pub mod sscr;
#[doc = "BPCR (rw) register accessor: LTDC Back Porch Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bpcr`]
module"]
pub type BPCR = crate::Reg<bpcr::BPCRrs>;
#[doc = "LTDC Back Porch Configuration Register"]
pub mod bpcr;
#[doc = "AWCR (rw) register accessor: LTDC Active Width Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awcr`]
module"]
pub type AWCR = crate::Reg<awcr::AWCRrs>;
#[doc = "LTDC Active Width Configuration Register"]
pub mod awcr;
#[doc = "TWCR (rw) register accessor: LTDC Total Width Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@twcr`]
module"]
pub type TWCR = crate::Reg<twcr::TWCRrs>;
#[doc = "LTDC Total Width Configuration Register"]
pub mod twcr;
#[doc = "GCR (rw) register accessor: LTDC Global Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gcr`]
module"]
pub type GCR = crate::Reg<gcr::GCRrs>;
#[doc = "LTDC Global Control Register"]
pub mod gcr;
#[doc = "SRCR (rw) register accessor: LTDC Shadow Reload Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcr`]
module"]
pub type SRCR = crate::Reg<srcr::SRCRrs>;
#[doc = "LTDC Shadow Reload Configuration Register"]
pub mod srcr;
#[doc = "BCCR (rw) register accessor: LTDC Background Color Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bccr`]
module"]
pub type BCCR = crate::Reg<bccr::BCCRrs>;
#[doc = "LTDC Background Color Configuration Register"]
pub mod bccr;
#[doc = "IER (rw) register accessor: LTDC Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
pub type IER = crate::Reg<ier::IERrs>;
#[doc = "LTDC Interrupt Enable Register"]
pub mod ier;
#[doc = "ISR (r) register accessor: LTDC Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
pub type ISR = crate::Reg<isr::ISRrs>;
#[doc = "LTDC Interrupt Status Register"]
pub mod isr;
#[doc = "ICR (w) register accessor: LTDC Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`]
module"]
pub type ICR = crate::Reg<icr::ICRrs>;
#[doc = "LTDC Interrupt Clear Register"]
pub mod icr;
#[doc = "LIPCR (rw) register accessor: LTDC Line Interrupt Position Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lipcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lipcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lipcr`]
module"]
pub type LIPCR = crate::Reg<lipcr::LIPCRrs>;
#[doc = "LTDC Line Interrupt Position Configuration Register"]
pub mod lipcr;
#[doc = "CPSR (r) register accessor: LTDC Current Position Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsr`]
module"]
pub type CPSR = crate::Reg<cpsr::CPSRrs>;
#[doc = "LTDC Current Position Status Register"]
pub mod cpsr;
#[doc = "CDSR (r) register accessor: LTDC Current Display Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdsr`]
module"]
pub type CDSR = crate::Reg<cdsr::CDSRrs>;
#[doc = "LTDC Current Display Status Register"]
pub mod cdsr;
#[doc = "L1CR (rw) register accessor: LTDC Layer Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l1cr`]
module"]
pub type L1CR = crate::Reg<l1cr::L1CRrs>;
#[doc = "LTDC Layer Control Register"]
pub mod l1cr;
#[doc = "L2CR (rw) register accessor: LTDC Layer Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l2cr`]
module"]
pub type L2CR = crate::Reg<l2cr::L2CRrs>;
#[doc = "LTDC Layer Control Register"]
pub mod l2cr;
#[doc = "L1WHPCR (rw) register accessor: LTDC Layer Window Horizontal Position Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1whpcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1whpcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l1whpcr`]
module"]
pub type L1WHPCR = crate::Reg<l1whpcr::L1WHPCRrs>;
#[doc = "LTDC Layer Window Horizontal Position Configuration Register"]
pub mod l1whpcr;
#[doc = "L2WHPCR (rw) register accessor: LTDC Layerx Window Horizontal Position Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2whpcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2whpcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l2whpcr`]
module"]
pub type L2WHPCR = crate::Reg<l2whpcr::L2WHPCRrs>;
#[doc = "LTDC Layerx Window Horizontal Position Configuration Register"]
pub mod l2whpcr;
#[doc = "L1WVPCR (rw) register accessor: LTDC Layer Window Vertical Position Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1wvpcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1wvpcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l1wvpcr`]
module"]
pub type L1WVPCR = crate::Reg<l1wvpcr::L1WVPCRrs>;
#[doc = "LTDC Layer Window Vertical Position Configuration Register"]
pub mod l1wvpcr;
#[doc = "L2WVPCR (rw) register accessor: LTDC Layer Window Vertical Position Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2wvpcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2wvpcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l2wvpcr`]
module"]
pub type L2WVPCR = crate::Reg<l2wvpcr::L2WVPCRrs>;
#[doc = "LTDC Layer Window Vertical Position Configuration Register"]
pub mod l2wvpcr;
#[doc = "L1CKCR (rw) register accessor: LTDC Layer Color Keying Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1ckcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1ckcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l1ckcr`]
module"]
pub type L1CKCR = crate::Reg<l1ckcr::L1CKCRrs>;
#[doc = "LTDC Layer Color Keying Configuration Register"]
pub mod l1ckcr;
#[doc = "L2CKCR (rw) register accessor: LTDC Layer Color Keying Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2ckcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2ckcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l2ckcr`]
module"]
pub type L2CKCR = crate::Reg<l2ckcr::L2CKCRrs>;
#[doc = "LTDC Layer Color Keying Configuration Register"]
pub mod l2ckcr;
#[doc = "L1PFCR (rw) register accessor: LTDC Layer Pixel Format Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1pfcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1pfcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l1pfcr`]
module"]
pub type L1PFCR = crate::Reg<l1pfcr::L1PFCRrs>;
#[doc = "LTDC Layer Pixel Format Configuration Register"]
pub mod l1pfcr;
#[doc = "L2PFCR (rw) register accessor: LTDC Layer Pixel Format Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2pfcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2pfcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l2pfcr`]
module"]
pub type L2PFCR = crate::Reg<l2pfcr::L2PFCRrs>;
#[doc = "LTDC Layer Pixel Format Configuration Register"]
pub mod l2pfcr;
#[doc = "L1CACR (rw) register accessor: LTDC Layer Constant Alpha Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1cacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1cacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l1cacr`]
module"]
pub type L1CACR = crate::Reg<l1cacr::L1CACRrs>;
#[doc = "LTDC Layer Constant Alpha Configuration Register"]
pub mod l1cacr;
#[doc = "L2CACR (rw) register accessor: LTDC Layer Constant Alpha Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2cacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2cacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l2cacr`]
module"]
pub type L2CACR = crate::Reg<l2cacr::L2CACRrs>;
#[doc = "LTDC Layer Constant Alpha Configuration Register"]
pub mod l2cacr;
#[doc = "L1DCCR (rw) register accessor: LTDC Layer Default Color Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1dccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1dccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l1dccr`]
module"]
pub type L1DCCR = crate::Reg<l1dccr::L1DCCRrs>;
#[doc = "LTDC Layer Default Color Configuration Register"]
pub mod l1dccr;
#[doc = "L2DCCR (rw) register accessor: LTDC Layer Default Color Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2dccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2dccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l2dccr`]
module"]
pub type L2DCCR = crate::Reg<l2dccr::L2DCCRrs>;
#[doc = "LTDC Layer Default Color Configuration Register"]
pub mod l2dccr;
#[doc = "L1BFCR (rw) register accessor: LTDC Layer Blending Factors Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1bfcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1bfcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l1bfcr`]
module"]
pub type L1BFCR = crate::Reg<l1bfcr::L1BFCRrs>;
#[doc = "LTDC Layer Blending Factors Configuration Register"]
pub mod l1bfcr;
#[doc = "L2BFCR (rw) register accessor: LTDC Layer Blending Factors Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2bfcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2bfcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l2bfcr`]
module"]
pub type L2BFCR = crate::Reg<l2bfcr::L2BFCRrs>;
#[doc = "LTDC Layer Blending Factors Configuration Register"]
pub mod l2bfcr;
#[doc = "L1CFBAR (rw) register accessor: LTDC Layer Color Frame Buffer Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1cfbar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1cfbar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l1cfbar`]
module"]
pub type L1CFBAR = crate::Reg<l1cfbar::L1CFBARrs>;
#[doc = "LTDC Layer Color Frame Buffer Address Register"]
pub mod l1cfbar;
#[doc = "L2CFBAR (rw) register accessor: LTDC Layer Color Frame Buffer Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2cfbar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2cfbar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l2cfbar`]
module"]
pub type L2CFBAR = crate::Reg<l2cfbar::L2CFBARrs>;
#[doc = "LTDC Layer Color Frame Buffer Address Register"]
pub mod l2cfbar;
#[doc = "L1CFBLR (rw) register accessor: LTDC Layer Color Frame Buffer Length Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1cfblr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1cfblr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l1cfblr`]
module"]
pub type L1CFBLR = crate::Reg<l1cfblr::L1CFBLRrs>;
#[doc = "LTDC Layer Color Frame Buffer Length Register"]
pub mod l1cfblr;
#[doc = "L2CFBLR (rw) register accessor: LTDC Layer Color Frame Buffer Length Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2cfblr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2cfblr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l2cfblr`]
module"]
pub type L2CFBLR = crate::Reg<l2cfblr::L2CFBLRrs>;
#[doc = "LTDC Layer Color Frame Buffer Length Register"]
pub mod l2cfblr;
#[doc = "L1CFBLNR (rw) register accessor: LTDC Layer ColorFrame Buffer Line Number Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1cfblnr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1cfblnr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l1cfblnr`]
module"]
pub type L1CFBLNR = crate::Reg<l1cfblnr::L1CFBLNRrs>;
#[doc = "LTDC Layer ColorFrame Buffer Line Number Register"]
pub mod l1cfblnr;
#[doc = "L2CFBLNR (rw) register accessor: LTDC Layer ColorFrame Buffer Line Number Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2cfblnr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2cfblnr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l2cfblnr`]
module"]
pub type L2CFBLNR = crate::Reg<l2cfblnr::L2CFBLNRrs>;
#[doc = "LTDC Layer ColorFrame Buffer Line Number Register"]
pub mod l2cfblnr;
#[doc = "L1CLUTWR (w) register accessor: LTDC Layerx CLUT Write Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1clutwr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l1clutwr`]
module"]
pub type L1CLUTWR = crate::Reg<l1clutwr::L1CLUTWRrs>;
#[doc = "LTDC Layerx CLUT Write Register"]
pub mod l1clutwr;
#[doc = "L2CLUTWR (w) register accessor: LTDC Layerx CLUT Write Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2clutwr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l2clutwr`]
module"]
pub type L2CLUTWR = crate::Reg<l2clutwr::L2CLUTWRrs>;
#[doc = "LTDC Layerx CLUT Write Register"]
pub mod l2clutwr;

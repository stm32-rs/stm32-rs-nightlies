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
    layer: [LAYER; 2],
}
impl RegisterBlock {
    #[doc = "0x08 - Synchronization Size Configuration Register"]
    #[inline(always)]
    pub const fn sscr(&self) -> &SSCR {
        &self.sscr
    }
    #[doc = "0x0c - Back Porch Configuration Register"]
    #[inline(always)]
    pub const fn bpcr(&self) -> &BPCR {
        &self.bpcr
    }
    #[doc = "0x10 - Active Width Configuration Register"]
    #[inline(always)]
    pub const fn awcr(&self) -> &AWCR {
        &self.awcr
    }
    #[doc = "0x14 - Total Width Configuration Register"]
    #[inline(always)]
    pub const fn twcr(&self) -> &TWCR {
        &self.twcr
    }
    #[doc = "0x18 - Global Control Register"]
    #[inline(always)]
    pub const fn gcr(&self) -> &GCR {
        &self.gcr
    }
    #[doc = "0x24 - Shadow Reload Configuration Register"]
    #[inline(always)]
    pub const fn srcr(&self) -> &SRCR {
        &self.srcr
    }
    #[doc = "0x2c - Background Color Configuration Register"]
    #[inline(always)]
    pub const fn bccr(&self) -> &BCCR {
        &self.bccr
    }
    #[doc = "0x34 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    #[doc = "0x38 - Interrupt Status Register"]
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    #[doc = "0x3c - Interrupt Clear Register"]
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    #[doc = "0x40 - Line Interrupt Position Configuration Register"]
    #[inline(always)]
    pub const fn lipcr(&self) -> &LIPCR {
        &self.lipcr
    }
    #[doc = "0x44 - Current Position Status Register"]
    #[inline(always)]
    pub const fn cpsr(&self) -> &CPSR {
        &self.cpsr
    }
    #[doc = "0x48 - Current Display Status Register"]
    #[inline(always)]
    pub const fn cdsr(&self) -> &CDSR {
        &self.cdsr
    }
    #[doc = "0x84..0x184 - Cluster LAYER%s, containing L?CR, L?WHPCR, L?WVPCR, L?CKCR, L?PFCR, L?CACR, L?DCCR, L?BFCR, L?CFBAR, L?CFBLR, L?CFBLNR, L?CLUTWR"]
    #[inline(always)]
    pub const fn layer(&self, n: usize) -> &LAYER {
        &self.layer[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x84..0x184 - Cluster LAYER%s, containing L?CR, L?WHPCR, L?WVPCR, L?CKCR, L?PFCR, L?CACR, L?DCCR, L?BFCR, L?CFBAR, L?CFBLR, L?CFBLNR, L?CLUTWR"]
    #[inline(always)]
    pub fn layer_iter(&self) -> impl Iterator<Item = &LAYER> {
        self.layer.iter()
    }
    #[doc = "0x84..0x104 - Cluster LAYER1, containing L?CR, L?WHPCR, L?WVPCR, L?CKCR, L?PFCR, L?CACR, L?DCCR, L?BFCR, L?CFBAR, L?CFBLR, L?CFBLNR, L?CLUTWR"]
    #[inline(always)]
    pub const fn layer1(&self) -> &LAYER {
        self.layer(0)
    }
    #[doc = "0x104..0x184 - Cluster LAYER2, containing L?CR, L?WHPCR, L?WVPCR, L?CKCR, L?PFCR, L?CACR, L?DCCR, L?BFCR, L?CFBAR, L?CFBLR, L?CFBLNR, L?CLUTWR"]
    #[inline(always)]
    pub const fn layer2(&self) -> &LAYER {
        self.layer(1)
    }
}
#[doc = "SSCR (rw) register accessor: Synchronization Size Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sscr`]
module"]
pub type SSCR = crate::Reg<sscr::SSCRrs>;
#[doc = "Synchronization Size Configuration Register"]
pub mod sscr;
#[doc = "BPCR (rw) register accessor: Back Porch Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bpcr`]
module"]
pub type BPCR = crate::Reg<bpcr::BPCRrs>;
#[doc = "Back Porch Configuration Register"]
pub mod bpcr;
#[doc = "AWCR (rw) register accessor: Active Width Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awcr`]
module"]
pub type AWCR = crate::Reg<awcr::AWCRrs>;
#[doc = "Active Width Configuration Register"]
pub mod awcr;
#[doc = "TWCR (rw) register accessor: Total Width Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@twcr`]
module"]
pub type TWCR = crate::Reg<twcr::TWCRrs>;
#[doc = "Total Width Configuration Register"]
pub mod twcr;
#[doc = "GCR (rw) register accessor: Global Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gcr`]
module"]
pub type GCR = crate::Reg<gcr::GCRrs>;
#[doc = "Global Control Register"]
pub mod gcr;
#[doc = "SRCR (rw) register accessor: Shadow Reload Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcr`]
module"]
pub type SRCR = crate::Reg<srcr::SRCRrs>;
#[doc = "Shadow Reload Configuration Register"]
pub mod srcr;
#[doc = "BCCR (rw) register accessor: Background Color Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bccr`]
module"]
pub type BCCR = crate::Reg<bccr::BCCRrs>;
#[doc = "Background Color Configuration Register"]
pub mod bccr;
#[doc = "IER (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
pub type IER = crate::Reg<ier::IERrs>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "ISR (r) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
pub type ISR = crate::Reg<isr::ISRrs>;
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "ICR (w) register accessor: Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`]
module"]
pub type ICR = crate::Reg<icr::ICRrs>;
#[doc = "Interrupt Clear Register"]
pub mod icr;
#[doc = "LIPCR (rw) register accessor: Line Interrupt Position Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lipcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lipcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lipcr`]
module"]
pub type LIPCR = crate::Reg<lipcr::LIPCRrs>;
#[doc = "Line Interrupt Position Configuration Register"]
pub mod lipcr;
#[doc = "CPSR (r) register accessor: Current Position Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsr`]
module"]
pub type CPSR = crate::Reg<cpsr::CPSRrs>;
#[doc = "Current Position Status Register"]
pub mod cpsr;
#[doc = "CDSR (r) register accessor: Current Display Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdsr`]
module"]
pub type CDSR = crate::Reg<cdsr::CDSRrs>;
#[doc = "Current Display Status Register"]
pub mod cdsr;
#[doc = "Cluster LAYER%s, containing L?CR, L?WHPCR, L?WVPCR, L?CKCR, L?PFCR, L?CACR, L?DCCR, L?BFCR, L?CFBAR, L?CFBLR, L?CFBLNR, L?CLUTWR"]
pub use self::layer::LAYER;
#[doc = r"Cluster"]
#[doc = "Cluster LAYER%s, containing L?CR, L?WHPCR, L?WVPCR, L?CKCR, L?PFCR, L?CACR, L?DCCR, L?BFCR, L?CFBAR, L?CFBLR, L?CFBLNR, L?CLUTWR"]
pub mod layer;

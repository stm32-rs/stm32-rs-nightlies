#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    hcfg: HCFG,
    hfir: HFIR,
    hfnum: HFNUM,
    _reserved3: [u8; 0x04],
    hptxsts: HPTXSTS,
    haint: HAINT,
    haintmsk: HAINTMSK,
    _reserved6: [u8; 0x24],
    hprt: HPRT,
    _reserved7: [u8; 0xbc],
    hc: [HC; 16],
}
impl RegisterBlock {
    #[doc = "0x00 - OTG_HS host configuration register"]
    #[inline(always)]
    pub const fn hcfg(&self) -> &HCFG {
        &self.hcfg
    }
    #[doc = "0x04 - OTG_HS Host frame interval register"]
    #[inline(always)]
    pub const fn hfir(&self) -> &HFIR {
        &self.hfir
    }
    #[doc = "0x08 - OTG_HS host frame number/frame time remaining register"]
    #[inline(always)]
    pub const fn hfnum(&self) -> &HFNUM {
        &self.hfnum
    }
    #[doc = "0x10 - OTG_HS_Host periodic transmit FIFO/queue status register"]
    #[inline(always)]
    pub const fn hptxsts(&self) -> &HPTXSTS {
        &self.hptxsts
    }
    #[doc = "0x14 - OTG_HS Host all channels interrupt register"]
    #[inline(always)]
    pub const fn haint(&self) -> &HAINT {
        &self.haint
    }
    #[doc = "0x18 - OTG_HS host all channels interrupt mask register"]
    #[inline(always)]
    pub const fn haintmsk(&self) -> &HAINTMSK {
        &self.haintmsk
    }
    #[doc = "0x40 - OTG_HS host port control and status register"]
    #[inline(always)]
    pub const fn hprt(&self) -> &HPRT {
        &self.hprt
    }
    #[doc = "0x100..0x300 - Host channel"]
    #[inline(always)]
    pub const fn hc(&self, n: usize) -> &HC {
        &self.hc[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x300 - Host channel"]
    #[inline(always)]
    pub fn hc_iter(&self) -> impl Iterator<Item = &HC> {
        self.hc.iter()
    }
}
#[doc = "HCFG (rw) register accessor: OTG_HS host configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcfg`]
module"]
pub type HCFG = crate::Reg<hcfg::HCFGrs>;
#[doc = "OTG_HS host configuration register"]
pub mod hcfg;
#[doc = "HFIR (rw) register accessor: OTG_HS Host frame interval register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfir`]
module"]
pub type HFIR = crate::Reg<hfir::HFIRrs>;
#[doc = "OTG_HS Host frame interval register"]
pub mod hfir;
#[doc = "HFNUM (r) register accessor: OTG_HS host frame number/frame time remaining register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfnum::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfnum`]
module"]
pub type HFNUM = crate::Reg<hfnum::HFNUMrs>;
#[doc = "OTG_HS host frame number/frame time remaining register"]
pub mod hfnum;
#[doc = "HPTXSTS (rw) register accessor: OTG_HS_Host periodic transmit FIFO/queue status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hptxsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hptxsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hptxsts`]
module"]
pub type HPTXSTS = crate::Reg<hptxsts::HPTXSTSrs>;
#[doc = "OTG_HS_Host periodic transmit FIFO/queue status register"]
pub mod hptxsts;
#[doc = "HAINT (r) register accessor: OTG_HS Host all channels interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`haint::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@haint`]
module"]
pub type HAINT = crate::Reg<haint::HAINTrs>;
#[doc = "OTG_HS Host all channels interrupt register"]
pub mod haint;
#[doc = "HAINTMSK (rw) register accessor: OTG_HS host all channels interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`haintmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`haintmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@haintmsk`]
module"]
pub type HAINTMSK = crate::Reg<haintmsk::HAINTMSKrs>;
#[doc = "OTG_HS host all channels interrupt mask register"]
pub mod haintmsk;
#[doc = "HPRT (rw) register accessor: OTG_HS host port control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hprt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hprt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hprt`]
module"]
pub type HPRT = crate::Reg<hprt::HPRTrs>;
#[doc = "OTG_HS host port control and status register"]
pub mod hprt;
#[doc = "Host channel"]
pub use self::hc::HC;
#[doc = r"Cluster"]
#[doc = "Host channel"]
pub mod hc;

#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gcr: GCR,
    ch: [CH; 2],
    pdmcr: PDMCR,
    pdmdly: PDMDLY,
}
impl RegisterBlock {
    #[doc = "0x00 - Global configuration register"]
    #[inline(always)]
    pub const fn gcr(&self) -> &GCR {
        &self.gcr
    }
    #[doc = "0x04..0x44 - Cluster CH%s, containing ?CR1, ?CR2, ?FRCR, ?SLOTR, ?IM, ?SR, ?CLRFR, ?DR"]
    #[inline(always)]
    pub const fn ch(&self, n: usize) -> &CH {
        &self.ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x04..0x44 - Cluster CH%s, containing ?CR1, ?CR2, ?FRCR, ?SLOTR, ?IM, ?SR, ?CLRFR, ?DR"]
    #[inline(always)]
    pub fn ch_iter(&self) -> impl Iterator<Item = &CH> {
        self.ch.iter()
    }
    #[doc = "0x04..0x24 - Cluster CHA, containing ?CR1, ?CR2, ?FRCR, ?SLOTR, ?IM, ?SR, ?CLRFR, ?DR"]
    #[inline(always)]
    pub const fn cha(&self) -> &CH {
        self.ch(0)
    }
    #[doc = "0x24..0x44 - Cluster CHB, containing ?CR1, ?CR2, ?FRCR, ?SLOTR, ?IM, ?SR, ?CLRFR, ?DR"]
    #[inline(always)]
    pub const fn chb(&self) -> &CH {
        self.ch(1)
    }
    #[doc = "0x44 - PDM control register"]
    #[inline(always)]
    pub const fn pdmcr(&self) -> &PDMCR {
        &self.pdmcr
    }
    #[doc = "0x48 - PDM delay register"]
    #[inline(always)]
    pub const fn pdmdly(&self) -> &PDMDLY {
        &self.pdmdly
    }
}
#[doc = "Cluster CH%s, containing ?CR1, ?CR2, ?FRCR, ?SLOTR, ?IM, ?SR, ?CLRFR, ?DR"]
pub use self::ch::CH;
#[doc = r"Cluster"]
#[doc = "Cluster CH%s, containing ?CR1, ?CR2, ?FRCR, ?SLOTR, ?IM, ?SR, ?CLRFR, ?DR"]
pub mod ch;
#[doc = "GCR (rw) register accessor: Global configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gcr`]
module"]
pub type GCR = crate::Reg<gcr::GCRrs>;
#[doc = "Global configuration register"]
pub mod gcr;
#[doc = "PDMCR (rw) register accessor: PDM control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdmcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdmcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdmcr`]
module"]
pub type PDMCR = crate::Reg<pdmcr::PDMCRrs>;
#[doc = "PDM control register"]
pub mod pdmcr;
#[doc = "PDMDLY (rw) register accessor: PDM delay register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdmdly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdmdly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdmdly`]
module"]
pub type PDMDLY = crate::Reg<pdmdly::PDMDLYrs>;
#[doc = "PDM delay register"]
pub mod pdmdly;

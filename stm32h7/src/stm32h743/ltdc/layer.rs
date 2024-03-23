#[repr(C)]
#[doc = "Cluster LAYER%s, containing L?CR, L?WHPCR, L?WVPCR, L?CKCR, L?PFCR, L?CACR, L?DCCR, L?BFCR, L?CFBAR, L?CFBLR, L?CFBLNR, L?CLUTWR"]
pub struct LAYER {
    cr: CR,
    whpcr: WHPCR,
    wvpcr: WVPCR,
    ckcr: CKCR,
    pfcr: PFCR,
    cacr: CACR,
    dccr: DCCR,
    bfcr: BFCR,
    _reserved8: [u8; 0x08],
    cfbar: CFBAR,
    cfblr: CFBLR,
    cfblnr: CFBLNR,
    _reserved11: [u8; 0x0c],
    clutwr: CLUTWR,
    _reserved_end: [u8; 0x3c],
}
impl LAYER {
    #[doc = "0x00 - Layerx Control Register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x04 - Layerx Window Horizontal Position Configuration Register"]
    #[inline(always)]
    pub const fn whpcr(&self) -> &WHPCR {
        &self.whpcr
    }
    #[doc = "0x08 - Layerx Window Vertical Position Configuration Register"]
    #[inline(always)]
    pub const fn wvpcr(&self) -> &WVPCR {
        &self.wvpcr
    }
    #[doc = "0x0c - Layerx Color Keying Configuration Register"]
    #[inline(always)]
    pub const fn ckcr(&self) -> &CKCR {
        &self.ckcr
    }
    #[doc = "0x10 - Layerx Pixel Format Configuration Register"]
    #[inline(always)]
    pub const fn pfcr(&self) -> &PFCR {
        &self.pfcr
    }
    #[doc = "0x14 - Layerx Constant Alpha Configuration Register"]
    #[inline(always)]
    pub const fn cacr(&self) -> &CACR {
        &self.cacr
    }
    #[doc = "0x18 - Layerx Default Color Configuration Register"]
    #[inline(always)]
    pub const fn dccr(&self) -> &DCCR {
        &self.dccr
    }
    #[doc = "0x1c - Layerx Blending Factors Configuration Register"]
    #[inline(always)]
    pub const fn bfcr(&self) -> &BFCR {
        &self.bfcr
    }
    #[doc = "0x28 - Layerx Color Frame Buffer Address Register"]
    #[inline(always)]
    pub const fn cfbar(&self) -> &CFBAR {
        &self.cfbar
    }
    #[doc = "0x2c - Layerx Color Frame Buffer Length Register"]
    #[inline(always)]
    pub const fn cfblr(&self) -> &CFBLR {
        &self.cfblr
    }
    #[doc = "0x30 - Layerx ColorFrame Buffer Line Number Register"]
    #[inline(always)]
    pub const fn cfblnr(&self) -> &CFBLNR {
        &self.cfblnr
    }
    #[doc = "0x40 - Layerx CLUT Write Register"]
    #[inline(always)]
    pub const fn clutwr(&self) -> &CLUTWR {
        &self.clutwr
    }
}
#[doc = "CR (rw) register accessor: Layerx Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "Layerx Control Register"]
pub mod cr;
#[doc = "WHPCR (rw) register accessor: Layerx Window Horizontal Position Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`whpcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`whpcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@whpcr`]
module"]
pub type WHPCR = crate::Reg<whpcr::WHPCRrs>;
#[doc = "Layerx Window Horizontal Position Configuration Register"]
pub mod whpcr;
#[doc = "WVPCR (rw) register accessor: Layerx Window Vertical Position Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wvpcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wvpcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wvpcr`]
module"]
pub type WVPCR = crate::Reg<wvpcr::WVPCRrs>;
#[doc = "Layerx Window Vertical Position Configuration Register"]
pub mod wvpcr;
#[doc = "CKCR (rw) register accessor: Layerx Color Keying Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ckcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ckcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ckcr`]
module"]
pub type CKCR = crate::Reg<ckcr::CKCRrs>;
#[doc = "Layerx Color Keying Configuration Register"]
pub mod ckcr;
#[doc = "PFCR (rw) register accessor: Layerx Pixel Format Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pfcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pfcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfcr`]
module"]
pub type PFCR = crate::Reg<pfcr::PFCRrs>;
#[doc = "Layerx Pixel Format Configuration Register"]
pub mod pfcr;
#[doc = "CACR (rw) register accessor: Layerx Constant Alpha Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cacr`]
module"]
pub type CACR = crate::Reg<cacr::CACRrs>;
#[doc = "Layerx Constant Alpha Configuration Register"]
pub mod cacr;
#[doc = "DCCR (rw) register accessor: Layerx Default Color Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dccr`]
module"]
pub type DCCR = crate::Reg<dccr::DCCRrs>;
#[doc = "Layerx Default Color Configuration Register"]
pub mod dccr;
#[doc = "BFCR (rw) register accessor: Layerx Blending Factors Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bfcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bfcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bfcr`]
module"]
pub type BFCR = crate::Reg<bfcr::BFCRrs>;
#[doc = "Layerx Blending Factors Configuration Register"]
pub mod bfcr;
#[doc = "CFBAR (rw) register accessor: Layerx Color Frame Buffer Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfbar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfbar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfbar`]
module"]
pub type CFBAR = crate::Reg<cfbar::CFBARrs>;
#[doc = "Layerx Color Frame Buffer Address Register"]
pub mod cfbar;
#[doc = "CFBLR (rw) register accessor: Layerx Color Frame Buffer Length Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfblr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfblr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfblr`]
module"]
pub type CFBLR = crate::Reg<cfblr::CFBLRrs>;
#[doc = "Layerx Color Frame Buffer Length Register"]
pub mod cfblr;
#[doc = "CFBLNR (rw) register accessor: Layerx ColorFrame Buffer Line Number Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfblnr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfblnr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfblnr`]
module"]
pub type CFBLNR = crate::Reg<cfblnr::CFBLNRrs>;
#[doc = "Layerx ColorFrame Buffer Line Number Register"]
pub mod cfblnr;
#[doc = "CLUTWR (w) register accessor: Layerx CLUT Write Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clutwr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clutwr`]
module"]
pub type CLUTWR = crate::Reg<clutwr::CLUTWRrs>;
#[doc = "Layerx CLUT Write Register"]
pub mod clutwr;

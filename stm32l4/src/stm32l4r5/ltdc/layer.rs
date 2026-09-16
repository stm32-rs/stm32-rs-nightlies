#[repr(C)]
#[derive(Debug)]
///Cluster LAYER%s, containing L?CR, L?WHPCR, L?WVPCR, L?CKCR, L?PFCR, L?CACR, L?DCCR, L?BFCR, L?CFBAR, L?CFBLR, L?CFBLNR, L?CLUTWR
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
    ///0x00 - LTDC Layer Control Register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - LTDC Layer Window Horizontal Position Configuration Register
    #[inline(always)]
    pub const fn whpcr(&self) -> &WHPCR {
        &self.whpcr
    }
    ///0x08 - LTDC Layer Window Vertical Position Configuration Register
    #[inline(always)]
    pub const fn wvpcr(&self) -> &WVPCR {
        &self.wvpcr
    }
    ///0x0c - LTDC Layer Color Keying Configuration Register
    #[inline(always)]
    pub const fn ckcr(&self) -> &CKCR {
        &self.ckcr
    }
    ///0x10 - LTDC Layer Pixel Format Configuration Register
    #[inline(always)]
    pub const fn pfcr(&self) -> &PFCR {
        &self.pfcr
    }
    ///0x14 - LTDC Layer Constant Alpha Configuration Register
    #[inline(always)]
    pub const fn cacr(&self) -> &CACR {
        &self.cacr
    }
    ///0x18 - LTDC Layer Default Color Configuration Register
    #[inline(always)]
    pub const fn dccr(&self) -> &DCCR {
        &self.dccr
    }
    ///0x1c - LTDC Layer Blending Factors Configuration Register
    #[inline(always)]
    pub const fn bfcr(&self) -> &BFCR {
        &self.bfcr
    }
    ///0x28 - LTDC Layer Color Frame Buffer Address Register
    #[inline(always)]
    pub const fn cfbar(&self) -> &CFBAR {
        &self.cfbar
    }
    ///0x2c - LTDC Layer Color Frame Buffer Length Register
    #[inline(always)]
    pub const fn cfblr(&self) -> &CFBLR {
        &self.cfblr
    }
    ///0x30 - LTDC Layer ColorFrame Buffer Line Number Register
    #[inline(always)]
    pub const fn cfblnr(&self) -> &CFBLNR {
        &self.cfblnr
    }
    ///0x40 - LTDC Layerx CLUT Write Register
    #[inline(always)]
    pub const fn clutwr(&self) -> &CLUTWR {
        &self.clutwr
    }
}
/**CR (rw) register accessor: LTDC Layer Control Register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///LTDC Layer Control Register
pub mod cr;
/**WHPCR (rw) register accessor: LTDC Layer Window Horizontal Position Configuration Register

You can [`read`](crate::Reg::read) this register and get [`whpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`whpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@whpcr`] module*/
pub type WHPCR = crate::Reg<whpcr::WHPCRrs>;
///LTDC Layer Window Horizontal Position Configuration Register
pub mod whpcr;
/**WVPCR (rw) register accessor: LTDC Layer Window Vertical Position Configuration Register

You can [`read`](crate::Reg::read) this register and get [`wvpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wvpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wvpcr`] module*/
pub type WVPCR = crate::Reg<wvpcr::WVPCRrs>;
///LTDC Layer Window Vertical Position Configuration Register
pub mod wvpcr;
/**CKCR (rw) register accessor: LTDC Layer Color Keying Configuration Register

You can [`read`](crate::Reg::read) this register and get [`ckcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ckcr`] module*/
pub type CKCR = crate::Reg<ckcr::CKCRrs>;
///LTDC Layer Color Keying Configuration Register
pub mod ckcr;
/**PFCR (rw) register accessor: LTDC Layer Pixel Format Configuration Register

You can [`read`](crate::Reg::read) this register and get [`pfcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pfcr`] module*/
pub type PFCR = crate::Reg<pfcr::PFCRrs>;
///LTDC Layer Pixel Format Configuration Register
pub mod pfcr;
/**CACR (rw) register accessor: LTDC Layer Constant Alpha Configuration Register

You can [`read`](crate::Reg::read) this register and get [`cacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cacr`] module*/
pub type CACR = crate::Reg<cacr::CACRrs>;
///LTDC Layer Constant Alpha Configuration Register
pub mod cacr;
/**DCCR (rw) register accessor: LTDC Layer Default Color Configuration Register

You can [`read`](crate::Reg::read) this register and get [`dccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dccr`] module*/
pub type DCCR = crate::Reg<dccr::DCCRrs>;
///LTDC Layer Default Color Configuration Register
pub mod dccr;
/**BFCR (rw) register accessor: LTDC Layer Blending Factors Configuration Register

You can [`read`](crate::Reg::read) this register and get [`bfcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bfcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bfcr`] module*/
pub type BFCR = crate::Reg<bfcr::BFCRrs>;
///LTDC Layer Blending Factors Configuration Register
pub mod bfcr;
/**CFBAR (rw) register accessor: LTDC Layer Color Frame Buffer Address Register

You can [`read`](crate::Reg::read) this register and get [`cfbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfbar`] module*/
pub type CFBAR = crate::Reg<cfbar::CFBARrs>;
///LTDC Layer Color Frame Buffer Address Register
pub mod cfbar;
/**CFBLR (rw) register accessor: LTDC Layer Color Frame Buffer Length Register

You can [`read`](crate::Reg::read) this register and get [`cfblr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfblr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfblr`] module*/
pub type CFBLR = crate::Reg<cfblr::CFBLRrs>;
///LTDC Layer Color Frame Buffer Length Register
pub mod cfblr;
/**CFBLNR (rw) register accessor: LTDC Layer ColorFrame Buffer Line Number Register

You can [`read`](crate::Reg::read) this register and get [`cfblnr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfblnr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfblnr`] module*/
pub type CFBLNR = crate::Reg<cfblnr::CFBLNRrs>;
///LTDC Layer ColorFrame Buffer Line Number Register
pub mod cfblnr;
/**CLUTWR (w) register accessor: LTDC Layerx CLUT Write Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clutwr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clutwr`] module*/
pub type CLUTWR = crate::Reg<clutwr::CLUTWRrs>;
///LTDC Layerx CLUT Write Register
pub mod clutwr;

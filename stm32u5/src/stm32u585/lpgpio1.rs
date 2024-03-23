#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    lpgpio_moder: LPGPIO_MODER,
    _reserved1: [u8; 0x0c],
    lpgpio_idr: LPGPIO_IDR,
    lpgpio_odr: LPGPIO_ODR,
    lpgpio_bsrr: LPGPIO_BSRR,
    _reserved4: [u8; 0x0c],
    lpgpio_brr: LPGPIO_BRR,
}
impl RegisterBlock {
    #[doc = "0x00 - LPGPIO port mode register"]
    #[inline(always)]
    pub const fn lpgpio_moder(&self) -> &LPGPIO_MODER {
        &self.lpgpio_moder
    }
    #[doc = "0x10 - LPGPIO port input data register"]
    #[inline(always)]
    pub const fn lpgpio_idr(&self) -> &LPGPIO_IDR {
        &self.lpgpio_idr
    }
    #[doc = "0x14 - LPGPIO port output data register"]
    #[inline(always)]
    pub const fn lpgpio_odr(&self) -> &LPGPIO_ODR {
        &self.lpgpio_odr
    }
    #[doc = "0x18 - LPGPIO port bit set/reset register"]
    #[inline(always)]
    pub const fn lpgpio_bsrr(&self) -> &LPGPIO_BSRR {
        &self.lpgpio_bsrr
    }
    #[doc = "0x28 - LPGPIO port bit reset register"]
    #[inline(always)]
    pub const fn lpgpio_brr(&self) -> &LPGPIO_BRR {
        &self.lpgpio_brr
    }
}
#[doc = "LPGPIO_MODER (rw) register accessor: LPGPIO port mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpgpio_moder::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpgpio_moder::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpgpio_moder`]
module"]
pub type LPGPIO_MODER = crate::Reg<lpgpio_moder::LPGPIO_MODERrs>;
#[doc = "LPGPIO port mode register"]
pub mod lpgpio_moder;
#[doc = "LPGPIO_IDR (r) register accessor: LPGPIO port input data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpgpio_idr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpgpio_idr`]
module"]
pub type LPGPIO_IDR = crate::Reg<lpgpio_idr::LPGPIO_IDRrs>;
#[doc = "LPGPIO port input data register"]
pub mod lpgpio_idr;
#[doc = "LPGPIO_ODR (rw) register accessor: LPGPIO port output data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpgpio_odr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpgpio_odr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpgpio_odr`]
module"]
pub type LPGPIO_ODR = crate::Reg<lpgpio_odr::LPGPIO_ODRrs>;
#[doc = "LPGPIO port output data register"]
pub mod lpgpio_odr;
#[doc = "LPGPIO_BSRR (w) register accessor: LPGPIO port bit set/reset register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpgpio_bsrr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpgpio_bsrr`]
module"]
pub type LPGPIO_BSRR = crate::Reg<lpgpio_bsrr::LPGPIO_BSRRrs>;
#[doc = "LPGPIO port bit set/reset register"]
pub mod lpgpio_bsrr;
#[doc = "LPGPIO_BRR (r) register accessor: LPGPIO port bit reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpgpio_brr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpgpio_brr`]
module"]
pub type LPGPIO_BRR = crate::Reg<lpgpio_brr::LPGPIO_BRRrs>;
#[doc = "LPGPIO port bit reset register"]
pub mod lpgpio_brr;

#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    moder: MODER,
    otyper: OTYPER,
    ospeedr: OSPEEDR,
    pupdr: PUPDR,
    idr: IDR,
    odr: ODR,
    bsrr: BSRR,
    lckr: LCKR,
    afrl: AFRL,
    afrh: AFRH,
    brr: BRR,
}
impl RegisterBlock {
    #[doc = "0x00 - GPIO port mode register"]
    #[inline(always)]
    pub const fn moder(&self) -> &MODER {
        &self.moder
    }
    #[doc = "0x04 - GPIO port output type register"]
    #[inline(always)]
    pub const fn otyper(&self) -> &OTYPER {
        &self.otyper
    }
    #[doc = "0x08 - GPIO port output speed register"]
    #[inline(always)]
    pub const fn ospeedr(&self) -> &OSPEEDR {
        &self.ospeedr
    }
    #[doc = "0x0c - GPIO port pull-up/pull-down register"]
    #[inline(always)]
    pub const fn pupdr(&self) -> &PUPDR {
        &self.pupdr
    }
    #[doc = "0x10 - GPIO port input data register"]
    #[inline(always)]
    pub const fn idr(&self) -> &IDR {
        &self.idr
    }
    #[doc = "0x14 - GPIO port output data register"]
    #[inline(always)]
    pub const fn odr(&self) -> &ODR {
        &self.odr
    }
    #[doc = "0x18 - GPIO port bit set/reset register"]
    #[inline(always)]
    pub const fn bsrr(&self) -> &BSRR {
        &self.bsrr
    }
    #[doc = "0x1c - GPIO port configuration lock register"]
    #[inline(always)]
    pub const fn lckr(&self) -> &LCKR {
        &self.lckr
    }
    #[doc = "0x20 - GPIO alternate function low register"]
    #[inline(always)]
    pub const fn afrl(&self) -> &AFRL {
        &self.afrl
    }
    #[doc = "0x24 - GPIO alternate function high register"]
    #[inline(always)]
    pub const fn afrh(&self) -> &AFRH {
        &self.afrh
    }
    #[doc = "0x28 - GPIO port bit reset register"]
    #[inline(always)]
    pub const fn brr(&self) -> &BRR {
        &self.brr
    }
}
#[doc = "MODER (rw) register accessor: GPIO port mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`moder::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`moder::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@moder`]
module"]
pub type MODER = crate::Reg<moder::MODERrs>;
#[doc = "GPIO port mode register"]
pub mod moder;
#[doc = "OTYPER (rw) register accessor: GPIO port output type register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otyper::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otyper::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otyper`]
module"]
pub type OTYPER = crate::Reg<otyper::OTYPERrs>;
#[doc = "GPIO port output type register"]
pub mod otyper;
#[doc = "OSPEEDR (rw) register accessor: GPIO port output speed register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospeedr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospeedr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospeedr`]
module"]
pub type OSPEEDR = crate::Reg<ospeedr::OSPEEDRrs>;
#[doc = "GPIO port output speed register"]
pub mod ospeedr;
#[doc = "PUPDR (rw) register accessor: GPIO port pull-up/pull-down register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pupdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pupdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pupdr`]
module"]
pub type PUPDR = crate::Reg<pupdr::PUPDRrs>;
#[doc = "GPIO port pull-up/pull-down register"]
pub mod pupdr;
#[doc = "IDR (r) register accessor: GPIO port input data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr`]
module"]
pub type IDR = crate::Reg<idr::IDRrs>;
#[doc = "GPIO port input data register"]
pub mod idr;
#[doc = "ODR (rw) register accessor: GPIO port output data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`odr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`odr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@odr`]
module"]
pub type ODR = crate::Reg<odr::ODRrs>;
#[doc = "GPIO port output data register"]
pub mod odr;
#[doc = "BSRR (w) register accessor: GPIO port bit set/reset register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bsrr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bsrr`]
module"]
pub type BSRR = crate::Reg<bsrr::BSRRrs>;
#[doc = "GPIO port bit set/reset register"]
pub mod bsrr;
#[doc = "LCKR (rw) register accessor: GPIO port configuration lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lckr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lckr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lckr`]
module"]
pub type LCKR = crate::Reg<lckr::LCKRrs>;
#[doc = "GPIO port configuration lock register"]
pub mod lckr;
#[doc = "AFRL (rw) register accessor: GPIO alternate function low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afrl`]
module"]
pub type AFRL = crate::Reg<afrl::AFRLrs>;
#[doc = "GPIO alternate function low register"]
pub mod afrl;
#[doc = "AFRH (rw) register accessor: GPIO alternate function high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afrh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afrh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afrh`]
module"]
pub type AFRH = crate::Reg<afrh::AFRHrs>;
#[doc = "GPIO alternate function high register"]
pub mod afrh;
#[doc = "BRR (rw) register accessor: GPIO port bit reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brr`]
module"]
pub type BRR = crate::Reg<brr::BRRrs>;
#[doc = "GPIO port bit reset register"]
pub mod brr;

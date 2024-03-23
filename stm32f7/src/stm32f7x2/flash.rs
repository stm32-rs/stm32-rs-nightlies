#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    acr: ACR,
    keyr: KEYR,
    optkeyr: OPTKEYR,
    sr: SR,
    cr: CR,
    optcr: OPTCR,
    optcr1: OPTCR1,
    optcr2: OPTCR2,
}
impl RegisterBlock {
    #[doc = "0x00 - Flash access control register"]
    #[inline(always)]
    pub const fn acr(&self) -> &ACR {
        &self.acr
    }
    #[doc = "0x04 - Flash key register"]
    #[inline(always)]
    pub const fn keyr(&self) -> &KEYR {
        &self.keyr
    }
    #[doc = "0x08 - Flash option key register"]
    #[inline(always)]
    pub const fn optkeyr(&self) -> &OPTKEYR {
        &self.optkeyr
    }
    #[doc = "0x0c - Status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x10 - Control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x14 - Flash option control register"]
    #[inline(always)]
    pub const fn optcr(&self) -> &OPTCR {
        &self.optcr
    }
    #[doc = "0x18 - Flash option control register 1"]
    #[inline(always)]
    pub const fn optcr1(&self) -> &OPTCR1 {
        &self.optcr1
    }
    #[doc = "0x1c - Flash option control register"]
    #[inline(always)]
    pub const fn optcr2(&self) -> &OPTCR2 {
        &self.optcr2
    }
}
#[doc = "ACR (rw) register accessor: Flash access control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acr`]
module"]
pub type ACR = crate::Reg<acr::ACRrs>;
#[doc = "Flash access control register"]
pub mod acr;
#[doc = "KEYR (w) register accessor: Flash key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyr`]
module"]
pub type KEYR = crate::Reg<keyr::KEYRrs>;
#[doc = "Flash key register"]
pub mod keyr;
#[doc = "OPTKEYR (w) register accessor: Flash option key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`optkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@optkeyr`]
module"]
pub type OPTKEYR = crate::Reg<optkeyr::OPTKEYRrs>;
#[doc = "Flash option key register"]
pub mod optkeyr;
#[doc = "SR (rw) register accessor: Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SRrs>;
#[doc = "Status register"]
pub mod sr;
#[doc = "CR (rw) register accessor: Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "Control register"]
pub mod cr;
#[doc = "OPTCR (rw) register accessor: Flash option control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`optcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`optcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@optcr`]
module"]
pub type OPTCR = crate::Reg<optcr::OPTCRrs>;
#[doc = "Flash option control register"]
pub mod optcr;
#[doc = "OPTCR1 (rw) register accessor: Flash option control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`optcr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`optcr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@optcr1`]
module"]
pub type OPTCR1 = crate::Reg<optcr1::OPTCR1rs>;
#[doc = "Flash option control register 1"]
pub mod optcr1;
#[doc = "OPTCR2 (rw) register accessor: Flash option control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`optcr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`optcr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@optcr2`]
module"]
pub type OPTCR2 = crate::Reg<optcr2::OPTCR2rs>;
#[doc = "Flash option control register"]
pub mod optcr2;

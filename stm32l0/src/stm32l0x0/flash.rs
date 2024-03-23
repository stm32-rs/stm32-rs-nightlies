#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    acr: ACR,
    pecr: PECR,
    pdkeyr: PDKEYR,
    pekeyr: PEKEYR,
    prgkeyr: PRGKEYR,
    optkeyr: OPTKEYR,
    sr: SR,
    optr: OPTR,
    wrprot1: WRPROT1,
    _reserved9: [u8; 0x5c],
    wrprot2: WRPROT2,
}
impl RegisterBlock {
    #[doc = "0x00 - Access control register"]
    #[inline(always)]
    pub const fn acr(&self) -> &ACR {
        &self.acr
    }
    #[doc = "0x04 - Program/erase control register"]
    #[inline(always)]
    pub const fn pecr(&self) -> &PECR {
        &self.pecr
    }
    #[doc = "0x08 - Power down key register"]
    #[inline(always)]
    pub const fn pdkeyr(&self) -> &PDKEYR {
        &self.pdkeyr
    }
    #[doc = "0x0c - Program/erase key register"]
    #[inline(always)]
    pub const fn pekeyr(&self) -> &PEKEYR {
        &self.pekeyr
    }
    #[doc = "0x10 - Program memory key register"]
    #[inline(always)]
    pub const fn prgkeyr(&self) -> &PRGKEYR {
        &self.prgkeyr
    }
    #[doc = "0x14 - Option byte key register"]
    #[inline(always)]
    pub const fn optkeyr(&self) -> &OPTKEYR {
        &self.optkeyr
    }
    #[doc = "0x18 - Status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x1c - Option byte register"]
    #[inline(always)]
    pub const fn optr(&self) -> &OPTR {
        &self.optr
    }
    #[doc = "0x20 - Write protection register"]
    #[inline(always)]
    pub const fn wrprot1(&self) -> &WRPROT1 {
        &self.wrprot1
    }
    #[doc = "0x80 - Write protection register"]
    #[inline(always)]
    pub const fn wrprot2(&self) -> &WRPROT2 {
        &self.wrprot2
    }
}
#[doc = "ACR (rw) register accessor: Access control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acr`]
module"]
pub type ACR = crate::Reg<acr::ACRrs>;
#[doc = "Access control register"]
pub mod acr;
#[doc = "PECR (rw) register accessor: Program/erase control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pecr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pecr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pecr`]
module"]
pub type PECR = crate::Reg<pecr::PECRrs>;
#[doc = "Program/erase control register"]
pub mod pecr;
#[doc = "PDKEYR (w) register accessor: Power down key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdkeyr`]
module"]
pub type PDKEYR = crate::Reg<pdkeyr::PDKEYRrs>;
#[doc = "Power down key register"]
pub mod pdkeyr;
#[doc = "PEKEYR (w) register accessor: Program/erase key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pekeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pekeyr`]
module"]
pub type PEKEYR = crate::Reg<pekeyr::PEKEYRrs>;
#[doc = "Program/erase key register"]
pub mod pekeyr;
#[doc = "PRGKEYR (w) register accessor: Program memory key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prgkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prgkeyr`]
module"]
pub type PRGKEYR = crate::Reg<prgkeyr::PRGKEYRrs>;
#[doc = "Program memory key register"]
pub mod prgkeyr;
#[doc = "OPTKEYR (w) register accessor: Option byte key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`optkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@optkeyr`]
module"]
pub type OPTKEYR = crate::Reg<optkeyr::OPTKEYRrs>;
#[doc = "Option byte key register"]
pub mod optkeyr;
#[doc = "SR (rw) register accessor: Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SRrs>;
#[doc = "Status register"]
pub mod sr;
#[doc = "OPTR (r) register accessor: Option byte register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`optr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@optr`]
module"]
pub type OPTR = crate::Reg<optr::OPTRrs>;
#[doc = "Option byte register"]
pub mod optr;
#[doc = "WRPROT1 (r) register accessor: Write protection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrprot1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrprot1`]
module"]
pub type WRPROT1 = crate::Reg<wrprot1::WRPROT1rs>;
#[doc = "Write protection register"]
pub mod wrprot1;
#[doc = "WRPROT2 (r) register accessor: Write protection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrprot2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrprot2`]
module"]
pub type WRPROT2 = crate::Reg<wrprot2::WRPROT2rs>;
#[doc = "Write protection register"]
pub mod wrprot2;

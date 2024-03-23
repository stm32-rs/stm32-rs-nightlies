#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    acr: ACR,
    keyr: KEYR,
    optkeyr: OPTKEYR,
    sr: SR,
    cr: CR,
    ar: AR,
    _reserved6: [u8; 0x04],
    obr: OBR,
    wrpr: WRPR,
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
    #[doc = "0x0c - Flash status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x10 - Flash control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x14 - Flash address register"]
    #[inline(always)]
    pub const fn ar(&self) -> &AR {
        &self.ar
    }
    #[doc = "0x1c - Option byte register"]
    #[inline(always)]
    pub const fn obr(&self) -> &OBR {
        &self.obr
    }
    #[doc = "0x20 - Write protection register"]
    #[inline(always)]
    pub const fn wrpr(&self) -> &WRPR {
        &self.wrpr
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
#[doc = "SR (rw) register accessor: Flash status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SRrs>;
#[doc = "Flash status register"]
pub mod sr;
#[doc = "CR (rw) register accessor: Flash control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "Flash control register"]
pub mod cr;
#[doc = "AR (w) register accessor: Flash address register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ar::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ar`]
module"]
pub type AR = crate::Reg<ar::ARrs>;
#[doc = "Flash address register"]
pub mod ar;
#[doc = "OBR (r) register accessor: Option byte register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`obr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obr`]
module"]
pub type OBR = crate::Reg<obr::OBRrs>;
#[doc = "Option byte register"]
pub mod obr;
#[doc = "WRPR (r) register accessor: Write protection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrpr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrpr`]
module"]
pub type WRPR = crate::Reg<wrpr::WRPRrs>;
#[doc = "Write protection register"]
pub mod wrpr;

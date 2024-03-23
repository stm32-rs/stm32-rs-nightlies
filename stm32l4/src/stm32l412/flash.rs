#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    acr: ACR,
    pdkeyr: PDKEYR,
    keyr: KEYR,
    optkeyr: OPTKEYR,
    sr: SR,
    cr: CR,
    eccr: ECCR,
    _reserved7: [u8; 0x04],
    optr: OPTR,
    pcrop1sr: PCROP1SR,
    pcrop1er: PCROP1ER,
    wrp1ar: WRP1AR,
    wrp1br: WRP1BR,
    _reserved12: [u8; 0x10],
    pcrop2sr: PCROP2SR,
    pcrop2er: PCROP2ER,
    wrp2ar: WRP2AR,
    wrp2br: WRP2BR,
}
impl RegisterBlock {
    #[doc = "0x00 - Access control register"]
    #[inline(always)]
    pub const fn acr(&self) -> &ACR {
        &self.acr
    }
    #[doc = "0x04 - Power down key register"]
    #[inline(always)]
    pub const fn pdkeyr(&self) -> &PDKEYR {
        &self.pdkeyr
    }
    #[doc = "0x08 - Flash key register"]
    #[inline(always)]
    pub const fn keyr(&self) -> &KEYR {
        &self.keyr
    }
    #[doc = "0x0c - Option byte key register"]
    #[inline(always)]
    pub const fn optkeyr(&self) -> &OPTKEYR {
        &self.optkeyr
    }
    #[doc = "0x10 - Status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x14 - Flash control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x18 - Flash ECC register"]
    #[inline(always)]
    pub const fn eccr(&self) -> &ECCR {
        &self.eccr
    }
    #[doc = "0x20 - Flash option register"]
    #[inline(always)]
    pub const fn optr(&self) -> &OPTR {
        &self.optr
    }
    #[doc = "0x24 - Flash Bank 1 PCROP Start address register"]
    #[inline(always)]
    pub const fn pcrop1sr(&self) -> &PCROP1SR {
        &self.pcrop1sr
    }
    #[doc = "0x28 - Flash Bank 1 PCROP End address register"]
    #[inline(always)]
    pub const fn pcrop1er(&self) -> &PCROP1ER {
        &self.pcrop1er
    }
    #[doc = "0x2c - Flash Bank 1 WRP area A address register"]
    #[inline(always)]
    pub const fn wrp1ar(&self) -> &WRP1AR {
        &self.wrp1ar
    }
    #[doc = "0x30 - Flash Bank 1 WRP area B address register"]
    #[inline(always)]
    pub const fn wrp1br(&self) -> &WRP1BR {
        &self.wrp1br
    }
    #[doc = "0x44 - Flash Bank 2 PCROP Start address register"]
    #[inline(always)]
    pub const fn pcrop2sr(&self) -> &PCROP2SR {
        &self.pcrop2sr
    }
    #[doc = "0x48 - Flash Bank 2 PCROP End address register"]
    #[inline(always)]
    pub const fn pcrop2er(&self) -> &PCROP2ER {
        &self.pcrop2er
    }
    #[doc = "0x4c - Flash Bank 2 WRP area A address register"]
    #[inline(always)]
    pub const fn wrp2ar(&self) -> &WRP2AR {
        &self.wrp2ar
    }
    #[doc = "0x50 - Flash Bank 2 WRP area B address register"]
    #[inline(always)]
    pub const fn wrp2br(&self) -> &WRP2BR {
        &self.wrp2br
    }
}
#[doc = "ACR (rw) register accessor: Access control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acr`]
module"]
pub type ACR = crate::Reg<acr::ACRrs>;
#[doc = "Access control register"]
pub mod acr;
#[doc = "PDKEYR (w) register accessor: Power down key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdkeyr`]
module"]
pub type PDKEYR = crate::Reg<pdkeyr::PDKEYRrs>;
#[doc = "Power down key register"]
pub mod pdkeyr;
#[doc = "KEYR (w) register accessor: Flash key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyr`]
module"]
pub type KEYR = crate::Reg<keyr::KEYRrs>;
#[doc = "Flash key register"]
pub mod keyr;
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
#[doc = "CR (rw) register accessor: Flash control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "Flash control register"]
pub mod cr;
#[doc = "ECCR (rw) register accessor: Flash ECC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccr`]
module"]
pub type ECCR = crate::Reg<eccr::ECCRrs>;
#[doc = "Flash ECC register"]
pub mod eccr;
#[doc = "OPTR (rw) register accessor: Flash option register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`optr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`optr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@optr`]
module"]
pub type OPTR = crate::Reg<optr::OPTRrs>;
#[doc = "Flash option register"]
pub mod optr;
#[doc = "PCROP1SR (rw) register accessor: Flash Bank 1 PCROP Start address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcrop1sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcrop1sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcrop1sr`]
module"]
pub type PCROP1SR = crate::Reg<pcrop1sr::PCROP1SRrs>;
#[doc = "Flash Bank 1 PCROP Start address register"]
pub mod pcrop1sr;
#[doc = "PCROP1ER (rw) register accessor: Flash Bank 1 PCROP End address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcrop1er::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcrop1er::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcrop1er`]
module"]
pub type PCROP1ER = crate::Reg<pcrop1er::PCROP1ERrs>;
#[doc = "Flash Bank 1 PCROP End address register"]
pub mod pcrop1er;
#[doc = "WRP1AR (rw) register accessor: Flash Bank 1 WRP area A address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrp1ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wrp1ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrp1ar`]
module"]
pub type WRP1AR = crate::Reg<wrp1ar::WRP1ARrs>;
#[doc = "Flash Bank 1 WRP area A address register"]
pub mod wrp1ar;
#[doc = "WRP1BR (rw) register accessor: Flash Bank 1 WRP area B address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrp1br::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wrp1br::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrp1br`]
module"]
pub type WRP1BR = crate::Reg<wrp1br::WRP1BRrs>;
#[doc = "Flash Bank 1 WRP area B address register"]
pub mod wrp1br;
#[doc = "PCROP2SR (rw) register accessor: Flash Bank 2 PCROP Start address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcrop2sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcrop2sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcrop2sr`]
module"]
pub type PCROP2SR = crate::Reg<pcrop2sr::PCROP2SRrs>;
#[doc = "Flash Bank 2 PCROP Start address register"]
pub mod pcrop2sr;
#[doc = "PCROP2ER (rw) register accessor: Flash Bank 2 PCROP End address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcrop2er::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcrop2er::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcrop2er`]
module"]
pub type PCROP2ER = crate::Reg<pcrop2er::PCROP2ERrs>;
#[doc = "Flash Bank 2 PCROP End address register"]
pub mod pcrop2er;
#[doc = "WRP2AR (rw) register accessor: Flash Bank 2 WRP area A address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrp2ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wrp2ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrp2ar`]
module"]
pub type WRP2AR = crate::Reg<wrp2ar::WRP2ARrs>;
#[doc = "Flash Bank 2 WRP area A address register"]
pub mod wrp2ar;
#[doc = "WRP2BR (rw) register accessor: Flash Bank 2 WRP area B address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrp2br::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wrp2br::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrp2br`]
module"]
pub type WRP2BR = crate::Reg<wrp2br::WRP2BRrs>;
#[doc = "Flash Bank 2 WRP area B address register"]
pub mod wrp2br;

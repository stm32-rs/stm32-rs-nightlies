#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    acr: ACR,
    _reserved1: [u8; 0x04],
    keyr: KEYR,
    optkeyr: OPTKEYR,
    sr: SR,
    cr: CR,
    eccr: ECCR,
    _reserved6: [u8; 0x04],
    optr: OPTR,
    pcrop1asr: PCROP1ASR,
    pcrop1aer: PCROP1AER,
    wrp1ar: WRP1AR,
    wrp1br: WRP1BR,
    pcrop1bsr: PCROP1BSR,
    pcrop1ber: PCROP1BER,
    _reserved13: [u8; 0x08],
    pcrop2asr: PCROP2ASR,
    pcrop2aer: PCROP2AER,
    wrp2ar: WRP2AR,
    wrp2br: WRP2BR,
    pcrop2bsr: PCROP2BSR,
    pcrop2ber: PCROP2BER,
    _reserved19: [u8; 0x24],
    secr: SECR,
}
impl RegisterBlock {
    #[doc = "0x00 - Access control register"]
    #[inline(always)]
    pub const fn acr(&self) -> &ACR {
        &self.acr
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
    #[doc = "0x24 - Flash PCROP zone A Start address register"]
    #[inline(always)]
    pub const fn pcrop1asr(&self) -> &PCROP1ASR {
        &self.pcrop1asr
    }
    #[doc = "0x28 - Flash PCROP zone A End address register"]
    #[inline(always)]
    pub const fn pcrop1aer(&self) -> &PCROP1AER {
        &self.pcrop1aer
    }
    #[doc = "0x2c - Flash WRP area A address register"]
    #[inline(always)]
    pub const fn wrp1ar(&self) -> &WRP1AR {
        &self.wrp1ar
    }
    #[doc = "0x30 - Flash WRP area B address register"]
    #[inline(always)]
    pub const fn wrp1br(&self) -> &WRP1BR {
        &self.wrp1br
    }
    #[doc = "0x34 - Flash PCROP zone B Start address register"]
    #[inline(always)]
    pub const fn pcrop1bsr(&self) -> &PCROP1BSR {
        &self.pcrop1bsr
    }
    #[doc = "0x38 - Flash PCROP area B End address register"]
    #[inline(always)]
    pub const fn pcrop1ber(&self) -> &PCROP1BER {
        &self.pcrop1ber
    }
    #[doc = "0x44 - Flash PCROP2 area A start address register"]
    #[inline(always)]
    pub const fn pcrop2asr(&self) -> &PCROP2ASR {
        &self.pcrop2asr
    }
    #[doc = "0x48 - Flash PCROP2 area A end address register"]
    #[inline(always)]
    pub const fn pcrop2aer(&self) -> &PCROP2AER {
        &self.pcrop2aer
    }
    #[doc = "0x4c - Flash WRP2 area A address register"]
    #[inline(always)]
    pub const fn wrp2ar(&self) -> &WRP2AR {
        &self.wrp2ar
    }
    #[doc = "0x50 - Flash WRP2 area B address register"]
    #[inline(always)]
    pub const fn wrp2br(&self) -> &WRP2BR {
        &self.wrp2br
    }
    #[doc = "0x54 - FLASH PCROP2 area B start address register"]
    #[inline(always)]
    pub const fn pcrop2bsr(&self) -> &PCROP2BSR {
        &self.pcrop2bsr
    }
    #[doc = "0x58 - FLASH PCROP2 area B end address register"]
    #[inline(always)]
    pub const fn pcrop2ber(&self) -> &PCROP2BER {
        &self.pcrop2ber
    }
    #[doc = "0x80 - Flash Security register"]
    #[inline(always)]
    pub const fn secr(&self) -> &SECR {
        &self.secr
    }
}
#[doc = "ACR (rw) register accessor: Access control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acr`]
module"]
pub type ACR = crate::Reg<acr::ACRrs>;
#[doc = "Access control register"]
pub mod acr;
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
#[doc = "PCROP1ASR (rw) register accessor: Flash PCROP zone A Start address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcrop1asr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcrop1asr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcrop1asr`]
module"]
pub type PCROP1ASR = crate::Reg<pcrop1asr::PCROP1ASRrs>;
#[doc = "Flash PCROP zone A Start address register"]
pub mod pcrop1asr;
#[doc = "PCROP1AER (rw) register accessor: Flash PCROP zone A End address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcrop1aer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcrop1aer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcrop1aer`]
module"]
pub type PCROP1AER = crate::Reg<pcrop1aer::PCROP1AERrs>;
#[doc = "Flash PCROP zone A End address register"]
pub mod pcrop1aer;
#[doc = "WRP1AR (rw) register accessor: Flash WRP area A address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrp1ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wrp1ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrp1ar`]
module"]
pub type WRP1AR = crate::Reg<wrp1ar::WRP1ARrs>;
#[doc = "Flash WRP area A address register"]
pub mod wrp1ar;
#[doc = "WRP1BR (rw) register accessor: Flash WRP area B address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrp1br::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wrp1br::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrp1br`]
module"]
pub type WRP1BR = crate::Reg<wrp1br::WRP1BRrs>;
#[doc = "Flash WRP area B address register"]
pub mod wrp1br;
#[doc = "PCROP1BSR (rw) register accessor: Flash PCROP zone B Start address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcrop1bsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcrop1bsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcrop1bsr`]
module"]
pub type PCROP1BSR = crate::Reg<pcrop1bsr::PCROP1BSRrs>;
#[doc = "Flash PCROP zone B Start address register"]
pub mod pcrop1bsr;
#[doc = "PCROP1BER (rw) register accessor: Flash PCROP area B End address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcrop1ber::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcrop1ber::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcrop1ber`]
module"]
pub type PCROP1BER = crate::Reg<pcrop1ber::PCROP1BERrs>;
#[doc = "Flash PCROP area B End address register"]
pub mod pcrop1ber;
#[doc = "PCROP2ASR (rw) register accessor: Flash PCROP2 area A start address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcrop2asr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcrop2asr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcrop2asr`]
module"]
pub type PCROP2ASR = crate::Reg<pcrop2asr::PCROP2ASRrs>;
#[doc = "Flash PCROP2 area A start address register"]
pub mod pcrop2asr;
#[doc = "PCROP2AER (rw) register accessor: Flash PCROP2 area A end address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcrop2aer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcrop2aer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcrop2aer`]
module"]
pub type PCROP2AER = crate::Reg<pcrop2aer::PCROP2AERrs>;
#[doc = "Flash PCROP2 area A end address register"]
pub mod pcrop2aer;
#[doc = "WRP2AR (rw) register accessor: Flash WRP2 area A address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrp2ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wrp2ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrp2ar`]
module"]
pub type WRP2AR = crate::Reg<wrp2ar::WRP2ARrs>;
#[doc = "Flash WRP2 area A address register"]
pub mod wrp2ar;
#[doc = "WRP2BR (rw) register accessor: Flash WRP2 area B address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrp2br::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wrp2br::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrp2br`]
module"]
pub type WRP2BR = crate::Reg<wrp2br::WRP2BRrs>;
#[doc = "Flash WRP2 area B address register"]
pub mod wrp2br;
#[doc = "PCROP2BSR (rw) register accessor: FLASH PCROP2 area B start address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcrop2bsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcrop2bsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcrop2bsr`]
module"]
pub type PCROP2BSR = crate::Reg<pcrop2bsr::PCROP2BSRrs>;
#[doc = "FLASH PCROP2 area B start address register"]
pub mod pcrop2bsr;
#[doc = "PCROP2BER (rw) register accessor: FLASH PCROP2 area B end address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcrop2ber::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcrop2ber::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcrop2ber`]
module"]
pub type PCROP2BER = crate::Reg<pcrop2ber::PCROP2BERrs>;
#[doc = "FLASH PCROP2 area B end address register"]
pub mod pcrop2ber;
#[doc = "SECR (rw) register accessor: Flash Security register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`secr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`secr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secr`]
module"]
pub type SECR = crate::Reg<secr::SECRrs>;
#[doc = "Flash Security register"]
pub mod secr;

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
    ipccbr: IPCCBR,
    _reserved14: [u8; 0x1c],
    c2acr: C2ACR,
    c2sr: C2SR,
    c2cr: C2CR,
    _reserved17: [u8; 0x18],
    sfr: SFR,
    srrvr: SRRVR,
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
    #[doc = "0x24 - Flash Bank 1 PCROP Start address zone A register"]
    #[inline(always)]
    pub const fn pcrop1asr(&self) -> &PCROP1ASR {
        &self.pcrop1asr
    }
    #[doc = "0x28 - Flash Bank 1 PCROP End address zone A register"]
    #[inline(always)]
    pub const fn pcrop1aer(&self) -> &PCROP1AER {
        &self.pcrop1aer
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
    #[doc = "0x34 - Flash Bank 1 PCROP Start address area B register"]
    #[inline(always)]
    pub const fn pcrop1bsr(&self) -> &PCROP1BSR {
        &self.pcrop1bsr
    }
    #[doc = "0x38 - Flash Bank 1 PCROP End address area B register"]
    #[inline(always)]
    pub const fn pcrop1ber(&self) -> &PCROP1BER {
        &self.pcrop1ber
    }
    #[doc = "0x3c - IPCC mailbox data buffer address register"]
    #[inline(always)]
    pub const fn ipccbr(&self) -> &IPCCBR {
        &self.ipccbr
    }
    #[doc = "0x5c - CPU2 cortex M0 access control register"]
    #[inline(always)]
    pub const fn c2acr(&self) -> &C2ACR {
        &self.c2acr
    }
    #[doc = "0x60 - CPU2 cortex M0 status register"]
    #[inline(always)]
    pub const fn c2sr(&self) -> &C2SR {
        &self.c2sr
    }
    #[doc = "0x64 - CPU2 cortex M0 control register"]
    #[inline(always)]
    pub const fn c2cr(&self) -> &C2CR {
        &self.c2cr
    }
    #[doc = "0x80 - Secure flash start address register"]
    #[inline(always)]
    pub const fn sfr(&self) -> &SFR {
        &self.sfr
    }
    #[doc = "0x84 - Secure SRAM2 start address and cortex M0 reset vector register"]
    #[inline(always)]
    pub const fn srrvr(&self) -> &SRRVR {
        &self.srrvr
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
#[doc = "PCROP1ASR (rw) register accessor: Flash Bank 1 PCROP Start address zone A register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcrop1asr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcrop1asr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcrop1asr`]
module"]
pub type PCROP1ASR = crate::Reg<pcrop1asr::PCROP1ASRrs>;
#[doc = "Flash Bank 1 PCROP Start address zone A register"]
pub mod pcrop1asr;
#[doc = "PCROP1AER (rw) register accessor: Flash Bank 1 PCROP End address zone A register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcrop1aer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcrop1aer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcrop1aer`]
module"]
pub type PCROP1AER = crate::Reg<pcrop1aer::PCROP1AERrs>;
#[doc = "Flash Bank 1 PCROP End address zone A register"]
pub mod pcrop1aer;
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
#[doc = "PCROP1BSR (rw) register accessor: Flash Bank 1 PCROP Start address area B register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcrop1bsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcrop1bsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcrop1bsr`]
module"]
pub type PCROP1BSR = crate::Reg<pcrop1bsr::PCROP1BSRrs>;
#[doc = "Flash Bank 1 PCROP Start address area B register"]
pub mod pcrop1bsr;
#[doc = "PCROP1BER (rw) register accessor: Flash Bank 1 PCROP End address area B register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcrop1ber::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcrop1ber::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcrop1ber`]
module"]
pub type PCROP1BER = crate::Reg<pcrop1ber::PCROP1BERrs>;
#[doc = "Flash Bank 1 PCROP End address area B register"]
pub mod pcrop1ber;
#[doc = "IPCCBR (rw) register accessor: IPCC mailbox data buffer address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipccbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipccbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipccbr`]
module"]
pub type IPCCBR = crate::Reg<ipccbr::IPCCBRrs>;
#[doc = "IPCC mailbox data buffer address register"]
pub mod ipccbr;
#[doc = "C2ACR (rw) register accessor: CPU2 cortex M0 access control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2acr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2acr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2acr`]
module"]
pub type C2ACR = crate::Reg<c2acr::C2ACRrs>;
#[doc = "CPU2 cortex M0 access control register"]
pub mod c2acr;
#[doc = "C2SR (rw) register accessor: CPU2 cortex M0 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2sr`]
module"]
pub type C2SR = crate::Reg<c2sr::C2SRrs>;
#[doc = "CPU2 cortex M0 status register"]
pub mod c2sr;
#[doc = "C2CR (rw) register accessor: CPU2 cortex M0 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2cr`]
module"]
pub type C2CR = crate::Reg<c2cr::C2CRrs>;
#[doc = "CPU2 cortex M0 control register"]
pub mod c2cr;
#[doc = "SFR (rw) register accessor: Secure flash start address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr`]
module"]
pub type SFR = crate::Reg<sfr::SFRrs>;
#[doc = "Secure flash start address register"]
pub mod sfr;
#[doc = "SRRVR (rw) register accessor: Secure SRAM2 start address and cortex M0 reset vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srrvr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srrvr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srrvr`]
module"]
pub type SRRVR = crate::Reg<srrvr::SRRVRrs>;
#[doc = "Secure SRAM2 start address and cortex M0 reset vector register"]
pub mod srrvr;

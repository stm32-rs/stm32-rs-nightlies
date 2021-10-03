#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Access control register"]
    pub acr: crate::Reg<acr::ACR_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - Flash key register"]
    pub keyr: crate::Reg<keyr::KEYR_SPEC>,
    #[doc = "0x0c - Option byte key register"]
    pub optkeyr: crate::Reg<optkeyr::OPTKEYR_SPEC>,
    #[doc = "0x10 - Status register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x14 - Flash control register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x18 - Flash ECC register"]
    pub eccr: crate::Reg<eccr::ECCR_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x20 - Flash option register"]
    pub optr: crate::Reg<optr::OPTR_SPEC>,
    #[doc = "0x24 - Flash Bank 1 PCROP Start address zone A register"]
    pub pcrop1asr: crate::Reg<pcrop1asr::PCROP1ASR_SPEC>,
    #[doc = "0x28 - Flash Bank 1 PCROP End address zone A register"]
    pub pcrop1aer: crate::Reg<pcrop1aer::PCROP1AER_SPEC>,
    #[doc = "0x2c - Flash Bank 1 WRP area A address register"]
    pub wrp1ar: crate::Reg<wrp1ar::WRP1AR_SPEC>,
    #[doc = "0x30 - Flash Bank 1 WRP area B address register"]
    pub wrp1br: crate::Reg<wrp1br::WRP1BR_SPEC>,
    #[doc = "0x34 - Flash Bank 1 PCROP Start address area B register"]
    pub pcrop1bsr: crate::Reg<pcrop1bsr::PCROP1BSR_SPEC>,
    #[doc = "0x38 - Flash Bank 1 PCROP End address area B register"]
    pub pcrop1ber: crate::Reg<pcrop1ber::PCROP1BER_SPEC>,
    #[doc = "0x3c - IPCC mailbox data buffer address register"]
    pub ipccbr: crate::Reg<ipccbr::IPCCBR_SPEC>,
    _reserved14: [u8; 0x1c],
    #[doc = "0x5c - CPU2 cortex M0 access control register"]
    pub c2acr: crate::Reg<c2acr::C2ACR_SPEC>,
    #[doc = "0x60 - CPU2 cortex M0 status register"]
    pub c2sr: crate::Reg<c2sr::C2SR_SPEC>,
    #[doc = "0x64 - CPU2 cortex M0 control register"]
    pub c2cr: crate::Reg<c2cr::C2CR_SPEC>,
    _reserved17: [u8; 0x18],
    #[doc = "0x80 - Secure flash start address register"]
    pub sfr: crate::Reg<sfr::SFR_SPEC>,
    #[doc = "0x84 - Secure SRAM2 start address and cortex M0 reset vector register"]
    pub srrvr: crate::Reg<srrvr::SRRVR_SPEC>,
}
#[doc = "ACR register accessor: an alias for `Reg<ACR_SPEC>`"]
pub type ACR = crate::Reg<acr::ACR_SPEC>;
#[doc = "Access control register"]
pub mod acr;
#[doc = "KEYR register accessor: an alias for `Reg<KEYR_SPEC>`"]
pub type KEYR = crate::Reg<keyr::KEYR_SPEC>;
#[doc = "Flash key register"]
pub mod keyr;
#[doc = "OPTKEYR register accessor: an alias for `Reg<OPTKEYR_SPEC>`"]
pub type OPTKEYR = crate::Reg<optkeyr::OPTKEYR_SPEC>;
#[doc = "Option byte key register"]
pub mod optkeyr;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status register"]
pub mod sr;
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Flash control register"]
pub mod cr;
#[doc = "ECCR register accessor: an alias for `Reg<ECCR_SPEC>`"]
pub type ECCR = crate::Reg<eccr::ECCR_SPEC>;
#[doc = "Flash ECC register"]
pub mod eccr;
#[doc = "OPTR register accessor: an alias for `Reg<OPTR_SPEC>`"]
pub type OPTR = crate::Reg<optr::OPTR_SPEC>;
#[doc = "Flash option register"]
pub mod optr;
#[doc = "PCROP1ASR register accessor: an alias for `Reg<PCROP1ASR_SPEC>`"]
pub type PCROP1ASR = crate::Reg<pcrop1asr::PCROP1ASR_SPEC>;
#[doc = "Flash Bank 1 PCROP Start address zone A register"]
pub mod pcrop1asr;
#[doc = "PCROP1AER register accessor: an alias for `Reg<PCROP1AER_SPEC>`"]
pub type PCROP1AER = crate::Reg<pcrop1aer::PCROP1AER_SPEC>;
#[doc = "Flash Bank 1 PCROP End address zone A register"]
pub mod pcrop1aer;
#[doc = "WRP1AR register accessor: an alias for `Reg<WRP1AR_SPEC>`"]
pub type WRP1AR = crate::Reg<wrp1ar::WRP1AR_SPEC>;
#[doc = "Flash Bank 1 WRP area A address register"]
pub mod wrp1ar;
#[doc = "WRP1BR register accessor: an alias for `Reg<WRP1BR_SPEC>`"]
pub type WRP1BR = crate::Reg<wrp1br::WRP1BR_SPEC>;
#[doc = "Flash Bank 1 WRP area B address register"]
pub mod wrp1br;
#[doc = "PCROP1BSR register accessor: an alias for `Reg<PCROP1BSR_SPEC>`"]
pub type PCROP1BSR = crate::Reg<pcrop1bsr::PCROP1BSR_SPEC>;
#[doc = "Flash Bank 1 PCROP Start address area B register"]
pub mod pcrop1bsr;
#[doc = "PCROP1BER register accessor: an alias for `Reg<PCROP1BER_SPEC>`"]
pub type PCROP1BER = crate::Reg<pcrop1ber::PCROP1BER_SPEC>;
#[doc = "Flash Bank 1 PCROP End address area B register"]
pub mod pcrop1ber;
#[doc = "IPCCBR register accessor: an alias for `Reg<IPCCBR_SPEC>`"]
pub type IPCCBR = crate::Reg<ipccbr::IPCCBR_SPEC>;
#[doc = "IPCC mailbox data buffer address register"]
pub mod ipccbr;
#[doc = "C2ACR register accessor: an alias for `Reg<C2ACR_SPEC>`"]
pub type C2ACR = crate::Reg<c2acr::C2ACR_SPEC>;
#[doc = "CPU2 cortex M0 access control register"]
pub mod c2acr;
#[doc = "C2SR register accessor: an alias for `Reg<C2SR_SPEC>`"]
pub type C2SR = crate::Reg<c2sr::C2SR_SPEC>;
#[doc = "CPU2 cortex M0 status register"]
pub mod c2sr;
#[doc = "C2CR register accessor: an alias for `Reg<C2CR_SPEC>`"]
pub type C2CR = crate::Reg<c2cr::C2CR_SPEC>;
#[doc = "CPU2 cortex M0 control register"]
pub mod c2cr;
#[doc = "SFR register accessor: an alias for `Reg<SFR_SPEC>`"]
pub type SFR = crate::Reg<sfr::SFR_SPEC>;
#[doc = "Secure flash start address register"]
pub mod sfr;
#[doc = "SRRVR register accessor: an alias for `Reg<SRRVR_SPEC>`"]
pub type SRRVR = crate::Reg<srrvr::SRRVR_SPEC>;
#[doc = "Secure SRAM2 start address and cortex M0 reset vector register"]
pub mod srrvr;

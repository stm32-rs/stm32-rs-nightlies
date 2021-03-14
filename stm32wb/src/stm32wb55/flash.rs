#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Access control register"]
    pub acr: ACR,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - Flash key register"]
    pub keyr: KEYR,
    #[doc = "0x0c - Option byte key register"]
    pub optkeyr: OPTKEYR,
    #[doc = "0x10 - Status register"]
    pub sr: SR,
    #[doc = "0x14 - Flash control register"]
    pub cr: CR,
    #[doc = "0x18 - Flash ECC register"]
    pub eccr: ECCR,
    _reserved6: [u8; 4usize],
    #[doc = "0x20 - Flash option register"]
    pub optr: OPTR,
    #[doc = "0x24 - Flash Bank 1 PCROP Start address zone A register"]
    pub pcrop1asr: PCROP1ASR,
    #[doc = "0x28 - Flash Bank 1 PCROP End address zone A register"]
    pub pcrop1aer: PCROP1AER,
    #[doc = "0x2c - Flash Bank 1 WRP area A address register"]
    pub wrp1ar: WRP1AR,
    #[doc = "0x30 - Flash Bank 1 WRP area B address register"]
    pub wrp1br: WRP1BR,
    #[doc = "0x34 - Flash Bank 1 PCROP Start address area B register"]
    pub pcrop1bsr: PCROP1BSR,
    #[doc = "0x38 - Flash Bank 1 PCROP End address area B register"]
    pub pcrop1ber: PCROP1BER,
    #[doc = "0x3c - IPCC mailbox data buffer address register"]
    pub ipccbr: IPCCBR,
    _reserved14: [u8; 28usize],
    #[doc = "0x5c - CPU2 cortex M0 access control register"]
    pub c2acr: C2ACR,
    #[doc = "0x60 - CPU2 cortex M0 status register"]
    pub c2sr: C2SR,
    #[doc = "0x64 - CPU2 cortex M0 control register"]
    pub c2cr: C2CR,
    _reserved17: [u8; 24usize],
    #[doc = "0x80 - Secure flash start address register"]
    pub sfr: SFR,
    #[doc = "0x84 - Secure SRAM2 start address and cortex M0 reset vector register"]
    pub srrvr: SRRVR,
}
#[doc = "Access control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acr](acr) module"]
pub type ACR = crate::Reg<u32, _ACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACR;
#[doc = "`read()` method returns [acr::R](acr::R) reader structure"]
impl crate::Readable for ACR {}
#[doc = "`write(|w| ..)` method takes [acr::W](acr::W) writer structure"]
impl crate::Writable for ACR {}
#[doc = "Access control register"]
pub mod acr;
#[doc = "Flash key register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyr](keyr) module"]
pub type KEYR = crate::Reg<u32, _KEYR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEYR;
#[doc = "`write(|w| ..)` method takes [keyr::W](keyr::W) writer structure"]
impl crate::Writable for KEYR {}
#[doc = "Flash key register"]
pub mod keyr;
#[doc = "Option byte key register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [optkeyr](optkeyr) module"]
pub type OPTKEYR = crate::Reg<u32, _OPTKEYR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPTKEYR;
#[doc = "`write(|w| ..)` method takes [optkeyr::W](optkeyr::W) writer structure"]
impl crate::Writable for OPTKEYR {}
#[doc = "Option byte key register"]
pub mod optkeyr;
#[doc = "Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "`write(|w| ..)` method takes [sr::W](sr::W) writer structure"]
impl crate::Writable for SR {}
#[doc = "Status register"]
pub mod sr;
#[doc = "Flash control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "Flash control register"]
pub mod cr;
#[doc = "Flash ECC register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eccr](eccr) module"]
pub type ECCR = crate::Reg<u32, _ECCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECCR;
#[doc = "`read()` method returns [eccr::R](eccr::R) reader structure"]
impl crate::Readable for ECCR {}
#[doc = "`write(|w| ..)` method takes [eccr::W](eccr::W) writer structure"]
impl crate::Writable for ECCR {}
#[doc = "Flash ECC register"]
pub mod eccr;
#[doc = "Flash option register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [optr](optr) module"]
pub type OPTR = crate::Reg<u32, _OPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPTR;
#[doc = "`read()` method returns [optr::R](optr::R) reader structure"]
impl crate::Readable for OPTR {}
#[doc = "`write(|w| ..)` method takes [optr::W](optr::W) writer structure"]
impl crate::Writable for OPTR {}
#[doc = "Flash option register"]
pub mod optr;
#[doc = "Flash Bank 1 PCROP Start address zone A register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcrop1asr](pcrop1asr) module"]
pub type PCROP1ASR = crate::Reg<u32, _PCROP1ASR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCROP1ASR;
#[doc = "`read()` method returns [pcrop1asr::R](pcrop1asr::R) reader structure"]
impl crate::Readable for PCROP1ASR {}
#[doc = "`write(|w| ..)` method takes [pcrop1asr::W](pcrop1asr::W) writer structure"]
impl crate::Writable for PCROP1ASR {}
#[doc = "Flash Bank 1 PCROP Start address zone A register"]
pub mod pcrop1asr;
#[doc = "Flash Bank 1 PCROP End address zone A register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcrop1aer](pcrop1aer) module"]
pub type PCROP1AER = crate::Reg<u32, _PCROP1AER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCROP1AER;
#[doc = "`read()` method returns [pcrop1aer::R](pcrop1aer::R) reader structure"]
impl crate::Readable for PCROP1AER {}
#[doc = "`write(|w| ..)` method takes [pcrop1aer::W](pcrop1aer::W) writer structure"]
impl crate::Writable for PCROP1AER {}
#[doc = "Flash Bank 1 PCROP End address zone A register"]
pub mod pcrop1aer;
#[doc = "Flash Bank 1 WRP area A address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrp1ar](wrp1ar) module"]
pub type WRP1AR = crate::Reg<u32, _WRP1AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WRP1AR;
#[doc = "`read()` method returns [wrp1ar::R](wrp1ar::R) reader structure"]
impl crate::Readable for WRP1AR {}
#[doc = "`write(|w| ..)` method takes [wrp1ar::W](wrp1ar::W) writer structure"]
impl crate::Writable for WRP1AR {}
#[doc = "Flash Bank 1 WRP area A address register"]
pub mod wrp1ar;
#[doc = "Flash Bank 1 WRP area B address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrp1br](wrp1br) module"]
pub type WRP1BR = crate::Reg<u32, _WRP1BR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WRP1BR;
#[doc = "`read()` method returns [wrp1br::R](wrp1br::R) reader structure"]
impl crate::Readable for WRP1BR {}
#[doc = "`write(|w| ..)` method takes [wrp1br::W](wrp1br::W) writer structure"]
impl crate::Writable for WRP1BR {}
#[doc = "Flash Bank 1 WRP area B address register"]
pub mod wrp1br;
#[doc = "Flash Bank 1 PCROP Start address area B register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcrop1bsr](pcrop1bsr) module"]
pub type PCROP1BSR = crate::Reg<u32, _PCROP1BSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCROP1BSR;
#[doc = "`read()` method returns [pcrop1bsr::R](pcrop1bsr::R) reader structure"]
impl crate::Readable for PCROP1BSR {}
#[doc = "`write(|w| ..)` method takes [pcrop1bsr::W](pcrop1bsr::W) writer structure"]
impl crate::Writable for PCROP1BSR {}
#[doc = "Flash Bank 1 PCROP Start address area B register"]
pub mod pcrop1bsr;
#[doc = "Flash Bank 1 PCROP End address area B register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcrop1ber](pcrop1ber) module"]
pub type PCROP1BER = crate::Reg<u32, _PCROP1BER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCROP1BER;
#[doc = "`read()` method returns [pcrop1ber::R](pcrop1ber::R) reader structure"]
impl crate::Readable for PCROP1BER {}
#[doc = "`write(|w| ..)` method takes [pcrop1ber::W](pcrop1ber::W) writer structure"]
impl crate::Writable for PCROP1BER {}
#[doc = "Flash Bank 1 PCROP End address area B register"]
pub mod pcrop1ber;
#[doc = "IPCC mailbox data buffer address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipccbr](ipccbr) module"]
pub type IPCCBR = crate::Reg<u32, _IPCCBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPCCBR;
#[doc = "`read()` method returns [ipccbr::R](ipccbr::R) reader structure"]
impl crate::Readable for IPCCBR {}
#[doc = "`write(|w| ..)` method takes [ipccbr::W](ipccbr::W) writer structure"]
impl crate::Writable for IPCCBR {}
#[doc = "IPCC mailbox data buffer address register"]
pub mod ipccbr;
#[doc = "CPU2 cortex M0 access control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2acr](c2acr) module"]
pub type C2ACR = crate::Reg<u32, _C2ACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2ACR;
#[doc = "`read()` method returns [c2acr::R](c2acr::R) reader structure"]
impl crate::Readable for C2ACR {}
#[doc = "`write(|w| ..)` method takes [c2acr::W](c2acr::W) writer structure"]
impl crate::Writable for C2ACR {}
#[doc = "CPU2 cortex M0 access control register"]
pub mod c2acr;
#[doc = "CPU2 cortex M0 status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2sr](c2sr) module"]
pub type C2SR = crate::Reg<u32, _C2SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2SR;
#[doc = "`read()` method returns [c2sr::R](c2sr::R) reader structure"]
impl crate::Readable for C2SR {}
#[doc = "`write(|w| ..)` method takes [c2sr::W](c2sr::W) writer structure"]
impl crate::Writable for C2SR {}
#[doc = "CPU2 cortex M0 status register"]
pub mod c2sr;
#[doc = "CPU2 cortex M0 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2cr](c2cr) module"]
pub type C2CR = crate::Reg<u32, _C2CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2CR;
#[doc = "`read()` method returns [c2cr::R](c2cr::R) reader structure"]
impl crate::Readable for C2CR {}
#[doc = "`write(|w| ..)` method takes [c2cr::W](c2cr::W) writer structure"]
impl crate::Writable for C2CR {}
#[doc = "CPU2 cortex M0 control register"]
pub mod c2cr;
#[doc = "Secure flash start address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfr](sfr) module"]
pub type SFR = crate::Reg<u32, _SFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SFR;
#[doc = "`read()` method returns [sfr::R](sfr::R) reader structure"]
impl crate::Readable for SFR {}
#[doc = "`write(|w| ..)` method takes [sfr::W](sfr::W) writer structure"]
impl crate::Writable for SFR {}
#[doc = "Secure flash start address register"]
pub mod sfr;
#[doc = "Secure SRAM2 start address and cortex M0 reset vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srrvr](srrvr) module"]
pub type SRRVR = crate::Reg<u32, _SRRVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRRVR;
#[doc = "`read()` method returns [srrvr::R](srrvr::R) reader structure"]
impl crate::Readable for SRRVR {}
#[doc = "`write(|w| ..)` method takes [srrvr::W](srrvr::W) writer structure"]
impl crate::Writable for SRRVR {}
#[doc = "Secure SRAM2 start address and cortex M0 reset vector register"]
pub mod srrvr;

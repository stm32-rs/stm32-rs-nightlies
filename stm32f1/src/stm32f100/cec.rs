#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - configuration register"]
    pub cfgr: CFGR,
    #[doc = "0x04 - CEC own address register"]
    pub oar: OAR,
    #[doc = "0x08 - Rx Data Register"]
    pub pres: PRES,
    #[doc = "0x0c - CEC error status register"]
    pub esr: ESR,
    #[doc = "0x10 - CEC control and status register"]
    pub csr: CSR,
    #[doc = "0x14 - CEC Tx data register"]
    pub txd: TXD,
    #[doc = "0x18 - CEC Rx data register"]
    pub rxd: RXD,
}
#[doc = "configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr](cfgr) module"]
pub type CFGR = crate::Reg<u32, _CFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGR;
#[doc = "`read()` method returns [cfgr::R](cfgr::R) reader structure"]
impl crate::Readable for CFGR {}
#[doc = "`write(|w| ..)` method takes [cfgr::W](cfgr::W) writer structure"]
impl crate::Writable for CFGR {}
#[doc = "configuration register"]
pub mod cfgr;
#[doc = "CEC own address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oar](oar) module"]
pub type OAR = crate::Reg<u32, _OAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OAR;
#[doc = "`read()` method returns [oar::R](oar::R) reader structure"]
impl crate::Readable for OAR {}
#[doc = "`write(|w| ..)` method takes [oar::W](oar::W) writer structure"]
impl crate::Writable for OAR {}
#[doc = "CEC own address register"]
pub mod oar;
#[doc = "Rx Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pres](pres) module"]
pub type PRES = crate::Reg<u32, _PRES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRES;
#[doc = "`read()` method returns [pres::R](pres::R) reader structure"]
impl crate::Readable for PRES {}
#[doc = "`write(|w| ..)` method takes [pres::W](pres::W) writer structure"]
impl crate::Writable for PRES {}
#[doc = "Rx Data Register"]
pub mod pres;
#[doc = "CEC error status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [esr](esr) module"]
pub type ESR = crate::Reg<u32, _ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ESR;
#[doc = "`read()` method returns [esr::R](esr::R) reader structure"]
impl crate::Readable for ESR {}
#[doc = "CEC error status register"]
pub mod esr;
#[doc = "CEC control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](csr) module"]
pub type CSR = crate::Reg<u32, _CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR;
#[doc = "`read()` method returns [csr::R](csr::R) reader structure"]
impl crate::Readable for CSR {}
#[doc = "`write(|w| ..)` method takes [csr::W](csr::W) writer structure"]
impl crate::Writable for CSR {}
#[doc = "CEC control and status register"]
pub mod csr;
#[doc = "CEC Tx data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txd](txd) module"]
pub type TXD = crate::Reg<u32, _TXD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXD;
#[doc = "`read()` method returns [txd::R](txd::R) reader structure"]
impl crate::Readable for TXD {}
#[doc = "`write(|w| ..)` method takes [txd::W](txd::W) writer structure"]
impl crate::Writable for TXD {}
#[doc = "CEC Tx data register"]
pub mod txd;
#[doc = "CEC Rx data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxd](rxd) module"]
pub type RXD = crate::Reg<u32, _RXD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXD;
#[doc = "`read()` method returns [rxd::R](rxd::R) reader structure"]
impl crate::Readable for RXD {}
#[doc = "CEC Rx data register"]
pub mod rxd;

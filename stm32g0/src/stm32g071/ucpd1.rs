#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - UCPD configuration register"]
    pub cfg1: CFG1,
    #[doc = "0x04 - UCPD configuration register 2"]
    pub cfg2: CFG2,
    #[doc = "0x08 - UCPD configuration register 3"]
    pub cfg3: CFG3,
    #[doc = "0x0c - UCPD control register"]
    pub cr: CR,
    #[doc = "0x10 - UCPD Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x14 - UCPD Status Register"]
    pub sr: SR,
    #[doc = "0x18 - UCPD Interrupt Clear Register"]
    pub icr: ICR,
    #[doc = "0x1c - UCPD Tx Ordered Set Type Register"]
    pub tx_ordset: TX_ORDSET,
    #[doc = "0x20 - UCPD Tx Paysize Register"]
    pub tx_paysz: TX_PAYSZ,
    #[doc = "0x24 - UCPD Tx Data Register"]
    pub txdr: TXDR,
    #[doc = "0x28 - UCPD Rx Ordered Set Register"]
    pub rx_ordset: RX_ORDSET,
    #[doc = "0x2c - UCPD Rx Paysize Register"]
    pub rx_paysz: RX_PAYSZ,
    #[doc = "0x30 - UCPD Receive Data Register"]
    pub rxdr: RXDR,
    #[doc = "0x34 - UCPD Rx Ordered Set Extension Register"]
    pub rx_ordext1: RX_ORDEXT1,
    #[doc = "0x38 - UCPD Rx Ordered Set Extension Register"]
    pub rx_ordext2: RX_ORDEXT2,
    _reserved15: [u8; 952usize],
    #[doc = "0x3f4 - UCPD IP ID register"]
    pub ipver: IPVER,
    #[doc = "0x3f8 - UCPD IP ID register"]
    pub ipid: IPID,
    #[doc = "0x3fc - UCPD IP ID register"]
    pub mid: MID,
}
#[doc = "UCPD configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg1](cfg1) module"]
pub type CFG1 = crate::Reg<u32, _CFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG1;
#[doc = "`read()` method returns [cfg1::R](cfg1::R) reader structure"]
impl crate::Readable for CFG1 {}
#[doc = "`write(|w| ..)` method takes [cfg1::W](cfg1::W) writer structure"]
impl crate::Writable for CFG1 {}
#[doc = "UCPD configuration register"]
pub mod cfg1;
#[doc = "UCPD configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg2](cfg2) module"]
pub type CFG2 = crate::Reg<u32, _CFG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG2;
#[doc = "`read()` method returns [cfg2::R](cfg2::R) reader structure"]
impl crate::Readable for CFG2 {}
#[doc = "`write(|w| ..)` method takes [cfg2::W](cfg2::W) writer structure"]
impl crate::Writable for CFG2 {}
#[doc = "UCPD configuration register 2"]
pub mod cfg2;
#[doc = "UCPD configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg3](cfg3) module"]
pub type CFG3 = crate::Reg<u32, _CFG3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG3;
#[doc = "`read()` method returns [cfg3::R](cfg3::R) reader structure"]
impl crate::Readable for CFG3 {}
#[doc = "`write(|w| ..)` method takes [cfg3::W](cfg3::W) writer structure"]
impl crate::Writable for CFG3 {}
#[doc = "UCPD configuration register 3"]
pub mod cfg3;
#[doc = "UCPD control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "UCPD control register"]
pub mod cr;
#[doc = "UCPD Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](imr) module"]
pub type IMR = crate::Reg<u32, _IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR;
#[doc = "`read()` method returns [imr::R](imr::R) reader structure"]
impl crate::Readable for IMR {}
#[doc = "`write(|w| ..)` method takes [imr::W](imr::W) writer structure"]
impl crate::Writable for IMR {}
#[doc = "UCPD Interrupt Mask Register"]
pub mod imr;
#[doc = "UCPD Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "UCPD Status Register"]
pub mod sr;
#[doc = "UCPD Interrupt Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](icr) module"]
pub type ICR = crate::Reg<u32, _ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICR;
#[doc = "`read()` method returns [icr::R](icr::R) reader structure"]
impl crate::Readable for ICR {}
#[doc = "`write(|w| ..)` method takes [icr::W](icr::W) writer structure"]
impl crate::Writable for ICR {}
#[doc = "UCPD Interrupt Clear Register"]
pub mod icr;
#[doc = "UCPD Tx Ordered Set Type Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_ordset](tx_ordset) module"]
pub type TX_ORDSET = crate::Reg<u32, _TX_ORDSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_ORDSET;
#[doc = "`read()` method returns [tx_ordset::R](tx_ordset::R) reader structure"]
impl crate::Readable for TX_ORDSET {}
#[doc = "`write(|w| ..)` method takes [tx_ordset::W](tx_ordset::W) writer structure"]
impl crate::Writable for TX_ORDSET {}
#[doc = "UCPD Tx Ordered Set Type Register"]
pub mod tx_ordset;
#[doc = "UCPD Tx Paysize Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_paysz](tx_paysz) module"]
pub type TX_PAYSZ = crate::Reg<u32, _TX_PAYSZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_PAYSZ;
#[doc = "`read()` method returns [tx_paysz::R](tx_paysz::R) reader structure"]
impl crate::Readable for TX_PAYSZ {}
#[doc = "`write(|w| ..)` method takes [tx_paysz::W](tx_paysz::W) writer structure"]
impl crate::Writable for TX_PAYSZ {}
#[doc = "UCPD Tx Paysize Register"]
pub mod tx_paysz;
#[doc = "UCPD Tx Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txdr](txdr) module"]
pub type TXDR = crate::Reg<u32, _TXDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXDR;
#[doc = "`read()` method returns [txdr::R](txdr::R) reader structure"]
impl crate::Readable for TXDR {}
#[doc = "`write(|w| ..)` method takes [txdr::W](txdr::W) writer structure"]
impl crate::Writable for TXDR {}
#[doc = "UCPD Tx Data Register"]
pub mod txdr;
#[doc = "UCPD Rx Ordered Set Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_ordset](rx_ordset) module"]
pub type RX_ORDSET = crate::Reg<u32, _RX_ORDSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_ORDSET;
#[doc = "`read()` method returns [rx_ordset::R](rx_ordset::R) reader structure"]
impl crate::Readable for RX_ORDSET {}
#[doc = "UCPD Rx Ordered Set Register"]
pub mod rx_ordset;
#[doc = "UCPD Rx Paysize Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_paysz](rx_paysz) module"]
pub type RX_PAYSZ = crate::Reg<u32, _RX_PAYSZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_PAYSZ;
#[doc = "`read()` method returns [rx_paysz::R](rx_paysz::R) reader structure"]
impl crate::Readable for RX_PAYSZ {}
#[doc = "`write(|w| ..)` method takes [rx_paysz::W](rx_paysz::W) writer structure"]
impl crate::Writable for RX_PAYSZ {}
#[doc = "UCPD Rx Paysize Register"]
pub mod rx_paysz;
#[doc = "UCPD Receive Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxdr](rxdr) module"]
pub type RXDR = crate::Reg<u32, _RXDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXDR;
#[doc = "`read()` method returns [rxdr::R](rxdr::R) reader structure"]
impl crate::Readable for RXDR {}
#[doc = "UCPD Receive Data Register"]
pub mod rxdr;
#[doc = "UCPD Rx Ordered Set Extension Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_ordext1](rx_ordext1) module"]
pub type RX_ORDEXT1 = crate::Reg<u32, _RX_ORDEXT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_ORDEXT1;
#[doc = "`read()` method returns [rx_ordext1::R](rx_ordext1::R) reader structure"]
impl crate::Readable for RX_ORDEXT1 {}
#[doc = "`write(|w| ..)` method takes [rx_ordext1::W](rx_ordext1::W) writer structure"]
impl crate::Writable for RX_ORDEXT1 {}
#[doc = "UCPD Rx Ordered Set Extension Register"]
pub mod rx_ordext1;
#[doc = "UCPD Rx Ordered Set Extension Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_ordext2](rx_ordext2) module"]
pub type RX_ORDEXT2 = crate::Reg<u32, _RX_ORDEXT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_ORDEXT2;
#[doc = "`read()` method returns [rx_ordext2::R](rx_ordext2::R) reader structure"]
impl crate::Readable for RX_ORDEXT2 {}
#[doc = "`write(|w| ..)` method takes [rx_ordext2::W](rx_ordext2::W) writer structure"]
impl crate::Writable for RX_ORDEXT2 {}
#[doc = "UCPD Rx Ordered Set Extension Register"]
pub mod rx_ordext2;
#[doc = "UCPD IP ID register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipver](ipver) module"]
pub type IPVER = crate::Reg<u32, _IPVER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPVER;
#[doc = "`read()` method returns [ipver::R](ipver::R) reader structure"]
impl crate::Readable for IPVER {}
#[doc = "UCPD IP ID register"]
pub mod ipver;
#[doc = "UCPD IP ID register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipid](ipid) module"]
pub type IPID = crate::Reg<u32, _IPID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPID;
#[doc = "`read()` method returns [ipid::R](ipid::R) reader structure"]
impl crate::Readable for IPID {}
#[doc = "UCPD IP ID register"]
pub mod ipid;
#[doc = "UCPD IP ID register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mid](mid) module"]
pub type MID = crate::Reg<u32, _MID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MID;
#[doc = "`read()` method returns [mid::R](mid::R) reader structure"]
impl crate::Readable for MID {}
#[doc = "UCPD IP ID register"]
pub mod mid;

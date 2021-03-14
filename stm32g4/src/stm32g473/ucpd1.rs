#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - UCPD configuration register 1"]
    pub cfg1: CFG1,
    #[doc = "0x04 - UCPD configuration register 2"]
    pub cfg2: CFG2,
    _reserved2: [u8; 4usize],
    #[doc = "0x0c - UCPD configuration register 2"]
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
    #[doc = "0x30 - UCPD Rx Data Register"]
    pub rxdr: RXDR,
    #[doc = "0x34 - UCPD Rx Ordered Set Extension Register 1"]
    pub rx_ordext1: RX_ORDEXT1,
    #[doc = "0x38 - UCPD Rx Ordered Set Extension Register 2"]
    pub rx_ordext2: RX_ORDEXT2,
}
#[doc = "UCPD configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg1](cfg1) module"]
pub type CFG1 = crate::Reg<u32, _CFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG1;
#[doc = "`read()` method returns [cfg1::R](cfg1::R) reader structure"]
impl crate::Readable for CFG1 {}
#[doc = "`write(|w| ..)` method takes [cfg1::W](cfg1::W) writer structure"]
impl crate::Writable for CFG1 {}
#[doc = "UCPD configuration register 1"]
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
#[doc = "UCPD configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "UCPD configuration register 2"]
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
#[doc = "UCPD Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "`write(|w| ..)` method takes [sr::W](sr::W) writer structure"]
impl crate::Writable for SR {}
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
#[doc = "UCPD Rx Paysize Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_paysz](rx_paysz) module"]
pub type RX_PAYSZ = crate::Reg<u32, _RX_PAYSZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_PAYSZ;
#[doc = "`read()` method returns [rx_paysz::R](rx_paysz::R) reader structure"]
impl crate::Readable for RX_PAYSZ {}
#[doc = "UCPD Rx Paysize Register"]
pub mod rx_paysz;
#[doc = "UCPD Rx Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxdr](rxdr) module"]
pub type RXDR = crate::Reg<u32, _RXDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXDR;
#[doc = "`read()` method returns [rxdr::R](rxdr::R) reader structure"]
impl crate::Readable for RXDR {}
#[doc = "UCPD Rx Data Register"]
pub mod rxdr;
#[doc = "UCPD Rx Ordered Set Extension Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_ordext1](rx_ordext1) module"]
pub type RX_ORDEXT1 = crate::Reg<u32, _RX_ORDEXT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_ORDEXT1;
#[doc = "`read()` method returns [rx_ordext1::R](rx_ordext1::R) reader structure"]
impl crate::Readable for RX_ORDEXT1 {}
#[doc = "`write(|w| ..)` method takes [rx_ordext1::W](rx_ordext1::W) writer structure"]
impl crate::Writable for RX_ORDEXT1 {}
#[doc = "UCPD Rx Ordered Set Extension Register 1"]
pub mod rx_ordext1;
#[doc = "UCPD Rx Ordered Set Extension Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_ordext2](rx_ordext2) module"]
pub type RX_ORDEXT2 = crate::Reg<u32, _RX_ORDEXT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_ORDEXT2;
#[doc = "`read()` method returns [rx_ordext2::R](rx_ordext2::R) reader structure"]
impl crate::Readable for RX_ORDEXT2 {}
#[doc = "`write(|w| ..)` method takes [rx_ordext2::W](rx_ordext2::W) writer structure"]
impl crate::Writable for RX_ORDEXT2 {}
#[doc = "UCPD Rx Ordered Set Extension Register 2"]
pub mod rx_ordext2;

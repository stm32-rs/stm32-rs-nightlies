#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register"]
    pub cr: CR,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - device configuration register"]
    pub dcr1: DCR1,
    #[doc = "0x0c - device configuration register 2"]
    pub dcr2: DCR2,
    #[doc = "0x10 - device configuration register 3"]
    pub dcr3: DCR3,
    #[doc = "0x14 - DCR4"]
    pub dcr4: DCR4,
    _reserved5: [u8; 8usize],
    #[doc = "0x20 - status register"]
    pub sr: SR,
    #[doc = "0x24 - flag clear register"]
    pub fcr: FCR,
    _reserved7: [u8; 24usize],
    #[doc = "0x40 - data length register"]
    pub dlr: DLR,
    _reserved8: [u8; 4usize],
    #[doc = "0x48 - address register"]
    pub ar: AR,
    _reserved9: [u8; 4usize],
    #[doc = "0x50 - data register"]
    pub dr: DR,
    _reserved10: [u8; 44usize],
    #[doc = "0x80 - polling status mask register"]
    pub psmkr: PSMKR,
    _reserved11: [u8; 4usize],
    #[doc = "0x88 - polling status match register"]
    pub psmar: PSMAR,
    _reserved12: [u8; 4usize],
    #[doc = "0x90 - polling interval register"]
    pub pir: PIR,
    _reserved13: [u8; 108usize],
    #[doc = "0x100 - communication configuration register"]
    pub ccr: CCR,
    _reserved14: [u8; 4usize],
    #[doc = "0x108 - timing configuration register"]
    pub tcr: TCR,
    _reserved15: [u8; 4usize],
    #[doc = "0x110 - instruction register"]
    pub ir: IR,
    _reserved16: [u8; 12usize],
    #[doc = "0x120 - alternate bytes register"]
    pub abr: ABR,
    _reserved17: [u8; 12usize],
    #[doc = "0x130 - low-power timeout register"]
    pub lptr: LPTR,
    _reserved18: [u8; 12usize],
    #[doc = "0x140 - write communication configuration register"]
    pub wpccr: WPCCR,
    _reserved19: [u8; 4usize],
    #[doc = "0x148 - write timing configuration register"]
    pub wptcr: WPTCR,
    _reserved20: [u8; 4usize],
    #[doc = "0x150 - write instruction register"]
    pub wpir: WPIR,
    _reserved21: [u8; 12usize],
    #[doc = "0x160 - write alternate bytes register"]
    pub wpabr: WPABR,
    _reserved22: [u8; 28usize],
    #[doc = "0x180 - WCCR"]
    pub wccr: WCCR,
    _reserved23: [u8; 4usize],
    #[doc = "0x188 - WTCR"]
    pub wtcr: WTCR,
    _reserved24: [u8; 4usize],
    #[doc = "0x190 - WIR"]
    pub wir: WIR,
    _reserved25: [u8; 12usize],
    #[doc = "0x1a0 - WABR"]
    pub wabr: WABR,
    _reserved26: [u8; 92usize],
    #[doc = "0x200 - HyperBusTM latency configuration register"]
    pub hlcr: HLCR,
}
#[doc = "control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "control register"]
pub mod cr;
#[doc = "device configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcr1](dcr1) module"]
pub type DCR1 = crate::Reg<u32, _DCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCR1;
#[doc = "`read()` method returns [dcr1::R](dcr1::R) reader structure"]
impl crate::Readable for DCR1 {}
#[doc = "`write(|w| ..)` method takes [dcr1::W](dcr1::W) writer structure"]
impl crate::Writable for DCR1 {}
#[doc = "device configuration register"]
pub mod dcr1;
#[doc = "device configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcr2](dcr2) module"]
pub type DCR2 = crate::Reg<u32, _DCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCR2;
#[doc = "`read()` method returns [dcr2::R](dcr2::R) reader structure"]
impl crate::Readable for DCR2 {}
#[doc = "`write(|w| ..)` method takes [dcr2::W](dcr2::W) writer structure"]
impl crate::Writable for DCR2 {}
#[doc = "device configuration register 2"]
pub mod dcr2;
#[doc = "device configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcr3](dcr3) module"]
pub type DCR3 = crate::Reg<u32, _DCR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCR3;
#[doc = "`read()` method returns [dcr3::R](dcr3::R) reader structure"]
impl crate::Readable for DCR3 {}
#[doc = "`write(|w| ..)` method takes [dcr3::W](dcr3::W) writer structure"]
impl crate::Writable for DCR3 {}
#[doc = "device configuration register 3"]
pub mod dcr3;
#[doc = "DCR4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcr4](dcr4) module"]
pub type DCR4 = crate::Reg<u32, _DCR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCR4;
#[doc = "`read()` method returns [dcr4::R](dcr4::R) reader structure"]
impl crate::Readable for DCR4 {}
#[doc = "`write(|w| ..)` method takes [dcr4::W](dcr4::W) writer structure"]
impl crate::Writable for DCR4 {}
#[doc = "DCR4"]
pub mod dcr4;
#[doc = "status register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`write(|w| ..)` method takes [sr::W](sr::W) writer structure"]
impl crate::Writable for SR {}
#[doc = "status register"]
pub mod sr;
#[doc = "flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcr](fcr) module"]
pub type FCR = crate::Reg<u32, _FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCR;
#[doc = "`read()` method returns [fcr::R](fcr::R) reader structure"]
impl crate::Readable for FCR {}
#[doc = "`write(|w| ..)` method takes [fcr::W](fcr::W) writer structure"]
impl crate::Writable for FCR {}
#[doc = "flag clear register"]
pub mod fcr;
#[doc = "data length register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dlr](dlr) module"]
pub type DLR = crate::Reg<u32, _DLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DLR;
#[doc = "`read()` method returns [dlr::R](dlr::R) reader structure"]
impl crate::Readable for DLR {}
#[doc = "`write(|w| ..)` method takes [dlr::W](dlr::W) writer structure"]
impl crate::Writable for DLR {}
#[doc = "data length register"]
pub mod dlr;
#[doc = "address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ar](ar) module"]
pub type AR = crate::Reg<u32, _AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AR;
#[doc = "`read()` method returns [ar::R](ar::R) reader structure"]
impl crate::Readable for AR {}
#[doc = "`write(|w| ..)` method takes [ar::W](ar::W) writer structure"]
impl crate::Writable for AR {}
#[doc = "address register"]
pub mod ar;
#[doc = "data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr](dr) module"]
pub type DR = crate::Reg<u32, _DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR;
#[doc = "`read()` method returns [dr::R](dr::R) reader structure"]
impl crate::Readable for DR {}
#[doc = "`write(|w| ..)` method takes [dr::W](dr::W) writer structure"]
impl crate::Writable for DR {}
#[doc = "data register"]
pub mod dr;
#[doc = "polling status mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psmkr](psmkr) module"]
pub type PSMKR = crate::Reg<u32, _PSMKR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSMKR;
#[doc = "`read()` method returns [psmkr::R](psmkr::R) reader structure"]
impl crate::Readable for PSMKR {}
#[doc = "`write(|w| ..)` method takes [psmkr::W](psmkr::W) writer structure"]
impl crate::Writable for PSMKR {}
#[doc = "polling status mask register"]
pub mod psmkr;
#[doc = "polling status match register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psmar](psmar) module"]
pub type PSMAR = crate::Reg<u32, _PSMAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSMAR;
#[doc = "`read()` method returns [psmar::R](psmar::R) reader structure"]
impl crate::Readable for PSMAR {}
#[doc = "`write(|w| ..)` method takes [psmar::W](psmar::W) writer structure"]
impl crate::Writable for PSMAR {}
#[doc = "polling status match register"]
pub mod psmar;
#[doc = "polling interval register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pir](pir) module"]
pub type PIR = crate::Reg<u32, _PIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIR;
#[doc = "`read()` method returns [pir::R](pir::R) reader structure"]
impl crate::Readable for PIR {}
#[doc = "`write(|w| ..)` method takes [pir::W](pir::W) writer structure"]
impl crate::Writable for PIR {}
#[doc = "polling interval register"]
pub mod pir;
#[doc = "communication configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr](ccr) module"]
pub type CCR = crate::Reg<u32, _CCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR;
#[doc = "`read()` method returns [ccr::R](ccr::R) reader structure"]
impl crate::Readable for CCR {}
#[doc = "`write(|w| ..)` method takes [ccr::W](ccr::W) writer structure"]
impl crate::Writable for CCR {}
#[doc = "communication configuration register"]
pub mod ccr;
#[doc = "timing configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcr](tcr) module"]
pub type TCR = crate::Reg<u32, _TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCR;
#[doc = "`read()` method returns [tcr::R](tcr::R) reader structure"]
impl crate::Readable for TCR {}
#[doc = "`write(|w| ..)` method takes [tcr::W](tcr::W) writer structure"]
impl crate::Writable for TCR {}
#[doc = "timing configuration register"]
pub mod tcr;
#[doc = "instruction register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ir](ir) module"]
pub type IR = crate::Reg<u32, _IR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IR;
#[doc = "`read()` method returns [ir::R](ir::R) reader structure"]
impl crate::Readable for IR {}
#[doc = "`write(|w| ..)` method takes [ir::W](ir::W) writer structure"]
impl crate::Writable for IR {}
#[doc = "instruction register"]
pub mod ir;
#[doc = "alternate bytes register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [abr](abr) module"]
pub type ABR = crate::Reg<u32, _ABR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ABR;
#[doc = "`read()` method returns [abr::R](abr::R) reader structure"]
impl crate::Readable for ABR {}
#[doc = "`write(|w| ..)` method takes [abr::W](abr::W) writer structure"]
impl crate::Writable for ABR {}
#[doc = "alternate bytes register"]
pub mod abr;
#[doc = "low-power timeout register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lptr](lptr) module"]
pub type LPTR = crate::Reg<u32, _LPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPTR;
#[doc = "`read()` method returns [lptr::R](lptr::R) reader structure"]
impl crate::Readable for LPTR {}
#[doc = "`write(|w| ..)` method takes [lptr::W](lptr::W) writer structure"]
impl crate::Writable for LPTR {}
#[doc = "low-power timeout register"]
pub mod lptr;
#[doc = "write communication configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpccr](wpccr) module"]
pub type WPCCR = crate::Reg<u32, _WPCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WPCCR;
#[doc = "`read()` method returns [wpccr::R](wpccr::R) reader structure"]
impl crate::Readable for WPCCR {}
#[doc = "`write(|w| ..)` method takes [wpccr::W](wpccr::W) writer structure"]
impl crate::Writable for WPCCR {}
#[doc = "write communication configuration register"]
pub mod wpccr;
#[doc = "write timing configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wptcr](wptcr) module"]
pub type WPTCR = crate::Reg<u32, _WPTCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WPTCR;
#[doc = "`read()` method returns [wptcr::R](wptcr::R) reader structure"]
impl crate::Readable for WPTCR {}
#[doc = "`write(|w| ..)` method takes [wptcr::W](wptcr::W) writer structure"]
impl crate::Writable for WPTCR {}
#[doc = "write timing configuration register"]
pub mod wptcr;
#[doc = "write instruction register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpir](wpir) module"]
pub type WPIR = crate::Reg<u32, _WPIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WPIR;
#[doc = "`read()` method returns [wpir::R](wpir::R) reader structure"]
impl crate::Readable for WPIR {}
#[doc = "`write(|w| ..)` method takes [wpir::W](wpir::W) writer structure"]
impl crate::Writable for WPIR {}
#[doc = "write instruction register"]
pub mod wpir;
#[doc = "write alternate bytes register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpabr](wpabr) module"]
pub type WPABR = crate::Reg<u32, _WPABR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WPABR;
#[doc = "`read()` method returns [wpabr::R](wpabr::R) reader structure"]
impl crate::Readable for WPABR {}
#[doc = "`write(|w| ..)` method takes [wpabr::W](wpabr::W) writer structure"]
impl crate::Writable for WPABR {}
#[doc = "write alternate bytes register"]
pub mod wpabr;
#[doc = "WCCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wccr](wccr) module"]
pub type WCCR = crate::Reg<u32, _WCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WCCR;
#[doc = "`read()` method returns [wccr::R](wccr::R) reader structure"]
impl crate::Readable for WCCR {}
#[doc = "`write(|w| ..)` method takes [wccr::W](wccr::W) writer structure"]
impl crate::Writable for WCCR {}
#[doc = "WCCR"]
pub mod wccr;
#[doc = "WTCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wtcr](wtcr) module"]
pub type WTCR = crate::Reg<u32, _WTCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WTCR;
#[doc = "`read()` method returns [wtcr::R](wtcr::R) reader structure"]
impl crate::Readable for WTCR {}
#[doc = "`write(|w| ..)` method takes [wtcr::W](wtcr::W) writer structure"]
impl crate::Writable for WTCR {}
#[doc = "WTCR"]
pub mod wtcr;
#[doc = "WIR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wir](wir) module"]
pub type WIR = crate::Reg<u32, _WIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WIR;
#[doc = "`read()` method returns [wir::R](wir::R) reader structure"]
impl crate::Readable for WIR {}
#[doc = "`write(|w| ..)` method takes [wir::W](wir::W) writer structure"]
impl crate::Writable for WIR {}
#[doc = "WIR"]
pub mod wir;
#[doc = "WABR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wabr](wabr) module"]
pub type WABR = crate::Reg<u32, _WABR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WABR;
#[doc = "`read()` method returns [wabr::R](wabr::R) reader structure"]
impl crate::Readable for WABR {}
#[doc = "`write(|w| ..)` method takes [wabr::W](wabr::W) writer structure"]
impl crate::Writable for WABR {}
#[doc = "WABR"]
pub mod wabr;
#[doc = "HyperBusTM latency configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hlcr](hlcr) module"]
pub type HLCR = crate::Reg<u32, _HLCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HLCR;
#[doc = "`read()` method returns [hlcr::R](hlcr::R) reader structure"]
impl crate::Readable for HLCR {}
#[doc = "`write(|w| ..)` method takes [hlcr::W](hlcr::W) writer structure"]
impl crate::Writable for HLCR {}
#[doc = "HyperBusTM latency configuration register"]
pub mod hlcr;

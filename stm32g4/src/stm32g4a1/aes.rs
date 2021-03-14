#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register"]
    pub cr: CR,
    #[doc = "0x04 - status register"]
    pub sr: SR,
    #[doc = "0x08 - data input register"]
    pub dinr: DINR,
    #[doc = "0x0c - data output register"]
    pub doutr: DOUTR,
    #[doc = "0x10 - key register 0"]
    pub keyr0: KEYR0,
    #[doc = "0x14 - key register 1"]
    pub keyr1: KEYR1,
    #[doc = "0x18 - key register 2"]
    pub keyr2: KEYR2,
    #[doc = "0x1c - key register 3"]
    pub keyr3: KEYR3,
    #[doc = "0x20 - initialization vector register 0"]
    pub ivr0: IVR0,
    #[doc = "0x24 - initialization vector register 1"]
    pub ivr1: IVR1,
    #[doc = "0x28 - initialization vector register 2"]
    pub ivr2: IVR2,
    #[doc = "0x2c - initialization vector register 3"]
    pub ivr3: IVR3,
    #[doc = "0x30 - key register 4"]
    pub keyr4: KEYR4,
    #[doc = "0x34 - key register 5"]
    pub keyr5: KEYR5,
    #[doc = "0x38 - key register 6"]
    pub keyr6: KEYR6,
    #[doc = "0x3c - key register 7"]
    pub keyr7: KEYR7,
    #[doc = "0x40 - suspend registers"]
    pub susp0r: SUSP0R,
    #[doc = "0x44 - suspend registers"]
    pub susp1r: SUSP1R,
    #[doc = "0x48 - suspend registers"]
    pub susp2r: SUSP2R,
    #[doc = "0x4c - suspend registers"]
    pub susp3r: SUSP3R,
    #[doc = "0x50 - suspend registers"]
    pub susp4r: SUSP4R,
    #[doc = "0x54 - suspend registers"]
    pub susp5r: SUSP5R,
    #[doc = "0x58 - suspend registers"]
    pub susp6r: SUSP6R,
    #[doc = "0x5c - suspend registers"]
    pub susp7r: SUSP7R,
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
#[doc = "status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "status register"]
pub mod sr;
#[doc = "data input register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dinr](dinr) module"]
pub type DINR = crate::Reg<u32, _DINR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DINR;
#[doc = "`read()` method returns [dinr::R](dinr::R) reader structure"]
impl crate::Readable for DINR {}
#[doc = "`write(|w| ..)` method takes [dinr::W](dinr::W) writer structure"]
impl crate::Writable for DINR {}
#[doc = "data input register"]
pub mod dinr;
#[doc = "data output register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doutr](doutr) module"]
pub type DOUTR = crate::Reg<u32, _DOUTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUTR;
#[doc = "`read()` method returns [doutr::R](doutr::R) reader structure"]
impl crate::Readable for DOUTR {}
#[doc = "data output register"]
pub mod doutr;
#[doc = "key register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyr0](keyr0) module"]
pub type KEYR0 = crate::Reg<u32, _KEYR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEYR0;
#[doc = "`read()` method returns [keyr0::R](keyr0::R) reader structure"]
impl crate::Readable for KEYR0 {}
#[doc = "`write(|w| ..)` method takes [keyr0::W](keyr0::W) writer structure"]
impl crate::Writable for KEYR0 {}
#[doc = "key register 0"]
pub mod keyr0;
#[doc = "key register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyr1](keyr1) module"]
pub type KEYR1 = crate::Reg<u32, _KEYR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEYR1;
#[doc = "`read()` method returns [keyr1::R](keyr1::R) reader structure"]
impl crate::Readable for KEYR1 {}
#[doc = "`write(|w| ..)` method takes [keyr1::W](keyr1::W) writer structure"]
impl crate::Writable for KEYR1 {}
#[doc = "key register 1"]
pub mod keyr1;
#[doc = "key register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyr2](keyr2) module"]
pub type KEYR2 = crate::Reg<u32, _KEYR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEYR2;
#[doc = "`read()` method returns [keyr2::R](keyr2::R) reader structure"]
impl crate::Readable for KEYR2 {}
#[doc = "`write(|w| ..)` method takes [keyr2::W](keyr2::W) writer structure"]
impl crate::Writable for KEYR2 {}
#[doc = "key register 2"]
pub mod keyr2;
#[doc = "key register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyr3](keyr3) module"]
pub type KEYR3 = crate::Reg<u32, _KEYR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEYR3;
#[doc = "`read()` method returns [keyr3::R](keyr3::R) reader structure"]
impl crate::Readable for KEYR3 {}
#[doc = "`write(|w| ..)` method takes [keyr3::W](keyr3::W) writer structure"]
impl crate::Writable for KEYR3 {}
#[doc = "key register 3"]
pub mod keyr3;
#[doc = "initialization vector register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ivr0](ivr0) module"]
pub type IVR0 = crate::Reg<u32, _IVR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IVR0;
#[doc = "`read()` method returns [ivr0::R](ivr0::R) reader structure"]
impl crate::Readable for IVR0 {}
#[doc = "`write(|w| ..)` method takes [ivr0::W](ivr0::W) writer structure"]
impl crate::Writable for IVR0 {}
#[doc = "initialization vector register 0"]
pub mod ivr0;
#[doc = "initialization vector register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ivr1](ivr1) module"]
pub type IVR1 = crate::Reg<u32, _IVR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IVR1;
#[doc = "`read()` method returns [ivr1::R](ivr1::R) reader structure"]
impl crate::Readable for IVR1 {}
#[doc = "`write(|w| ..)` method takes [ivr1::W](ivr1::W) writer structure"]
impl crate::Writable for IVR1 {}
#[doc = "initialization vector register 1"]
pub mod ivr1;
#[doc = "initialization vector register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ivr2](ivr2) module"]
pub type IVR2 = crate::Reg<u32, _IVR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IVR2;
#[doc = "`read()` method returns [ivr2::R](ivr2::R) reader structure"]
impl crate::Readable for IVR2 {}
#[doc = "`write(|w| ..)` method takes [ivr2::W](ivr2::W) writer structure"]
impl crate::Writable for IVR2 {}
#[doc = "initialization vector register 2"]
pub mod ivr2;
#[doc = "initialization vector register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ivr3](ivr3) module"]
pub type IVR3 = crate::Reg<u32, _IVR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IVR3;
#[doc = "`read()` method returns [ivr3::R](ivr3::R) reader structure"]
impl crate::Readable for IVR3 {}
#[doc = "`write(|w| ..)` method takes [ivr3::W](ivr3::W) writer structure"]
impl crate::Writable for IVR3 {}
#[doc = "initialization vector register 3"]
pub mod ivr3;
#[doc = "key register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyr4](keyr4) module"]
pub type KEYR4 = crate::Reg<u32, _KEYR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEYR4;
#[doc = "`read()` method returns [keyr4::R](keyr4::R) reader structure"]
impl crate::Readable for KEYR4 {}
#[doc = "`write(|w| ..)` method takes [keyr4::W](keyr4::W) writer structure"]
impl crate::Writable for KEYR4 {}
#[doc = "key register 4"]
pub mod keyr4;
#[doc = "key register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyr5](keyr5) module"]
pub type KEYR5 = crate::Reg<u32, _KEYR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEYR5;
#[doc = "`read()` method returns [keyr5::R](keyr5::R) reader structure"]
impl crate::Readable for KEYR5 {}
#[doc = "`write(|w| ..)` method takes [keyr5::W](keyr5::W) writer structure"]
impl crate::Writable for KEYR5 {}
#[doc = "key register 5"]
pub mod keyr5;
#[doc = "key register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyr6](keyr6) module"]
pub type KEYR6 = crate::Reg<u32, _KEYR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEYR6;
#[doc = "`read()` method returns [keyr6::R](keyr6::R) reader structure"]
impl crate::Readable for KEYR6 {}
#[doc = "`write(|w| ..)` method takes [keyr6::W](keyr6::W) writer structure"]
impl crate::Writable for KEYR6 {}
#[doc = "key register 6"]
pub mod keyr6;
#[doc = "key register 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyr7](keyr7) module"]
pub type KEYR7 = crate::Reg<u32, _KEYR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEYR7;
#[doc = "`read()` method returns [keyr7::R](keyr7::R) reader structure"]
impl crate::Readable for KEYR7 {}
#[doc = "`write(|w| ..)` method takes [keyr7::W](keyr7::W) writer structure"]
impl crate::Writable for KEYR7 {}
#[doc = "key register 7"]
pub mod keyr7;
#[doc = "suspend registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [susp0r](susp0r) module"]
pub type SUSP0R = crate::Reg<u32, _SUSP0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUSP0R;
#[doc = "`read()` method returns [susp0r::R](susp0r::R) reader structure"]
impl crate::Readable for SUSP0R {}
#[doc = "`write(|w| ..)` method takes [susp0r::W](susp0r::W) writer structure"]
impl crate::Writable for SUSP0R {}
#[doc = "suspend registers"]
pub mod susp0r;
#[doc = "suspend registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [susp1r](susp1r) module"]
pub type SUSP1R = crate::Reg<u32, _SUSP1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUSP1R;
#[doc = "`read()` method returns [susp1r::R](susp1r::R) reader structure"]
impl crate::Readable for SUSP1R {}
#[doc = "`write(|w| ..)` method takes [susp1r::W](susp1r::W) writer structure"]
impl crate::Writable for SUSP1R {}
#[doc = "suspend registers"]
pub mod susp1r;
#[doc = "suspend registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [susp2r](susp2r) module"]
pub type SUSP2R = crate::Reg<u32, _SUSP2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUSP2R;
#[doc = "`read()` method returns [susp2r::R](susp2r::R) reader structure"]
impl crate::Readable for SUSP2R {}
#[doc = "`write(|w| ..)` method takes [susp2r::W](susp2r::W) writer structure"]
impl crate::Writable for SUSP2R {}
#[doc = "suspend registers"]
pub mod susp2r;
#[doc = "suspend registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [susp3r](susp3r) module"]
pub type SUSP3R = crate::Reg<u32, _SUSP3R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUSP3R;
#[doc = "`read()` method returns [susp3r::R](susp3r::R) reader structure"]
impl crate::Readable for SUSP3R {}
#[doc = "`write(|w| ..)` method takes [susp3r::W](susp3r::W) writer structure"]
impl crate::Writable for SUSP3R {}
#[doc = "suspend registers"]
pub mod susp3r;
#[doc = "suspend registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [susp4r](susp4r) module"]
pub type SUSP4R = crate::Reg<u32, _SUSP4R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUSP4R;
#[doc = "`read()` method returns [susp4r::R](susp4r::R) reader structure"]
impl crate::Readable for SUSP4R {}
#[doc = "`write(|w| ..)` method takes [susp4r::W](susp4r::W) writer structure"]
impl crate::Writable for SUSP4R {}
#[doc = "suspend registers"]
pub mod susp4r;
#[doc = "suspend registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [susp5r](susp5r) module"]
pub type SUSP5R = crate::Reg<u32, _SUSP5R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUSP5R;
#[doc = "`read()` method returns [susp5r::R](susp5r::R) reader structure"]
impl crate::Readable for SUSP5R {}
#[doc = "`write(|w| ..)` method takes [susp5r::W](susp5r::W) writer structure"]
impl crate::Writable for SUSP5R {}
#[doc = "suspend registers"]
pub mod susp5r;
#[doc = "suspend registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [susp6r](susp6r) module"]
pub type SUSP6R = crate::Reg<u32, _SUSP6R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUSP6R;
#[doc = "`read()` method returns [susp6r::R](susp6r::R) reader structure"]
impl crate::Readable for SUSP6R {}
#[doc = "`write(|w| ..)` method takes [susp6r::W](susp6r::W) writer structure"]
impl crate::Writable for SUSP6R {}
#[doc = "suspend registers"]
pub mod susp6r;
#[doc = "suspend registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [susp7r](susp7r) module"]
pub type SUSP7R = crate::Reg<u32, _SUSP7R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUSP7R;
#[doc = "`read()` method returns [susp7r::R](susp7r::R) reader structure"]
impl crate::Readable for SUSP7R {}
#[doc = "`write(|w| ..)` method takes [susp7r::W](susp7r::W) writer structure"]
impl crate::Writable for SUSP7R {}
#[doc = "suspend registers"]
pub mod susp7r;

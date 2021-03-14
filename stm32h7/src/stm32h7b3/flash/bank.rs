#[doc = "FLASH key register for bank 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyr](keyr) module"]
pub type KEYR = crate::Reg<u32, _KEYR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEYR;
#[doc = "`read()` method returns [keyr::R](keyr::R) reader structure"]
impl crate::Readable for KEYR {}
#[doc = "`write(|w| ..)` method takes [keyr::W](keyr::W) writer structure"]
impl crate::Writable for KEYR {}
#[doc = "FLASH key register for bank 1"]
pub mod keyr;
#[doc = "FLASH control register for bank 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "FLASH control register for bank 1"]
pub mod cr;
#[doc = "FLASH status register for bank 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "`write(|w| ..)` method takes [sr::W](sr::W) writer structure"]
impl crate::Writable for SR {}
#[doc = "FLASH status register for bank 1"]
pub mod sr;
#[doc = "FLASH clear control register for bank 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr](ccr) module"]
pub type CCR = crate::Reg<u32, _CCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR;
#[doc = "`read()` method returns [ccr::R](ccr::R) reader structure"]
impl crate::Readable for CCR {}
#[doc = "`write(|w| ..)` method takes [ccr::W](ccr::W) writer structure"]
impl crate::Writable for CCR {}
#[doc = "FLASH clear control register for bank 1"]
pub mod ccr;
#[doc = "FLASH protection address for bank 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prar_cur](prar_cur) module"]
pub type PRAR_CUR = crate::Reg<u32, _PRAR_CUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRAR_CUR;
#[doc = "`read()` method returns [prar_cur::R](prar_cur::R) reader structure"]
impl crate::Readable for PRAR_CUR {}
#[doc = "FLASH protection address for bank 1"]
pub mod prar_cur;
#[doc = "FLASH protection address for bank 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prar_prg](prar_prg) module"]
pub type PRAR_PRG = crate::Reg<u32, _PRAR_PRG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRAR_PRG;
#[doc = "`read()` method returns [prar_prg::R](prar_prg::R) reader structure"]
impl crate::Readable for PRAR_PRG {}
#[doc = "`write(|w| ..)` method takes [prar_prg::W](prar_prg::W) writer structure"]
impl crate::Writable for PRAR_PRG {}
#[doc = "FLASH protection address for bank 1"]
pub mod prar_prg;
#[doc = "FLASH secure address for bank 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scar_cur](scar_cur) module"]
pub type SCAR_CUR = crate::Reg<u32, _SCAR_CUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCAR_CUR;
#[doc = "`read()` method returns [scar_cur::R](scar_cur::R) reader structure"]
impl crate::Readable for SCAR_CUR {}
#[doc = "`write(|w| ..)` method takes [scar_cur::W](scar_cur::W) writer structure"]
impl crate::Writable for SCAR_CUR {}
#[doc = "FLASH secure address for bank 1"]
pub mod scar_cur;
#[doc = "FLASH secure address for bank 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scar_prg](scar_prg) module"]
pub type SCAR_PRG = crate::Reg<u32, _SCAR_PRG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCAR_PRG;
#[doc = "`read()` method returns [scar_prg::R](scar_prg::R) reader structure"]
impl crate::Readable for SCAR_PRG {}
#[doc = "`write(|w| ..)` method takes [scar_prg::W](scar_prg::W) writer structure"]
impl crate::Writable for SCAR_PRG {}
#[doc = "FLASH secure address for bank 1"]
pub mod scar_prg;
#[doc = "FLASH write sector protection for bank 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpsn_curr](wpsn_curr) module"]
pub type WPSN_CURR = crate::Reg<u32, _WPSN_CURR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WPSN_CURR;
#[doc = "`read()` method returns [wpsn_curr::R](wpsn_curr::R) reader structure"]
impl crate::Readable for WPSN_CURR {}
#[doc = "FLASH write sector protection for bank 1"]
pub mod wpsn_curr;
#[doc = "FLASH write sector protection for bank 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpsn_prgr](wpsn_prgr) module"]
pub type WPSN_PRGR = crate::Reg<u32, _WPSN_PRGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WPSN_PRGR;
#[doc = "`read()` method returns [wpsn_prgr::R](wpsn_prgr::R) reader structure"]
impl crate::Readable for WPSN_PRGR {}
#[doc = "`write(|w| ..)` method takes [wpsn_prgr::W](wpsn_prgr::W) writer structure"]
impl crate::Writable for WPSN_PRGR {}
#[doc = "FLASH write sector protection for bank 1"]
pub mod wpsn_prgr;
#[doc = "FLASH CRC control register for bank 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crccr](crccr) module"]
pub type CRCCR = crate::Reg<u32, _CRCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRCCR;
#[doc = "`read()` method returns [crccr::R](crccr::R) reader structure"]
impl crate::Readable for CRCCR {}
#[doc = "`write(|w| ..)` method takes [crccr::W](crccr::W) writer structure"]
impl crate::Writable for CRCCR {}
#[doc = "FLASH CRC control register for bank 1"]
pub mod crccr;
#[doc = "FLASH CRC start address register for bank 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcsaddr](crcsaddr) module"]
pub type CRCSADDR = crate::Reg<u32, _CRCSADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRCSADDR;
#[doc = "`read()` method returns [crcsaddr::R](crcsaddr::R) reader structure"]
impl crate::Readable for CRCSADDR {}
#[doc = "`write(|w| ..)` method takes [crcsaddr::W](crcsaddr::W) writer structure"]
impl crate::Writable for CRCSADDR {}
#[doc = "FLASH CRC start address register for bank 1"]
pub mod crcsaddr;
#[doc = "FLASH CRC end address register for bank 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crceaddr](crceaddr) module"]
pub type CRCEADDR = crate::Reg<u32, _CRCEADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRCEADDR;
#[doc = "`read()` method returns [crceaddr::R](crceaddr::R) reader structure"]
impl crate::Readable for CRCEADDR {}
#[doc = "`write(|w| ..)` method takes [crceaddr::W](crceaddr::W) writer structure"]
impl crate::Writable for CRCEADDR {}
#[doc = "FLASH CRC end address register for bank 1"]
pub mod crceaddr;
#[doc = "FLASH ECC fail address for bank 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [far](far) module"]
pub type FAR = crate::Reg<u32, _FAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FAR;
#[doc = "`read()` method returns [far::R](far::R) reader structure"]
impl crate::Readable for FAR {}
#[doc = "FLASH ECC fail address for bank 1"]
pub mod far;

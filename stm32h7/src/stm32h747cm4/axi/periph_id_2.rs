#[doc = "Register `PERIPH_ID_2` reader"]
pub struct R(crate::R<PERIPH_ID_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERIPH_ID_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERIPH_ID_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERIPH_ID_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `JEP106ID` reader - JEP106 Identity bits 4 to 6"]
pub struct JEP106ID_R(crate::FieldReader<u8, u8>);
impl JEP106ID_R {
    pub(crate) fn new(bits: u8) -> Self {
        JEP106ID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JEP106ID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JEDEC` reader - JEP106 code flag"]
pub struct JEDEC_R(crate::FieldReader<bool, bool>);
impl JEDEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        JEDEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JEDEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REVISION` reader - Peripheral revision number"]
pub struct REVISION_R(crate::FieldReader<u8, u8>);
impl REVISION_R {
    pub(crate) fn new(bits: u8) -> Self {
        REVISION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REVISION_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:2 - JEP106 Identity bits 4 to 6"]
    #[inline(always)]
    pub fn jep106id(&self) -> JEP106ID_R {
        JEP106ID_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - JEP106 code flag"]
    #[inline(always)]
    pub fn jedec(&self) -> JEDEC_R {
        JEDEC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Peripheral revision number"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "AXI interconnect - peripheral ID2 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [periph_id_2](index.html) module"]
pub struct PERIPH_ID_2_SPEC;
impl crate::RegisterSpec for PERIPH_ID_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [periph_id_2::R](R) reader structure"]
impl crate::Readable for PERIPH_ID_2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PERIPH_ID_2 to value 0x04"]
impl crate::Resettable for PERIPH_ID_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}

#[doc = "Register `PERIPH_ID_4` reader"]
pub struct R(crate::R<PERIPH_ID_4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERIPH_ID_4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERIPH_ID_4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERIPH_ID_4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `JEP106CON` reader - JEP106 continuation code"]
pub struct JEP106CON_R(crate::FieldReader<u8, u8>);
impl JEP106CON_R {
    pub(crate) fn new(bits: u8) -> Self {
        JEP106CON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JEP106CON_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KCOUNT4` reader - Register file size"]
pub struct KCOUNT4_R(crate::FieldReader<u8, u8>);
impl KCOUNT4_R {
    pub(crate) fn new(bits: u8) -> Self {
        KCOUNT4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KCOUNT4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - JEP106 continuation code"]
    #[inline(always)]
    pub fn jep106con(&self) -> JEP106CON_R {
        JEP106CON_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Register file size"]
    #[inline(always)]
    pub fn kcount4(&self) -> KCOUNT4_R {
        KCOUNT4_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "AXI interconnect - peripheral ID4 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [periph_id_4](index.html) module"]
pub struct PERIPH_ID_4_SPEC;
impl crate::RegisterSpec for PERIPH_ID_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [periph_id_4::R](R) reader structure"]
impl crate::Readable for PERIPH_ID_4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PERIPH_ID_4 to value 0x04"]
impl crate::Resettable for PERIPH_ID_4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}

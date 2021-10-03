#[doc = "Register `DINR8` reader"]
pub struct R(crate::R<DINR8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DINR8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DINR8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DINR8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DIN8` reader - Input data received from MDIO Master during write frames"]
pub struct DIN8_R(crate::FieldReader<u16, u16>);
impl DIN8_R {
    pub(crate) fn new(bits: u16) -> Self {
        DIN8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIN8_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din8(&self) -> DIN8_R {
        DIN8_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 8\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dinr8](index.html) module"]
pub struct DINR8_SPEC;
impl crate::RegisterSpec for DINR8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dinr8::R](R) reader structure"]
impl crate::Readable for DINR8_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DINR8 to value 0"]
impl crate::Resettable for DINR8_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

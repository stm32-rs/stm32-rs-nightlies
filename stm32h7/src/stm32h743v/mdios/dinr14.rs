#[doc = "Register `DINR14` reader"]
pub struct R(crate::R<DINR14_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DINR14_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DINR14_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DINR14_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DIN14` reader - Input data received from MDIO Master during write frames"]
pub struct DIN14_R(crate::FieldReader<u16, u16>);
impl DIN14_R {
    pub(crate) fn new(bits: u16) -> Self {
        DIN14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIN14_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din14(&self) -> DIN14_R {
        DIN14_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 14\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dinr14](index.html) module"]
pub struct DINR14_SPEC;
impl crate::RegisterSpec for DINR14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dinr14::R](R) reader structure"]
impl crate::Readable for DINR14_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DINR14 to value 0"]
impl crate::Resettable for DINR14_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

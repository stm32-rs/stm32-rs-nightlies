#[doc = "Register `DINR20` reader"]
pub struct R(crate::R<DINR20_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DINR20_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DINR20_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DINR20_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DIN20` reader - Input data received from MDIO Master during write frames"]
pub struct DIN20_R(crate::FieldReader<u16, u16>);
impl DIN20_R {
    pub(crate) fn new(bits: u16) -> Self {
        DIN20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIN20_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din20(&self) -> DIN20_R {
        DIN20_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 20\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dinr20](index.html) module"]
pub struct DINR20_SPEC;
impl crate::RegisterSpec for DINR20_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dinr20::R](R) reader structure"]
impl crate::Readable for DINR20_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DINR20 to value 0"]
impl crate::Resettable for DINR20_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

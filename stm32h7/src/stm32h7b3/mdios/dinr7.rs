#[doc = "Register `DINR7` reader"]
pub struct R(crate::R<DINR7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DINR7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DINR7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DINR7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DIN7` reader - Input data received from MDIO Master during write frames"]
pub struct DIN7_R(crate::FieldReader<u16, u16>);
impl DIN7_R {
    pub(crate) fn new(bits: u16) -> Self {
        DIN7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIN7_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din7(&self) -> DIN7_R {
        DIN7_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dinr7](index.html) module"]
pub struct DINR7_SPEC;
impl crate::RegisterSpec for DINR7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dinr7::R](R) reader structure"]
impl crate::Readable for DINR7_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DINR7 to value 0"]
impl crate::Resettable for DINR7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

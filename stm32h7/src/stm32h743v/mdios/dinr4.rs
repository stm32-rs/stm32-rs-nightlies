#[doc = "Register `DINR4` reader"]
pub struct R(crate::R<DINR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DINR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DINR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DINR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DIN4` reader - Input data received from MDIO Master during write frames"]
pub struct DIN4_R(crate::FieldReader<u16, u16>);
impl DIN4_R {
    pub(crate) fn new(bits: u16) -> Self {
        DIN4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIN4_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din4(&self) -> DIN4_R {
        DIN4_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dinr4](index.html) module"]
pub struct DINR4_SPEC;
impl crate::RegisterSpec for DINR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dinr4::R](R) reader structure"]
impl crate::Readable for DINR4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DINR4 to value 0"]
impl crate::Resettable for DINR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

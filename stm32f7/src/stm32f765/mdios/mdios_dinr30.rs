#[doc = "Register `MDIOS_DINR30` reader"]
pub struct R(crate::R<MDIOS_DINR30_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDIOS_DINR30_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDIOS_DINR30_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDIOS_DINR30_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DIN30` reader - Input data received from MDIO Master during write frames"]
pub struct DIN30_R(crate::FieldReader<u16, u16>);
impl DIN30_R {
    pub(crate) fn new(bits: u16) -> Self {
        DIN30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIN30_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din30(&self) -> DIN30_R {
        DIN30_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 30\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_dinr30](index.html) module"]
pub struct MDIOS_DINR30_SPEC;
impl crate::RegisterSpec for MDIOS_DINR30_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdios_dinr30::R](R) reader structure"]
impl crate::Readable for MDIOS_DINR30_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MDIOS_DINR30 to value 0"]
impl crate::Resettable for MDIOS_DINR30_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

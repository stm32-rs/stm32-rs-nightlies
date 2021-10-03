#[doc = "Register `MDIOS_DINR3` reader"]
pub struct R(crate::R<MDIOS_DINR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDIOS_DINR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDIOS_DINR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDIOS_DINR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DIN3` reader - Input data received from MDIO Master during write frames"]
pub struct DIN3_R(crate::FieldReader<u16, u16>);
impl DIN3_R {
    pub(crate) fn new(bits: u16) -> Self {
        DIN3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIN3_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din3(&self) -> DIN3_R {
        DIN3_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_dinr3](index.html) module"]
pub struct MDIOS_DINR3_SPEC;
impl crate::RegisterSpec for MDIOS_DINR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdios_dinr3::R](R) reader structure"]
impl crate::Readable for MDIOS_DINR3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MDIOS_DINR3 to value 0"]
impl crate::Resettable for MDIOS_DINR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

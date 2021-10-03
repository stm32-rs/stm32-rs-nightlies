#[doc = "Register `MDIOS_DINR24` reader"]
pub struct R(crate::R<MDIOS_DINR24_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDIOS_DINR24_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDIOS_DINR24_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDIOS_DINR24_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DIN24` reader - Input data received from MDIO Master during write frames"]
pub struct DIN24_R(crate::FieldReader<u16, u16>);
impl DIN24_R {
    pub(crate) fn new(bits: u16) -> Self {
        DIN24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIN24_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din24(&self) -> DIN24_R {
        DIN24_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 24\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_dinr24](index.html) module"]
pub struct MDIOS_DINR24_SPEC;
impl crate::RegisterSpec for MDIOS_DINR24_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdios_dinr24::R](R) reader structure"]
impl crate::Readable for MDIOS_DINR24_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MDIOS_DINR24 to value 0"]
impl crate::Resettable for MDIOS_DINR24_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

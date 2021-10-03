#[doc = "Register `MDIOS_DINR25` reader"]
pub struct R(crate::R<MDIOS_DINR25_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDIOS_DINR25_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDIOS_DINR25_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDIOS_DINR25_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DIN25` reader - Input data received from MDIO Master during write frames"]
pub struct DIN25_R(crate::FieldReader<u16, u16>);
impl DIN25_R {
    pub(crate) fn new(bits: u16) -> Self {
        DIN25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIN25_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din25(&self) -> DIN25_R {
        DIN25_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 25\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_dinr25](index.html) module"]
pub struct MDIOS_DINR25_SPEC;
impl crate::RegisterSpec for MDIOS_DINR25_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdios_dinr25::R](R) reader structure"]
impl crate::Readable for MDIOS_DINR25_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MDIOS_DINR25 to value 0"]
impl crate::Resettable for MDIOS_DINR25_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

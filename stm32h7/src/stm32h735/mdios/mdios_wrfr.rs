#[doc = "Register `MDIOS_WRFR` reader"]
pub struct R(crate::R<MDIOS_WRFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDIOS_WRFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDIOS_WRFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDIOS_WRFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WRF` reader - Write flags for MDIO registers 0 to 31"]
pub struct WRF_R(crate::FieldReader<u32, u32>);
impl WRF_R {
    pub(crate) fn new(bits: u32) -> Self {
        WRF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRF_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Write flags for MDIO registers 0 to 31"]
    #[inline(always)]
    pub fn wrf(&self) -> WRF_R {
        WRF_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "MDIOS write flag register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_wrfr](index.html) module"]
pub struct MDIOS_WRFR_SPEC;
impl crate::RegisterSpec for MDIOS_WRFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdios_wrfr::R](R) reader structure"]
impl crate::Readable for MDIOS_WRFR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MDIOS_WRFR to value 0"]
impl crate::Resettable for MDIOS_WRFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

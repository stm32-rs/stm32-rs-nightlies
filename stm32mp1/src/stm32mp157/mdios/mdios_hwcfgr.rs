#[doc = "Register `MDIOS_HWCFGR` reader"]
pub struct R(crate::R<MDIOS_HWCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDIOS_HWCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDIOS_HWCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDIOS_HWCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NBREG` reader - NBREG"]
pub struct NBREG_R(crate::FieldReader<u8, u8>);
impl NBREG_R {
    pub(crate) fn new(bits: u8) -> Self {
        NBREG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NBREG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - NBREG"]
    #[inline(always)]
    pub fn nbreg(&self) -> NBREG_R {
        NBREG_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "MDIOS HW configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_hwcfgr](index.html) module"]
pub struct MDIOS_HWCFGR_SPEC;
impl crate::RegisterSpec for MDIOS_HWCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdios_hwcfgr::R](R) reader structure"]
impl crate::Readable for MDIOS_HWCFGR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MDIOS_HWCFGR to value 0x20"]
impl crate::Resettable for MDIOS_HWCFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x20
    }
}

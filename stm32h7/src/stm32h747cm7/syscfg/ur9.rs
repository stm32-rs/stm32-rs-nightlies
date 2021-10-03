#[doc = "Register `UR9` reader"]
pub struct R(crate::R<UR9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UR9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UR9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UR9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WRPS_2` reader - Write protection for flash bank 2"]
pub struct WRPS_2_R(crate::FieldReader<u8, u8>);
impl WRPS_2_R {
    pub(crate) fn new(bits: u8) -> Self {
        WRPS_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRPS_2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PA_BEG_2` reader - Protected area start address for bank 2"]
pub struct PA_BEG_2_R(crate::FieldReader<u16, u16>);
impl PA_BEG_2_R {
    pub(crate) fn new(bits: u16) -> Self {
        PA_BEG_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_BEG_2_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Write protection for flash bank 2"]
    #[inline(always)]
    pub fn wrps_2(&self) -> WRPS_2_R {
        WRPS_2_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:27 - Protected area start address for bank 2"]
    #[inline(always)]
    pub fn pa_beg_2(&self) -> PA_BEG_2_R {
        PA_BEG_2_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[doc = "SYSCFG user register 9\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ur9](index.html) module"]
pub struct UR9_SPEC;
impl crate::RegisterSpec for UR9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ur9::R](R) reader structure"]
impl crate::Readable for UR9_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UR9 to value 0"]
impl crate::Resettable for UR9_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

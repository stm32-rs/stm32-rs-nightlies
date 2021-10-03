#[doc = "Register `OTG_HCDMAB15` reader"]
pub struct R(crate::R<OTG_HCDMAB15_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_HCDMAB15_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_HCDMAB15_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_HCDMAB15_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HCDMAB` reader - HCDMAB"]
pub struct HCDMAB_R(crate::FieldReader<u32, u32>);
impl HCDMAB_R {
    pub(crate) fn new(bits: u32) -> Self {
        HCDMAB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HCDMAB_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - HCDMAB"]
    #[inline(always)]
    pub fn hcdmab(&self) -> HCDMAB_R {
        HCDMAB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "OTG host channel-n DMA address buffer register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcdmab15](index.html) module"]
pub struct OTG_HCDMAB15_SPEC;
impl crate::RegisterSpec for OTG_HCDMAB15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_hcdmab15::R](R) reader structure"]
impl crate::Readable for OTG_HCDMAB15_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OTG_HCDMAB15 to value 0"]
impl crate::Resettable for OTG_HCDMAB15_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

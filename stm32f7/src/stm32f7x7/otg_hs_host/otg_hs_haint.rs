#[doc = "Register `OTG_HS_HAINT` reader"]
pub struct R(crate::R<OTG_HS_HAINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_HS_HAINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_HS_HAINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_HS_HAINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HAINT` reader - Channel interrupts"]
pub struct HAINT_R(crate::FieldReader<u16, u16>);
impl HAINT_R {
    pub(crate) fn new(bits: u16) -> Self {
        HAINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HAINT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Channel interrupts"]
    #[inline(always)]
    pub fn haint(&self) -> HAINT_R {
        HAINT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "OTG_HS Host all channels interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_haint](index.html) module"]
pub struct OTG_HS_HAINT_SPEC;
impl crate::RegisterSpec for OTG_HS_HAINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_hs_haint::R](R) reader structure"]
impl crate::Readable for OTG_HS_HAINT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OTG_HS_HAINT to value 0"]
impl crate::Resettable for OTG_HS_HAINT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

#[doc = "Register `HWCFGR4` reader"]
pub struct R(crate::R<HWCFGR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWCFGR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWCFGR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWCFGR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EVENT_TRG` reader - HW configuration event trigger type"]
pub struct EVENT_TRG_R(crate::FieldReader<u32, u32>);
impl EVENT_TRG_R {
    pub(crate) fn new(bits: u32) -> Self {
        EVENT_TRG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVENT_TRG_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - HW configuration event trigger type"]
    #[inline(always)]
    pub fn event_trg(&self) -> EVENT_TRG_R {
        EVENT_TRG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Hardware configuration registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwcfgr4](index.html) module"]
pub struct HWCFGR4_SPEC;
impl crate::RegisterSpec for HWCFGR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwcfgr4::R](R) reader structure"]
impl crate::Readable for HWCFGR4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HWCFGR4 to value 0"]
impl crate::Resettable for HWCFGR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

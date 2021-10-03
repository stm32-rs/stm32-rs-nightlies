#[doc = "Register `TZC_PID0` reader"]
pub struct R(crate::R<TZC_PID0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_PID0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_PID0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_PID0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PER_ID_0` reader - PER_ID_0"]
pub struct PER_ID_0_R(crate::FieldReader<u8, u8>);
impl PER_ID_0_R {
    pub(crate) fn new(bits: u8) -> Self {
        PER_ID_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PER_ID_0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - PER_ID_0"]
    #[inline(always)]
    pub fn per_id_0(&self) -> PER_ID_0_R {
        PER_ID_0_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Peripheral ID 0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_pid0](index.html) module"]
pub struct TZC_PID0_SPEC;
impl crate::RegisterSpec for TZC_PID0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_pid0::R](R) reader structure"]
impl crate::Readable for TZC_PID0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TZC_PID0 to value 0x60"]
impl crate::Resettable for TZC_PID0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x60
    }
}

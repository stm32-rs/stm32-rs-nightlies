#[doc = "Register `STGENR_PIDR5` reader"]
pub struct R(crate::R<STGENR_PIDR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STGENR_PIDR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STGENR_PIDR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STGENR_PIDR5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PIDR5` reader - PIDR5"]
pub struct PIDR5_R(crate::FieldReader<u32, u32>);
impl PIDR5_R {
    pub(crate) fn new(bits: u32) -> Self {
        PIDR5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIDR5_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - PIDR5"]
    #[inline(always)]
    pub fn pidr5(&self) -> PIDR5_R {
        PIDR5_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "STGENR peripheral ID5 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stgenr_pidr5](index.html) module"]
pub struct STGENR_PIDR5_SPEC;
impl crate::RegisterSpec for STGENR_PIDR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stgenr_pidr5::R](R) reader structure"]
impl crate::Readable for STGENR_PIDR5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STGENR_PIDR5 to value 0"]
impl crate::Resettable for STGENR_PIDR5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

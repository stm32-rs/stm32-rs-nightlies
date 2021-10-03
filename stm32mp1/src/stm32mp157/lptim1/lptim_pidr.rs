#[doc = "Register `LPTIM_PIDR` reader"]
pub struct R(crate::R<LPTIM_PIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPTIM_PIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPTIM_PIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPTIM_PIDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `P_ID` reader - P_ID"]
pub struct P_ID_R(crate::FieldReader<u32, u32>);
impl P_ID_R {
    pub(crate) fn new(bits: u32) -> Self {
        P_ID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P_ID_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - P_ID"]
    #[inline(always)]
    pub fn p_id(&self) -> P_ID_R {
        P_ID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "LPTIM peripheral type identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lptim_pidr](index.html) module"]
pub struct LPTIM_PIDR_SPEC;
impl crate::RegisterSpec for LPTIM_PIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lptim_pidr::R](R) reader structure"]
impl crate::Readable for LPTIM_PIDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LPTIM_PIDR to value 0x0012_0011"]
impl crate::Resettable for LPTIM_PIDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0012_0011
    }
}

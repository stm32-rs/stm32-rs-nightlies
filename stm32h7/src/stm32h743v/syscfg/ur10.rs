#[doc = "Register `UR10` reader"]
pub struct R(crate::R<UR10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UR10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UR10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UR10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PA_END_2` reader - Protected area end address for bank 2"]
pub struct PA_END_2_R(crate::FieldReader<u16, u16>);
impl PA_END_2_R {
    pub(crate) fn new(bits: u16) -> Self {
        PA_END_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_END_2_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SA_BEG_2` reader - Secured area start address for bank 2"]
pub struct SA_BEG_2_R(crate::FieldReader<u16, u16>);
impl SA_BEG_2_R {
    pub(crate) fn new(bits: u16) -> Self {
        SA_BEG_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SA_BEG_2_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:11 - Protected area end address for bank 2"]
    #[inline(always)]
    pub fn pa_end_2(&self) -> PA_END_2_R {
        PA_END_2_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Secured area start address for bank 2"]
    #[inline(always)]
    pub fn sa_beg_2(&self) -> SA_BEG_2_R {
        SA_BEG_2_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[doc = "SYSCFG user register 10\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ur10](index.html) module"]
pub struct UR10_SPEC;
impl crate::RegisterSpec for UR10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ur10::R](R) reader structure"]
impl crate::Readable for UR10_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UR10 to value 0"]
impl crate::Resettable for UR10_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

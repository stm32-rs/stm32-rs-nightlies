#[doc = "Register `UR7` reader"]
pub struct R(crate::R<UR7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UR7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UR7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UR7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SA_BEG_1` reader - Secured area start address for bank 1"]
pub struct SA_BEG_1_R(crate::FieldReader<u16, u16>);
impl SA_BEG_1_R {
    pub(crate) fn new(bits: u16) -> Self {
        SA_BEG_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SA_BEG_1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SA_END_1` reader - Secured area end address for bank 1"]
pub struct SA_END_1_R(crate::FieldReader<u16, u16>);
impl SA_END_1_R {
    pub(crate) fn new(bits: u16) -> Self {
        SA_END_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SA_END_1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:11 - Secured area start address for bank 1"]
    #[inline(always)]
    pub fn sa_beg_1(&self) -> SA_BEG_1_R {
        SA_BEG_1_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Secured area end address for bank 1"]
    #[inline(always)]
    pub fn sa_end_1(&self) -> SA_END_1_R {
        SA_END_1_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[doc = "SYSCFG user register 7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ur7](index.html) module"]
pub struct UR7_SPEC;
impl crate::RegisterSpec for UR7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ur7::R](R) reader structure"]
impl crate::Readable for UR7_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UR7 to value 0"]
impl crate::Resettable for UR7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

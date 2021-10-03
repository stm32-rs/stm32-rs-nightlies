#[doc = "Register `TAMP_BKP14R` reader"]
pub struct R(crate::R<TAMP_BKP14R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAMP_BKP14R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAMP_BKP14R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAMP_BKP14R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAMP_BKP14R` writer"]
pub struct W(crate::W<TAMP_BKP14R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAMP_BKP14R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<TAMP_BKP14R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAMP_BKP14R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BKP` reader - BKP"]
pub struct BKP_R(crate::FieldReader<u32, u32>);
impl BKP_R {
    pub(crate) fn new(bits: u32) -> Self {
        BKP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BKP_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BKP` writer - BKP"]
pub struct BKP_W<'a> {
    w: &'a mut W,
}
impl<'a> BKP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - BKP"]
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - BKP"]
    #[inline(always)]
    pub fn bkp(&mut self) -> BKP_W {
        BKP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TAMP backup 14 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_bkp14r](index.html) module"]
pub struct TAMP_BKP14R_SPEC;
impl crate::RegisterSpec for TAMP_BKP14R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tamp_bkp14r::R](R) reader structure"]
impl crate::Readable for TAMP_BKP14R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tamp_bkp14r::W](W) writer structure"]
impl crate::Writable for TAMP_BKP14R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TAMP_BKP14R to value 0"]
impl crate::Resettable for TAMP_BKP14R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

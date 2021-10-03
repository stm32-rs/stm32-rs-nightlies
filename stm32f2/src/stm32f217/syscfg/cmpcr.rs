#[doc = "Register `CMPCR` reader"]
pub struct R(crate::R<CMPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMPCR` writer"]
pub struct W(crate::W<CMPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMPCR_SPEC>;
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
impl From<crate::W<CMPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `READY` reader - READY"]
pub struct READY_R(crate::FieldReader<bool, bool>);
impl READY_R {
    pub(crate) fn new(bits: bool) -> Self {
        READY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for READY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP_PD` reader - Compensation cell power-down"]
pub struct CMP_PD_R(crate::FieldReader<bool, bool>);
impl CMP_PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP_PD` writer - Compensation cell power-down"]
pub struct CMP_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_PD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - READY"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Compensation cell power-down"]
    #[inline(always)]
    pub fn cmp_pd(&self) -> CMP_PD_R {
        CMP_PD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Compensation cell power-down"]
    #[inline(always)]
    pub fn cmp_pd(&mut self) -> CMP_PD_W {
        CMP_PD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Compensation cell control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpcr](index.html) module"]
pub struct CMPCR_SPEC;
impl crate::RegisterSpec for CMPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmpcr::R](R) reader structure"]
impl crate::Readable for CMPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmpcr::W](W) writer structure"]
impl crate::Writable for CMPCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMPCR to value 0"]
impl crate::Resettable for CMPCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

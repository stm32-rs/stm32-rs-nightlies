#[doc = "Register `OPTCCR` reader"]
pub struct R(crate::R<OPTCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPTCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPTCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPTCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPTCCR` writer"]
pub struct W(crate::W<OPTCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPTCCR_SPEC>;
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
impl From<crate::W<OPTCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPTCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLR_OPTCHANGEERR` reader - OPTCHANGEERR reset bit"]
pub struct CLR_OPTCHANGEERR_R(crate::FieldReader<bool, bool>);
impl CLR_OPTCHANGEERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLR_OPTCHANGEERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLR_OPTCHANGEERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLR_OPTCHANGEERR` writer - OPTCHANGEERR reset bit"]
pub struct CLR_OPTCHANGEERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_OPTCHANGEERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 30 - OPTCHANGEERR reset bit"]
    #[inline(always)]
    pub fn clr_optchangeerr(&self) -> CLR_OPTCHANGEERR_R {
        CLR_OPTCHANGEERR_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - OPTCHANGEERR reset bit"]
    #[inline(always)]
    pub fn clr_optchangeerr(&mut self) -> CLR_OPTCHANGEERR_W {
        CLR_OPTCHANGEERR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH option clear control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [optccr](index.html) module"]
pub struct OPTCCR_SPEC;
impl crate::RegisterSpec for OPTCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [optccr::R](R) reader structure"]
impl crate::Readable for OPTCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [optccr::W](W) writer structure"]
impl crate::Writable for OPTCCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OPTCCR to value 0"]
impl crate::Resettable for OPTCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

#[doc = "Register `MTLISR` reader"]
pub struct R(crate::R<MTLISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTLISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTLISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTLISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTLISR` writer"]
pub struct W(crate::W<MTLISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTLISR_SPEC>;
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
impl From<crate::W<MTLISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTLISR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Q0IS` reader - Queue interrupt status"]
pub struct Q0IS_R(crate::FieldReader<bool, bool>);
impl Q0IS_R {
    pub(crate) fn new(bits: bool) -> Self {
        Q0IS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for Q0IS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Q0IS` writer - Queue interrupt status"]
pub struct Q0IS_W<'a> {
    w: &'a mut W,
}
impl<'a> Q0IS_W<'a> {
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
    #[doc = "Bit 0 - Queue interrupt status"]
    #[inline(always)]
    pub fn q0is(&self) -> Q0IS_R {
        Q0IS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Queue interrupt status"]
    #[inline(always)]
    pub fn q0is(&mut self) -> Q0IS_W {
        Q0IS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtlisr](index.html) module"]
pub struct MTLISR_SPEC;
impl crate::RegisterSpec for MTLISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtlisr::R](R) reader structure"]
impl crate::Readable for MTLISR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtlisr::W](W) writer structure"]
impl crate::Writable for MTLISR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MTLISR to value 0"]
impl crate::Resettable for MTLISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

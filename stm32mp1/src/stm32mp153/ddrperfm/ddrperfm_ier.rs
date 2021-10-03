#[doc = "Register `DDRPERFM_IER` reader"]
pub struct R(crate::R<DDRPERFM_IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPERFM_IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPERFM_IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPERFM_IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRPERFM_IER` writer"]
pub struct W(crate::W<DDRPERFM_IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPERFM_IER_SPEC>;
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
impl From<crate::W<DDRPERFM_IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPERFM_IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OVFIE` reader - OVFIE"]
pub struct OVFIE_R(crate::FieldReader<bool, bool>);
impl OVFIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVFIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVFIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVFIE` writer - OVFIE"]
pub struct OVFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> OVFIE_W<'a> {
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
    #[doc = "Bit 0 - OVFIE"]
    #[inline(always)]
    pub fn ovfie(&self) -> OVFIE_R {
        OVFIE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OVFIE"]
    #[inline(always)]
    pub fn ovfie(&mut self) -> OVFIE_W {
        OVFIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRPERFM interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrperfm_ier](index.html) module"]
pub struct DDRPERFM_IER_SPEC;
impl crate::RegisterSpec for DDRPERFM_IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrperfm_ier::R](R) reader structure"]
impl crate::Readable for DDRPERFM_IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrperfm_ier::W](W) writer structure"]
impl crate::Writable for DDRPERFM_IER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRPERFM_IER to value 0"]
impl crate::Resettable for DDRPERFM_IER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

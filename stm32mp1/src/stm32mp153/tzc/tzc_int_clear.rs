#[doc = "Register `TZC_INT_CLEAR` reader"]
pub struct R(crate::R<TZC_INT_CLEAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_INT_CLEAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_INT_CLEAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_INT_CLEAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TZC_INT_CLEAR` writer"]
pub struct W(crate::W<TZC_INT_CLEAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZC_INT_CLEAR_SPEC>;
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
impl From<crate::W<TZC_INT_CLEAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZC_INT_CLEAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLEAR` writer - CLEAR"]
pub struct CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> CLEAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:1 - CLEAR"]
    #[inline(always)]
    pub fn clear(&mut self) -> CLEAR_W {
        CLEAR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt clear for each filter.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_int_clear](index.html) module"]
pub struct TZC_INT_CLEAR_SPEC;
impl crate::RegisterSpec for TZC_INT_CLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_int_clear::R](R) reader structure"]
impl crate::Readable for TZC_INT_CLEAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tzc_int_clear::W](W) writer structure"]
impl crate::Writable for TZC_INT_CLEAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TZC_INT_CLEAR to value 0"]
impl crate::Resettable for TZC_INT_CLEAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

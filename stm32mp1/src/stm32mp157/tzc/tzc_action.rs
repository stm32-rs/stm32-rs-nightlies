#[doc = "Register `TZC_ACTION` reader"]
pub struct R(crate::R<TZC_ACTION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_ACTION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_ACTION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_ACTION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TZC_ACTION` writer"]
pub struct W(crate::W<TZC_ACTION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZC_ACTION_SPEC>;
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
impl From<crate::W<TZC_ACTION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZC_ACTION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REACTION_VALUE` reader - REACTION_VALUE"]
pub struct REACTION_VALUE_R(crate::FieldReader<u8, u8>);
impl REACTION_VALUE_R {
    pub(crate) fn new(bits: u8) -> Self {
        REACTION_VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REACTION_VALUE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REACTION_VALUE` writer - REACTION_VALUE"]
pub struct REACTION_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> REACTION_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - REACTION_VALUE"]
    #[inline(always)]
    pub fn reaction_value(&self) -> REACTION_VALUE_R {
        REACTION_VALUE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - REACTION_VALUE"]
    #[inline(always)]
    pub fn reaction_value(&mut self) -> REACTION_VALUE_W {
        REACTION_VALUE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controls interrupt and bus error response behavior when regions permission failures occur.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_action](index.html) module"]
pub struct TZC_ACTION_SPEC;
impl crate::RegisterSpec for TZC_ACTION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_action::R](R) reader structure"]
impl crate::Readable for TZC_ACTION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tzc_action::W](W) writer structure"]
impl crate::Writable for TZC_ACTION_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TZC_ACTION to value 0"]
impl crate::Resettable for TZC_ACTION_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

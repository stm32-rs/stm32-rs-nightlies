#[doc = "Register `PRESC` reader"]
pub struct R(crate::R<PRESC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRESC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRESC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRESC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRESC` writer"]
pub struct W(crate::W<PRESC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRESC_SPEC>;
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
impl From<crate::W<PRESC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRESC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRESCALER` reader - Clock prescaler"]
pub struct PRESCALER_R(crate::FieldReader<u8, u8>);
impl PRESCALER_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRESCALER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRESCALER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRESCALER` writer - Clock prescaler"]
pub struct PRESCALER_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESCALER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Clock prescaler"]
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Clock prescaler"]
    #[inline(always)]
    pub fn prescaler(&mut self) -> PRESCALER_W {
        PRESCALER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Prescaler register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [presc](index.html) module"]
pub struct PRESC_SPEC;
impl crate::RegisterSpec for PRESC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [presc::R](R) reader structure"]
impl crate::Readable for PRESC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [presc::W](W) writer structure"]
impl crate::Writable for PRESC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRESC to value 0"]
impl crate::Resettable for PRESC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

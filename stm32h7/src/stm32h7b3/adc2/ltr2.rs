#[doc = "Register `LTR2` reader"]
pub struct R(crate::R<LTR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LTR2` writer"]
pub struct W(crate::W<LTR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LTR2_SPEC>;
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
impl From<crate::W<LTR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LTR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LTR2` reader - Analog watchdog 2 lower threshold"]
pub struct LTR2_R(crate::FieldReader<u32, u32>);
impl LTR2_R {
    pub(crate) fn new(bits: u32) -> Self {
        LTR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LTR2_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LTR2` writer - Analog watchdog 2 lower threshold"]
pub struct LTR2_W<'a> {
    w: &'a mut W,
}
impl<'a> LTR2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff_ffff) | (value as u32 & 0x03ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:25 - Analog watchdog 2 lower threshold"]
    #[inline(always)]
    pub fn ltr2(&self) -> LTR2_R {
        LTR2_R::new((self.bits & 0x03ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:25 - Analog watchdog 2 lower threshold"]
    #[inline(always)]
    pub fn ltr2(&mut self) -> LTR2_W {
        LTR2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC watchdog lower threshold register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltr2](index.html) module"]
pub struct LTR2_SPEC;
impl crate::RegisterSpec for LTR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltr2::R](R) reader structure"]
impl crate::Readable for LTR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ltr2::W](W) writer structure"]
impl crate::Writable for LTR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LTR2 to value 0"]
impl crate::Resettable for LTR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

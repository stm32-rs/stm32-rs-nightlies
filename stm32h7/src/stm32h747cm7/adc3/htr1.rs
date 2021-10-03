#[doc = "Register `HTR1` reader"]
pub struct R(crate::R<HTR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HTR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HTR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HTR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HTR1` writer"]
pub struct W(crate::W<HTR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HTR1_SPEC>;
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
impl From<crate::W<HTR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HTR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HTR1` reader - ADC analog watchdog 2 threshold low"]
pub struct HTR1_R(crate::FieldReader<u32, u32>);
impl HTR1_R {
    pub(crate) fn new(bits: u32) -> Self {
        HTR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HTR1_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HTR1` writer - ADC analog watchdog 2 threshold low"]
pub struct HTR1_W<'a> {
    w: &'a mut W,
}
impl<'a> HTR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff_ffff) | (value as u32 & 0x03ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:25 - ADC analog watchdog 2 threshold low"]
    #[inline(always)]
    pub fn htr1(&self) -> HTR1_R {
        HTR1_R::new((self.bits & 0x03ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:25 - ADC analog watchdog 2 threshold low"]
    #[inline(always)]
    pub fn htr1(&mut self) -> HTR1_W {
        HTR1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC analog watchdog 2 threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [htr1](index.html) module"]
pub struct HTR1_SPEC;
impl crate::RegisterSpec for HTR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [htr1::R](R) reader structure"]
impl crate::Readable for HTR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [htr1::W](W) writer structure"]
impl crate::Writable for HTR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HTR1 to value 0x0fff_0000"]
impl crate::Resettable for HTR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0fff_0000
    }
}

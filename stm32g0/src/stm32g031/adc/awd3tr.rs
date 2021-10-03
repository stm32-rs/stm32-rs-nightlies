#[doc = "Register `AWD3TR` reader"]
pub struct R(crate::R<AWD3TR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AWD3TR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AWD3TR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AWD3TR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AWD3TR` writer"]
pub struct W(crate::W<AWD3TR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AWD3TR_SPEC>;
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
impl From<crate::W<AWD3TR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AWD3TR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HT3` reader - ADC analog watchdog 3 threshold high"]
pub struct HT3_R(crate::FieldReader<u16, u16>);
impl HT3_R {
    pub(crate) fn new(bits: u16) -> Self {
        HT3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HT3_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HT3` writer - ADC analog watchdog 3 threshold high"]
pub struct HT3_W<'a> {
    w: &'a mut W,
}
impl<'a> HT3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | ((value as u32 & 0x0fff) << 16);
        self.w
    }
}
#[doc = "Field `LT3` reader - ADC analog watchdog 3 threshold high"]
pub struct LT3_R(crate::FieldReader<u16, u16>);
impl LT3_R {
    pub(crate) fn new(bits: u16) -> Self {
        LT3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LT3_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LT3` writer - ADC analog watchdog 3 threshold high"]
pub struct LT3_W<'a> {
    w: &'a mut W,
}
impl<'a> LT3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:27 - ADC analog watchdog 3 threshold high"]
    #[inline(always)]
    pub fn ht3(&self) -> HT3_R {
        HT3_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:11 - ADC analog watchdog 3 threshold high"]
    #[inline(always)]
    pub fn lt3(&self) -> LT3_R {
        LT3_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:27 - ADC analog watchdog 3 threshold high"]
    #[inline(always)]
    pub fn ht3(&mut self) -> HT3_W {
        HT3_W { w: self }
    }
    #[doc = "Bits 0:11 - ADC analog watchdog 3 threshold high"]
    #[inline(always)]
    pub fn lt3(&mut self) -> LT3_W {
        LT3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "watchdog threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [awd3tr](index.html) module"]
pub struct AWD3TR_SPEC;
impl crate::RegisterSpec for AWD3TR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [awd3tr::R](R) reader structure"]
impl crate::Readable for AWD3TR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [awd3tr::W](W) writer structure"]
impl crate::Writable for AWD3TR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AWD3TR to value 0x0fff_0000"]
impl crate::Resettable for AWD3TR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0fff_0000
    }
}

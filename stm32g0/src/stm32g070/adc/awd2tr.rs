#[doc = "Register `AWD2TR` reader"]
pub struct R(crate::R<AWD2TR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AWD2TR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AWD2TR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AWD2TR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AWD2TR` writer"]
pub struct W(crate::W<AWD2TR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AWD2TR_SPEC>;
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
impl From<crate::W<AWD2TR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AWD2TR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HT2` reader - ADC analog watchdog 2 threshold high"]
pub struct HT2_R(crate::FieldReader<u16, u16>);
impl HT2_R {
    pub(crate) fn new(bits: u16) -> Self {
        HT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HT2_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HT2` writer - ADC analog watchdog 2 threshold high"]
pub struct HT2_W<'a> {
    w: &'a mut W,
}
impl<'a> HT2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | ((value as u32 & 0x0fff) << 16);
        self.w
    }
}
#[doc = "Field `LT2` reader - ADC analog watchdog 2 threshold low"]
pub struct LT2_R(crate::FieldReader<u16, u16>);
impl LT2_R {
    pub(crate) fn new(bits: u16) -> Self {
        LT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LT2_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LT2` writer - ADC analog watchdog 2 threshold low"]
pub struct LT2_W<'a> {
    w: &'a mut W,
}
impl<'a> LT2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:27 - ADC analog watchdog 2 threshold high"]
    #[inline(always)]
    pub fn ht2(&self) -> HT2_R {
        HT2_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:11 - ADC analog watchdog 2 threshold low"]
    #[inline(always)]
    pub fn lt2(&self) -> LT2_R {
        LT2_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:27 - ADC analog watchdog 2 threshold high"]
    #[inline(always)]
    pub fn ht2(&mut self) -> HT2_W {
        HT2_W { w: self }
    }
    #[doc = "Bits 0:11 - ADC analog watchdog 2 threshold low"]
    #[inline(always)]
    pub fn lt2(&mut self) -> LT2_W {
        LT2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "watchdog threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [awd2tr](index.html) module"]
pub struct AWD2TR_SPEC;
impl crate::RegisterSpec for AWD2TR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [awd2tr::R](R) reader structure"]
impl crate::Readable for AWD2TR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [awd2tr::W](W) writer structure"]
impl crate::Writable for AWD2TR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AWD2TR to value 0x0fff_0000"]
impl crate::Resettable for AWD2TR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0fff_0000
    }
}

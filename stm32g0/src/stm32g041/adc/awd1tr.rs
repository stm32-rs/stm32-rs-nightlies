#[doc = "Register `AWD1TR` reader"]
pub struct R(crate::R<AWD1TR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AWD1TR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AWD1TR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AWD1TR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AWD1TR` writer"]
pub struct W(crate::W<AWD1TR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AWD1TR_SPEC>;
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
impl From<crate::W<AWD1TR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AWD1TR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HT1` reader - ADC analog watchdog 1 threshold high"]
pub struct HT1_R(crate::FieldReader<u16, u16>);
impl HT1_R {
    pub(crate) fn new(bits: u16) -> Self {
        HT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HT1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HT1` writer - ADC analog watchdog 1 threshold high"]
pub struct HT1_W<'a> {
    w: &'a mut W,
}
impl<'a> HT1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | ((value as u32 & 0x0fff) << 16);
        self.w
    }
}
#[doc = "Field `LT1` reader - ADC analog watchdog 1 threshold low"]
pub struct LT1_R(crate::FieldReader<u16, u16>);
impl LT1_R {
    pub(crate) fn new(bits: u16) -> Self {
        LT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LT1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LT1` writer - ADC analog watchdog 1 threshold low"]
pub struct LT1_W<'a> {
    w: &'a mut W,
}
impl<'a> LT1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:27 - ADC analog watchdog 1 threshold high"]
    #[inline(always)]
    pub fn ht1(&self) -> HT1_R {
        HT1_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:11 - ADC analog watchdog 1 threshold low"]
    #[inline(always)]
    pub fn lt1(&self) -> LT1_R {
        LT1_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:27 - ADC analog watchdog 1 threshold high"]
    #[inline(always)]
    pub fn ht1(&mut self) -> HT1_W {
        HT1_W { w: self }
    }
    #[doc = "Bits 0:11 - ADC analog watchdog 1 threshold low"]
    #[inline(always)]
    pub fn lt1(&mut self) -> LT1_W {
        LT1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "watchdog threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [awd1tr](index.html) module"]
pub struct AWD1TR_SPEC;
impl crate::RegisterSpec for AWD1TR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [awd1tr::R](R) reader structure"]
impl crate::Readable for AWD1TR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [awd1tr::W](W) writer structure"]
impl crate::Writable for AWD1TR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AWD1TR to value 0x0fff_0000"]
impl crate::Resettable for AWD1TR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0fff_0000
    }
}

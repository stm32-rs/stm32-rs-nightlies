#[doc = "Reader of register ADCPS1"]
pub type R = crate::R<u32, super::ADCPS1>;
#[doc = "Writer for register ADCPS1"]
pub type W = crate::W<u32, super::ADCPS1>;
#[doc = "Register ADCPS1 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADCPS1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC5PSC`"]
pub type ADC5PSC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC5PSC`"]
pub struct ADC5PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC5PSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
#[doc = "Reader of field `ADC4PSC`"]
pub type ADC4PSC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC4PSC`"]
pub struct ADC4PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC4PSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 18)) | (((value as u32) & 0x1f) << 18);
        self.w
    }
}
#[doc = "Reader of field `ADC3PSC`"]
pub type ADC3PSC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC3PSC`"]
pub struct ADC3PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC3PSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 12)) | (((value as u32) & 0x1f) << 12);
        self.w
    }
}
#[doc = "Reader of field `ADC2PSC`"]
pub type ADC2PSC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC2PSC`"]
pub struct ADC2PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC2PSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 6)) | (((value as u32) & 0x1f) << 6);
        self.w
    }
}
#[doc = "Reader of field `ADC1PSC`"]
pub type ADC1PSC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC1PSC`"]
pub struct ADC1PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1PSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:28 - ADC5PSC"]
    #[inline(always)]
    pub fn adc5psc(&self) -> ADC5PSC_R {
        ADC5PSC_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 18:22 - ADC4PSC"]
    #[inline(always)]
    pub fn adc4psc(&self) -> ADC4PSC_R {
        ADC4PSC_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 12:16 - ADC3PSC"]
    #[inline(always)]
    pub fn adc3psc(&self) -> ADC3PSC_R {
        ADC3PSC_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 6:10 - ADC2PSC"]
    #[inline(always)]
    pub fn adc2psc(&self) -> ADC2PSC_R {
        ADC2PSC_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - ADC1PSC"]
    #[inline(always)]
    pub fn adc1psc(&self) -> ADC1PSC_R {
        ADC1PSC_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:28 - ADC5PSC"]
    #[inline(always)]
    pub fn adc5psc(&mut self) -> ADC5PSC_W {
        ADC5PSC_W { w: self }
    }
    #[doc = "Bits 18:22 - ADC4PSC"]
    #[inline(always)]
    pub fn adc4psc(&mut self) -> ADC4PSC_W {
        ADC4PSC_W { w: self }
    }
    #[doc = "Bits 12:16 - ADC3PSC"]
    #[inline(always)]
    pub fn adc3psc(&mut self) -> ADC3PSC_W {
        ADC3PSC_W { w: self }
    }
    #[doc = "Bits 6:10 - ADC2PSC"]
    #[inline(always)]
    pub fn adc2psc(&mut self) -> ADC2PSC_W {
        ADC2PSC_W { w: self }
    }
    #[doc = "Bits 0:4 - ADC1PSC"]
    #[inline(always)]
    pub fn adc1psc(&mut self) -> ADC1PSC_W {
        ADC1PSC_W { w: self }
    }
}

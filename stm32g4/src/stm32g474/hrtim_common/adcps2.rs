#[doc = "Reader of register ADCPS2"]
pub type R = crate::R<u32, super::ADCPS2>;
#[doc = "Writer for register ADCPS2"]
pub type W = crate::W<u32, super::ADCPS2>;
#[doc = "Register ADCPS2 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADCPS2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC10PSC`"]
pub type ADC10PSC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC10PSC`"]
pub struct ADC10PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC10PSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
#[doc = "Reader of field `ADC9PSC`"]
pub type ADC9PSC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC9PSC`"]
pub struct ADC9PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC9PSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 18)) | (((value as u32) & 0x1f) << 18);
        self.w
    }
}
#[doc = "Reader of field `ADC8PSC`"]
pub type ADC8PSC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC8PSC`"]
pub struct ADC8PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC8PSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 12)) | (((value as u32) & 0x1f) << 12);
        self.w
    }
}
#[doc = "Reader of field `ADC7PSC`"]
pub type ADC7PSC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC7PSC`"]
pub struct ADC7PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC7PSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 6)) | (((value as u32) & 0x1f) << 6);
        self.w
    }
}
#[doc = "Reader of field `ADC6PSC`"]
pub type ADC6PSC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC6PSC`"]
pub struct ADC6PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC6PSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:28 - ADC10PSC"]
    #[inline(always)]
    pub fn adc10psc(&self) -> ADC10PSC_R {
        ADC10PSC_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 18:22 - ADC9PSC"]
    #[inline(always)]
    pub fn adc9psc(&self) -> ADC9PSC_R {
        ADC9PSC_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 12:16 - ADC8PSC"]
    #[inline(always)]
    pub fn adc8psc(&self) -> ADC8PSC_R {
        ADC8PSC_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 6:10 - ADC7PSC"]
    #[inline(always)]
    pub fn adc7psc(&self) -> ADC7PSC_R {
        ADC7PSC_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - ADC6PSC"]
    #[inline(always)]
    pub fn adc6psc(&self) -> ADC6PSC_R {
        ADC6PSC_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:28 - ADC10PSC"]
    #[inline(always)]
    pub fn adc10psc(&mut self) -> ADC10PSC_W {
        ADC10PSC_W { w: self }
    }
    #[doc = "Bits 18:22 - ADC9PSC"]
    #[inline(always)]
    pub fn adc9psc(&mut self) -> ADC9PSC_W {
        ADC9PSC_W { w: self }
    }
    #[doc = "Bits 12:16 - ADC8PSC"]
    #[inline(always)]
    pub fn adc8psc(&mut self) -> ADC8PSC_W {
        ADC8PSC_W { w: self }
    }
    #[doc = "Bits 6:10 - ADC7PSC"]
    #[inline(always)]
    pub fn adc7psc(&mut self) -> ADC7PSC_W {
        ADC7PSC_W { w: self }
    }
    #[doc = "Bits 0:4 - ADC6PSC"]
    #[inline(always)]
    pub fn adc6psc(&mut self) -> ADC6PSC_W {
        ADC6PSC_W { w: self }
    }
}

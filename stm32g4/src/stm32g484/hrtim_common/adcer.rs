#[doc = "Reader of register ADCER"]
pub type R = crate::R<u32, super::ADCER>;
#[doc = "Writer for register ADCER"]
pub type W = crate::W<u32, super::ADCER>;
#[doc = "Register ADCER `reset()`'s with value 0"]
impl crate::ResetValue for super::ADCER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC10TRG`"]
pub type ADC10TRG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC10TRG`"]
pub struct ADC10TRG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC10TRG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 26)) | (((value as u32) & 0x1f) << 26);
        self.w
    }
}
#[doc = "Reader of field `ADC9TRG`"]
pub type ADC9TRG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC9TRG`"]
pub struct ADC9TRG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC9TRG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 21)) | (((value as u32) & 0x1f) << 21);
        self.w
    }
}
#[doc = "Reader of field `ADC8TRG`"]
pub type ADC8TRG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC8TRG`"]
pub struct ADC8TRG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC8TRG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `ADC7TRG`"]
pub type ADC7TRG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC7TRG`"]
pub struct ADC7TRG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC7TRG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | (((value as u32) & 0x1f) << 10);
        self.w
    }
}
#[doc = "Reader of field `ADC6TRG`"]
pub type ADC6TRG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC6TRG`"]
pub struct ADC6TRG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC6TRG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | (((value as u32) & 0x1f) << 5);
        self.w
    }
}
#[doc = "Reader of field `ADC5TRG`"]
pub type ADC5TRG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC5TRG`"]
pub struct ADC5TRG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC5TRG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 26:30 - ADC10TRG"]
    #[inline(always)]
    pub fn adc10trg(&self) -> ADC10TRG_R {
        ADC10TRG_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25 - ADC9TRG"]
    #[inline(always)]
    pub fn adc9trg(&self) -> ADC9TRG_R {
        ADC9TRG_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - ADC8TRG"]
    #[inline(always)]
    pub fn adc8trg(&self) -> ADC8TRG_R {
        ADC8TRG_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - ADC7TRG"]
    #[inline(always)]
    pub fn adc7trg(&self) -> ADC7TRG_R {
        ADC7TRG_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - ADC6TRG"]
    #[inline(always)]
    pub fn adc6trg(&self) -> ADC6TRG_R {
        ADC6TRG_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - ADC5TRG"]
    #[inline(always)]
    pub fn adc5trg(&self) -> ADC5TRG_R {
        ADC5TRG_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 26:30 - ADC10TRG"]
    #[inline(always)]
    pub fn adc10trg(&mut self) -> ADC10TRG_W {
        ADC10TRG_W { w: self }
    }
    #[doc = "Bits 21:25 - ADC9TRG"]
    #[inline(always)]
    pub fn adc9trg(&mut self) -> ADC9TRG_W {
        ADC9TRG_W { w: self }
    }
    #[doc = "Bits 16:20 - ADC8TRG"]
    #[inline(always)]
    pub fn adc8trg(&mut self) -> ADC8TRG_W {
        ADC8TRG_W { w: self }
    }
    #[doc = "Bits 10:14 - ADC7TRG"]
    #[inline(always)]
    pub fn adc7trg(&mut self) -> ADC7TRG_W {
        ADC7TRG_W { w: self }
    }
    #[doc = "Bits 5:9 - ADC6TRG"]
    #[inline(always)]
    pub fn adc6trg(&mut self) -> ADC6TRG_W {
        ADC6TRG_W { w: self }
    }
    #[doc = "Bits 0:4 - ADC5TRG"]
    #[inline(always)]
    pub fn adc5trg(&mut self) -> ADC5TRG_W {
        ADC5TRG_W { w: self }
    }
}

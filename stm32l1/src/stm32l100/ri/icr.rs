#[doc = "Reader of register ICR"]
pub type R = crate::R<u32, super::ICR>;
#[doc = "Writer for register ICR"]
pub type W = crate::W<u32, super::ICR>;
#[doc = "Register ICR `reset()`'s with value 0"]
impl crate::ResetValue for super::ICR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IC4`"]
pub type IC4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IC4`"]
pub struct IC4_W<'a> {
    w: &'a mut W,
}
impl<'a> IC4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `IC3`"]
pub type IC3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IC3`"]
pub struct IC3_W<'a> {
    w: &'a mut W,
}
impl<'a> IC3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `IC2`"]
pub type IC2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IC2`"]
pub struct IC2_W<'a> {
    w: &'a mut W,
}
impl<'a> IC2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `IC1`"]
pub type IC1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IC1`"]
pub struct IC1_W<'a> {
    w: &'a mut W,
}
impl<'a> IC1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `TIM`"]
pub type TIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TIM`"]
pub struct TIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `IC4IOS`"]
pub type IC4IOS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IC4IOS`"]
pub struct IC4IOS_W<'a> {
    w: &'a mut W,
}
impl<'a> IC4IOS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `IC3IOS`"]
pub type IC3IOS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IC3IOS`"]
pub struct IC3IOS_W<'a> {
    w: &'a mut W,
}
impl<'a> IC3IOS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `IC2IOS`"]
pub type IC2IOS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IC2IOS`"]
pub struct IC2IOS_W<'a> {
    w: &'a mut W,
}
impl<'a> IC2IOS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `IC1IOS`"]
pub type IC1IOS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IC1IOS`"]
pub struct IC1IOS_W<'a> {
    w: &'a mut W,
}
impl<'a> IC1IOS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 21 - IC4"]
    #[inline(always)]
    pub fn ic4(&self) -> IC4_R {
        IC4_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - IC3"]
    #[inline(always)]
    pub fn ic3(&self) -> IC3_R {
        IC3_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - IC2"]
    #[inline(always)]
    pub fn ic2(&self) -> IC2_R {
        IC2_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - IC1"]
    #[inline(always)]
    pub fn ic1(&self) -> IC1_R {
        IC1_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Timer select bits"]
    #[inline(always)]
    pub fn tim(&self) -> TIM_R {
        TIM_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 12:15 - Input capture 4 select bits"]
    #[inline(always)]
    pub fn ic4ios(&self) -> IC4IOS_R {
        IC4IOS_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Input capture 3 select bits"]
    #[inline(always)]
    pub fn ic3ios(&self) -> IC3IOS_R {
        IC3IOS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Input capture 2 select bits"]
    #[inline(always)]
    pub fn ic2ios(&self) -> IC2IOS_R {
        IC2IOS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - Input capture 1 select bits"]
    #[inline(always)]
    pub fn ic1ios(&self) -> IC1IOS_R {
        IC1IOS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 21 - IC4"]
    #[inline(always)]
    pub fn ic4(&mut self) -> IC4_W {
        IC4_W { w: self }
    }
    #[doc = "Bit 20 - IC3"]
    #[inline(always)]
    pub fn ic3(&mut self) -> IC3_W {
        IC3_W { w: self }
    }
    #[doc = "Bit 19 - IC2"]
    #[inline(always)]
    pub fn ic2(&mut self) -> IC2_W {
        IC2_W { w: self }
    }
    #[doc = "Bit 18 - IC1"]
    #[inline(always)]
    pub fn ic1(&mut self) -> IC1_W {
        IC1_W { w: self }
    }
    #[doc = "Bits 16:17 - Timer select bits"]
    #[inline(always)]
    pub fn tim(&mut self) -> TIM_W {
        TIM_W { w: self }
    }
    #[doc = "Bits 12:15 - Input capture 4 select bits"]
    #[inline(always)]
    pub fn ic4ios(&mut self) -> IC4IOS_W {
        IC4IOS_W { w: self }
    }
    #[doc = "Bits 8:11 - Input capture 3 select bits"]
    #[inline(always)]
    pub fn ic3ios(&mut self) -> IC3IOS_W {
        IC3IOS_W { w: self }
    }
    #[doc = "Bits 4:7 - Input capture 2 select bits"]
    #[inline(always)]
    pub fn ic2ios(&mut self) -> IC2IOS_W {
        IC2IOS_W { w: self }
    }
    #[doc = "Bits 0:3 - Input capture 1 select bits"]
    #[inline(always)]
    pub fn ic1ios(&mut self) -> IC1IOS_W {
        IC1IOS_W { w: self }
    }
}

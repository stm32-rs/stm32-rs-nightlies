#[doc = "Reader of register CNT"]
pub type R = crate::R<u32, super::CNT>;
#[doc = "Writer for register CNT"]
pub type W = crate::W<u32, super::CNT>;
#[doc = "Register CNT `reset()`'s with value 0"]
impl crate::ResetValue for super::CNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CNT_H`"]
pub type CNT_H_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CNT_H`"]
pub struct CNT_H_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff << 16)) | (((value as u32) & 0x7fff) << 16);
        self.w
    }
}
#[doc = "Reader of field `CNT_L`"]
pub type CNT_L_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CNT_L`"]
pub struct CNT_L_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `CNT_bit31`"]
pub type CNT_BIT31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNT_bit31`"]
pub struct CNT_BIT31_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_BIT31_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:30 - Most significant part counter value (on TIM2 and TIM5)"]
    #[inline(always)]
    pub fn cnt_h(&self) -> CNT_H_R {
        CNT_H_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bits 0:15 - Least significant part of counter value"]
    #[inline(always)]
    pub fn cnt_l(&self) -> CNT_L_R {
        CNT_L_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Most significant bit of counter value (on TIM2 and TIM5)"]
    #[inline(always)]
    pub fn cnt_bit31(&self) -> CNT_BIT31_R {
        CNT_BIT31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:30 - Most significant part counter value (on TIM2 and TIM5)"]
    #[inline(always)]
    pub fn cnt_h(&mut self) -> CNT_H_W {
        CNT_H_W { w: self }
    }
    #[doc = "Bits 0:15 - Least significant part of counter value"]
    #[inline(always)]
    pub fn cnt_l(&mut self) -> CNT_L_W {
        CNT_L_W { w: self }
    }
    #[doc = "Bit 31 - Most significant bit of counter value (on TIM2 and TIM5)"]
    #[inline(always)]
    pub fn cnt_bit31(&mut self) -> CNT_BIT31_W {
        CNT_BIT31_W { w: self }
    }
}

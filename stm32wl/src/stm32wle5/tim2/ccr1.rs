#[doc = "Reader of register CCR1"]
pub type R = crate::R<u32, super::CCR1>;
#[doc = "Writer for register CCR1"]
pub type W = crate::W<u32, super::CCR1>;
#[doc = "Register CCR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CCR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CCR1_H`"]
pub type CCR1_H_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CCR1_H`"]
pub struct CCR1_H_W<'a> {
    w: &'a mut W,
}
impl<'a> CCR1_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `CCR1_L`"]
pub type CCR1_L_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CCR1_L`"]
pub struct CCR1_L_W<'a> {
    w: &'a mut W,
}
impl<'a> CCR1_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - High Capture/Compare 1 value (TIM2 only)"]
    #[inline(always)]
    pub fn ccr1_h(&self) -> CCR1_H_R {
        CCR1_H_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Low Capture/Compare 1 value"]
    #[inline(always)]
    pub fn ccr1_l(&self) -> CCR1_L_R {
        CCR1_L_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - High Capture/Compare 1 value (TIM2 only)"]
    #[inline(always)]
    pub fn ccr1_h(&mut self) -> CCR1_H_W {
        CCR1_H_W { w: self }
    }
    #[doc = "Bits 0:15 - Low Capture/Compare 1 value"]
    #[inline(always)]
    pub fn ccr1_l(&mut self) -> CCR1_L_W {
        CCR1_L_W { w: self }
    }
}

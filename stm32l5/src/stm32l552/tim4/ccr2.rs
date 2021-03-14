#[doc = "Reader of register CCR2"]
pub type R = crate::R<u32, super::CCR2>;
#[doc = "Writer for register CCR2"]
pub type W = crate::W<u32, super::CCR2>;
#[doc = "Register CCR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CCR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CCR2_H`"]
pub type CCR2_H_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CCR2_H`"]
pub struct CCR2_H_W<'a> {
    w: &'a mut W,
}
impl<'a> CCR2_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `CCR2_L`"]
pub type CCR2_L_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CCR2_L`"]
pub struct CCR2_L_W<'a> {
    w: &'a mut W,
}
impl<'a> CCR2_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - High Capture/Compare 2 value (TIM2 only)"]
    #[inline(always)]
    pub fn ccr2_h(&self) -> CCR2_H_R {
        CCR2_H_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Low Capture/Compare 2 value"]
    #[inline(always)]
    pub fn ccr2_l(&self) -> CCR2_L_R {
        CCR2_L_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - High Capture/Compare 2 value (TIM2 only)"]
    #[inline(always)]
    pub fn ccr2_h(&mut self) -> CCR2_H_W {
        CCR2_H_W { w: self }
    }
    #[doc = "Bits 0:15 - Low Capture/Compare 2 value"]
    #[inline(always)]
    pub fn ccr2_l(&mut self) -> CCR2_L_W {
        CCR2_L_W { w: self }
    }
}

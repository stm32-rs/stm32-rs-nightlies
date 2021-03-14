#[doc = "Reader of register CCR4"]
pub type R = crate::R<u32, super::CCR4>;
#[doc = "Writer for register CCR4"]
pub type W = crate::W<u32, super::CCR4>;
#[doc = "Register CCR4 `reset()`'s with value 0"]
impl crate::ResetValue for super::CCR4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CCR4_H`"]
pub type CCR4_H_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CCR4_H`"]
pub struct CCR4_H_W<'a> {
    w: &'a mut W,
}
impl<'a> CCR4_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `CCR4_L`"]
pub type CCR4_L_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CCR4_L`"]
pub struct CCR4_L_W<'a> {
    w: &'a mut W,
}
impl<'a> CCR4_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - High Capture/Compare value (TIM2 only)"]
    #[inline(always)]
    pub fn ccr4_h(&self) -> CCR4_H_R {
        CCR4_H_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Low Capture/Compare value"]
    #[inline(always)]
    pub fn ccr4_l(&self) -> CCR4_L_R {
        CCR4_L_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - High Capture/Compare value (TIM2 only)"]
    #[inline(always)]
    pub fn ccr4_h(&mut self) -> CCR4_H_W {
        CCR4_H_W { w: self }
    }
    #[doc = "Bits 0:15 - Low Capture/Compare value"]
    #[inline(always)]
    pub fn ccr4_l(&mut self) -> CCR4_L_W {
        CCR4_L_W { w: self }
    }
}

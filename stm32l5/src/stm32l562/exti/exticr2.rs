#[doc = "Reader of register EXTICR2"]
pub type R = crate::R<u32, super::EXTICR2>;
#[doc = "Writer for register EXTICR2"]
pub type W = crate::W<u32, super::EXTICR2>;
#[doc = "Register EXTICR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTICR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXTI0_7`"]
pub type EXTI0_7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTI0_7`"]
pub struct EXTI0_7_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI0_7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `EXTI8_15`"]
pub type EXTI8_15_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTI8_15`"]
pub struct EXTI8_15_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI8_15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `EXTI16_23`"]
pub type EXTI16_23_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTI16_23`"]
pub struct EXTI16_23_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI16_23_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `EXTI24_31`"]
pub type EXTI24_31_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTI24_31`"]
pub struct EXTI24_31_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI24_31_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - EXTIm GPIO port selection"]
    #[inline(always)]
    pub fn exti0_7(&self) -> EXTI0_7_R {
        EXTI0_7_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - EXTIm+1 GPIO port selection"]
    #[inline(always)]
    pub fn exti8_15(&self) -> EXTI8_15_R {
        EXTI8_15_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - EXTIm+2 GPIO port selection"]
    #[inline(always)]
    pub fn exti16_23(&self) -> EXTI16_23_R {
        EXTI16_23_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - EXTIm+3 GPIO port selection"]
    #[inline(always)]
    pub fn exti24_31(&self) -> EXTI24_31_R {
        EXTI24_31_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - EXTIm GPIO port selection"]
    #[inline(always)]
    pub fn exti0_7(&mut self) -> EXTI0_7_W {
        EXTI0_7_W { w: self }
    }
    #[doc = "Bits 8:15 - EXTIm+1 GPIO port selection"]
    #[inline(always)]
    pub fn exti8_15(&mut self) -> EXTI8_15_W {
        EXTI8_15_W { w: self }
    }
    #[doc = "Bits 16:23 - EXTIm+2 GPIO port selection"]
    #[inline(always)]
    pub fn exti16_23(&mut self) -> EXTI16_23_W {
        EXTI16_23_W { w: self }
    }
    #[doc = "Bits 24:31 - EXTIm+3 GPIO port selection"]
    #[inline(always)]
    pub fn exti24_31(&mut self) -> EXTI24_31_W {
        EXTI24_31_W { w: self }
    }
}

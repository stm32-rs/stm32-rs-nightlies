#[doc = "Reader of register EXTI_EXTICR2"]
pub type R = crate::R<u32, super::EXTI_EXTICR2>;
#[doc = "Writer for register EXTI_EXTICR2"]
pub type W = crate::W<u32, super::EXTI_EXTICR2>;
#[doc = "Register EXTI_EXTICR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTI_EXTICR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXTI4`"]
pub type EXTI4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTI4`"]
pub struct EXTI4_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `EXTI5`"]
pub type EXTI5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTI5`"]
pub struct EXTI5_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `EXTI6`"]
pub type EXTI6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTI6`"]
pub struct EXTI6_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `EXTI7`"]
pub type EXTI7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTI7`"]
pub struct EXTI7_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - EXTI4"]
    #[inline(always)]
    pub fn exti4(&self) -> EXTI4_R {
        EXTI4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - EXTI5"]
    #[inline(always)]
    pub fn exti5(&self) -> EXTI5_R {
        EXTI5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - EXTI6"]
    #[inline(always)]
    pub fn exti6(&self) -> EXTI6_R {
        EXTI6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - EXTI7"]
    #[inline(always)]
    pub fn exti7(&self) -> EXTI7_R {
        EXTI7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - EXTI4"]
    #[inline(always)]
    pub fn exti4(&mut self) -> EXTI4_W {
        EXTI4_W { w: self }
    }
    #[doc = "Bits 8:15 - EXTI5"]
    #[inline(always)]
    pub fn exti5(&mut self) -> EXTI5_W {
        EXTI5_W { w: self }
    }
    #[doc = "Bits 16:23 - EXTI6"]
    #[inline(always)]
    pub fn exti6(&mut self) -> EXTI6_W {
        EXTI6_W { w: self }
    }
    #[doc = "Bits 24:31 - EXTI7"]
    #[inline(always)]
    pub fn exti7(&mut self) -> EXTI7_W {
        EXTI7_W { w: self }
    }
}

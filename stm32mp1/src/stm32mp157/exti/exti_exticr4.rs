#[doc = "Reader of register EXTI_EXTICR4"]
pub type R = crate::R<u32, super::EXTI_EXTICR4>;
#[doc = "Writer for register EXTI_EXTICR4"]
pub type W = crate::W<u32, super::EXTI_EXTICR4>;
#[doc = "Register EXTI_EXTICR4 `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTI_EXTICR4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXTI12`"]
pub type EXTI12_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTI12`"]
pub struct EXTI12_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `EXTI13`"]
pub type EXTI13_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTI13`"]
pub struct EXTI13_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `EXTI14`"]
pub type EXTI14_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTI14`"]
pub struct EXTI14_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `EXTI15`"]
pub type EXTI15_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTI15`"]
pub struct EXTI15_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - EXTI12"]
    #[inline(always)]
    pub fn exti12(&self) -> EXTI12_R {
        EXTI12_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - EXTI13"]
    #[inline(always)]
    pub fn exti13(&self) -> EXTI13_R {
        EXTI13_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - EXTI14"]
    #[inline(always)]
    pub fn exti14(&self) -> EXTI14_R {
        EXTI14_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - EXTI15"]
    #[inline(always)]
    pub fn exti15(&self) -> EXTI15_R {
        EXTI15_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - EXTI12"]
    #[inline(always)]
    pub fn exti12(&mut self) -> EXTI12_W {
        EXTI12_W { w: self }
    }
    #[doc = "Bits 8:15 - EXTI13"]
    #[inline(always)]
    pub fn exti13(&mut self) -> EXTI13_W {
        EXTI13_W { w: self }
    }
    #[doc = "Bits 16:23 - EXTI14"]
    #[inline(always)]
    pub fn exti14(&mut self) -> EXTI14_W {
        EXTI14_W { w: self }
    }
    #[doc = "Bits 24:31 - EXTI15"]
    #[inline(always)]
    pub fn exti15(&mut self) -> EXTI15_W {
        EXTI15_W { w: self }
    }
}

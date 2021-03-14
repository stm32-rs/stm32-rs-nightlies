#[doc = "Reader of register GPIOA_AFRH"]
pub type R = crate::R<u32, super::GPIOA_AFRH>;
#[doc = "Writer for register GPIOA_AFRH"]
pub type W = crate::W<u32, super::GPIOA_AFRH>;
#[doc = "Register GPIOA_AFRH `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIOA_AFRH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AFR8`"]
pub type AFR8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AFR8`"]
pub struct AFR8_W<'a> {
    w: &'a mut W,
}
impl<'a> AFR8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `AFR9`"]
pub type AFR9_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AFR9`"]
pub struct AFR9_W<'a> {
    w: &'a mut W,
}
impl<'a> AFR9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `AFR10`"]
pub type AFR10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AFR10`"]
pub struct AFR10_W<'a> {
    w: &'a mut W,
}
impl<'a> AFR10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `AFR11`"]
pub type AFR11_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AFR11`"]
pub struct AFR11_W<'a> {
    w: &'a mut W,
}
impl<'a> AFR11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `AFR12`"]
pub type AFR12_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AFR12`"]
pub struct AFR12_W<'a> {
    w: &'a mut W,
}
impl<'a> AFR12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `AFR13`"]
pub type AFR13_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AFR13`"]
pub struct AFR13_W<'a> {
    w: &'a mut W,
}
impl<'a> AFR13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `AFR14`"]
pub type AFR14_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AFR14`"]
pub struct AFR14_W<'a> {
    w: &'a mut W,
}
impl<'a> AFR14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `AFR15`"]
pub type AFR15_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AFR15`"]
pub struct AFR15_W<'a> {
    w: &'a mut W,
}
impl<'a> AFR15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - AFR8"]
    #[inline(always)]
    pub fn afr8(&self) -> AFR8_R {
        AFR8_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - AFR9"]
    #[inline(always)]
    pub fn afr9(&self) -> AFR9_R {
        AFR9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - AFR10"]
    #[inline(always)]
    pub fn afr10(&self) -> AFR10_R {
        AFR10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - AFR11"]
    #[inline(always)]
    pub fn afr11(&self) -> AFR11_R {
        AFR11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - AFR12"]
    #[inline(always)]
    pub fn afr12(&self) -> AFR12_R {
        AFR12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - AFR13"]
    #[inline(always)]
    pub fn afr13(&self) -> AFR13_R {
        AFR13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - AFR14"]
    #[inline(always)]
    pub fn afr14(&self) -> AFR14_R {
        AFR14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - AFR15"]
    #[inline(always)]
    pub fn afr15(&self) -> AFR15_R {
        AFR15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - AFR8"]
    #[inline(always)]
    pub fn afr8(&mut self) -> AFR8_W {
        AFR8_W { w: self }
    }
    #[doc = "Bits 4:7 - AFR9"]
    #[inline(always)]
    pub fn afr9(&mut self) -> AFR9_W {
        AFR9_W { w: self }
    }
    #[doc = "Bits 8:11 - AFR10"]
    #[inline(always)]
    pub fn afr10(&mut self) -> AFR10_W {
        AFR10_W { w: self }
    }
    #[doc = "Bits 12:15 - AFR11"]
    #[inline(always)]
    pub fn afr11(&mut self) -> AFR11_W {
        AFR11_W { w: self }
    }
    #[doc = "Bits 16:19 - AFR12"]
    #[inline(always)]
    pub fn afr12(&mut self) -> AFR12_W {
        AFR12_W { w: self }
    }
    #[doc = "Bits 20:23 - AFR13"]
    #[inline(always)]
    pub fn afr13(&mut self) -> AFR13_W {
        AFR13_W { w: self }
    }
    #[doc = "Bits 24:27 - AFR14"]
    #[inline(always)]
    pub fn afr14(&mut self) -> AFR14_W {
        AFR14_W { w: self }
    }
    #[doc = "Bits 28:31 - AFR15"]
    #[inline(always)]
    pub fn afr15(&mut self) -> AFR15_W {
        AFR15_W { w: self }
    }
}

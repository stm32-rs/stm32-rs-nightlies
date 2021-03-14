#[doc = "Reader of register GPIOC_AFRL"]
pub type R = crate::R<u32, super::GPIOC_AFRL>;
#[doc = "Writer for register GPIOC_AFRL"]
pub type W = crate::W<u32, super::GPIOC_AFRL>;
#[doc = "Register GPIOC_AFRL `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIOC_AFRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AFR0`"]
pub type AFR0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AFR0`"]
pub struct AFR0_W<'a> {
    w: &'a mut W,
}
impl<'a> AFR0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `AFR1`"]
pub type AFR1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AFR1`"]
pub struct AFR1_W<'a> {
    w: &'a mut W,
}
impl<'a> AFR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `AFR2`"]
pub type AFR2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AFR2`"]
pub struct AFR2_W<'a> {
    w: &'a mut W,
}
impl<'a> AFR2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `AFR3`"]
pub type AFR3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AFR3`"]
pub struct AFR3_W<'a> {
    w: &'a mut W,
}
impl<'a> AFR3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `AFR4`"]
pub type AFR4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AFR4`"]
pub struct AFR4_W<'a> {
    w: &'a mut W,
}
impl<'a> AFR4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `AFR5`"]
pub type AFR5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AFR5`"]
pub struct AFR5_W<'a> {
    w: &'a mut W,
}
impl<'a> AFR5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `AFR6`"]
pub type AFR6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AFR6`"]
pub struct AFR6_W<'a> {
    w: &'a mut W,
}
impl<'a> AFR6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `AFR7`"]
pub type AFR7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AFR7`"]
pub struct AFR7_W<'a> {
    w: &'a mut W,
}
impl<'a> AFR7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - AFR0"]
    #[inline(always)]
    pub fn afr0(&self) -> AFR0_R {
        AFR0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - AFR1"]
    #[inline(always)]
    pub fn afr1(&self) -> AFR1_R {
        AFR1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - AFR2"]
    #[inline(always)]
    pub fn afr2(&self) -> AFR2_R {
        AFR2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - AFR3"]
    #[inline(always)]
    pub fn afr3(&self) -> AFR3_R {
        AFR3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - AFR4"]
    #[inline(always)]
    pub fn afr4(&self) -> AFR4_R {
        AFR4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - AFR5"]
    #[inline(always)]
    pub fn afr5(&self) -> AFR5_R {
        AFR5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - AFR6"]
    #[inline(always)]
    pub fn afr6(&self) -> AFR6_R {
        AFR6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - AFR7"]
    #[inline(always)]
    pub fn afr7(&self) -> AFR7_R {
        AFR7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - AFR0"]
    #[inline(always)]
    pub fn afr0(&mut self) -> AFR0_W {
        AFR0_W { w: self }
    }
    #[doc = "Bits 4:7 - AFR1"]
    #[inline(always)]
    pub fn afr1(&mut self) -> AFR1_W {
        AFR1_W { w: self }
    }
    #[doc = "Bits 8:11 - AFR2"]
    #[inline(always)]
    pub fn afr2(&mut self) -> AFR2_W {
        AFR2_W { w: self }
    }
    #[doc = "Bits 12:15 - AFR3"]
    #[inline(always)]
    pub fn afr3(&mut self) -> AFR3_W {
        AFR3_W { w: self }
    }
    #[doc = "Bits 16:19 - AFR4"]
    #[inline(always)]
    pub fn afr4(&mut self) -> AFR4_W {
        AFR4_W { w: self }
    }
    #[doc = "Bits 20:23 - AFR5"]
    #[inline(always)]
    pub fn afr5(&mut self) -> AFR5_W {
        AFR5_W { w: self }
    }
    #[doc = "Bits 24:27 - AFR6"]
    #[inline(always)]
    pub fn afr6(&mut self) -> AFR6_W {
        AFR6_W { w: self }
    }
    #[doc = "Bits 28:31 - AFR7"]
    #[inline(always)]
    pub fn afr7(&mut self) -> AFR7_W {
        AFR7_W { w: self }
    }
}

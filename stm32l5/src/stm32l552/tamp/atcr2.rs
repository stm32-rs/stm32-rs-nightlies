#[doc = "Reader of register ATCR2"]
pub type R = crate::R<u32, super::ATCR2>;
#[doc = "Writer for register ATCR2"]
pub type W = crate::W<u32, super::ATCR2>;
#[doc = "Register ATCR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::ATCR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ATOSEL1`"]
pub type ATOSEL1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ATOSEL1`"]
pub struct ATOSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> ATOSEL1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `ATOSEL2`"]
pub type ATOSEL2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ATOSEL2`"]
pub struct ATOSEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> ATOSEL2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | (((value as u32) & 0x07) << 11);
        self.w
    }
}
#[doc = "Reader of field `ATOSEL3`"]
pub type ATOSEL3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ATOSEL3`"]
pub struct ATOSEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> ATOSEL3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 14)) | (((value as u32) & 0x07) << 14);
        self.w
    }
}
#[doc = "Reader of field `ATOSEL4`"]
pub type ATOSEL4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ATOSEL4`"]
pub struct ATOSEL4_W<'a> {
    w: &'a mut W,
}
impl<'a> ATOSEL4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 17)) | (((value as u32) & 0x07) << 17);
        self.w
    }
}
#[doc = "Reader of field `ATOSEL5`"]
pub type ATOSEL5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ATOSEL5`"]
pub struct ATOSEL5_W<'a> {
    w: &'a mut W,
}
impl<'a> ATOSEL5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Reader of field `ATOSEL6`"]
pub type ATOSEL6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ATOSEL6`"]
pub struct ATOSEL6_W<'a> {
    w: &'a mut W,
}
impl<'a> ATOSEL6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 23)) | (((value as u32) & 0x07) << 23);
        self.w
    }
}
#[doc = "Reader of field `ATOSEL7`"]
pub type ATOSEL7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ATOSEL7`"]
pub struct ATOSEL7_W<'a> {
    w: &'a mut W,
}
impl<'a> ATOSEL7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 26)) | (((value as u32) & 0x07) << 26);
        self.w
    }
}
#[doc = "Reader of field `ATOSEL8`"]
pub type ATOSEL8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ATOSEL8`"]
pub struct ATOSEL8_W<'a> {
    w: &'a mut W,
}
impl<'a> ATOSEL8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 29)) | (((value as u32) & 0x07) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:10 - ATOSEL1"]
    #[inline(always)]
    pub fn atosel1(&self) -> ATOSEL1_R {
        ATOSEL1_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 11:13 - ATOSEL2"]
    #[inline(always)]
    pub fn atosel2(&self) -> ATOSEL2_R {
        ATOSEL2_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bits 14:16 - ATOSEL3"]
    #[inline(always)]
    pub fn atosel3(&self) -> ATOSEL3_R {
        ATOSEL3_R::new(((self.bits >> 14) & 0x07) as u8)
    }
    #[doc = "Bits 17:19 - ATOSEL4"]
    #[inline(always)]
    pub fn atosel4(&self) -> ATOSEL4_R {
        ATOSEL4_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - ATOSEL5"]
    #[inline(always)]
    pub fn atosel5(&self) -> ATOSEL5_R {
        ATOSEL5_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 23:25 - ATOSEL6"]
    #[inline(always)]
    pub fn atosel6(&self) -> ATOSEL6_R {
        ATOSEL6_R::new(((self.bits >> 23) & 0x07) as u8)
    }
    #[doc = "Bits 26:28 - ATOSEL7"]
    #[inline(always)]
    pub fn atosel7(&self) -> ATOSEL7_R {
        ATOSEL7_R::new(((self.bits >> 26) & 0x07) as u8)
    }
    #[doc = "Bits 29:31 - ATOSEL8"]
    #[inline(always)]
    pub fn atosel8(&self) -> ATOSEL8_R {
        ATOSEL8_R::new(((self.bits >> 29) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 8:10 - ATOSEL1"]
    #[inline(always)]
    pub fn atosel1(&mut self) -> ATOSEL1_W {
        ATOSEL1_W { w: self }
    }
    #[doc = "Bits 11:13 - ATOSEL2"]
    #[inline(always)]
    pub fn atosel2(&mut self) -> ATOSEL2_W {
        ATOSEL2_W { w: self }
    }
    #[doc = "Bits 14:16 - ATOSEL3"]
    #[inline(always)]
    pub fn atosel3(&mut self) -> ATOSEL3_W {
        ATOSEL3_W { w: self }
    }
    #[doc = "Bits 17:19 - ATOSEL4"]
    #[inline(always)]
    pub fn atosel4(&mut self) -> ATOSEL4_W {
        ATOSEL4_W { w: self }
    }
    #[doc = "Bits 20:22 - ATOSEL5"]
    #[inline(always)]
    pub fn atosel5(&mut self) -> ATOSEL5_W {
        ATOSEL5_W { w: self }
    }
    #[doc = "Bits 23:25 - ATOSEL6"]
    #[inline(always)]
    pub fn atosel6(&mut self) -> ATOSEL6_W {
        ATOSEL6_W { w: self }
    }
    #[doc = "Bits 26:28 - ATOSEL7"]
    #[inline(always)]
    pub fn atosel7(&mut self) -> ATOSEL7_W {
        ATOSEL7_W { w: self }
    }
    #[doc = "Bits 29:31 - ATOSEL8"]
    #[inline(always)]
    pub fn atosel8(&mut self) -> ATOSEL8_W {
        ATOSEL8_W { w: self }
    }
}

#[doc = "Reader of register CONFCHR1"]
pub type R = crate::R<u32, super::CONFCHR1>;
#[doc = "Writer for register CONFCHR1"]
pub type W = crate::W<u32, super::CONFCHR1>;
#[doc = "Register CONFCHR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CONFCHR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CONFCH7`"]
pub type CONFCH7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CONFCH7`"]
pub struct CONFCH7_W<'a> {
    w: &'a mut W,
}
impl<'a> CONFCH7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `CONFCH6`"]
pub type CONFCH6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CONFCH6`"]
pub struct CONFCH6_W<'a> {
    w: &'a mut W,
}
impl<'a> CONFCH6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `CONFCH5`"]
pub type CONFCH5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CONFCH5`"]
pub struct CONFCH5_W<'a> {
    w: &'a mut W,
}
impl<'a> CONFCH5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `CONFCH4`"]
pub type CONFCH4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CONFCH4`"]
pub struct CONFCH4_W<'a> {
    w: &'a mut W,
}
impl<'a> CONFCH4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `CONFCH3`"]
pub type CONFCH3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CONFCH3`"]
pub struct CONFCH3_W<'a> {
    w: &'a mut W,
}
impl<'a> CONFCH3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `CONFCH2`"]
pub type CONFCH2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CONFCH2`"]
pub struct CONFCH2_W<'a> {
    w: &'a mut W,
}
impl<'a> CONFCH2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `CONFCH1`"]
pub type CONFCH1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CONFCH1`"]
pub struct CONFCH1_W<'a> {
    w: &'a mut W,
}
impl<'a> CONFCH1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `CONFCH0`"]
pub type CONFCH0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CONFCH0`"]
pub struct CONFCH0_W<'a> {
    w: &'a mut W,
}
impl<'a> CONFCH0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:29 - CONFCH7"]
    #[inline(always)]
    pub fn confch7(&self) -> CONFCH7_R {
        CONFCH7_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - CONFCH6"]
    #[inline(always)]
    pub fn confch6(&self) -> CONFCH6_R {
        CONFCH6_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - CONFCH5"]
    #[inline(always)]
    pub fn confch5(&self) -> CONFCH5_R {
        CONFCH5_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - CONFCH4"]
    #[inline(always)]
    pub fn confch4(&self) -> CONFCH4_R {
        CONFCH4_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - CONFCH3"]
    #[inline(always)]
    pub fn confch3(&self) -> CONFCH3_R {
        CONFCH3_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - CONFCH2"]
    #[inline(always)]
    pub fn confch2(&self) -> CONFCH2_R {
        CONFCH2_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - CONFCH1"]
    #[inline(always)]
    pub fn confch1(&self) -> CONFCH1_R {
        CONFCH1_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - CONFCH0"]
    #[inline(always)]
    pub fn confch0(&self) -> CONFCH0_R {
        CONFCH0_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 28:29 - CONFCH7"]
    #[inline(always)]
    pub fn confch7(&mut self) -> CONFCH7_W {
        CONFCH7_W { w: self }
    }
    #[doc = "Bits 24:25 - CONFCH6"]
    #[inline(always)]
    pub fn confch6(&mut self) -> CONFCH6_W {
        CONFCH6_W { w: self }
    }
    #[doc = "Bits 20:21 - CONFCH5"]
    #[inline(always)]
    pub fn confch5(&mut self) -> CONFCH5_W {
        CONFCH5_W { w: self }
    }
    #[doc = "Bits 16:17 - CONFCH4"]
    #[inline(always)]
    pub fn confch4(&mut self) -> CONFCH4_W {
        CONFCH4_W { w: self }
    }
    #[doc = "Bits 12:13 - CONFCH3"]
    #[inline(always)]
    pub fn confch3(&mut self) -> CONFCH3_W {
        CONFCH3_W { w: self }
    }
    #[doc = "Bits 8:9 - CONFCH2"]
    #[inline(always)]
    pub fn confch2(&mut self) -> CONFCH2_W {
        CONFCH2_W { w: self }
    }
    #[doc = "Bits 4:5 - CONFCH1"]
    #[inline(always)]
    pub fn confch1(&mut self) -> CONFCH1_W {
        CONFCH1_W { w: self }
    }
    #[doc = "Bits 0:1 - CONFCH0"]
    #[inline(always)]
    pub fn confch0(&mut self) -> CONFCH0_W {
        CONFCH0_W { w: self }
    }
}

#[doc = "Reader of register DDRPHYC_DX2DQTR"]
pub type R = crate::R<u32, super::DDRPHYC_DX2DQTR>;
#[doc = "Writer for register DDRPHYC_DX2DQTR"]
pub type W = crate::W<u32, super::DDRPHYC_DX2DQTR>;
#[doc = "Register DDRPHYC_DX2DQTR `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::DDRPHYC_DX2DQTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `DQDLY0`"]
pub type DQDLY0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DQDLY0`"]
pub struct DQDLY0_W<'a> {
    w: &'a mut W,
}
impl<'a> DQDLY0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `DQDLY1`"]
pub type DQDLY1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DQDLY1`"]
pub struct DQDLY1_W<'a> {
    w: &'a mut W,
}
impl<'a> DQDLY1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `DQDLY2`"]
pub type DQDLY2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DQDLY2`"]
pub struct DQDLY2_W<'a> {
    w: &'a mut W,
}
impl<'a> DQDLY2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `DQDLY3`"]
pub type DQDLY3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DQDLY3`"]
pub struct DQDLY3_W<'a> {
    w: &'a mut W,
}
impl<'a> DQDLY3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `DQDLY4`"]
pub type DQDLY4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DQDLY4`"]
pub struct DQDLY4_W<'a> {
    w: &'a mut W,
}
impl<'a> DQDLY4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `DQDLY5`"]
pub type DQDLY5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DQDLY5`"]
pub struct DQDLY5_W<'a> {
    w: &'a mut W,
}
impl<'a> DQDLY5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `DQDLY6`"]
pub type DQDLY6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DQDLY6`"]
pub struct DQDLY6_W<'a> {
    w: &'a mut W,
}
impl<'a> DQDLY6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `DQDLY7`"]
pub type DQDLY7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DQDLY7`"]
pub struct DQDLY7_W<'a> {
    w: &'a mut W,
}
impl<'a> DQDLY7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - DQDLY0"]
    #[inline(always)]
    pub fn dqdly0(&self) -> DQDLY0_R {
        DQDLY0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - DQDLY1"]
    #[inline(always)]
    pub fn dqdly1(&self) -> DQDLY1_R {
        DQDLY1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - DQDLY2"]
    #[inline(always)]
    pub fn dqdly2(&self) -> DQDLY2_R {
        DQDLY2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - DQDLY3"]
    #[inline(always)]
    pub fn dqdly3(&self) -> DQDLY3_R {
        DQDLY3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - DQDLY4"]
    #[inline(always)]
    pub fn dqdly4(&self) -> DQDLY4_R {
        DQDLY4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - DQDLY5"]
    #[inline(always)]
    pub fn dqdly5(&self) -> DQDLY5_R {
        DQDLY5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - DQDLY6"]
    #[inline(always)]
    pub fn dqdly6(&self) -> DQDLY6_R {
        DQDLY6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - DQDLY7"]
    #[inline(always)]
    pub fn dqdly7(&self) -> DQDLY7_R {
        DQDLY7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - DQDLY0"]
    #[inline(always)]
    pub fn dqdly0(&mut self) -> DQDLY0_W {
        DQDLY0_W { w: self }
    }
    #[doc = "Bits 4:7 - DQDLY1"]
    #[inline(always)]
    pub fn dqdly1(&mut self) -> DQDLY1_W {
        DQDLY1_W { w: self }
    }
    #[doc = "Bits 8:11 - DQDLY2"]
    #[inline(always)]
    pub fn dqdly2(&mut self) -> DQDLY2_W {
        DQDLY2_W { w: self }
    }
    #[doc = "Bits 12:15 - DQDLY3"]
    #[inline(always)]
    pub fn dqdly3(&mut self) -> DQDLY3_W {
        DQDLY3_W { w: self }
    }
    #[doc = "Bits 16:19 - DQDLY4"]
    #[inline(always)]
    pub fn dqdly4(&mut self) -> DQDLY4_W {
        DQDLY4_W { w: self }
    }
    #[doc = "Bits 20:23 - DQDLY5"]
    #[inline(always)]
    pub fn dqdly5(&mut self) -> DQDLY5_W {
        DQDLY5_W { w: self }
    }
    #[doc = "Bits 24:27 - DQDLY6"]
    #[inline(always)]
    pub fn dqdly6(&mut self) -> DQDLY6_W {
        DQDLY6_W { w: self }
    }
    #[doc = "Bits 28:31 - DQDLY7"]
    #[inline(always)]
    pub fn dqdly7(&mut self) -> DQDLY7_W {
        DQDLY7_W { w: self }
    }
}

#[doc = "Reader of register ECR"]
pub type R = crate::R<u32, super::ECR>;
#[doc = "Writer for register ECR"]
pub type W = crate::W<u32, super::ECR>;
#[doc = "Register ECR `reset()`'s with value 0"]
impl crate::ResetValue for super::ECR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IE`"]
pub type IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IE`"]
pub struct IE_W<'a> {
    w: &'a mut W,
}
impl<'a> IE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `IDIR`"]
pub type IDIR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IDIR`"]
pub struct IDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> IDIR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Reader of field `IBLK`"]
pub type IBLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IBLK`"]
pub struct IBLK_W<'a> {
    w: &'a mut W,
}
impl<'a> IBLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Reader of field `FIDX`"]
pub type FIDX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FIDX`"]
pub struct FIDX_W<'a> {
    w: &'a mut W,
}
impl<'a> FIDX_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `IPOS`"]
pub type IPOS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IPOS`"]
pub struct IPOS_W<'a> {
    w: &'a mut W,
}
impl<'a> IPOS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `PW`"]
pub type PW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PW`"]
pub struct PW_W<'a> {
    w: &'a mut W,
}
impl<'a> PW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `PWPRSC`"]
pub type PWPRSC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PWPRSC`"]
pub struct PWPRSC_W<'a> {
    w: &'a mut W,
}
impl<'a> PWPRSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Index Enable"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Index Direction"]
    #[inline(always)]
    pub fn idir(&self) -> IDIR_R {
        IDIR_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bits 3:4 - Index Blanking"]
    #[inline(always)]
    pub fn iblk(&self) -> IBLK_R {
        IBLK_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 5 - First Index"]
    #[inline(always)]
    pub fn fidx(&self) -> FIDX_R {
        FIDX_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Index Positioning"]
    #[inline(always)]
    pub fn ipos(&self) -> IPOS_R {
        IPOS_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 16:23 - Pulse width"]
    #[inline(always)]
    pub fn pw(&self) -> PW_R {
        PW_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:26 - Pulse Width prescaler"]
    #[inline(always)]
    pub fn pwprsc(&self) -> PWPRSC_R {
        PWPRSC_R::new(((self.bits >> 24) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Index Enable"]
    #[inline(always)]
    pub fn ie(&mut self) -> IE_W {
        IE_W { w: self }
    }
    #[doc = "Bits 1:2 - Index Direction"]
    #[inline(always)]
    pub fn idir(&mut self) -> IDIR_W {
        IDIR_W { w: self }
    }
    #[doc = "Bits 3:4 - Index Blanking"]
    #[inline(always)]
    pub fn iblk(&mut self) -> IBLK_W {
        IBLK_W { w: self }
    }
    #[doc = "Bit 5 - First Index"]
    #[inline(always)]
    pub fn fidx(&mut self) -> FIDX_W {
        FIDX_W { w: self }
    }
    #[doc = "Bits 6:7 - Index Positioning"]
    #[inline(always)]
    pub fn ipos(&mut self) -> IPOS_W {
        IPOS_W { w: self }
    }
    #[doc = "Bits 16:23 - Pulse width"]
    #[inline(always)]
    pub fn pw(&mut self) -> PW_W {
        PW_W { w: self }
    }
    #[doc = "Bits 24:26 - Pulse Width prescaler"]
    #[inline(always)]
    pub fn pwprsc(&mut self) -> PWPRSC_W {
        PWPRSC_W { w: self }
    }
}

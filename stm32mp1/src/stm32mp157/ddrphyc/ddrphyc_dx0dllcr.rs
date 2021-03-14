#[doc = "Reader of register DDRPHYC_DX0DLLCR"]
pub type R = crate::R<u32, super::DDRPHYC_DX0DLLCR>;
#[doc = "Writer for register DDRPHYC_DX0DLLCR"]
pub type W = crate::W<u32, super::DDRPHYC_DX0DLLCR>;
#[doc = "Register DDRPHYC_DX0DLLCR `reset()`'s with value 0x4000_0000"]
impl crate::ResetValue for super::DDRPHYC_DX0DLLCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x4000_0000
    }
}
#[doc = "Reader of field `SFBDLY`"]
pub type SFBDLY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SFBDLY`"]
pub struct SFBDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> SFBDLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `SFWDLY`"]
pub type SFWDLY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SFWDLY`"]
pub struct SFWDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> SFWDLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "Reader of field `MFBDLY`"]
pub type MFBDLY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MFBDLY`"]
pub struct MFBDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> MFBDLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | (((value as u32) & 0x07) << 6);
        self.w
    }
}
#[doc = "Reader of field `MFWDLY`"]
pub type MFWDLY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MFWDLY`"]
pub struct MFWDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> MFWDLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | (((value as u32) & 0x07) << 9);
        self.w
    }
}
#[doc = "Reader of field `SSTART`"]
pub type SSTART_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SSTART`"]
pub struct SSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> SSTART_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `SDPHASE`"]
pub type SDPHASE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SDPHASE`"]
pub struct SDPHASE_W<'a> {
    w: &'a mut W,
}
impl<'a> SDPHASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 14)) | (((value as u32) & 0x0f) << 14);
        self.w
    }
}
#[doc = "Reader of field `ATESTEN`"]
pub type ATESTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ATESTEN`"]
pub struct ATESTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ATESTEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `SDLBMODE`"]
pub type SDLBMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDLBMODE`"]
pub struct SDLBMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SDLBMODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `DLLSRST`"]
pub type DLLSRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DLLSRST`"]
pub struct DLLSRST_W<'a> {
    w: &'a mut W,
}
impl<'a> DLLSRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `DLLDIS`"]
pub type DLLDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DLLDIS`"]
pub struct DLLDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DLLDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - SFBDLY"]
    #[inline(always)]
    pub fn sfbdly(&self) -> SFBDLY_R {
        SFBDLY_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - SFWDLY"]
    #[inline(always)]
    pub fn sfwdly(&self) -> SFWDLY_R {
        SFWDLY_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 6:8 - MFBDLY"]
    #[inline(always)]
    pub fn mfbdly(&self) -> MFBDLY_R {
        MFBDLY_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 9:11 - MFWDLY"]
    #[inline(always)]
    pub fn mfwdly(&self) -> MFWDLY_R {
        MFWDLY_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bits 12:13 - SSTART"]
    #[inline(always)]
    pub fn sstart(&self) -> SSTART_R {
        SSTART_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:17 - SDPHASE"]
    #[inline(always)]
    pub fn sdphase(&self) -> SDPHASE_R {
        SDPHASE_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bit 18 - ATESTEN"]
    #[inline(always)]
    pub fn atesten(&self) -> ATESTEN_R {
        ATESTEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - SDLBMODE"]
    #[inline(always)]
    pub fn sdlbmode(&self) -> SDLBMODE_R {
        SDLBMODE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 30 - DLLSRST"]
    #[inline(always)]
    pub fn dllsrst(&self) -> DLLSRST_R {
        DLLSRST_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - DLLDIS"]
    #[inline(always)]
    pub fn dlldis(&self) -> DLLDIS_R {
        DLLDIS_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - SFBDLY"]
    #[inline(always)]
    pub fn sfbdly(&mut self) -> SFBDLY_W {
        SFBDLY_W { w: self }
    }
    #[doc = "Bits 3:5 - SFWDLY"]
    #[inline(always)]
    pub fn sfwdly(&mut self) -> SFWDLY_W {
        SFWDLY_W { w: self }
    }
    #[doc = "Bits 6:8 - MFBDLY"]
    #[inline(always)]
    pub fn mfbdly(&mut self) -> MFBDLY_W {
        MFBDLY_W { w: self }
    }
    #[doc = "Bits 9:11 - MFWDLY"]
    #[inline(always)]
    pub fn mfwdly(&mut self) -> MFWDLY_W {
        MFWDLY_W { w: self }
    }
    #[doc = "Bits 12:13 - SSTART"]
    #[inline(always)]
    pub fn sstart(&mut self) -> SSTART_W {
        SSTART_W { w: self }
    }
    #[doc = "Bits 14:17 - SDPHASE"]
    #[inline(always)]
    pub fn sdphase(&mut self) -> SDPHASE_W {
        SDPHASE_W { w: self }
    }
    #[doc = "Bit 18 - ATESTEN"]
    #[inline(always)]
    pub fn atesten(&mut self) -> ATESTEN_W {
        ATESTEN_W { w: self }
    }
    #[doc = "Bit 19 - SDLBMODE"]
    #[inline(always)]
    pub fn sdlbmode(&mut self) -> SDLBMODE_W {
        SDLBMODE_W { w: self }
    }
    #[doc = "Bit 30 - DLLSRST"]
    #[inline(always)]
    pub fn dllsrst(&mut self) -> DLLSRST_W {
        DLLSRST_W { w: self }
    }
    #[doc = "Bit 31 - DLLDIS"]
    #[inline(always)]
    pub fn dlldis(&mut self) -> DLLDIS_W {
        DLLDIS_W { w: self }
    }
}

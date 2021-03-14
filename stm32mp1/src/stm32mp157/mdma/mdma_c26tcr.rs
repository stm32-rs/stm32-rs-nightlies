#[doc = "Reader of register MDMA_C26TCR"]
pub type R = crate::R<u32, super::MDMA_C26TCR>;
#[doc = "Writer for register MDMA_C26TCR"]
pub type W = crate::W<u32, super::MDMA_C26TCR>;
#[doc = "Register MDMA_C26TCR `reset()`'s with value 0"]
impl crate::ResetValue for super::MDMA_C26TCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SINC`"]
pub type SINC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SINC`"]
pub struct SINC_W<'a> {
    w: &'a mut W,
}
impl<'a> SINC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `DINC`"]
pub type DINC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DINC`"]
pub struct DINC_W<'a> {
    w: &'a mut W,
}
impl<'a> DINC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `SSIZE`"]
pub type SSIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SSIZE`"]
pub struct SSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `DSIZE`"]
pub type DSIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DSIZE`"]
pub struct DSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> DSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `SINCOS`"]
pub type SINCOS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SINCOS`"]
pub struct SINCOS_W<'a> {
    w: &'a mut W,
}
impl<'a> SINCOS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `DINCOS`"]
pub type DINCOS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DINCOS`"]
pub struct DINCOS_W<'a> {
    w: &'a mut W,
}
impl<'a> DINCOS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `SBURST`"]
pub type SBURST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SBURST`"]
pub struct SBURST_W<'a> {
    w: &'a mut W,
}
impl<'a> SBURST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `DBURST`"]
pub type DBURST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DBURST`"]
pub struct DBURST_W<'a> {
    w: &'a mut W,
}
impl<'a> DBURST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 15)) | (((value as u32) & 0x07) << 15);
        self.w
    }
}
#[doc = "Reader of field `TLEN`"]
pub type TLEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TLEN`"]
pub struct TLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 18)) | (((value as u32) & 0x7f) << 18);
        self.w
    }
}
#[doc = "Reader of field `PKE`"]
pub type PKE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PKE`"]
pub struct PKE_W<'a> {
    w: &'a mut W,
}
impl<'a> PKE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `PAM`"]
pub type PAM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PAM`"]
pub struct PAM_W<'a> {
    w: &'a mut W,
}
impl<'a> PAM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Reader of field `TRGM`"]
pub type TRGM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRGM`"]
pub struct TRGM_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `SWRM`"]
pub type SWRM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWRM`"]
pub struct SWRM_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRM_W<'a> {
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
#[doc = "Reader of field `BWM`"]
pub type BWM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BWM`"]
pub struct BWM_W<'a> {
    w: &'a mut W,
}
impl<'a> BWM_W<'a> {
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
    #[doc = "Bits 0:1 - SINC"]
    #[inline(always)]
    pub fn sinc(&self) -> SINC_R {
        SINC_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - DINC"]
    #[inline(always)]
    pub fn dinc(&self) -> DINC_R {
        DINC_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - SSIZE"]
    #[inline(always)]
    pub fn ssize(&self) -> SSIZE_R {
        SSIZE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - DSIZE"]
    #[inline(always)]
    pub fn dsize(&self) -> DSIZE_R {
        DSIZE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - SINCOS"]
    #[inline(always)]
    pub fn sincos(&self) -> SINCOS_R {
        SINCOS_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - DINCOS"]
    #[inline(always)]
    pub fn dincos(&self) -> DINCOS_R {
        DINCOS_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:14 - SBURST"]
    #[inline(always)]
    pub fn sburst(&self) -> SBURST_R {
        SBURST_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 15:17 - DBURST"]
    #[inline(always)]
    pub fn dburst(&self) -> DBURST_R {
        DBURST_R::new(((self.bits >> 15) & 0x07) as u8)
    }
    #[doc = "Bits 18:24 - TLEN"]
    #[inline(always)]
    pub fn tlen(&self) -> TLEN_R {
        TLEN_R::new(((self.bits >> 18) & 0x7f) as u8)
    }
    #[doc = "Bit 25 - PKE"]
    #[inline(always)]
    pub fn pke(&self) -> PKE_R {
        PKE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 26:27 - PAM"]
    #[inline(always)]
    pub fn pam(&self) -> PAM_R {
        PAM_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - TRGM"]
    #[inline(always)]
    pub fn trgm(&self) -> TRGM_R {
        TRGM_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bit 30 - SWRM"]
    #[inline(always)]
    pub fn swrm(&self) -> SWRM_R {
        SWRM_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - BWM"]
    #[inline(always)]
    pub fn bwm(&self) -> BWM_R {
        BWM_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - SINC"]
    #[inline(always)]
    pub fn sinc(&mut self) -> SINC_W {
        SINC_W { w: self }
    }
    #[doc = "Bits 2:3 - DINC"]
    #[inline(always)]
    pub fn dinc(&mut self) -> DINC_W {
        DINC_W { w: self }
    }
    #[doc = "Bits 4:5 - SSIZE"]
    #[inline(always)]
    pub fn ssize(&mut self) -> SSIZE_W {
        SSIZE_W { w: self }
    }
    #[doc = "Bits 6:7 - DSIZE"]
    #[inline(always)]
    pub fn dsize(&mut self) -> DSIZE_W {
        DSIZE_W { w: self }
    }
    #[doc = "Bits 8:9 - SINCOS"]
    #[inline(always)]
    pub fn sincos(&mut self) -> SINCOS_W {
        SINCOS_W { w: self }
    }
    #[doc = "Bits 10:11 - DINCOS"]
    #[inline(always)]
    pub fn dincos(&mut self) -> DINCOS_W {
        DINCOS_W { w: self }
    }
    #[doc = "Bits 12:14 - SBURST"]
    #[inline(always)]
    pub fn sburst(&mut self) -> SBURST_W {
        SBURST_W { w: self }
    }
    #[doc = "Bits 15:17 - DBURST"]
    #[inline(always)]
    pub fn dburst(&mut self) -> DBURST_W {
        DBURST_W { w: self }
    }
    #[doc = "Bits 18:24 - TLEN"]
    #[inline(always)]
    pub fn tlen(&mut self) -> TLEN_W {
        TLEN_W { w: self }
    }
    #[doc = "Bit 25 - PKE"]
    #[inline(always)]
    pub fn pke(&mut self) -> PKE_W {
        PKE_W { w: self }
    }
    #[doc = "Bits 26:27 - PAM"]
    #[inline(always)]
    pub fn pam(&mut self) -> PAM_W {
        PAM_W { w: self }
    }
    #[doc = "Bits 28:29 - TRGM"]
    #[inline(always)]
    pub fn trgm(&mut self) -> TRGM_W {
        TRGM_W { w: self }
    }
    #[doc = "Bit 30 - SWRM"]
    #[inline(always)]
    pub fn swrm(&mut self) -> SWRM_W {
        SWRM_W { w: self }
    }
    #[doc = "Bit 31 - BWM"]
    #[inline(always)]
    pub fn bwm(&mut self) -> BWM_W {
        BWM_W { w: self }
    }
}

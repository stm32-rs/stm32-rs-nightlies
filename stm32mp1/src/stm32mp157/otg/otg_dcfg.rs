#[doc = "Reader of register OTG_DCFG"]
pub type R = crate::R<u32, super::OTG_DCFG>;
#[doc = "Writer for register OTG_DCFG"]
pub type W = crate::W<u32, super::OTG_DCFG>;
#[doc = "Register OTG_DCFG `reset()`'s with value 0x0220_0000"]
impl crate::ResetValue for super::OTG_DCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0220_0000
    }
}
#[doc = "Reader of field `DSPD`"]
pub type DSPD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DSPD`"]
pub struct DSPD_W<'a> {
    w: &'a mut W,
}
impl<'a> DSPD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `NZLSOHSK`"]
pub type NZLSOHSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NZLSOHSK`"]
pub struct NZLSOHSK_W<'a> {
    w: &'a mut W,
}
impl<'a> NZLSOHSK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `DAD`"]
pub type DAD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DAD`"]
pub struct DAD_W<'a> {
    w: &'a mut W,
}
impl<'a> DAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 4)) | (((value as u32) & 0x7f) << 4);
        self.w
    }
}
#[doc = "Reader of field `PFIVL`"]
pub type PFIVL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PFIVL`"]
pub struct PFIVL_W<'a> {
    w: &'a mut W,
}
impl<'a> PFIVL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
    }
}
#[doc = "Reader of field `XCVRDLY`"]
pub type XCVRDLY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XCVRDLY`"]
pub struct XCVRDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> XCVRDLY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `ERRATIM`"]
pub type ERRATIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERRATIM`"]
pub struct ERRATIM_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRATIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `PERSCHIVL`"]
pub type PERSCHIVL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PERSCHIVL`"]
pub struct PERSCHIVL_W<'a> {
    w: &'a mut W,
}
impl<'a> PERSCHIVL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - DSPD"]
    #[inline(always)]
    pub fn dspd(&self) -> DSPD_R {
        DSPD_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - NZLSOHSK"]
    #[inline(always)]
    pub fn nzlsohsk(&self) -> NZLSOHSK_R {
        NZLSOHSK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 4:10 - DAD"]
    #[inline(always)]
    pub fn dad(&self) -> DAD_R {
        DAD_R::new(((self.bits >> 4) & 0x7f) as u8)
    }
    #[doc = "Bits 11:12 - PFIVL"]
    #[inline(always)]
    pub fn pfivl(&self) -> PFIVL_R {
        PFIVL_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bit 14 - XCVRDLY"]
    #[inline(always)]
    pub fn xcvrdly(&self) -> XCVRDLY_R {
        XCVRDLY_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - ERRATIM"]
    #[inline(always)]
    pub fn erratim(&self) -> ERRATIM_R {
        ERRATIM_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - PERSCHIVL"]
    #[inline(always)]
    pub fn perschivl(&self) -> PERSCHIVL_R {
        PERSCHIVL_R::new(((self.bits >> 24) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - DSPD"]
    #[inline(always)]
    pub fn dspd(&mut self) -> DSPD_W {
        DSPD_W { w: self }
    }
    #[doc = "Bit 2 - NZLSOHSK"]
    #[inline(always)]
    pub fn nzlsohsk(&mut self) -> NZLSOHSK_W {
        NZLSOHSK_W { w: self }
    }
    #[doc = "Bits 4:10 - DAD"]
    #[inline(always)]
    pub fn dad(&mut self) -> DAD_W {
        DAD_W { w: self }
    }
    #[doc = "Bits 11:12 - PFIVL"]
    #[inline(always)]
    pub fn pfivl(&mut self) -> PFIVL_W {
        PFIVL_W { w: self }
    }
    #[doc = "Bit 14 - XCVRDLY"]
    #[inline(always)]
    pub fn xcvrdly(&mut self) -> XCVRDLY_W {
        XCVRDLY_W { w: self }
    }
    #[doc = "Bit 15 - ERRATIM"]
    #[inline(always)]
    pub fn erratim(&mut self) -> ERRATIM_W {
        ERRATIM_W { w: self }
    }
    #[doc = "Bits 24:25 - PERSCHIVL"]
    #[inline(always)]
    pub fn perschivl(&mut self) -> PERSCHIVL_W {
        PERSCHIVL_W { w: self }
    }
}

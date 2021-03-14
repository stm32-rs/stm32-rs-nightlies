#[doc = "Reader of register SFR"]
pub type R = crate::R<u32, super::SFR>;
#[doc = "Writer for register SFR"]
pub type W = crate::W<u32, super::SFR>;
#[doc = "Register SFR `reset()`'s with value 0xffff_efff"]
impl crate::ResetValue for super::SFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_efff
    }
}
#[doc = "Reader of field `SFSA`"]
pub type SFSA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SFSA`"]
pub struct SFSA_W<'a> {
    w: &'a mut W,
}
impl<'a> SFSA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `FSD`"]
pub type FSD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSD`"]
pub struct FSD_W<'a> {
    w: &'a mut W,
}
impl<'a> FSD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `DDS`"]
pub type DDS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DDS`"]
pub struct DDS_W<'a> {
    w: &'a mut W,
}
impl<'a> DDS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `HDPSA`"]
pub type HDPSA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HDPSA`"]
pub struct HDPSA_W<'a> {
    w: &'a mut W,
}
impl<'a> HDPSA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = "Reader of field `HDPAD`"]
pub type HDPAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HDPAD`"]
pub struct HDPAD_W<'a> {
    w: &'a mut W,
}
impl<'a> HDPAD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `SUBGHSPISD`"]
pub type SUBGHSPISD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SUBGHSPISD`"]
pub struct SUBGHSPISD_W<'a> {
    w: &'a mut W,
}
impl<'a> SUBGHSPISD_W<'a> {
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
    #[doc = "Bits 0:6 - Secure Flash start address"]
    #[inline(always)]
    pub fn sfsa(&self) -> SFSA_R {
        SFSA_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Flash security disabled"]
    #[inline(always)]
    pub fn fsd(&self) -> FSD_R {
        FSD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 12 - DDS"]
    #[inline(always)]
    pub fn dds(&self) -> DDS_R {
        DDS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 16:22 - User Flash hide protection area start address"]
    #[inline(always)]
    pub fn hdpsa(&self) -> HDPSA_R {
        HDPSA_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - User Flash hide protection area disabled"]
    #[inline(always)]
    pub fn hdpad(&self) -> HDPAD_R {
        HDPAD_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 31 - sub-GHz radio SPI security disable"]
    #[inline(always)]
    pub fn subghspisd(&self) -> SUBGHSPISD_R {
        SUBGHSPISD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Secure Flash start address"]
    #[inline(always)]
    pub fn sfsa(&mut self) -> SFSA_W {
        SFSA_W { w: self }
    }
    #[doc = "Bit 7 - Flash security disabled"]
    #[inline(always)]
    pub fn fsd(&mut self) -> FSD_W {
        FSD_W { w: self }
    }
    #[doc = "Bit 12 - DDS"]
    #[inline(always)]
    pub fn dds(&mut self) -> DDS_W {
        DDS_W { w: self }
    }
    #[doc = "Bits 16:22 - User Flash hide protection area start address"]
    #[inline(always)]
    pub fn hdpsa(&mut self) -> HDPSA_W {
        HDPSA_W { w: self }
    }
    #[doc = "Bit 23 - User Flash hide protection area disabled"]
    #[inline(always)]
    pub fn hdpad(&mut self) -> HDPAD_W {
        HDPAD_W { w: self }
    }
    #[doc = "Bit 31 - sub-GHz radio SPI security disable"]
    #[inline(always)]
    pub fn subghspisd(&mut self) -> SUBGHSPISD_W {
        SUBGHSPISD_W { w: self }
    }
}

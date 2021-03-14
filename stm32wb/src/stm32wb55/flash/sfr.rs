#[doc = "Reader of register SFR"]
pub type R = crate::R<u32, super::SFR>;
#[doc = "Writer for register SFR"]
pub type W = crate::W<u32, super::SFR>;
#[doc = "Register SFR `reset()`'s with value 0xffff_ee00"]
impl crate::ResetValue for super::SFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ee00
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
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Secure flash start address"]
    #[inline(always)]
    pub fn sfsa(&self) -> SFSA_R {
        SFSA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 12 - Disable Cortex M0 debug access"]
    #[inline(always)]
    pub fn dds(&self) -> DDS_R {
        DDS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Flash security disable"]
    #[inline(always)]
    pub fn fsd(&self) -> FSD_R {
        FSD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Secure flash start address"]
    #[inline(always)]
    pub fn sfsa(&mut self) -> SFSA_W {
        SFSA_W { w: self }
    }
    #[doc = "Bit 12 - Disable Cortex M0 debug access"]
    #[inline(always)]
    pub fn dds(&mut self) -> DDS_W {
        DDS_W { w: self }
    }
    #[doc = "Bit 8 - Flash security disable"]
    #[inline(always)]
    pub fn fsd(&mut self) -> FSD_W {
        FSD_W { w: self }
    }
}

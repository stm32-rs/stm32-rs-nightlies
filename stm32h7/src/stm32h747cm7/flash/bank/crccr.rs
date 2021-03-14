#[doc = "Reader of register CRCCR"]
pub type R = crate::R<u32, super::CRCCR>;
#[doc = "Writer for register CRCCR"]
pub type W = crate::W<u32, super::CRCCR>;
#[doc = "Register CRCCR `reset()`'s with value 0x001c_0000"]
impl crate::ResetValue for super::CRCCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x001c_0000
    }
}
#[doc = "Reader of field `ALL_BANK`"]
pub type ALL_BANK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALL_BANK`"]
pub struct ALL_BANK_W<'a> {
    w: &'a mut W,
}
impl<'a> ALL_BANK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `CRC_BURST`"]
pub type CRC_BURST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CRC_BURST`"]
pub struct CRC_BURST_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_BURST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `CLEAN_CRC`"]
pub type CLEAN_CRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLEAN_CRC`"]
pub struct CLEAN_CRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CLEAN_CRC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `START_CRC`"]
pub type START_CRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `START_CRC`"]
pub struct START_CRC_W<'a> {
    w: &'a mut W,
}
impl<'a> START_CRC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `CLEAN_SECT`"]
pub type CLEAN_SECT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLEAN_SECT`"]
pub struct CLEAN_SECT_W<'a> {
    w: &'a mut W,
}
impl<'a> CLEAN_SECT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `ADD_SECT`"]
pub type ADD_SECT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADD_SECT`"]
pub struct ADD_SECT_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD_SECT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `CRC_BY_SECT`"]
pub type CRC_BY_SECT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRC_BY_SECT`"]
pub struct CRC_BY_SECT_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_BY_SECT_W<'a> {
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
#[doc = "Reader of field `CRC_SECT`"]
pub type CRC_SECT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CRC_SECT`"]
pub struct CRC_SECT_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_SECT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bit 22 - Bank 1 CRC select bit"]
    #[inline(always)]
    pub fn all_bank(&self) -> ALL_BANK_R {
        ALL_BANK_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 20:21 - Bank 1 CRC burst size"]
    #[inline(always)]
    pub fn crc_burst(&self) -> CRC_BURST_R {
        CRC_BURST_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bit 17 - Bank 1 CRC clear bit"]
    #[inline(always)]
    pub fn clean_crc(&self) -> CLEAN_CRC_R {
        CLEAN_CRC_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Bank 1 CRC start bit"]
    #[inline(always)]
    pub fn start_crc(&self) -> START_CRC_R {
        START_CRC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Bank 1 CRC sector list clear bit"]
    #[inline(always)]
    pub fn clean_sect(&self) -> CLEAN_SECT_R {
        CLEAN_SECT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Bank 1 CRC sector select bit"]
    #[inline(always)]
    pub fn add_sect(&self) -> ADD_SECT_R {
        ADD_SECT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Bank 1 CRC sector mode select bit"]
    #[inline(always)]
    pub fn crc_by_sect(&self) -> CRC_BY_SECT_R {
        CRC_BY_SECT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:2 - Bank 1 CRC sector number"]
    #[inline(always)]
    pub fn crc_sect(&self) -> CRC_SECT_R {
        CRC_SECT_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 22 - Bank 1 CRC select bit"]
    #[inline(always)]
    pub fn all_bank(&mut self) -> ALL_BANK_W {
        ALL_BANK_W { w: self }
    }
    #[doc = "Bits 20:21 - Bank 1 CRC burst size"]
    #[inline(always)]
    pub fn crc_burst(&mut self) -> CRC_BURST_W {
        CRC_BURST_W { w: self }
    }
    #[doc = "Bit 17 - Bank 1 CRC clear bit"]
    #[inline(always)]
    pub fn clean_crc(&mut self) -> CLEAN_CRC_W {
        CLEAN_CRC_W { w: self }
    }
    #[doc = "Bit 16 - Bank 1 CRC start bit"]
    #[inline(always)]
    pub fn start_crc(&mut self) -> START_CRC_W {
        START_CRC_W { w: self }
    }
    #[doc = "Bit 10 - Bank 1 CRC sector list clear bit"]
    #[inline(always)]
    pub fn clean_sect(&mut self) -> CLEAN_SECT_W {
        CLEAN_SECT_W { w: self }
    }
    #[doc = "Bit 9 - Bank 1 CRC sector select bit"]
    #[inline(always)]
    pub fn add_sect(&mut self) -> ADD_SECT_W {
        ADD_SECT_W { w: self }
    }
    #[doc = "Bit 8 - Bank 1 CRC sector mode select bit"]
    #[inline(always)]
    pub fn crc_by_sect(&mut self) -> CRC_BY_SECT_W {
        CRC_BY_SECT_W { w: self }
    }
    #[doc = "Bits 0:2 - Bank 1 CRC sector number"]
    #[inline(always)]
    pub fn crc_sect(&mut self) -> CRC_SECT_W {
        CRC_SECT_W { w: self }
    }
}

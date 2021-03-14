#[doc = "Reader of register ETH_DMASBMR"]
pub type R = crate::R<u32, super::ETH_DMASBMR>;
#[doc = "Writer for register ETH_DMASBMR"]
pub type W = crate::W<u32, super::ETH_DMASBMR>;
#[doc = "Register ETH_DMASBMR `reset()`'s with value 0x8000"]
impl crate::ResetValue for super::ETH_DMASBMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000
    }
}
#[doc = "Reader of field `FB`"]
pub type FB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FB`"]
pub struct FB_W<'a> {
    w: &'a mut W,
}
impl<'a> FB_W<'a> {
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
#[doc = "Reader of field `BLEN4`"]
pub type BLEN4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BLEN4`"]
pub struct BLEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEN4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `BLEN8`"]
pub type BLEN8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BLEN8`"]
pub struct BLEN8_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEN8_W<'a> {
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
#[doc = "Reader of field `BLEN16`"]
pub type BLEN16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BLEN16`"]
pub struct BLEN16_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEN16_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `BLEN32`"]
pub type BLEN32_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BLEN32`"]
pub struct BLEN32_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEN32_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `BLEN64`"]
pub type BLEN64_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BLEN64`"]
pub struct BLEN64_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEN64_W<'a> {
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
#[doc = "Reader of field `BLEN128`"]
pub type BLEN128_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BLEN128`"]
pub struct BLEN128_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEN128_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `BLEN256`"]
pub type BLEN256_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BLEN256`"]
pub struct BLEN256_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEN256_W<'a> {
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
#[doc = "Reader of field `AAL`"]
pub type AAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AAL`"]
pub struct AAL_W<'a> {
    w: &'a mut W,
}
impl<'a> AAL_W<'a> {
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
#[doc = "Reader of field `ONEKBBE`"]
pub type ONEKBBE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ONEKBBE`"]
pub struct ONEKBBE_W<'a> {
    w: &'a mut W,
}
impl<'a> ONEKBBE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `RD_OSR_LMT`"]
pub type RD_OSR_LMT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RD_OSR_LMT`"]
pub struct RD_OSR_LMT_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_OSR_LMT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `WR_OSR_LMT`"]
pub type WR_OSR_LMT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WR_OSR_LMT`"]
pub struct WR_OSR_LMT_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_OSR_LMT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `LPI_XIT_PKT`"]
pub type LPI_XIT_PKT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPI_XIT_PKT`"]
pub struct LPI_XIT_PKT_W<'a> {
    w: &'a mut W,
}
impl<'a> LPI_XIT_PKT_W<'a> {
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
#[doc = "Reader of field `EN_LPI`"]
pub type EN_LPI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN_LPI`"]
pub struct EN_LPI_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_LPI_W<'a> {
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
    #[doc = "Bit 0 - Fixed Burst Length"]
    #[inline(always)]
    pub fn fb(&self) -> FB_R {
        FB_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - BLEN4"]
    #[inline(always)]
    pub fn blen4(&self) -> BLEN4_R {
        BLEN4_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - BLEN8"]
    #[inline(always)]
    pub fn blen8(&self) -> BLEN8_R {
        BLEN8_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - BLEN16"]
    #[inline(always)]
    pub fn blen16(&self) -> BLEN16_R {
        BLEN16_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - BLEN32"]
    #[inline(always)]
    pub fn blen32(&self) -> BLEN32_R {
        BLEN32_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - BLEN64"]
    #[inline(always)]
    pub fn blen64(&self) -> BLEN64_R {
        BLEN64_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - BLEN128"]
    #[inline(always)]
    pub fn blen128(&self) -> BLEN128_R {
        BLEN128_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - BLEN256"]
    #[inline(always)]
    pub fn blen256(&self) -> BLEN256_R {
        BLEN256_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Address-Aligned Beats"]
    #[inline(always)]
    pub fn aal(&self) -> AAL_R {
        AAL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - ONEKBBE"]
    #[inline(always)]
    pub fn onekbbe(&self) -> ONEKBBE_R {
        ONEKBBE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - RD_OSR_LMT"]
    #[inline(always)]
    pub fn rd_osr_lmt(&self) -> RD_OSR_LMT_R {
        RD_OSR_LMT_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - WR_OSR_LMT"]
    #[inline(always)]
    pub fn wr_osr_lmt(&self) -> WR_OSR_LMT_R {
        WR_OSR_LMT_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 30 - LPI_XIT_PKT"]
    #[inline(always)]
    pub fn lpi_xit_pkt(&self) -> LPI_XIT_PKT_R {
        LPI_XIT_PKT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - EN_LPI"]
    #[inline(always)]
    pub fn en_lpi(&self) -> EN_LPI_R {
        EN_LPI_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fixed Burst Length"]
    #[inline(always)]
    pub fn fb(&mut self) -> FB_W {
        FB_W { w: self }
    }
    #[doc = "Bit 1 - BLEN4"]
    #[inline(always)]
    pub fn blen4(&mut self) -> BLEN4_W {
        BLEN4_W { w: self }
    }
    #[doc = "Bit 2 - BLEN8"]
    #[inline(always)]
    pub fn blen8(&mut self) -> BLEN8_W {
        BLEN8_W { w: self }
    }
    #[doc = "Bit 3 - BLEN16"]
    #[inline(always)]
    pub fn blen16(&mut self) -> BLEN16_W {
        BLEN16_W { w: self }
    }
    #[doc = "Bit 4 - BLEN32"]
    #[inline(always)]
    pub fn blen32(&mut self) -> BLEN32_W {
        BLEN32_W { w: self }
    }
    #[doc = "Bit 5 - BLEN64"]
    #[inline(always)]
    pub fn blen64(&mut self) -> BLEN64_W {
        BLEN64_W { w: self }
    }
    #[doc = "Bit 6 - BLEN128"]
    #[inline(always)]
    pub fn blen128(&mut self) -> BLEN128_W {
        BLEN128_W { w: self }
    }
    #[doc = "Bit 7 - BLEN256"]
    #[inline(always)]
    pub fn blen256(&mut self) -> BLEN256_W {
        BLEN256_W { w: self }
    }
    #[doc = "Bit 12 - Address-Aligned Beats"]
    #[inline(always)]
    pub fn aal(&mut self) -> AAL_W {
        AAL_W { w: self }
    }
    #[doc = "Bit 13 - ONEKBBE"]
    #[inline(always)]
    pub fn onekbbe(&mut self) -> ONEKBBE_W {
        ONEKBBE_W { w: self }
    }
    #[doc = "Bits 16:17 - RD_OSR_LMT"]
    #[inline(always)]
    pub fn rd_osr_lmt(&mut self) -> RD_OSR_LMT_W {
        RD_OSR_LMT_W { w: self }
    }
    #[doc = "Bits 24:25 - WR_OSR_LMT"]
    #[inline(always)]
    pub fn wr_osr_lmt(&mut self) -> WR_OSR_LMT_W {
        WR_OSR_LMT_W { w: self }
    }
    #[doc = "Bit 30 - LPI_XIT_PKT"]
    #[inline(always)]
    pub fn lpi_xit_pkt(&mut self) -> LPI_XIT_PKT_W {
        LPI_XIT_PKT_W { w: self }
    }
    #[doc = "Bit 31 - EN_LPI"]
    #[inline(always)]
    pub fn en_lpi(&mut self) -> EN_LPI_W {
        EN_LPI_W { w: self }
    }
}

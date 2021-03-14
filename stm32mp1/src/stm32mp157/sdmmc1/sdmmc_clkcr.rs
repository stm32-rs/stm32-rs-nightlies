#[doc = "Reader of register SDMMC_CLKCR"]
pub type R = crate::R<u32, super::SDMMC_CLKCR>;
#[doc = "Writer for register SDMMC_CLKCR"]
pub type W = crate::W<u32, super::SDMMC_CLKCR>;
#[doc = "Register SDMMC_CLKCR `reset()`'s with value 0"]
impl crate::ResetValue for super::SDMMC_CLKCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLKDIV`"]
pub type CLKDIV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CLKDIV`"]
pub struct CLKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Reader of field `PWRSAV`"]
pub type PWRSAV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWRSAV`"]
pub struct PWRSAV_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRSAV_W<'a> {
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
#[doc = "Reader of field `WIDBUS`"]
pub type WIDBUS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WIDBUS`"]
pub struct WIDBUS_W<'a> {
    w: &'a mut W,
}
impl<'a> WIDBUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `NEGEDGE`"]
pub type NEGEDGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NEGEDGE`"]
pub struct NEGEDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> NEGEDGE_W<'a> {
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
#[doc = "Reader of field `HWFC_EN`"]
pub type HWFC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HWFC_EN`"]
pub struct HWFC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HWFC_EN_W<'a> {
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
#[doc = "Reader of field `DDR`"]
pub type DDR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DDR`"]
pub struct DDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DDR_W<'a> {
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
#[doc = "Reader of field `BUSSPEED`"]
pub type BUSSPEED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUSSPEED`"]
pub struct BUSSPEED_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSSPEED_W<'a> {
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
#[doc = "Reader of field `SELCLKRX`"]
pub type SELCLKRX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SELCLKRX`"]
pub struct SELCLKRX_W<'a> {
    w: &'a mut W,
}
impl<'a> SELCLKRX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - CLKDIV"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 12 - PWRSAV"]
    #[inline(always)]
    pub fn pwrsav(&self) -> PWRSAV_R {
        PWRSAV_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - WIDBUS"]
    #[inline(always)]
    pub fn widbus(&self) -> WIDBUS_R {
        WIDBUS_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 16 - NEGEDGE"]
    #[inline(always)]
    pub fn negedge(&self) -> NEGEDGE_R {
        NEGEDGE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - HWFC_EN"]
    #[inline(always)]
    pub fn hwfc_en(&self) -> HWFC_EN_R {
        HWFC_EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - DDR"]
    #[inline(always)]
    pub fn ddr(&self) -> DDR_R {
        DDR_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - BUSSPEED"]
    #[inline(always)]
    pub fn busspeed(&self) -> BUSSPEED_R {
        BUSSPEED_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 20:21 - SELCLKRX"]
    #[inline(always)]
    pub fn selclkrx(&self) -> SELCLKRX_R {
        SELCLKRX_R::new(((self.bits >> 20) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - CLKDIV"]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> CLKDIV_W {
        CLKDIV_W { w: self }
    }
    #[doc = "Bit 12 - PWRSAV"]
    #[inline(always)]
    pub fn pwrsav(&mut self) -> PWRSAV_W {
        PWRSAV_W { w: self }
    }
    #[doc = "Bits 14:15 - WIDBUS"]
    #[inline(always)]
    pub fn widbus(&mut self) -> WIDBUS_W {
        WIDBUS_W { w: self }
    }
    #[doc = "Bit 16 - NEGEDGE"]
    #[inline(always)]
    pub fn negedge(&mut self) -> NEGEDGE_W {
        NEGEDGE_W { w: self }
    }
    #[doc = "Bit 17 - HWFC_EN"]
    #[inline(always)]
    pub fn hwfc_en(&mut self) -> HWFC_EN_W {
        HWFC_EN_W { w: self }
    }
    #[doc = "Bit 18 - DDR"]
    #[inline(always)]
    pub fn ddr(&mut self) -> DDR_W {
        DDR_W { w: self }
    }
    #[doc = "Bit 19 - BUSSPEED"]
    #[inline(always)]
    pub fn busspeed(&mut self) -> BUSSPEED_W {
        BUSSPEED_W { w: self }
    }
    #[doc = "Bits 20:21 - SELCLKRX"]
    #[inline(always)]
    pub fn selclkrx(&mut self) -> SELCLKRX_W {
        SELCLKRX_W { w: self }
    }
}

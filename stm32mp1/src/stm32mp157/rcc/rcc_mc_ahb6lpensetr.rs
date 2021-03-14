#[doc = "Reader of register RCC_MC_AHB6LPENSETR"]
pub type R = crate::R<u32, super::RCC_MC_AHB6LPENSETR>;
#[doc = "Writer for register RCC_MC_AHB6LPENSETR"]
pub type W = crate::W<u32, super::RCC_MC_AHB6LPENSETR>;
#[doc = "Register RCC_MC_AHB6LPENSETR `reset()`'s with value 0x0113_57a1"]
impl crate::ResetValue for super::RCC_MC_AHB6LPENSETR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0113_57a1
    }
}
#[doc = "Reader of field `MDMALPEN`"]
pub type MDMALPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MDMALPEN`"]
pub struct MDMALPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MDMALPEN_W<'a> {
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
#[doc = "Reader of field `GPULPEN`"]
pub type GPULPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPULPEN`"]
pub struct GPULPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPULPEN_W<'a> {
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
#[doc = "Reader of field `ETHCKLPEN`"]
pub type ETHCKLPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ETHCKLPEN`"]
pub struct ETHCKLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ETHCKLPEN_W<'a> {
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
#[doc = "Reader of field `ETHTXLPEN`"]
pub type ETHTXLPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ETHTXLPEN`"]
pub struct ETHTXLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ETHTXLPEN_W<'a> {
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
#[doc = "Reader of field `ETHRXLPEN`"]
pub type ETHRXLPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ETHRXLPEN`"]
pub struct ETHRXLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ETHRXLPEN_W<'a> {
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
#[doc = "Reader of field `ETHMACLPEN`"]
pub type ETHMACLPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ETHMACLPEN`"]
pub struct ETHMACLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ETHMACLPEN_W<'a> {
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
#[doc = "Reader of field `ETHSTPEN`"]
pub type ETHSTPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ETHSTPEN`"]
pub struct ETHSTPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ETHSTPEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `FMCLPEN`"]
pub type FMCLPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FMCLPEN`"]
pub struct FMCLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FMCLPEN_W<'a> {
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
#[doc = "Reader of field `QSPILPEN`"]
pub type QSPILPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QSPILPEN`"]
pub struct QSPILPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> QSPILPEN_W<'a> {
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
#[doc = "Reader of field `SDMMC1LPEN`"]
pub type SDMMC1LPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDMMC1LPEN`"]
pub struct SDMMC1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMMC1LPEN_W<'a> {
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
#[doc = "Reader of field `SDMMC2LPEN`"]
pub type SDMMC2LPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDMMC2LPEN`"]
pub struct SDMMC2LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMMC2LPEN_W<'a> {
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
#[doc = "Reader of field `CRC1LPEN`"]
pub type CRC1LPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRC1LPEN`"]
pub struct CRC1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC1LPEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `USBHLPEN`"]
pub type USBHLPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBHLPEN`"]
pub struct USBHLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USBHLPEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - MDMALPEN"]
    #[inline(always)]
    pub fn mdmalpen(&self) -> MDMALPEN_R {
        MDMALPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 5 - GPULPEN"]
    #[inline(always)]
    pub fn gpulpen(&self) -> GPULPEN_R {
        GPULPEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ETHCKLPEN"]
    #[inline(always)]
    pub fn ethcklpen(&self) -> ETHCKLPEN_R {
        ETHCKLPEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ETHTXLPEN"]
    #[inline(always)]
    pub fn ethtxlpen(&self) -> ETHTXLPEN_R {
        ETHTXLPEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ETHRXLPEN"]
    #[inline(always)]
    pub fn ethrxlpen(&self) -> ETHRXLPEN_R {
        ETHRXLPEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ETHMACLPEN"]
    #[inline(always)]
    pub fn ethmaclpen(&self) -> ETHMACLPEN_R {
        ETHMACLPEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - ETHSTPEN"]
    #[inline(always)]
    pub fn ethstpen(&self) -> ETHSTPEN_R {
        ETHSTPEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - FMCLPEN"]
    #[inline(always)]
    pub fn fmclpen(&self) -> FMCLPEN_R {
        FMCLPEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - QSPILPEN"]
    #[inline(always)]
    pub fn qspilpen(&self) -> QSPILPEN_R {
        QSPILPEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SDMMC1LPEN"]
    #[inline(always)]
    pub fn sdmmc1lpen(&self) -> SDMMC1LPEN_R {
        SDMMC1LPEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - SDMMC2LPEN"]
    #[inline(always)]
    pub fn sdmmc2lpen(&self) -> SDMMC2LPEN_R {
        SDMMC2LPEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 20 - CRC1LPEN"]
    #[inline(always)]
    pub fn crc1lpen(&self) -> CRC1LPEN_R {
        CRC1LPEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - USBHLPEN"]
    #[inline(always)]
    pub fn usbhlpen(&self) -> USBHLPEN_R {
        USBHLPEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MDMALPEN"]
    #[inline(always)]
    pub fn mdmalpen(&mut self) -> MDMALPEN_W {
        MDMALPEN_W { w: self }
    }
    #[doc = "Bit 5 - GPULPEN"]
    #[inline(always)]
    pub fn gpulpen(&mut self) -> GPULPEN_W {
        GPULPEN_W { w: self }
    }
    #[doc = "Bit 7 - ETHCKLPEN"]
    #[inline(always)]
    pub fn ethcklpen(&mut self) -> ETHCKLPEN_W {
        ETHCKLPEN_W { w: self }
    }
    #[doc = "Bit 8 - ETHTXLPEN"]
    #[inline(always)]
    pub fn ethtxlpen(&mut self) -> ETHTXLPEN_W {
        ETHTXLPEN_W { w: self }
    }
    #[doc = "Bit 9 - ETHRXLPEN"]
    #[inline(always)]
    pub fn ethrxlpen(&mut self) -> ETHRXLPEN_W {
        ETHRXLPEN_W { w: self }
    }
    #[doc = "Bit 10 - ETHMACLPEN"]
    #[inline(always)]
    pub fn ethmaclpen(&mut self) -> ETHMACLPEN_W {
        ETHMACLPEN_W { w: self }
    }
    #[doc = "Bit 11 - ETHSTPEN"]
    #[inline(always)]
    pub fn ethstpen(&mut self) -> ETHSTPEN_W {
        ETHSTPEN_W { w: self }
    }
    #[doc = "Bit 12 - FMCLPEN"]
    #[inline(always)]
    pub fn fmclpen(&mut self) -> FMCLPEN_W {
        FMCLPEN_W { w: self }
    }
    #[doc = "Bit 14 - QSPILPEN"]
    #[inline(always)]
    pub fn qspilpen(&mut self) -> QSPILPEN_W {
        QSPILPEN_W { w: self }
    }
    #[doc = "Bit 16 - SDMMC1LPEN"]
    #[inline(always)]
    pub fn sdmmc1lpen(&mut self) -> SDMMC1LPEN_W {
        SDMMC1LPEN_W { w: self }
    }
    #[doc = "Bit 17 - SDMMC2LPEN"]
    #[inline(always)]
    pub fn sdmmc2lpen(&mut self) -> SDMMC2LPEN_W {
        SDMMC2LPEN_W { w: self }
    }
    #[doc = "Bit 20 - CRC1LPEN"]
    #[inline(always)]
    pub fn crc1lpen(&mut self) -> CRC1LPEN_W {
        CRC1LPEN_W { w: self }
    }
    #[doc = "Bit 24 - USBHLPEN"]
    #[inline(always)]
    pub fn usbhlpen(&mut self) -> USBHLPEN_W {
        USBHLPEN_W { w: self }
    }
}

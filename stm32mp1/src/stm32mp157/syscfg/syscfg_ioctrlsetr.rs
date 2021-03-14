#[doc = "Reader of register SYSCFG_IOCTRLSETR"]
pub type R = crate::R<u32, super::SYSCFG_IOCTRLSETR>;
#[doc = "Writer for register SYSCFG_IOCTRLSETR"]
pub type W = crate::W<u32, super::SYSCFG_IOCTRLSETR>;
#[doc = "Register SYSCFG_IOCTRLSETR `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSCFG_IOCTRLSETR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HSLVEN_TRACE`"]
pub type HSLVEN_TRACE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSLVEN_TRACE`"]
pub struct HSLVEN_TRACE_W<'a> {
    w: &'a mut W,
}
impl<'a> HSLVEN_TRACE_W<'a> {
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
#[doc = "Reader of field `HSLVEN_QUADSPI`"]
pub type HSLVEN_QUADSPI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSLVEN_QUADSPI`"]
pub struct HSLVEN_QUADSPI_W<'a> {
    w: &'a mut W,
}
impl<'a> HSLVEN_QUADSPI_W<'a> {
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
#[doc = "Reader of field `HSLVEN_ETH`"]
pub type HSLVEN_ETH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSLVEN_ETH`"]
pub struct HSLVEN_ETH_W<'a> {
    w: &'a mut W,
}
impl<'a> HSLVEN_ETH_W<'a> {
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
#[doc = "Reader of field `HSLVEN_SDMMC`"]
pub type HSLVEN_SDMMC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSLVEN_SDMMC`"]
pub struct HSLVEN_SDMMC_W<'a> {
    w: &'a mut W,
}
impl<'a> HSLVEN_SDMMC_W<'a> {
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
#[doc = "Reader of field `HSLVEN_SPI`"]
pub type HSLVEN_SPI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSLVEN_SPI`"]
pub struct HSLVEN_SPI_W<'a> {
    w: &'a mut W,
}
impl<'a> HSLVEN_SPI_W<'a> {
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
impl R {
    #[doc = "Bit 0 - HSLVEN_TRACE"]
    #[inline(always)]
    pub fn hslven_trace(&self) -> HSLVEN_TRACE_R {
        HSLVEN_TRACE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - HSLVEN_QUADSPI"]
    #[inline(always)]
    pub fn hslven_quadspi(&self) -> HSLVEN_QUADSPI_R {
        HSLVEN_QUADSPI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HSLVEN_ETH"]
    #[inline(always)]
    pub fn hslven_eth(&self) -> HSLVEN_ETH_R {
        HSLVEN_ETH_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - HSLVEN_SDMMC"]
    #[inline(always)]
    pub fn hslven_sdmmc(&self) -> HSLVEN_SDMMC_R {
        HSLVEN_SDMMC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - HSLVEN_SPI"]
    #[inline(always)]
    pub fn hslven_spi(&self) -> HSLVEN_SPI_R {
        HSLVEN_SPI_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HSLVEN_TRACE"]
    #[inline(always)]
    pub fn hslven_trace(&mut self) -> HSLVEN_TRACE_W {
        HSLVEN_TRACE_W { w: self }
    }
    #[doc = "Bit 1 - HSLVEN_QUADSPI"]
    #[inline(always)]
    pub fn hslven_quadspi(&mut self) -> HSLVEN_QUADSPI_W {
        HSLVEN_QUADSPI_W { w: self }
    }
    #[doc = "Bit 2 - HSLVEN_ETH"]
    #[inline(always)]
    pub fn hslven_eth(&mut self) -> HSLVEN_ETH_W {
        HSLVEN_ETH_W { w: self }
    }
    #[doc = "Bit 3 - HSLVEN_SDMMC"]
    #[inline(always)]
    pub fn hslven_sdmmc(&mut self) -> HSLVEN_SDMMC_W {
        HSLVEN_SDMMC_W { w: self }
    }
    #[doc = "Bit 4 - HSLVEN_SPI"]
    #[inline(always)]
    pub fn hslven_spi(&mut self) -> HSLVEN_SPI_W {
        HSLVEN_SPI_W { w: self }
    }
}

#[doc = "Reader of register IER2"]
pub type R = crate::R<u32, super::IER2>;
#[doc = "Writer for register IER2"]
pub type W = crate::W<u32, super::IER2>;
#[doc = "Register IER2 `reset()`'s with value 0"]
impl crate::ResetValue for super::IER2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIM8IE`"]
pub type TIM8IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM8IE`"]
pub struct TIM8IE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM8IE_W<'a> {
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
#[doc = "Reader of field `USART1IE`"]
pub type USART1IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USART1IE`"]
pub struct USART1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1IE_W<'a> {
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
#[doc = "Reader of field `TIM15IE`"]
pub type TIM15IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM15IE`"]
pub struct TIM15IE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM15IE_W<'a> {
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
#[doc = "Reader of field `TIM16IE`"]
pub type TIM16IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM16IE`"]
pub struct TIM16IE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM16IE_W<'a> {
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
#[doc = "Reader of field `TIM17IE`"]
pub type TIM17IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM17IE`"]
pub struct TIM17IE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM17IE_W<'a> {
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
#[doc = "Reader of field `SAI1IE`"]
pub type SAI1IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SAI1IE`"]
pub struct SAI1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI1IE_W<'a> {
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
#[doc = "Reader of field `SAI2IE`"]
pub type SAI2IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SAI2IE`"]
pub struct SAI2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI2IE_W<'a> {
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
#[doc = "Reader of field `DFSDM1IE`"]
pub type DFSDM1IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DFSDM1IE`"]
pub struct DFSDM1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> DFSDM1IE_W<'a> {
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
#[doc = "Reader of field `CRCIE`"]
pub type CRCIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRCIE`"]
pub struct CRCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCIE_W<'a> {
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
#[doc = "Reader of field `TSCIE`"]
pub type TSCIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSCIE`"]
pub struct TSCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSCIE_W<'a> {
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
#[doc = "Reader of field `ICACHEIE`"]
pub type ICACHEIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ICACHEIE`"]
pub struct ICACHEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACHEIE_W<'a> {
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
#[doc = "Reader of field `ADCIE`"]
pub type ADCIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADCIE`"]
pub struct ADCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCIE_W<'a> {
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
#[doc = "Reader of field `AESIE`"]
pub type AESIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AESIE`"]
pub struct AESIE_W<'a> {
    w: &'a mut W,
}
impl<'a> AESIE_W<'a> {
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
#[doc = "Reader of field `HASHIE`"]
pub type HASHIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HASHIE`"]
pub struct HASHIE_W<'a> {
    w: &'a mut W,
}
impl<'a> HASHIE_W<'a> {
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
#[doc = "Reader of field `RNGIE`"]
pub type RNGIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RNGIE`"]
pub struct RNGIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RNGIE_W<'a> {
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
#[doc = "Reader of field `PKAIE`"]
pub type PKAIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PKAIE`"]
pub struct PKAIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PKAIE_W<'a> {
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
#[doc = "Reader of field `SDMMC1IE`"]
pub type SDMMC1IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDMMC1IE`"]
pub struct SDMMC1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMMC1IE_W<'a> {
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
#[doc = "Reader of field `FMC_REGIE`"]
pub type FMC_REGIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FMC_REGIE`"]
pub struct FMC_REGIE_W<'a> {
    w: &'a mut W,
}
impl<'a> FMC_REGIE_W<'a> {
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
#[doc = "Reader of field `OCTOSPI1_REGIE`"]
pub type OCTOSPI1_REGIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OCTOSPI1_REGIE`"]
pub struct OCTOSPI1_REGIE_W<'a> {
    w: &'a mut W,
}
impl<'a> OCTOSPI1_REGIE_W<'a> {
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
#[doc = "Reader of field `RTCIE`"]
pub type RTCIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCIE`"]
pub struct RTCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCIE_W<'a> {
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
#[doc = "Reader of field `PWRIE`"]
pub type PWRIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWRIE`"]
pub struct PWRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRIE_W<'a> {
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
#[doc = "Reader of field `SYSCFGIE`"]
pub type SYSCFGIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCFGIE`"]
pub struct SYSCFGIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCFGIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `DMA1IE`"]
pub type DMA1IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA1IE`"]
pub struct DMA1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1IE_W<'a> {
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
#[doc = "Reader of field `DMA2IE`"]
pub type DMA2IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA2IE`"]
pub struct DMA2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2IE_W<'a> {
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
#[doc = "Reader of field `DMAMUX1IE`"]
pub type DMAMUX1IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAMUX1IE`"]
pub struct DMAMUX1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAMUX1IE_W<'a> {
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
#[doc = "Reader of field `RCCIE`"]
pub type RCCIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RCCIE`"]
pub struct RCCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RCCIE_W<'a> {
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
#[doc = "Reader of field `FLASHIE`"]
pub type FLASHIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLASHIE`"]
pub struct FLASHIE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASHIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `FLASH_REGIE`"]
pub type FLASH_REGIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLASH_REGIE`"]
pub struct FLASH_REGIE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_REGIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `EXTIIE`"]
pub type EXTIIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTIIE`"]
pub struct EXTIIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `OTFDEC1IE`"]
pub type OTFDEC1IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OTFDEC1IE`"]
pub struct OTFDEC1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> OTFDEC1IE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - TIM8IE"]
    #[inline(always)]
    pub fn tim8ie(&self) -> TIM8IE_R {
        TIM8IE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USART1IE"]
    #[inline(always)]
    pub fn usart1ie(&self) -> USART1IE_R {
        USART1IE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TIM15IE"]
    #[inline(always)]
    pub fn tim15ie(&self) -> TIM15IE_R {
        TIM15IE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TIM16IE"]
    #[inline(always)]
    pub fn tim16ie(&self) -> TIM16IE_R {
        TIM16IE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TIM17IE"]
    #[inline(always)]
    pub fn tim17ie(&self) -> TIM17IE_R {
        TIM17IE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SAI1IE"]
    #[inline(always)]
    pub fn sai1ie(&self) -> SAI1IE_R {
        SAI1IE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SAI2IE"]
    #[inline(always)]
    pub fn sai2ie(&self) -> SAI2IE_R {
        SAI2IE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DFSDM1IE"]
    #[inline(always)]
    pub fn dfsdm1ie(&self) -> DFSDM1IE_R {
        DFSDM1IE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - CRCIE"]
    #[inline(always)]
    pub fn crcie(&self) -> CRCIE_R {
        CRCIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TSCIE"]
    #[inline(always)]
    pub fn tscie(&self) -> TSCIE_R {
        TSCIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ICACHEIE"]
    #[inline(always)]
    pub fn icacheie(&self) -> ICACHEIE_R {
        ICACHEIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - ADCIE"]
    #[inline(always)]
    pub fn adcie(&self) -> ADCIE_R {
        ADCIE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - AESIE"]
    #[inline(always)]
    pub fn aesie(&self) -> AESIE_R {
        AESIE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - HASHIE"]
    #[inline(always)]
    pub fn hashie(&self) -> HASHIE_R {
        HASHIE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - RNGIE"]
    #[inline(always)]
    pub fn rngie(&self) -> RNGIE_R {
        RNGIE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - PKAIE"]
    #[inline(always)]
    pub fn pkaie(&self) -> PKAIE_R {
        PKAIE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SDMMC1IE"]
    #[inline(always)]
    pub fn sdmmc1ie(&self) -> SDMMC1IE_R {
        SDMMC1IE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - FMC_REGIE"]
    #[inline(always)]
    pub fn fmc_regie(&self) -> FMC_REGIE_R {
        FMC_REGIE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - OCTOSPI1_REGIE"]
    #[inline(always)]
    pub fn octospi1_regie(&self) -> OCTOSPI1_REGIE_R {
        OCTOSPI1_REGIE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - RTCIE"]
    #[inline(always)]
    pub fn rtcie(&self) -> RTCIE_R {
        RTCIE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - PWRIE"]
    #[inline(always)]
    pub fn pwrie(&self) -> PWRIE_R {
        PWRIE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - SYSCFGIE"]
    #[inline(always)]
    pub fn syscfgie(&self) -> SYSCFGIE_R {
        SYSCFGIE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - DMA1IE"]
    #[inline(always)]
    pub fn dma1ie(&self) -> DMA1IE_R {
        DMA1IE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - DMA2IE"]
    #[inline(always)]
    pub fn dma2ie(&self) -> DMA2IE_R {
        DMA2IE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - DMAMUX1IE"]
    #[inline(always)]
    pub fn dmamux1ie(&self) -> DMAMUX1IE_R {
        DMAMUX1IE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - RCCIE"]
    #[inline(always)]
    pub fn rccie(&self) -> RCCIE_R {
        RCCIE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - FLASHIE"]
    #[inline(always)]
    pub fn flashie(&self) -> FLASHIE_R {
        FLASHIE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - FLASH_REGIE"]
    #[inline(always)]
    pub fn flash_regie(&self) -> FLASH_REGIE_R {
        FLASH_REGIE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - EXTIIE"]
    #[inline(always)]
    pub fn extiie(&self) -> EXTIIE_R {
        EXTIIE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - OTFDEC1IE"]
    #[inline(always)]
    pub fn otfdec1ie(&self) -> OTFDEC1IE_R {
        OTFDEC1IE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM8IE"]
    #[inline(always)]
    pub fn tim8ie(&mut self) -> TIM8IE_W {
        TIM8IE_W { w: self }
    }
    #[doc = "Bit 1 - USART1IE"]
    #[inline(always)]
    pub fn usart1ie(&mut self) -> USART1IE_W {
        USART1IE_W { w: self }
    }
    #[doc = "Bit 2 - TIM15IE"]
    #[inline(always)]
    pub fn tim15ie(&mut self) -> TIM15IE_W {
        TIM15IE_W { w: self }
    }
    #[doc = "Bit 3 - TIM16IE"]
    #[inline(always)]
    pub fn tim16ie(&mut self) -> TIM16IE_W {
        TIM16IE_W { w: self }
    }
    #[doc = "Bit 4 - TIM17IE"]
    #[inline(always)]
    pub fn tim17ie(&mut self) -> TIM17IE_W {
        TIM17IE_W { w: self }
    }
    #[doc = "Bit 5 - SAI1IE"]
    #[inline(always)]
    pub fn sai1ie(&mut self) -> SAI1IE_W {
        SAI1IE_W { w: self }
    }
    #[doc = "Bit 6 - SAI2IE"]
    #[inline(always)]
    pub fn sai2ie(&mut self) -> SAI2IE_W {
        SAI2IE_W { w: self }
    }
    #[doc = "Bit 7 - DFSDM1IE"]
    #[inline(always)]
    pub fn dfsdm1ie(&mut self) -> DFSDM1IE_W {
        DFSDM1IE_W { w: self }
    }
    #[doc = "Bit 8 - CRCIE"]
    #[inline(always)]
    pub fn crcie(&mut self) -> CRCIE_W {
        CRCIE_W { w: self }
    }
    #[doc = "Bit 9 - TSCIE"]
    #[inline(always)]
    pub fn tscie(&mut self) -> TSCIE_W {
        TSCIE_W { w: self }
    }
    #[doc = "Bit 10 - ICACHEIE"]
    #[inline(always)]
    pub fn icacheie(&mut self) -> ICACHEIE_W {
        ICACHEIE_W { w: self }
    }
    #[doc = "Bit 11 - ADCIE"]
    #[inline(always)]
    pub fn adcie(&mut self) -> ADCIE_W {
        ADCIE_W { w: self }
    }
    #[doc = "Bit 12 - AESIE"]
    #[inline(always)]
    pub fn aesie(&mut self) -> AESIE_W {
        AESIE_W { w: self }
    }
    #[doc = "Bit 13 - HASHIE"]
    #[inline(always)]
    pub fn hashie(&mut self) -> HASHIE_W {
        HASHIE_W { w: self }
    }
    #[doc = "Bit 14 - RNGIE"]
    #[inline(always)]
    pub fn rngie(&mut self) -> RNGIE_W {
        RNGIE_W { w: self }
    }
    #[doc = "Bit 15 - PKAIE"]
    #[inline(always)]
    pub fn pkaie(&mut self) -> PKAIE_W {
        PKAIE_W { w: self }
    }
    #[doc = "Bit 16 - SDMMC1IE"]
    #[inline(always)]
    pub fn sdmmc1ie(&mut self) -> SDMMC1IE_W {
        SDMMC1IE_W { w: self }
    }
    #[doc = "Bit 17 - FMC_REGIE"]
    #[inline(always)]
    pub fn fmc_regie(&mut self) -> FMC_REGIE_W {
        FMC_REGIE_W { w: self }
    }
    #[doc = "Bit 18 - OCTOSPI1_REGIE"]
    #[inline(always)]
    pub fn octospi1_regie(&mut self) -> OCTOSPI1_REGIE_W {
        OCTOSPI1_REGIE_W { w: self }
    }
    #[doc = "Bit 19 - RTCIE"]
    #[inline(always)]
    pub fn rtcie(&mut self) -> RTCIE_W {
        RTCIE_W { w: self }
    }
    #[doc = "Bit 20 - PWRIE"]
    #[inline(always)]
    pub fn pwrie(&mut self) -> PWRIE_W {
        PWRIE_W { w: self }
    }
    #[doc = "Bit 21 - SYSCFGIE"]
    #[inline(always)]
    pub fn syscfgie(&mut self) -> SYSCFGIE_W {
        SYSCFGIE_W { w: self }
    }
    #[doc = "Bit 22 - DMA1IE"]
    #[inline(always)]
    pub fn dma1ie(&mut self) -> DMA1IE_W {
        DMA1IE_W { w: self }
    }
    #[doc = "Bit 23 - DMA2IE"]
    #[inline(always)]
    pub fn dma2ie(&mut self) -> DMA2IE_W {
        DMA2IE_W { w: self }
    }
    #[doc = "Bit 24 - DMAMUX1IE"]
    #[inline(always)]
    pub fn dmamux1ie(&mut self) -> DMAMUX1IE_W {
        DMAMUX1IE_W { w: self }
    }
    #[doc = "Bit 25 - RCCIE"]
    #[inline(always)]
    pub fn rccie(&mut self) -> RCCIE_W {
        RCCIE_W { w: self }
    }
    #[doc = "Bit 26 - FLASHIE"]
    #[inline(always)]
    pub fn flashie(&mut self) -> FLASHIE_W {
        FLASHIE_W { w: self }
    }
    #[doc = "Bit 27 - FLASH_REGIE"]
    #[inline(always)]
    pub fn flash_regie(&mut self) -> FLASH_REGIE_W {
        FLASH_REGIE_W { w: self }
    }
    #[doc = "Bit 28 - EXTIIE"]
    #[inline(always)]
    pub fn extiie(&mut self) -> EXTIIE_W {
        EXTIIE_W { w: self }
    }
    #[doc = "Bit 29 - OTFDEC1IE"]
    #[inline(always)]
    pub fn otfdec1ie(&mut self) -> OTFDEC1IE_W {
        OTFDEC1IE_W { w: self }
    }
}

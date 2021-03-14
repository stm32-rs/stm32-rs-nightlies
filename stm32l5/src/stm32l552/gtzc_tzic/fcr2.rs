#[doc = "Reader of register FCR2"]
pub type R = crate::R<u32, super::FCR2>;
#[doc = "Writer for register FCR2"]
pub type W = crate::W<u32, super::FCR2>;
#[doc = "Register FCR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::FCR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIM8FC`"]
pub type TIM8FC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM8FC`"]
pub struct TIM8FC_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM8FC_W<'a> {
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
#[doc = "Reader of field `USART1FC`"]
pub type USART1FC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USART1FC`"]
pub struct USART1FC_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1FC_W<'a> {
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
#[doc = "Reader of field `TIM15FC`"]
pub type TIM15FC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM15FC`"]
pub struct TIM15FC_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM15FC_W<'a> {
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
#[doc = "Reader of field `TIM16FC`"]
pub type TIM16FC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM16FC`"]
pub struct TIM16FC_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM16FC_W<'a> {
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
#[doc = "Reader of field `TIM17FC`"]
pub type TIM17FC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM17FC`"]
pub struct TIM17FC_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM17FC_W<'a> {
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
#[doc = "Reader of field `SAI1FC`"]
pub type SAI1FC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SAI1FC`"]
pub struct SAI1FC_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI1FC_W<'a> {
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
#[doc = "Reader of field `SAI2FC`"]
pub type SAI2FC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SAI2FC`"]
pub struct SAI2FC_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI2FC_W<'a> {
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
#[doc = "Reader of field `DFSDM1FC`"]
pub type DFSDM1FC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DFSDM1FC`"]
pub struct DFSDM1FC_W<'a> {
    w: &'a mut W,
}
impl<'a> DFSDM1FC_W<'a> {
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
#[doc = "Reader of field `CRCFC`"]
pub type CRCFC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRCFC`"]
pub struct CRCFC_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCFC_W<'a> {
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
#[doc = "Reader of field `TSCFC`"]
pub type TSCFC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSCFC`"]
pub struct TSCFC_W<'a> {
    w: &'a mut W,
}
impl<'a> TSCFC_W<'a> {
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
#[doc = "Reader of field `ICACHEFC`"]
pub type ICACHEFC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ICACHEFC`"]
pub struct ICACHEFC_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACHEFC_W<'a> {
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
#[doc = "Reader of field `ADCFC`"]
pub type ADCFC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADCFC`"]
pub struct ADCFC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCFC_W<'a> {
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
#[doc = "Reader of field `AESFC`"]
pub type AESFC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AESFC`"]
pub struct AESFC_W<'a> {
    w: &'a mut W,
}
impl<'a> AESFC_W<'a> {
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
#[doc = "Reader of field `HASHFC`"]
pub type HASHFC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HASHFC`"]
pub struct HASHFC_W<'a> {
    w: &'a mut W,
}
impl<'a> HASHFC_W<'a> {
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
#[doc = "Reader of field `RNGFC`"]
pub type RNGFC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RNGFC`"]
pub struct RNGFC_W<'a> {
    w: &'a mut W,
}
impl<'a> RNGFC_W<'a> {
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
#[doc = "Reader of field `PKAFC`"]
pub type PKAFC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PKAFC`"]
pub struct PKAFC_W<'a> {
    w: &'a mut W,
}
impl<'a> PKAFC_W<'a> {
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
#[doc = "Reader of field `SDMMC1FC`"]
pub type SDMMC1FC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDMMC1FC`"]
pub struct SDMMC1FC_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMMC1FC_W<'a> {
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
#[doc = "Reader of field `FMC_REGFC`"]
pub type FMC_REGFC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FMC_REGFC`"]
pub struct FMC_REGFC_W<'a> {
    w: &'a mut W,
}
impl<'a> FMC_REGFC_W<'a> {
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
#[doc = "Reader of field `OCTOSPI1_REGFC`"]
pub type OCTOSPI1_REGFC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OCTOSPI1_REGFC`"]
pub struct OCTOSPI1_REGFC_W<'a> {
    w: &'a mut W,
}
impl<'a> OCTOSPI1_REGFC_W<'a> {
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
#[doc = "Reader of field `RTCFC`"]
pub type RTCFC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCFC`"]
pub struct RTCFC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCFC_W<'a> {
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
#[doc = "Reader of field `PWRFC`"]
pub type PWRFC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWRFC`"]
pub struct PWRFC_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRFC_W<'a> {
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
#[doc = "Reader of field `SYSCFGFC`"]
pub type SYSCFGFC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCFGFC`"]
pub struct SYSCFGFC_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCFGFC_W<'a> {
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
#[doc = "Reader of field `DMA1FC`"]
pub type DMA1FC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA1FC`"]
pub struct DMA1FC_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1FC_W<'a> {
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
#[doc = "Reader of field `DMA2FC`"]
pub type DMA2FC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA2FC`"]
pub struct DMA2FC_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2FC_W<'a> {
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
#[doc = "Reader of field `DMAMUX1FC`"]
pub type DMAMUX1FC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAMUX1FC`"]
pub struct DMAMUX1FC_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAMUX1FC_W<'a> {
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
#[doc = "Reader of field `RCCFC`"]
pub type RCCFC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RCCFC`"]
pub struct RCCFC_W<'a> {
    w: &'a mut W,
}
impl<'a> RCCFC_W<'a> {
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
#[doc = "Reader of field `FLASHFC`"]
pub type FLASHFC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLASHFC`"]
pub struct FLASHFC_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASHFC_W<'a> {
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
#[doc = "Reader of field `FLASH_REGFC`"]
pub type FLASH_REGFC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLASH_REGFC`"]
pub struct FLASH_REGFC_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_REGFC_W<'a> {
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
#[doc = "Reader of field `EXTIFC`"]
pub type EXTIFC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTIFC`"]
pub struct EXTIFC_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIFC_W<'a> {
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
#[doc = "Reader of field `OTFDEC1FC`"]
pub type OTFDEC1FC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OTFDEC1FC`"]
pub struct OTFDEC1FC_W<'a> {
    w: &'a mut W,
}
impl<'a> OTFDEC1FC_W<'a> {
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
    #[doc = "Bit 0 - TIM8FC"]
    #[inline(always)]
    pub fn tim8fc(&self) -> TIM8FC_R {
        TIM8FC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USART1FC"]
    #[inline(always)]
    pub fn usart1fc(&self) -> USART1FC_R {
        USART1FC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TIM15FC"]
    #[inline(always)]
    pub fn tim15fc(&self) -> TIM15FC_R {
        TIM15FC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TIM16FC"]
    #[inline(always)]
    pub fn tim16fc(&self) -> TIM16FC_R {
        TIM16FC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TIM17FC"]
    #[inline(always)]
    pub fn tim17fc(&self) -> TIM17FC_R {
        TIM17FC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SAI1FC"]
    #[inline(always)]
    pub fn sai1fc(&self) -> SAI1FC_R {
        SAI1FC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SAI2FC"]
    #[inline(always)]
    pub fn sai2fc(&self) -> SAI2FC_R {
        SAI2FC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DFSDM1FC"]
    #[inline(always)]
    pub fn dfsdm1fc(&self) -> DFSDM1FC_R {
        DFSDM1FC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - CRCFC"]
    #[inline(always)]
    pub fn crcfc(&self) -> CRCFC_R {
        CRCFC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TSCFC"]
    #[inline(always)]
    pub fn tscfc(&self) -> TSCFC_R {
        TSCFC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ICACHEFC"]
    #[inline(always)]
    pub fn icachefc(&self) -> ICACHEFC_R {
        ICACHEFC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - ADCFC"]
    #[inline(always)]
    pub fn adcfc(&self) -> ADCFC_R {
        ADCFC_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - AESFC"]
    #[inline(always)]
    pub fn aesfc(&self) -> AESFC_R {
        AESFC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - HASHFC"]
    #[inline(always)]
    pub fn hashfc(&self) -> HASHFC_R {
        HASHFC_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - RNGFC"]
    #[inline(always)]
    pub fn rngfc(&self) -> RNGFC_R {
        RNGFC_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - PKAFC"]
    #[inline(always)]
    pub fn pkafc(&self) -> PKAFC_R {
        PKAFC_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SDMMC1FC"]
    #[inline(always)]
    pub fn sdmmc1fc(&self) -> SDMMC1FC_R {
        SDMMC1FC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - FMC_REGFC"]
    #[inline(always)]
    pub fn fmc_regfc(&self) -> FMC_REGFC_R {
        FMC_REGFC_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - OCTOSPI1_REGFC"]
    #[inline(always)]
    pub fn octospi1_regfc(&self) -> OCTOSPI1_REGFC_R {
        OCTOSPI1_REGFC_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - RTCFC"]
    #[inline(always)]
    pub fn rtcfc(&self) -> RTCFC_R {
        RTCFC_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - PWRFC"]
    #[inline(always)]
    pub fn pwrfc(&self) -> PWRFC_R {
        PWRFC_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - SYSCFGFC"]
    #[inline(always)]
    pub fn syscfgfc(&self) -> SYSCFGFC_R {
        SYSCFGFC_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - DMA1FC"]
    #[inline(always)]
    pub fn dma1fc(&self) -> DMA1FC_R {
        DMA1FC_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - DMA2FC"]
    #[inline(always)]
    pub fn dma2fc(&self) -> DMA2FC_R {
        DMA2FC_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - DMAMUX1FC"]
    #[inline(always)]
    pub fn dmamux1fc(&self) -> DMAMUX1FC_R {
        DMAMUX1FC_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - RCCFC"]
    #[inline(always)]
    pub fn rccfc(&self) -> RCCFC_R {
        RCCFC_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - FLASHFC"]
    #[inline(always)]
    pub fn flashfc(&self) -> FLASHFC_R {
        FLASHFC_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - FLASH_REGFC"]
    #[inline(always)]
    pub fn flash_regfc(&self) -> FLASH_REGFC_R {
        FLASH_REGFC_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - EXTIFC"]
    #[inline(always)]
    pub fn extifc(&self) -> EXTIFC_R {
        EXTIFC_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - OTFDEC1FC"]
    #[inline(always)]
    pub fn otfdec1fc(&self) -> OTFDEC1FC_R {
        OTFDEC1FC_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM8FC"]
    #[inline(always)]
    pub fn tim8fc(&mut self) -> TIM8FC_W {
        TIM8FC_W { w: self }
    }
    #[doc = "Bit 1 - USART1FC"]
    #[inline(always)]
    pub fn usart1fc(&mut self) -> USART1FC_W {
        USART1FC_W { w: self }
    }
    #[doc = "Bit 2 - TIM15FC"]
    #[inline(always)]
    pub fn tim15fc(&mut self) -> TIM15FC_W {
        TIM15FC_W { w: self }
    }
    #[doc = "Bit 3 - TIM16FC"]
    #[inline(always)]
    pub fn tim16fc(&mut self) -> TIM16FC_W {
        TIM16FC_W { w: self }
    }
    #[doc = "Bit 4 - TIM17FC"]
    #[inline(always)]
    pub fn tim17fc(&mut self) -> TIM17FC_W {
        TIM17FC_W { w: self }
    }
    #[doc = "Bit 5 - SAI1FC"]
    #[inline(always)]
    pub fn sai1fc(&mut self) -> SAI1FC_W {
        SAI1FC_W { w: self }
    }
    #[doc = "Bit 6 - SAI2FC"]
    #[inline(always)]
    pub fn sai2fc(&mut self) -> SAI2FC_W {
        SAI2FC_W { w: self }
    }
    #[doc = "Bit 7 - DFSDM1FC"]
    #[inline(always)]
    pub fn dfsdm1fc(&mut self) -> DFSDM1FC_W {
        DFSDM1FC_W { w: self }
    }
    #[doc = "Bit 8 - CRCFC"]
    #[inline(always)]
    pub fn crcfc(&mut self) -> CRCFC_W {
        CRCFC_W { w: self }
    }
    #[doc = "Bit 9 - TSCFC"]
    #[inline(always)]
    pub fn tscfc(&mut self) -> TSCFC_W {
        TSCFC_W { w: self }
    }
    #[doc = "Bit 10 - ICACHEFC"]
    #[inline(always)]
    pub fn icachefc(&mut self) -> ICACHEFC_W {
        ICACHEFC_W { w: self }
    }
    #[doc = "Bit 11 - ADCFC"]
    #[inline(always)]
    pub fn adcfc(&mut self) -> ADCFC_W {
        ADCFC_W { w: self }
    }
    #[doc = "Bit 12 - AESFC"]
    #[inline(always)]
    pub fn aesfc(&mut self) -> AESFC_W {
        AESFC_W { w: self }
    }
    #[doc = "Bit 13 - HASHFC"]
    #[inline(always)]
    pub fn hashfc(&mut self) -> HASHFC_W {
        HASHFC_W { w: self }
    }
    #[doc = "Bit 14 - RNGFC"]
    #[inline(always)]
    pub fn rngfc(&mut self) -> RNGFC_W {
        RNGFC_W { w: self }
    }
    #[doc = "Bit 15 - PKAFC"]
    #[inline(always)]
    pub fn pkafc(&mut self) -> PKAFC_W {
        PKAFC_W { w: self }
    }
    #[doc = "Bit 16 - SDMMC1FC"]
    #[inline(always)]
    pub fn sdmmc1fc(&mut self) -> SDMMC1FC_W {
        SDMMC1FC_W { w: self }
    }
    #[doc = "Bit 17 - FMC_REGFC"]
    #[inline(always)]
    pub fn fmc_regfc(&mut self) -> FMC_REGFC_W {
        FMC_REGFC_W { w: self }
    }
    #[doc = "Bit 18 - OCTOSPI1_REGFC"]
    #[inline(always)]
    pub fn octospi1_regfc(&mut self) -> OCTOSPI1_REGFC_W {
        OCTOSPI1_REGFC_W { w: self }
    }
    #[doc = "Bit 19 - RTCFC"]
    #[inline(always)]
    pub fn rtcfc(&mut self) -> RTCFC_W {
        RTCFC_W { w: self }
    }
    #[doc = "Bit 20 - PWRFC"]
    #[inline(always)]
    pub fn pwrfc(&mut self) -> PWRFC_W {
        PWRFC_W { w: self }
    }
    #[doc = "Bit 21 - SYSCFGFC"]
    #[inline(always)]
    pub fn syscfgfc(&mut self) -> SYSCFGFC_W {
        SYSCFGFC_W { w: self }
    }
    #[doc = "Bit 22 - DMA1FC"]
    #[inline(always)]
    pub fn dma1fc(&mut self) -> DMA1FC_W {
        DMA1FC_W { w: self }
    }
    #[doc = "Bit 23 - DMA2FC"]
    #[inline(always)]
    pub fn dma2fc(&mut self) -> DMA2FC_W {
        DMA2FC_W { w: self }
    }
    #[doc = "Bit 24 - DMAMUX1FC"]
    #[inline(always)]
    pub fn dmamux1fc(&mut self) -> DMAMUX1FC_W {
        DMAMUX1FC_W { w: self }
    }
    #[doc = "Bit 25 - RCCFC"]
    #[inline(always)]
    pub fn rccfc(&mut self) -> RCCFC_W {
        RCCFC_W { w: self }
    }
    #[doc = "Bit 26 - FLASHFC"]
    #[inline(always)]
    pub fn flashfc(&mut self) -> FLASHFC_W {
        FLASHFC_W { w: self }
    }
    #[doc = "Bit 27 - FLASH_REGFC"]
    #[inline(always)]
    pub fn flash_regfc(&mut self) -> FLASH_REGFC_W {
        FLASH_REGFC_W { w: self }
    }
    #[doc = "Bit 28 - EXTIFC"]
    #[inline(always)]
    pub fn extifc(&mut self) -> EXTIFC_W {
        EXTIFC_W { w: self }
    }
    #[doc = "Bit 29 - OTFDEC1FC"]
    #[inline(always)]
    pub fn otfdec1fc(&mut self) -> OTFDEC1FC_W {
        OTFDEC1FC_W { w: self }
    }
}

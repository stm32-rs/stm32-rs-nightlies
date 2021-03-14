#[doc = "Reader of register MAPR2"]
pub type R = crate::R<u32, super::MAPR2>;
#[doc = "Writer for register MAPR2"]
pub type W = crate::W<u32, super::MAPR2>;
#[doc = "Register MAPR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::MAPR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIM15_REMAP`"]
pub type TIM15_REMAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM15_REMAP`"]
pub struct TIM15_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM15_REMAP_W<'a> {
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
#[doc = "Reader of field `TIM16_REMAP`"]
pub type TIM16_REMAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM16_REMAP`"]
pub struct TIM16_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM16_REMAP_W<'a> {
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
#[doc = "Reader of field `TIM17_REMAP`"]
pub type TIM17_REMAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM17_REMAP`"]
pub struct TIM17_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM17_REMAP_W<'a> {
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
#[doc = "Reader of field `TIM13_REMAP`"]
pub type TIM13_REMAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM13_REMAP`"]
pub struct TIM13_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM13_REMAP_W<'a> {
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
#[doc = "Reader of field `TIM14_REMAP`"]
pub type TIM14_REMAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM14_REMAP`"]
pub struct TIM14_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM14_REMAP_W<'a> {
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
#[doc = "Reader of field `FSMC_NADV`"]
pub type FSMC_NADV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSMC_NADV`"]
pub struct FSMC_NADV_W<'a> {
    w: &'a mut W,
}
impl<'a> FSMC_NADV_W<'a> {
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
#[doc = "Reader of field `CEC_REMAP`"]
pub type CEC_REMAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CEC_REMAP`"]
pub struct CEC_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> CEC_REMAP_W<'a> {
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
#[doc = "Reader of field `TIM1_DMA_REMAP`"]
pub type TIM1_DMA_REMAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM1_DMA_REMAP`"]
pub struct TIM1_DMA_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM1_DMA_REMAP_W<'a> {
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
#[doc = "Reader of field `TIM67_DAC_DMA_REMAP`"]
pub type TIM67_DAC_DMA_REMAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM67_DAC_DMA_REMAP`"]
pub struct TIM67_DAC_DMA_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM67_DAC_DMA_REMAP_W<'a> {
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
#[doc = "Reader of field `TIM12_REMAP`"]
pub type TIM12_REMAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM12_REMAP`"]
pub struct TIM12_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM12_REMAP_W<'a> {
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
#[doc = "Reader of field `MISC_REMAP`"]
pub type MISC_REMAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MISC_REMAP`"]
pub struct MISC_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> MISC_REMAP_W<'a> {
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
impl R {
    #[doc = "Bit 0 - TIM15 remapping"]
    #[inline(always)]
    pub fn tim15_remap(&self) -> TIM15_REMAP_R {
        TIM15_REMAP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TIM16 remapping"]
    #[inline(always)]
    pub fn tim16_remap(&self) -> TIM16_REMAP_R {
        TIM16_REMAP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TIM17 remapping"]
    #[inline(always)]
    pub fn tim17_remap(&self) -> TIM17_REMAP_R {
        TIM17_REMAP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TIM13 remapping"]
    #[inline(always)]
    pub fn tim13_remap(&self) -> TIM13_REMAP_R {
        TIM13_REMAP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TIM14 remapping"]
    #[inline(always)]
    pub fn tim14_remap(&self) -> TIM14_REMAP_R {
        TIM14_REMAP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - NADV connect/disconnect"]
    #[inline(always)]
    pub fn fsmc_nadv(&self) -> FSMC_NADV_R {
        FSMC_NADV_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CEC remapping"]
    #[inline(always)]
    pub fn cec_remap(&self) -> CEC_REMAP_R {
        CEC_REMAP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TIM1 DMA remapping"]
    #[inline(always)]
    pub fn tim1_dma_remap(&self) -> TIM1_DMA_REMAP_R {
        TIM1_DMA_REMAP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TIM67_DAC DMA remapping"]
    #[inline(always)]
    pub fn tim67_dac_dma_remap(&self) -> TIM67_DAC_DMA_REMAP_R {
        TIM67_DAC_DMA_REMAP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - TIM12 remapping"]
    #[inline(always)]
    pub fn tim12_remap(&self) -> TIM12_REMAP_R {
        TIM12_REMAP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Miscellaneous features remapping"]
    #[inline(always)]
    pub fn misc_remap(&self) -> MISC_REMAP_R {
        MISC_REMAP_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM15 remapping"]
    #[inline(always)]
    pub fn tim15_remap(&mut self) -> TIM15_REMAP_W {
        TIM15_REMAP_W { w: self }
    }
    #[doc = "Bit 1 - TIM16 remapping"]
    #[inline(always)]
    pub fn tim16_remap(&mut self) -> TIM16_REMAP_W {
        TIM16_REMAP_W { w: self }
    }
    #[doc = "Bit 2 - TIM17 remapping"]
    #[inline(always)]
    pub fn tim17_remap(&mut self) -> TIM17_REMAP_W {
        TIM17_REMAP_W { w: self }
    }
    #[doc = "Bit 8 - TIM13 remapping"]
    #[inline(always)]
    pub fn tim13_remap(&mut self) -> TIM13_REMAP_W {
        TIM13_REMAP_W { w: self }
    }
    #[doc = "Bit 9 - TIM14 remapping"]
    #[inline(always)]
    pub fn tim14_remap(&mut self) -> TIM14_REMAP_W {
        TIM14_REMAP_W { w: self }
    }
    #[doc = "Bit 10 - NADV connect/disconnect"]
    #[inline(always)]
    pub fn fsmc_nadv(&mut self) -> FSMC_NADV_W {
        FSMC_NADV_W { w: self }
    }
    #[doc = "Bit 3 - CEC remapping"]
    #[inline(always)]
    pub fn cec_remap(&mut self) -> CEC_REMAP_W {
        CEC_REMAP_W { w: self }
    }
    #[doc = "Bit 4 - TIM1 DMA remapping"]
    #[inline(always)]
    pub fn tim1_dma_remap(&mut self) -> TIM1_DMA_REMAP_W {
        TIM1_DMA_REMAP_W { w: self }
    }
    #[doc = "Bit 11 - TIM67_DAC DMA remapping"]
    #[inline(always)]
    pub fn tim67_dac_dma_remap(&mut self) -> TIM67_DAC_DMA_REMAP_W {
        TIM67_DAC_DMA_REMAP_W { w: self }
    }
    #[doc = "Bit 12 - TIM12 remapping"]
    #[inline(always)]
    pub fn tim12_remap(&mut self) -> TIM12_REMAP_W {
        TIM12_REMAP_W { w: self }
    }
    #[doc = "Bit 13 - Miscellaneous features remapping"]
    #[inline(always)]
    pub fn misc_remap(&mut self) -> MISC_REMAP_W {
        MISC_REMAP_W { w: self }
    }
}

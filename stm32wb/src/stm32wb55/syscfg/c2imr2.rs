#[doc = "Reader of register C2IMR2"]
pub type R = crate::R<u32, super::C2IMR2>;
#[doc = "Writer for register C2IMR2"]
pub type W = crate::W<u32, super::C2IMR2>;
#[doc = "Register C2IMR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::C2IMR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMA1_CH1_IM`"]
pub type DMA1_CH1_IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA1_CH1_IM`"]
pub struct DMA1_CH1_IM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1_CH1_IM_W<'a> {
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
#[doc = "Reader of field `DMA1_CH2_IM`"]
pub type DMA1_CH2_IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA1_CH2_IM`"]
pub struct DMA1_CH2_IM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1_CH2_IM_W<'a> {
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
#[doc = "Reader of field `DMA1_CH3_IM`"]
pub type DMA1_CH3_IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA1_CH3_IM`"]
pub struct DMA1_CH3_IM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1_CH3_IM_W<'a> {
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
#[doc = "Reader of field `DMA1_CH4_IM`"]
pub type DMA1_CH4_IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA1_CH4_IM`"]
pub struct DMA1_CH4_IM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1_CH4_IM_W<'a> {
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
#[doc = "Reader of field `DMA1_CH5_IM`"]
pub type DMA1_CH5_IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA1_CH5_IM`"]
pub struct DMA1_CH5_IM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1_CH5_IM_W<'a> {
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
#[doc = "Reader of field `DMA1_CH6_IM`"]
pub type DMA1_CH6_IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA1_CH6_IM`"]
pub struct DMA1_CH6_IM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1_CH6_IM_W<'a> {
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
#[doc = "Reader of field `DMA1_CH7_IM`"]
pub type DMA1_CH7_IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA1_CH7_IM`"]
pub struct DMA1_CH7_IM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1_CH7_IM_W<'a> {
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
#[doc = "Reader of field `DMA2_CH1_IM`"]
pub type DMA2_CH1_IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA2_CH1_IM`"]
pub struct DMA2_CH1_IM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2_CH1_IM_W<'a> {
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
#[doc = "Reader of field `DMA2_CH2_IM`"]
pub type DMA2_CH2_IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA2_CH2_IM`"]
pub struct DMA2_CH2_IM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2_CH2_IM_W<'a> {
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
#[doc = "Reader of field `DMA2_CH3_IM`"]
pub type DMA2_CH3_IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA2_CH3_IM`"]
pub struct DMA2_CH3_IM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2_CH3_IM_W<'a> {
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
#[doc = "Reader of field `DMA2_CH4_IM`"]
pub type DMA2_CH4_IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA2_CH4_IM`"]
pub struct DMA2_CH4_IM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2_CH4_IM_W<'a> {
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
#[doc = "Reader of field `DMA2_CH5_IM`"]
pub type DMA2_CH5_IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA2_CH5_IM`"]
pub struct DMA2_CH5_IM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2_CH5_IM_W<'a> {
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
#[doc = "Reader of field `DMA2_CH6_IM`"]
pub type DMA2_CH6_IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA2_CH6_IM`"]
pub struct DMA2_CH6_IM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2_CH6_IM_W<'a> {
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
#[doc = "Reader of field `DMA2_CH7_IM`"]
pub type DMA2_CH7_IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA2_CH7_IM`"]
pub struct DMA2_CH7_IM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2_CH7_IM_W<'a> {
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
#[doc = "Reader of field `DMAM_UX1_IM`"]
pub type DMAM_UX1_IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAM_UX1_IM`"]
pub struct DMAM_UX1_IM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAM_UX1_IM_W<'a> {
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
#[doc = "Reader of field `PVM1IM`"]
pub type PVM1IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PVM1IM`"]
pub struct PVM1IM_W<'a> {
    w: &'a mut W,
}
impl<'a> PVM1IM_W<'a> {
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
#[doc = "Reader of field `PVM3IM`"]
pub type PVM3IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PVM3IM`"]
pub struct PVM3IM_W<'a> {
    w: &'a mut W,
}
impl<'a> PVM3IM_W<'a> {
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
#[doc = "Reader of field `PVDIM`"]
pub type PVDIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PVDIM`"]
pub struct PVDIM_W<'a> {
    w: &'a mut W,
}
impl<'a> PVDIM_W<'a> {
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
#[doc = "Reader of field `TSCIM`"]
pub type TSCIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSCIM`"]
pub struct TSCIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TSCIM_W<'a> {
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
#[doc = "Reader of field `LCDIM`"]
pub type LCDIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDIM`"]
pub struct LCDIM_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDIM_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Peripheral DMA1 CH1 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma1_ch1_im(&self) -> DMA1_CH1_IM_R {
        DMA1_CH1_IM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Peripheral DMA1 CH2 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma1_ch2_im(&self) -> DMA1_CH2_IM_R {
        DMA1_CH2_IM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Peripheral DMA1 CH3 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma1_ch3_im(&self) -> DMA1_CH3_IM_R {
        DMA1_CH3_IM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Peripheral DMA1 CH4 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma1_ch4_im(&self) -> DMA1_CH4_IM_R {
        DMA1_CH4_IM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Peripheral DMA1 CH5 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma1_ch5_im(&self) -> DMA1_CH5_IM_R {
        DMA1_CH5_IM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Peripheral DMA1 CH6 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma1_ch6_im(&self) -> DMA1_CH6_IM_R {
        DMA1_CH6_IM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Peripheral DMA1 CH7 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma1_ch7_im(&self) -> DMA1_CH7_IM_R {
        DMA1_CH7_IM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Peripheral DMA2 CH1 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn dma2_ch1_im(&self) -> DMA2_CH1_IM_R {
        DMA2_CH1_IM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Peripheral DMA2 CH2 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn dma2_ch2_im(&self) -> DMA2_CH2_IM_R {
        DMA2_CH2_IM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Peripheral DMA2 CH3 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn dma2_ch3_im(&self) -> DMA2_CH3_IM_R {
        DMA2_CH3_IM_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Peripheral DMA2 CH4 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn dma2_ch4_im(&self) -> DMA2_CH4_IM_R {
        DMA2_CH4_IM_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Peripheral DMA2 CH5 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn dma2_ch5_im(&self) -> DMA2_CH5_IM_R {
        DMA2_CH5_IM_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Peripheral DMA2 CH6 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn dma2_ch6_im(&self) -> DMA2_CH6_IM_R {
        DMA2_CH6_IM_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Peripheral DMA2 CH7 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn dma2_ch7_im(&self) -> DMA2_CH7_IM_R {
        DMA2_CH7_IM_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Peripheral DMAM UX1 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn dmam_ux1_im(&self) -> DMAM_UX1_IM_R {
        DMAM_UX1_IM_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Peripheral PVM1IM interrupt mask to CPU1"]
    #[inline(always)]
    pub fn pvm1im(&self) -> PVM1IM_R {
        PVM1IM_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Peripheral PVM3IM interrupt mask to CPU1"]
    #[inline(always)]
    pub fn pvm3im(&self) -> PVM3IM_R {
        PVM3IM_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Peripheral PVDIM interrupt mask to CPU1"]
    #[inline(always)]
    pub fn pvdim(&self) -> PVDIM_R {
        PVDIM_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Peripheral TSCIM interrupt mask to CPU1"]
    #[inline(always)]
    pub fn tscim(&self) -> TSCIM_R {
        TSCIM_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Peripheral LCDIM interrupt mask to CPU1"]
    #[inline(always)]
    pub fn lcdim(&self) -> LCDIM_R {
        LCDIM_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Peripheral DMA1 CH1 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma1_ch1_im(&mut self) -> DMA1_CH1_IM_W {
        DMA1_CH1_IM_W { w: self }
    }
    #[doc = "Bit 1 - Peripheral DMA1 CH2 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma1_ch2_im(&mut self) -> DMA1_CH2_IM_W {
        DMA1_CH2_IM_W { w: self }
    }
    #[doc = "Bit 2 - Peripheral DMA1 CH3 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma1_ch3_im(&mut self) -> DMA1_CH3_IM_W {
        DMA1_CH3_IM_W { w: self }
    }
    #[doc = "Bit 3 - Peripheral DMA1 CH4 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma1_ch4_im(&mut self) -> DMA1_CH4_IM_W {
        DMA1_CH4_IM_W { w: self }
    }
    #[doc = "Bit 4 - Peripheral DMA1 CH5 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma1_ch5_im(&mut self) -> DMA1_CH5_IM_W {
        DMA1_CH5_IM_W { w: self }
    }
    #[doc = "Bit 5 - Peripheral DMA1 CH6 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma1_ch6_im(&mut self) -> DMA1_CH6_IM_W {
        DMA1_CH6_IM_W { w: self }
    }
    #[doc = "Bit 6 - Peripheral DMA1 CH7 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma1_ch7_im(&mut self) -> DMA1_CH7_IM_W {
        DMA1_CH7_IM_W { w: self }
    }
    #[doc = "Bit 8 - Peripheral DMA2 CH1 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn dma2_ch1_im(&mut self) -> DMA2_CH1_IM_W {
        DMA2_CH1_IM_W { w: self }
    }
    #[doc = "Bit 9 - Peripheral DMA2 CH2 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn dma2_ch2_im(&mut self) -> DMA2_CH2_IM_W {
        DMA2_CH2_IM_W { w: self }
    }
    #[doc = "Bit 10 - Peripheral DMA2 CH3 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn dma2_ch3_im(&mut self) -> DMA2_CH3_IM_W {
        DMA2_CH3_IM_W { w: self }
    }
    #[doc = "Bit 11 - Peripheral DMA2 CH4 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn dma2_ch4_im(&mut self) -> DMA2_CH4_IM_W {
        DMA2_CH4_IM_W { w: self }
    }
    #[doc = "Bit 12 - Peripheral DMA2 CH5 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn dma2_ch5_im(&mut self) -> DMA2_CH5_IM_W {
        DMA2_CH5_IM_W { w: self }
    }
    #[doc = "Bit 13 - Peripheral DMA2 CH6 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn dma2_ch6_im(&mut self) -> DMA2_CH6_IM_W {
        DMA2_CH6_IM_W { w: self }
    }
    #[doc = "Bit 14 - Peripheral DMA2 CH7 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn dma2_ch7_im(&mut self) -> DMA2_CH7_IM_W {
        DMA2_CH7_IM_W { w: self }
    }
    #[doc = "Bit 15 - Peripheral DMAM UX1 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn dmam_ux1_im(&mut self) -> DMAM_UX1_IM_W {
        DMAM_UX1_IM_W { w: self }
    }
    #[doc = "Bit 16 - Peripheral PVM1IM interrupt mask to CPU1"]
    #[inline(always)]
    pub fn pvm1im(&mut self) -> PVM1IM_W {
        PVM1IM_W { w: self }
    }
    #[doc = "Bit 18 - Peripheral PVM3IM interrupt mask to CPU1"]
    #[inline(always)]
    pub fn pvm3im(&mut self) -> PVM3IM_W {
        PVM3IM_W { w: self }
    }
    #[doc = "Bit 20 - Peripheral PVDIM interrupt mask to CPU1"]
    #[inline(always)]
    pub fn pvdim(&mut self) -> PVDIM_W {
        PVDIM_W { w: self }
    }
    #[doc = "Bit 21 - Peripheral TSCIM interrupt mask to CPU1"]
    #[inline(always)]
    pub fn tscim(&mut self) -> TSCIM_W {
        TSCIM_W { w: self }
    }
    #[doc = "Bit 22 - Peripheral LCDIM interrupt mask to CPU1"]
    #[inline(always)]
    pub fn lcdim(&mut self) -> LCDIM_W {
        LCDIM_W { w: self }
    }
}

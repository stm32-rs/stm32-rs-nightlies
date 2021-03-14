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
#[doc = "Reader of field `DMAMUX1IM`"]
pub type DMAMUX1IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAMUX1IM`"]
pub struct DMAMUX1IM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAMUX1IM_W<'a> {
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
#[doc = "Reader of field `DMA2CH7IM`"]
pub type DMA2CH7IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA2CH7IM`"]
pub struct DMA2CH7IM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2CH7IM_W<'a> {
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
#[doc = "Reader of field `DMA2CH6IM`"]
pub type DMA2CH6IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA2CH6IM`"]
pub struct DMA2CH6IM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2CH6IM_W<'a> {
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
#[doc = "Reader of field `DMA2CH5IM`"]
pub type DMA2CH5IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA2CH5IM`"]
pub struct DMA2CH5IM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2CH5IM_W<'a> {
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
#[doc = "Reader of field `DMA2CH4IM`"]
pub type DMA2CH4IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA2CH4IM`"]
pub struct DMA2CH4IM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2CH4IM_W<'a> {
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
#[doc = "Reader of field `DMA2CH3IM`"]
pub type DMA2CH3IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA2CH3IM`"]
pub struct DMA2CH3IM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2CH3IM_W<'a> {
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
#[doc = "Reader of field `DMA2CH2IM`"]
pub type DMA2CH2IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA2CH2IM`"]
pub struct DMA2CH2IM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2CH2IM_W<'a> {
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
#[doc = "Reader of field `DMA2CH1IM`"]
pub type DMA2CH1IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA2CH1IM`"]
pub struct DMA2CH1IM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2CH1IM_W<'a> {
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
#[doc = "Reader of field `DMA1CH7IM`"]
pub type DMA1CH7IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA1CH7IM`"]
pub struct DMA1CH7IM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1CH7IM_W<'a> {
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
#[doc = "Reader of field `DMA1CH6IM`"]
pub type DMA1CH6IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA1CH6IM`"]
pub struct DMA1CH6IM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1CH6IM_W<'a> {
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
#[doc = "Reader of field `DMA1CH5IM`"]
pub type DMA1CH5IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA1CH5IM`"]
pub struct DMA1CH5IM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1CH5IM_W<'a> {
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
#[doc = "Reader of field `DMA1CH4IM`"]
pub type DMA1CH4IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA1CH4IM`"]
pub struct DMA1CH4IM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1CH4IM_W<'a> {
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
#[doc = "Reader of field `DMA1CH3IM`"]
pub type DMA1CH3IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA1CH3IM`"]
pub struct DMA1CH3IM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1CH3IM_W<'a> {
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
#[doc = "Reader of field `DMA1CH2IM`"]
pub type DMA1CH2IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA1CH2IM`"]
pub struct DMA1CH2IM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1CH2IM_W<'a> {
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
#[doc = "Reader of field `DMA1CH1IM`"]
pub type DMA1CH1IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA1CH1IM`"]
pub struct DMA1CH1IM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1CH1IM_W<'a> {
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
impl R {
    #[doc = "Bit 20 - Peripheral PVD interrupt mask to CPU2"]
    #[inline(always)]
    pub fn pvdim(&self) -> PVDIM_R {
        PVDIM_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Peripheral PVM3 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn pvm3im(&self) -> PVM3IM_R {
        PVM3IM_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Peripheral DMAMUX1 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dmamux1im(&self) -> DMAMUX1IM_R {
        DMAMUX1IM_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Peripheral DMA2CH7 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma2ch7im(&self) -> DMA2CH7IM_R {
        DMA2CH7IM_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Peripheral DMA2CH6 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma2ch6im(&self) -> DMA2CH6IM_R {
        DMA2CH6IM_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Peripheral DMA2CH5 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma2ch5im(&self) -> DMA2CH5IM_R {
        DMA2CH5IM_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Peripheral DMA2CH4 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma2ch4im(&self) -> DMA2CH4IM_R {
        DMA2CH4IM_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Peripheral DMA2CH3 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma2ch3im(&self) -> DMA2CH3IM_R {
        DMA2CH3IM_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Peripheral DMA2CH2 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma2ch2im(&self) -> DMA2CH2IM_R {
        DMA2CH2IM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Peripheral DMA2CH1 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma2ch1im(&self) -> DMA2CH1IM_R {
        DMA2CH1IM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Peripheral DMA1CH7 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma1ch7im(&self) -> DMA1CH7IM_R {
        DMA1CH7IM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Peripheral DMA1CH6 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma1ch6im(&self) -> DMA1CH6IM_R {
        DMA1CH6IM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Peripheral DMA1CH5 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma1ch5im(&self) -> DMA1CH5IM_R {
        DMA1CH5IM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Peripheral DMA1CH4 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma1ch4im(&self) -> DMA1CH4IM_R {
        DMA1CH4IM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Peripheral DMA1CH3 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma1ch3im(&self) -> DMA1CH3IM_R {
        DMA1CH3IM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Peripheral DMA1CH2 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma1ch2im(&self) -> DMA1CH2IM_R {
        DMA1CH2IM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Peripheral DMA1CH1 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma1ch1im(&self) -> DMA1CH1IM_R {
        DMA1CH1IM_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 20 - Peripheral PVD interrupt mask to CPU2"]
    #[inline(always)]
    pub fn pvdim(&mut self) -> PVDIM_W {
        PVDIM_W { w: self }
    }
    #[doc = "Bit 18 - Peripheral PVM3 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn pvm3im(&mut self) -> PVM3IM_W {
        PVM3IM_W { w: self }
    }
    #[doc = "Bit 15 - Peripheral DMAMUX1 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dmamux1im(&mut self) -> DMAMUX1IM_W {
        DMAMUX1IM_W { w: self }
    }
    #[doc = "Bit 14 - Peripheral DMA2CH7 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma2ch7im(&mut self) -> DMA2CH7IM_W {
        DMA2CH7IM_W { w: self }
    }
    #[doc = "Bit 13 - Peripheral DMA2CH6 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma2ch6im(&mut self) -> DMA2CH6IM_W {
        DMA2CH6IM_W { w: self }
    }
    #[doc = "Bit 12 - Peripheral DMA2CH5 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma2ch5im(&mut self) -> DMA2CH5IM_W {
        DMA2CH5IM_W { w: self }
    }
    #[doc = "Bit 11 - Peripheral DMA2CH4 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma2ch4im(&mut self) -> DMA2CH4IM_W {
        DMA2CH4IM_W { w: self }
    }
    #[doc = "Bit 10 - Peripheral DMA2CH3 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma2ch3im(&mut self) -> DMA2CH3IM_W {
        DMA2CH3IM_W { w: self }
    }
    #[doc = "Bit 9 - Peripheral DMA2CH2 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma2ch2im(&mut self) -> DMA2CH2IM_W {
        DMA2CH2IM_W { w: self }
    }
    #[doc = "Bit 8 - Peripheral DMA2CH1 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma2ch1im(&mut self) -> DMA2CH1IM_W {
        DMA2CH1IM_W { w: self }
    }
    #[doc = "Bit 6 - Peripheral DMA1CH7 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma1ch7im(&mut self) -> DMA1CH7IM_W {
        DMA1CH7IM_W { w: self }
    }
    #[doc = "Bit 5 - Peripheral DMA1CH6 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma1ch6im(&mut self) -> DMA1CH6IM_W {
        DMA1CH6IM_W { w: self }
    }
    #[doc = "Bit 4 - Peripheral DMA1CH5 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma1ch5im(&mut self) -> DMA1CH5IM_W {
        DMA1CH5IM_W { w: self }
    }
    #[doc = "Bit 3 - Peripheral DMA1CH4 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma1ch4im(&mut self) -> DMA1CH4IM_W {
        DMA1CH4IM_W { w: self }
    }
    #[doc = "Bit 2 - Peripheral DMA1CH3 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma1ch3im(&mut self) -> DMA1CH3IM_W {
        DMA1CH3IM_W { w: self }
    }
    #[doc = "Bit 1 - Peripheral DMA1CH2 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma1ch2im(&mut self) -> DMA1CH2IM_W {
        DMA1CH2IM_W { w: self }
    }
    #[doc = "Bit 0 - Peripheral DMA1CH1 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma1ch1im(&mut self) -> DMA1CH1IM_W {
        DMA1CH1IM_W { w: self }
    }
}

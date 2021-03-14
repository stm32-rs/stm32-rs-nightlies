#[doc = "Reader of register AHB1RSTR"]
pub type R = crate::R<u32, super::AHB1RSTR>;
#[doc = "Writer for register AHB1RSTR"]
pub type W = crate::W<u32, super::AHB1RSTR>;
#[doc = "Register AHB1RSTR `reset()`'s with value 0"]
impl crate::ResetValue for super::AHB1RSTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "DMA1 block reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA1RST_A {
    #[doc = "1: Reset the selected module"]
    RESET = 1,
}
impl From<DMA1RST_A> for bool {
    #[inline(always)]
    fn from(variant: DMA1RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMA1RST`"]
pub type DMA1RST_R = crate::R<bool, DMA1RST_A>;
impl DMA1RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, DMA1RST_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(DMA1RST_A::RESET),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == DMA1RST_A::RESET
    }
}
#[doc = "Write proxy for field `DMA1RST`"]
pub struct DMA1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA1RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(DMA1RST_A::RESET)
    }
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
#[doc = "DMA2 block reset"]
pub type DMA2RST_A = DMA1RST_A;
#[doc = "Reader of field `DMA2RST`"]
pub type DMA2RST_R = crate::R<bool, DMA1RST_A>;
#[doc = "Write proxy for field `DMA2RST`"]
pub struct DMA2RST_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA2RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(DMA1RST_A::RESET)
    }
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
#[doc = "ADC1&2 block reset"]
pub type ADC12RST_A = DMA1RST_A;
#[doc = "Reader of field `ADC12RST`"]
pub type ADC12RST_R = crate::R<bool, DMA1RST_A>;
#[doc = "Write proxy for field `ADC12RST`"]
pub struct ADC12RST_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(DMA1RST_A::RESET)
    }
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
#[doc = "ETH1MAC block reset"]
pub type ETH1MACRST_A = DMA1RST_A;
#[doc = "Reader of field `ETH1MACRST`"]
pub type ETH1MACRST_R = crate::R<bool, DMA1RST_A>;
#[doc = "Write proxy for field `ETH1MACRST`"]
pub struct ETH1MACRST_W<'a> {
    w: &'a mut W,
}
impl<'a> ETH1MACRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETH1MACRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(DMA1RST_A::RESET)
    }
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
#[doc = "USB1OTG block reset"]
pub type USB1OTGRST_A = DMA1RST_A;
#[doc = "Reader of field `USB1OTGRST`"]
pub type USB1OTGRST_R = crate::R<bool, DMA1RST_A>;
#[doc = "Write proxy for field `USB1OTGRST`"]
pub struct USB1OTGRST_W<'a> {
    w: &'a mut W,
}
impl<'a> USB1OTGRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB1OTGRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(DMA1RST_A::RESET)
    }
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
#[doc = "USB2OTG block reset"]
pub type USB2OTGRST_A = DMA1RST_A;
#[doc = "Reader of field `USB2OTGRST`"]
pub type USB2OTGRST_R = crate::R<bool, DMA1RST_A>;
#[doc = "Write proxy for field `USB2OTGRST`"]
pub struct USB2OTGRST_W<'a> {
    w: &'a mut W,
}
impl<'a> USB2OTGRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB2OTGRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(DMA1RST_A::RESET)
    }
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
impl R {
    #[doc = "Bit 0 - DMA1 block reset"]
    #[inline(always)]
    pub fn dma1rst(&self) -> DMA1RST_R {
        DMA1RST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DMA2 block reset"]
    #[inline(always)]
    pub fn dma2rst(&self) -> DMA2RST_R {
        DMA2RST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ADC1&2 block reset"]
    #[inline(always)]
    pub fn adc12rst(&self) -> ADC12RST_R {
        ADC12RST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 15 - ETH1MAC block reset"]
    #[inline(always)]
    pub fn eth1macrst(&self) -> ETH1MACRST_R {
        ETH1MACRST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 25 - USB1OTG block reset"]
    #[inline(always)]
    pub fn usb1otgrst(&self) -> USB1OTGRST_R {
        USB1OTGRST_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 27 - USB2OTG block reset"]
    #[inline(always)]
    pub fn usb2otgrst(&self) -> USB2OTGRST_R {
        USB2OTGRST_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 block reset"]
    #[inline(always)]
    pub fn dma1rst(&mut self) -> DMA1RST_W {
        DMA1RST_W { w: self }
    }
    #[doc = "Bit 1 - DMA2 block reset"]
    #[inline(always)]
    pub fn dma2rst(&mut self) -> DMA2RST_W {
        DMA2RST_W { w: self }
    }
    #[doc = "Bit 5 - ADC1&2 block reset"]
    #[inline(always)]
    pub fn adc12rst(&mut self) -> ADC12RST_W {
        ADC12RST_W { w: self }
    }
    #[doc = "Bit 15 - ETH1MAC block reset"]
    #[inline(always)]
    pub fn eth1macrst(&mut self) -> ETH1MACRST_W {
        ETH1MACRST_W { w: self }
    }
    #[doc = "Bit 25 - USB1OTG block reset"]
    #[inline(always)]
    pub fn usb1otgrst(&mut self) -> USB1OTGRST_W {
        USB1OTGRST_W { w: self }
    }
    #[doc = "Bit 27 - USB2OTG block reset"]
    #[inline(always)]
    pub fn usb2otgrst(&mut self) -> USB2OTGRST_W {
        USB2OTGRST_W { w: self }
    }
}

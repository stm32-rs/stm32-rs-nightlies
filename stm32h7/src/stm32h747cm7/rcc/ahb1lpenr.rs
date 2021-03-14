#[doc = "Reader of register AHB1LPENR"]
pub type R = crate::R<u32, super::AHB1LPENR>;
#[doc = "Writer for register AHB1LPENR"]
pub type W = crate::W<u32, super::AHB1LPENR>;
#[doc = "Register AHB1LPENR `reset()`'s with value 0"]
impl crate::ResetValue for super::AHB1LPENR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "DMA1 Clock Enable During CSleep Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA1LPEN_A {
    #[doc = "0: The selected clock is disabled during csleep mode"]
    DISABLED = 0,
    #[doc = "1: The selected clock is enabled during csleep mode"]
    ENABLED = 1,
}
impl From<DMA1LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMA1LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMA1LPEN`"]
pub type DMA1LPEN_R = crate::R<bool, DMA1LPEN_A>;
impl DMA1LPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA1LPEN_A {
        match self.bits {
            false => DMA1LPEN_A::DISABLED,
            true => DMA1LPEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMA1LPEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMA1LPEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `DMA1LPEN`"]
pub struct DMA1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA1LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA1LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA1LPEN_A::ENABLED)
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
#[doc = "DMA2 Clock Enable During CSleep Mode"]
pub type DMA2LPEN_A = DMA1LPEN_A;
#[doc = "Reader of field `DMA2LPEN`"]
pub type DMA2LPEN_R = crate::R<bool, DMA1LPEN_A>;
#[doc = "Write proxy for field `DMA2LPEN`"]
pub struct DMA2LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA2LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA1LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA1LPEN_A::ENABLED)
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
#[doc = "ADC1/2 Peripheral Clocks Enable During CSleep Mode"]
pub type ADC12LPEN_A = DMA1LPEN_A;
#[doc = "Reader of field `ADC12LPEN`"]
pub type ADC12LPEN_R = crate::R<bool, DMA1LPEN_A>;
#[doc = "Write proxy for field `ADC12LPEN`"]
pub struct ADC12LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA1LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA1LPEN_A::ENABLED)
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
#[doc = "Ethernet MAC bus interface Clock Enable During CSleep Mode"]
pub type ETH1MACLPEN_A = DMA1LPEN_A;
#[doc = "Reader of field `ETH1MACLPEN`"]
pub type ETH1MACLPEN_R = crate::R<bool, DMA1LPEN_A>;
#[doc = "Write proxy for field `ETH1MACLPEN`"]
pub struct ETH1MACLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ETH1MACLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETH1MACLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA1LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA1LPEN_A::ENABLED)
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
#[doc = "Ethernet Transmission Clock Enable During CSleep Mode"]
pub type ETH1TXLPEN_A = DMA1LPEN_A;
#[doc = "Reader of field `ETH1TXLPEN`"]
pub type ETH1TXLPEN_R = crate::R<bool, DMA1LPEN_A>;
#[doc = "Write proxy for field `ETH1TXLPEN`"]
pub struct ETH1TXLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ETH1TXLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETH1TXLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA1LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA1LPEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Ethernet Reception Clock Enable During CSleep Mode"]
pub type ETH1RXLPEN_A = DMA1LPEN_A;
#[doc = "Reader of field `ETH1RXLPEN`"]
pub type ETH1RXLPEN_R = crate::R<bool, DMA1LPEN_A>;
#[doc = "Write proxy for field `ETH1RXLPEN`"]
pub struct ETH1RXLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ETH1RXLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETH1RXLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA1LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA1LPEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "USB1OTG peripheral clock enable during CSleep mode"]
pub type USB1OTGLPEN_A = DMA1LPEN_A;
#[doc = "Reader of field `USB1OTGLPEN`"]
pub type USB1OTGLPEN_R = crate::R<bool, DMA1LPEN_A>;
#[doc = "Write proxy for field `USB1OTGLPEN`"]
pub struct USB1OTGLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USB1OTGLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB1OTGLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA1LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA1LPEN_A::ENABLED)
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
#[doc = "USB_PHY1 clock enable during CSleep mode"]
pub type USB1ULPILPEN_A = DMA1LPEN_A;
#[doc = "Reader of field `USB1ULPILPEN`"]
pub type USB1ULPILPEN_R = crate::R<bool, DMA1LPEN_A>;
#[doc = "Write proxy for field `USB1ULPILPEN`"]
pub struct USB1ULPILPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USB1ULPILPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB1ULPILPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA1LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA1LPEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "USB2OTG peripheral clock enable during CSleep mode"]
pub type USB2OTGLPEN_A = DMA1LPEN_A;
#[doc = "Reader of field `USB2OTGLPEN`"]
pub type USB2OTGLPEN_R = crate::R<bool, DMA1LPEN_A>;
#[doc = "Write proxy for field `USB2OTGLPEN`"]
pub struct USB2OTGLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USB2OTGLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB2OTGLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA1LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA1LPEN_A::ENABLED)
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
#[doc = "USB_PHY2 clocks enable during CSleep mode"]
pub type USB2ULPILPEN_A = DMA1LPEN_A;
#[doc = "Reader of field `USB2ULPILPEN`"]
pub type USB2ULPILPEN_R = crate::R<bool, DMA1LPEN_A>;
#[doc = "Write proxy for field `USB2ULPILPEN`"]
pub struct USB2ULPILPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USB2ULPILPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB2ULPILPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA1LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA1LPEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "ART Clock Enable During CSleep Mode"]
pub type ARTLPEN_A = DMA1LPEN_A;
#[doc = "Reader of field `ARTLPEN`"]
pub type ARTLPEN_R = crate::R<bool, DMA1LPEN_A>;
#[doc = "Write proxy for field `ARTLPEN`"]
pub struct ARTLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ARTLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ARTLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA1LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA1LPEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DMA1 Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn dma1lpen(&self) -> DMA1LPEN_R {
        DMA1LPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DMA2 Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn dma2lpen(&self) -> DMA2LPEN_R {
        DMA2LPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ADC1/2 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn adc12lpen(&self) -> ADC12LPEN_R {
        ADC12LPEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Ethernet MAC bus interface Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn eth1maclpen(&self) -> ETH1MACLPEN_R {
        ETH1MACLPEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Ethernet Transmission Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn eth1txlpen(&self) -> ETH1TXLPEN_R {
        ETH1TXLPEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Ethernet Reception Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn eth1rxlpen(&self) -> ETH1RXLPEN_R {
        ETH1RXLPEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 25 - USB1OTG peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn usb1otglpen(&self) -> USB1OTGLPEN_R {
        USB1OTGLPEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - USB_PHY1 clock enable during CSleep mode"]
    #[inline(always)]
    pub fn usb1ulpilpen(&self) -> USB1ULPILPEN_R {
        USB1ULPILPEN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - USB2OTG peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn usb2otglpen(&self) -> USB2OTGLPEN_R {
        USB2OTGLPEN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - USB_PHY2 clocks enable during CSleep mode"]
    #[inline(always)]
    pub fn usb2ulpilpen(&self) -> USB2ULPILPEN_R {
        USB2ULPILPEN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 14 - ART Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn artlpen(&self) -> ARTLPEN_R {
        ARTLPEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn dma1lpen(&mut self) -> DMA1LPEN_W {
        DMA1LPEN_W { w: self }
    }
    #[doc = "Bit 1 - DMA2 Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn dma2lpen(&mut self) -> DMA2LPEN_W {
        DMA2LPEN_W { w: self }
    }
    #[doc = "Bit 5 - ADC1/2 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn adc12lpen(&mut self) -> ADC12LPEN_W {
        ADC12LPEN_W { w: self }
    }
    #[doc = "Bit 15 - Ethernet MAC bus interface Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn eth1maclpen(&mut self) -> ETH1MACLPEN_W {
        ETH1MACLPEN_W { w: self }
    }
    #[doc = "Bit 16 - Ethernet Transmission Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn eth1txlpen(&mut self) -> ETH1TXLPEN_W {
        ETH1TXLPEN_W { w: self }
    }
    #[doc = "Bit 17 - Ethernet Reception Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn eth1rxlpen(&mut self) -> ETH1RXLPEN_W {
        ETH1RXLPEN_W { w: self }
    }
    #[doc = "Bit 25 - USB1OTG peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn usb1otglpen(&mut self) -> USB1OTGLPEN_W {
        USB1OTGLPEN_W { w: self }
    }
    #[doc = "Bit 26 - USB_PHY1 clock enable during CSleep mode"]
    #[inline(always)]
    pub fn usb1ulpilpen(&mut self) -> USB1ULPILPEN_W {
        USB1ULPILPEN_W { w: self }
    }
    #[doc = "Bit 27 - USB2OTG peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn usb2otglpen(&mut self) -> USB2OTGLPEN_W {
        USB2OTGLPEN_W { w: self }
    }
    #[doc = "Bit 28 - USB_PHY2 clocks enable during CSleep mode"]
    #[inline(always)]
    pub fn usb2ulpilpen(&mut self) -> USB2ULPILPEN_W {
        USB2ULPILPEN_W { w: self }
    }
    #[doc = "Bit 14 - ART Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn artlpen(&mut self) -> ARTLPEN_W {
        ARTLPEN_W { w: self }
    }
}

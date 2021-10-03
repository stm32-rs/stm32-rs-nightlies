#[doc = "Register `AHB1LPENR` reader"]
pub struct R(crate::R<AHB1LPENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB1LPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB1LPENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB1LPENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB1LPENR` writer"]
pub struct W(crate::W<AHB1LPENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB1LPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<AHB1LPENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB1LPENR_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `DMA1LPEN` reader - DMA1 Clock Enable During CSleep Mode"]
pub struct DMA1LPEN_R(crate::FieldReader<bool, DMA1LPEN_A>);
impl DMA1LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA1LPEN_R(crate::FieldReader::new(bits))
    }
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
        **self == DMA1LPEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DMA1LPEN_A::ENABLED
    }
}
impl core::ops::Deref for DMA1LPEN_R {
    type Target = crate::FieldReader<bool, DMA1LPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA1LPEN` writer - DMA1 Clock Enable During CSleep Mode"]
pub struct DMA1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA1LPEN_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "DMA2 Clock Enable During CSleep Mode"]
pub type DMA2LPEN_A = DMA1LPEN_A;
#[doc = "Field `DMA2LPEN` reader - DMA2 Clock Enable During CSleep Mode"]
pub type DMA2LPEN_R = DMA1LPEN_R;
#[doc = "Field `DMA2LPEN` writer - DMA2 Clock Enable During CSleep Mode"]
pub struct DMA2LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA2LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA2LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA2LPEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "ADC1/2 Peripheral Clocks Enable During CSleep Mode"]
pub type ADC12LPEN_A = DMA1LPEN_A;
#[doc = "Field `ADC12LPEN` reader - ADC1/2 Peripheral Clocks Enable During CSleep Mode"]
pub type ADC12LPEN_R = DMA1LPEN_R;
#[doc = "Field `ADC12LPEN` writer - ADC1/2 Peripheral Clocks Enable During CSleep Mode"]
pub struct ADC12LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADC12LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADC12LPEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Ethernet MAC bus interface Clock Enable During CSleep Mode"]
pub type ETH1MACLPEN_A = DMA1LPEN_A;
#[doc = "Field `ETH1MACLPEN` reader - Ethernet MAC bus interface Clock Enable During CSleep Mode"]
pub type ETH1MACLPEN_R = DMA1LPEN_R;
#[doc = "Field `ETH1MACLPEN` writer - Ethernet MAC bus interface Clock Enable During CSleep Mode"]
pub struct ETH1MACLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ETH1MACLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETH1MACLPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ETH1MACLPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ETH1MACLPEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Ethernet Transmission Clock Enable During CSleep Mode"]
pub type ETH1TXLPEN_A = DMA1LPEN_A;
#[doc = "Field `ETH1TXLPEN` reader - Ethernet Transmission Clock Enable During CSleep Mode"]
pub type ETH1TXLPEN_R = DMA1LPEN_R;
#[doc = "Field `ETH1TXLPEN` writer - Ethernet Transmission Clock Enable During CSleep Mode"]
pub struct ETH1TXLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ETH1TXLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETH1TXLPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ETH1TXLPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ETH1TXLPEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Ethernet Reception Clock Enable During CSleep Mode"]
pub type ETH1RXLPEN_A = DMA1LPEN_A;
#[doc = "Field `ETH1RXLPEN` reader - Ethernet Reception Clock Enable During CSleep Mode"]
pub type ETH1RXLPEN_R = DMA1LPEN_R;
#[doc = "Field `ETH1RXLPEN` writer - Ethernet Reception Clock Enable During CSleep Mode"]
pub struct ETH1RXLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ETH1RXLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETH1RXLPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ETH1RXLPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ETH1RXLPEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "USB1OTG peripheral clock enable during CSleep mode"]
pub type USB1OTGLPEN_A = DMA1LPEN_A;
#[doc = "Field `USB1OTGLPEN` reader - USB1OTG peripheral clock enable during CSleep mode"]
pub type USB1OTGLPEN_R = DMA1LPEN_R;
#[doc = "Field `USB1OTGLPEN` writer - USB1OTG peripheral clock enable during CSleep mode"]
pub struct USB1OTGLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USB1OTGLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB1OTGLPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USB1OTGLPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USB1OTGLPEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "USB_PHY1 clock enable during CSleep mode"]
pub type USB1ULPILPEN_A = DMA1LPEN_A;
#[doc = "Field `USB1ULPILPEN` reader - USB_PHY1 clock enable during CSleep mode"]
pub type USB1ULPILPEN_R = DMA1LPEN_R;
#[doc = "Field `USB1ULPILPEN` writer - USB_PHY1 clock enable during CSleep mode"]
pub struct USB1ULPILPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USB1ULPILPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB1ULPILPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USB1ULPILPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USB1ULPILPEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC AHB1 Sleep Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb1lpenr](index.html) module"]
pub struct AHB1LPENR_SPEC;
impl crate::RegisterSpec for AHB1LPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb1lpenr::R](R) reader structure"]
impl crate::Readable for AHB1LPENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb1lpenr::W](W) writer structure"]
impl crate::Writable for AHB1LPENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHB1LPENR to value 0"]
impl crate::Resettable for AHB1LPENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

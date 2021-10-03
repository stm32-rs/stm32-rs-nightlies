#[doc = "Register `C1_AHB1ENR` reader"]
pub struct R(crate::R<C1_AHB1ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1_AHB1ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1_AHB1ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1_AHB1ENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C1_AHB1ENR` writer"]
pub struct W(crate::W<C1_AHB1ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C1_AHB1ENR_SPEC>;
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
impl From<crate::W<C1_AHB1ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C1_AHB1ENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "DMA1 Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA1EN_A {
    #[doc = "0: The selected clock is disabled"]
    DISABLED = 0,
    #[doc = "1: The selected clock is enabled"]
    ENABLED = 1,
}
impl From<DMA1EN_A> for bool {
    #[inline(always)]
    fn from(variant: DMA1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA1EN` reader - DMA1 Clock Enable"]
pub struct DMA1EN_R(crate::FieldReader<bool, DMA1EN_A>);
impl DMA1EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA1EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA1EN_A {
        match self.bits {
            false => DMA1EN_A::DISABLED,
            true => DMA1EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DMA1EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DMA1EN_A::ENABLED
    }
}
impl core::ops::Deref for DMA1EN_R {
    type Target = crate::FieldReader<bool, DMA1EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA1EN` writer - DMA1 Clock Enable"]
pub struct DMA1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA1EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA1EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA1EN_A::ENABLED)
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
#[doc = "DMA2 Clock Enable"]
pub type DMA2EN_A = DMA1EN_A;
#[doc = "Field `DMA2EN` reader - DMA2 Clock Enable"]
pub type DMA2EN_R = DMA1EN_R;
#[doc = "Field `DMA2EN` writer - DMA2 Clock Enable"]
pub struct DMA2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA2EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA2EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA2EN_A::ENABLED)
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
#[doc = "ADC1/2 Peripheral Clocks Enable"]
pub type ADC12EN_A = DMA1EN_A;
#[doc = "Field `ADC12EN` reader - ADC1/2 Peripheral Clocks Enable"]
pub type ADC12EN_R = DMA1EN_R;
#[doc = "Field `ADC12EN` writer - ADC1/2 Peripheral Clocks Enable"]
pub struct ADC12EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADC12EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADC12EN_A::ENABLED)
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
#[doc = "Ethernet MAC bus interface Clock Enable"]
pub type ETH1MACEN_A = DMA1EN_A;
#[doc = "Field `ETH1MACEN` reader - Ethernet MAC bus interface Clock Enable"]
pub type ETH1MACEN_R = DMA1EN_R;
#[doc = "Field `ETH1MACEN` writer - Ethernet MAC bus interface Clock Enable"]
pub struct ETH1MACEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ETH1MACEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETH1MACEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ETH1MACEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ETH1MACEN_A::ENABLED)
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
#[doc = "Ethernet Transmission Clock Enable"]
pub type ETH1TXEN_A = DMA1EN_A;
#[doc = "Field `ETH1TXEN` reader - Ethernet Transmission Clock Enable"]
pub type ETH1TXEN_R = DMA1EN_R;
#[doc = "Field `ETH1TXEN` writer - Ethernet Transmission Clock Enable"]
pub struct ETH1TXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ETH1TXEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETH1TXEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ETH1TXEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ETH1TXEN_A::ENABLED)
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
#[doc = "Ethernet Reception Clock Enable"]
pub type ETH1RXEN_A = DMA1EN_A;
#[doc = "Field `ETH1RXEN` reader - Ethernet Reception Clock Enable"]
pub type ETH1RXEN_R = DMA1EN_R;
#[doc = "Field `ETH1RXEN` writer - Ethernet Reception Clock Enable"]
pub struct ETH1RXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ETH1RXEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETH1RXEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ETH1RXEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ETH1RXEN_A::ENABLED)
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
#[doc = "USB1OTG Peripheral Clocks Enable"]
pub type USB1OTGEN_A = DMA1EN_A;
#[doc = "Field `USB1OTGEN` reader - USB1OTG Peripheral Clocks Enable"]
pub type USB1OTGEN_R = DMA1EN_R;
#[doc = "Field `USB1OTGEN` writer - USB1OTG Peripheral Clocks Enable"]
pub struct USB1OTGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USB1OTGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB1OTGEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USB1OTGEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USB1OTGEN_A::ENABLED)
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
#[doc = "USB_PHY1 Clocks Enable"]
pub type USB1ULPIEN_A = DMA1EN_A;
#[doc = "Field `USB1ULPIEN` reader - USB_PHY1 Clocks Enable"]
pub type USB1ULPIEN_R = DMA1EN_R;
#[doc = "Field `USB1ULPIEN` writer - USB_PHY1 Clocks Enable"]
pub struct USB1ULPIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USB1ULPIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB1ULPIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USB1ULPIEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USB1ULPIEN_A::ENABLED)
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
#[doc = "USB2OTG Peripheral Clocks Enable"]
pub type USB2OTGEN_A = DMA1EN_A;
#[doc = "Field `USB2OTGEN` reader - USB2OTG Peripheral Clocks Enable"]
pub type USB2OTGEN_R = DMA1EN_R;
#[doc = "Field `USB2OTGEN` writer - USB2OTG Peripheral Clocks Enable"]
pub struct USB2OTGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USB2OTGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB2OTGEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USB2OTGEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USB2OTGEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "USB_PHY2 Clocks Enable"]
pub type USB2ULPIEN_A = DMA1EN_A;
#[doc = "Field `USB2ULPIEN` reader - USB_PHY2 Clocks Enable"]
pub type USB2ULPIEN_R = DMA1EN_R;
#[doc = "Field `USB2ULPIEN` writer - USB_PHY2 Clocks Enable"]
pub struct USB2ULPIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USB2ULPIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB2ULPIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USB2ULPIEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USB2ULPIEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DMA1 Clock Enable"]
    #[inline(always)]
    pub fn dma1en(&self) -> DMA1EN_R {
        DMA1EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DMA2 Clock Enable"]
    #[inline(always)]
    pub fn dma2en(&self) -> DMA2EN_R {
        DMA2EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ADC1/2 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn adc12en(&self) -> ADC12EN_R {
        ADC12EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Ethernet MAC bus interface Clock Enable"]
    #[inline(always)]
    pub fn eth1macen(&self) -> ETH1MACEN_R {
        ETH1MACEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Ethernet Transmission Clock Enable"]
    #[inline(always)]
    pub fn eth1txen(&self) -> ETH1TXEN_R {
        ETH1TXEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Ethernet Reception Clock Enable"]
    #[inline(always)]
    pub fn eth1rxen(&self) -> ETH1RXEN_R {
        ETH1RXEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 25 - USB1OTG Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn usb1otgen(&self) -> USB1OTGEN_R {
        USB1OTGEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - USB_PHY1 Clocks Enable"]
    #[inline(always)]
    pub fn usb1ulpien(&self) -> USB1ULPIEN_R {
        USB1ULPIEN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - USB2OTG Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn usb2otgen(&self) -> USB2OTGEN_R {
        USB2OTGEN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - USB_PHY2 Clocks Enable"]
    #[inline(always)]
    pub fn usb2ulpien(&self) -> USB2ULPIEN_R {
        USB2ULPIEN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 Clock Enable"]
    #[inline(always)]
    pub fn dma1en(&mut self) -> DMA1EN_W {
        DMA1EN_W { w: self }
    }
    #[doc = "Bit 1 - DMA2 Clock Enable"]
    #[inline(always)]
    pub fn dma2en(&mut self) -> DMA2EN_W {
        DMA2EN_W { w: self }
    }
    #[doc = "Bit 5 - ADC1/2 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn adc12en(&mut self) -> ADC12EN_W {
        ADC12EN_W { w: self }
    }
    #[doc = "Bit 15 - Ethernet MAC bus interface Clock Enable"]
    #[inline(always)]
    pub fn eth1macen(&mut self) -> ETH1MACEN_W {
        ETH1MACEN_W { w: self }
    }
    #[doc = "Bit 16 - Ethernet Transmission Clock Enable"]
    #[inline(always)]
    pub fn eth1txen(&mut self) -> ETH1TXEN_W {
        ETH1TXEN_W { w: self }
    }
    #[doc = "Bit 17 - Ethernet Reception Clock Enable"]
    #[inline(always)]
    pub fn eth1rxen(&mut self) -> ETH1RXEN_W {
        ETH1RXEN_W { w: self }
    }
    #[doc = "Bit 25 - USB1OTG Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn usb1otgen(&mut self) -> USB1OTGEN_W {
        USB1OTGEN_W { w: self }
    }
    #[doc = "Bit 26 - USB_PHY1 Clocks Enable"]
    #[inline(always)]
    pub fn usb1ulpien(&mut self) -> USB1ULPIEN_W {
        USB1ULPIEN_W { w: self }
    }
    #[doc = "Bit 27 - USB2OTG Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn usb2otgen(&mut self) -> USB2OTGEN_W {
        USB2OTGEN_W { w: self }
    }
    #[doc = "Bit 28 - USB_PHY2 Clocks Enable"]
    #[inline(always)]
    pub fn usb2ulpien(&mut self) -> USB2ULPIEN_W {
        USB2ULPIEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC AHB1 Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1_ahb1enr](index.html) module"]
pub struct C1_AHB1ENR_SPEC;
impl crate::RegisterSpec for C1_AHB1ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c1_ahb1enr::R](R) reader structure"]
impl crate::Readable for C1_AHB1ENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c1_ahb1enr::W](W) writer structure"]
impl crate::Writable for C1_AHB1ENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C1_AHB1ENR to value 0"]
impl crate::Resettable for C1_AHB1ENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

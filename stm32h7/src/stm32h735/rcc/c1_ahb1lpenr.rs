#[doc = "Register `C1_AHB1LPENR` reader"]
pub type R = crate::R<C1_AHB1LPENRrs>;
#[doc = "Register `C1_AHB1LPENR` writer"]
pub type W = crate::W<C1_AHB1LPENRrs>;
#[doc = "DMA1 Clock Enable During CSleep Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA1LPEN {
    #[doc = "0: The selected clock is disabled during csleep mode"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled during csleep mode"]
    Enabled = 1,
}
impl From<DMA1LPEN> for bool {
    #[inline(always)]
    fn from(variant: DMA1LPEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA1LPEN` reader - DMA1 Clock Enable During CSleep Mode"]
pub type DMA1LPEN_R = crate::BitReader<DMA1LPEN>;
impl DMA1LPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA1LPEN {
        match self.bits {
            false => DMA1LPEN::Disabled,
            true => DMA1LPEN::Enabled,
        }
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMA1LPEN::Disabled
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMA1LPEN::Enabled
    }
}
#[doc = "Field `DMA1LPEN` writer - DMA1 Clock Enable During CSleep Mode"]
pub type DMA1LPEN_W<'a, REG> = crate::BitWriter<'a, REG, DMA1LPEN>;
impl<'a, REG> DMA1LPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1LPEN::Disabled)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1LPEN::Enabled)
    }
}
#[doc = "Field `DMA2LPEN` reader - DMA2 Clock Enable During CSleep Mode"]
pub use DMA1LPEN_R as DMA2LPEN_R;
#[doc = "Field `ADC12LPEN` reader - ADC1/2 Peripheral Clocks Enable During CSleep Mode"]
pub use DMA1LPEN_R as ADC12LPEN_R;
#[doc = "Field `ETH1MACLPEN` reader - Ethernet MAC bus interface Clock Enable During CSleep Mode"]
pub use DMA1LPEN_R as ETH1MACLPEN_R;
#[doc = "Field `ETH1TXLPEN` reader - Ethernet Transmission Clock Enable During CSleep Mode"]
pub use DMA1LPEN_R as ETH1TXLPEN_R;
#[doc = "Field `ETH1RXLPEN` reader - Ethernet Reception Clock Enable During CSleep Mode"]
pub use DMA1LPEN_R as ETH1RXLPEN_R;
#[doc = "Field `USB1OTGLPEN` reader - USB1OTG peripheral clock enable during CSleep mode"]
pub use DMA1LPEN_R as USB1OTGLPEN_R;
#[doc = "Field `USB1ULPILPEN` reader - USB_PHY1 clock enable during CSleep mode"]
pub use DMA1LPEN_R as USB1ULPILPEN_R;
#[doc = "Field `DMA2LPEN` writer - DMA2 Clock Enable During CSleep Mode"]
pub use DMA1LPEN_W as DMA2LPEN_W;
#[doc = "Field `ADC12LPEN` writer - ADC1/2 Peripheral Clocks Enable During CSleep Mode"]
pub use DMA1LPEN_W as ADC12LPEN_W;
#[doc = "Field `ETH1MACLPEN` writer - Ethernet MAC bus interface Clock Enable During CSleep Mode"]
pub use DMA1LPEN_W as ETH1MACLPEN_W;
#[doc = "Field `ETH1TXLPEN` writer - Ethernet Transmission Clock Enable During CSleep Mode"]
pub use DMA1LPEN_W as ETH1TXLPEN_W;
#[doc = "Field `ETH1RXLPEN` writer - Ethernet Reception Clock Enable During CSleep Mode"]
pub use DMA1LPEN_W as ETH1RXLPEN_W;
#[doc = "Field `USB1OTGLPEN` writer - USB1OTG peripheral clock enable during CSleep mode"]
pub use DMA1LPEN_W as USB1OTGLPEN_W;
#[doc = "Field `USB1ULPILPEN` writer - USB_PHY1 clock enable during CSleep mode"]
pub use DMA1LPEN_W as USB1ULPILPEN_W;
impl R {
    #[doc = "Bit 0 - DMA1 Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn dma1lpen(&self) -> DMA1LPEN_R {
        DMA1LPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA2 Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn dma2lpen(&self) -> DMA2LPEN_R {
        DMA2LPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC1/2 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn adc12lpen(&self) -> ADC12LPEN_R {
        ADC12LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 15 - Ethernet MAC bus interface Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn eth1maclpen(&self) -> ETH1MACLPEN_R {
        ETH1MACLPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Ethernet Transmission Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn eth1txlpen(&self) -> ETH1TXLPEN_R {
        ETH1TXLPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Ethernet Reception Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn eth1rxlpen(&self) -> ETH1RXLPEN_R {
        ETH1RXLPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 25 - USB1OTG peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn usb1otglpen(&self) -> USB1OTGLPEN_R {
        USB1OTGLPEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - USB_PHY1 clock enable during CSleep mode"]
    #[inline(always)]
    pub fn usb1ulpilpen(&self) -> USB1ULPILPEN_R {
        USB1ULPILPEN_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 Clock Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dma1lpen(&mut self) -> DMA1LPEN_W<C1_AHB1LPENRrs> {
        DMA1LPEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA2 Clock Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dma2lpen(&mut self) -> DMA2LPEN_W<C1_AHB1LPENRrs> {
        DMA2LPEN_W::new(self, 1)
    }
    #[doc = "Bit 5 - ADC1/2 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn adc12lpen(&mut self) -> ADC12LPEN_W<C1_AHB1LPENRrs> {
        ADC12LPEN_W::new(self, 5)
    }
    #[doc = "Bit 15 - Ethernet MAC bus interface Clock Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn eth1maclpen(&mut self) -> ETH1MACLPEN_W<C1_AHB1LPENRrs> {
        ETH1MACLPEN_W::new(self, 15)
    }
    #[doc = "Bit 16 - Ethernet Transmission Clock Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn eth1txlpen(&mut self) -> ETH1TXLPEN_W<C1_AHB1LPENRrs> {
        ETH1TXLPEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - Ethernet Reception Clock Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn eth1rxlpen(&mut self) -> ETH1RXLPEN_W<C1_AHB1LPENRrs> {
        ETH1RXLPEN_W::new(self, 17)
    }
    #[doc = "Bit 25 - USB1OTG peripheral clock enable during CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn usb1otglpen(&mut self) -> USB1OTGLPEN_W<C1_AHB1LPENRrs> {
        USB1OTGLPEN_W::new(self, 25)
    }
    #[doc = "Bit 26 - USB_PHY1 clock enable during CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn usb1ulpilpen(&mut self) -> USB1ULPILPEN_W<C1_AHB1LPENRrs> {
        USB1ULPILPEN_W::new(self, 26)
    }
}
#[doc = "RCC AHB1 Sleep Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1_ahb1lpenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1_ahb1lpenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C1_AHB1LPENRrs;
impl crate::RegisterSpec for C1_AHB1LPENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c1_ahb1lpenr::R`](R) reader structure"]
impl crate::Readable for C1_AHB1LPENRrs {}
#[doc = "`write(|w| ..)` method takes [`c1_ahb1lpenr::W`](W) writer structure"]
impl crate::Writable for C1_AHB1LPENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C1_AHB1LPENR to value 0"]
impl crate::Resettable for C1_AHB1LPENRrs {
    const RESET_VALUE: u32 = 0;
}

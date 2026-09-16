///Register `C1_AHB1LPENR` reader
pub type R = crate::R<C1_AHB1LPENRrs>;
///Register `C1_AHB1LPENR` writer
pub type W = crate::W<C1_AHB1LPENRrs>;
/**DMA1 Clock Enable During CSleep Mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA1LPEN {
    ///0: The selected clock is disabled during csleep mode
    Disabled = 0,
    ///1: The selected clock is enabled during csleep mode
    Enabled = 1,
}
impl From<DMA1LPEN> for bool {
    #[inline(always)]
    fn from(variant: DMA1LPEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DMA1LPEN` reader - DMA1 Clock Enable During CSleep Mode
pub type DMA1LPEN_R = crate::BitReader<DMA1LPEN>;
impl DMA1LPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMA1LPEN {
        match self.bits {
            false => DMA1LPEN::Disabled,
            true => DMA1LPEN::Enabled,
        }
    }
    ///The selected clock is disabled during csleep mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMA1LPEN::Disabled
    }
    ///The selected clock is enabled during csleep mode
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMA1LPEN::Enabled
    }
}
///Field `DMA1LPEN` writer - DMA1 Clock Enable During CSleep Mode
pub type DMA1LPEN_W<'a, REG> = crate::BitWriter<'a, REG, DMA1LPEN>;
impl<'a, REG> DMA1LPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected clock is disabled during csleep mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1LPEN::Disabled)
    }
    ///The selected clock is enabled during csleep mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1LPEN::Enabled)
    }
}
///Field `DMA2LPEN` reader - DMA2 Clock Enable During CSleep Mode
pub use DMA1LPEN_R as DMA2LPEN_R;
///Field `ADC12LPEN` reader - ADC1/2 Peripheral Clocks Enable During CSleep Mode
pub use DMA1LPEN_R as ADC12LPEN_R;
///Field `ARTLPEN` reader - ART Clock Enable During CSleep Mode
pub use DMA1LPEN_R as ARTLPEN_R;
///Field `ETH1MACLPEN` reader - Ethernet MAC bus interface Clock Enable During CSleep Mode
pub use DMA1LPEN_R as ETH1MACLPEN_R;
///Field `ETH1TXLPEN` reader - Ethernet Transmission Clock Enable During CSleep Mode
pub use DMA1LPEN_R as ETH1TXLPEN_R;
///Field `ETH1RXLPEN` reader - Ethernet Reception Clock Enable During CSleep Mode
pub use DMA1LPEN_R as ETH1RXLPEN_R;
///Field `USB1OTGLPEN` reader - USB1OTG peripheral clock enable during CSleep mode
pub use DMA1LPEN_R as USB1OTGLPEN_R;
///Field `USB1OTGULPILPEN` reader - USB_PHY1 clock enable during CSleep mode
pub use DMA1LPEN_R as USB1OTGULPILPEN_R;
///Field `USB2OTGLPEN` reader - USB2OTG peripheral clock enable during CSleep mode
pub use DMA1LPEN_R as USB2OTGLPEN_R;
///Field `USB2ULPILPEN` reader - USB_PHY2 clocks enable during CSleep mode
pub use DMA1LPEN_R as USB2ULPILPEN_R;
///Field `DMA2LPEN` writer - DMA2 Clock Enable During CSleep Mode
pub use DMA1LPEN_W as DMA2LPEN_W;
///Field `ADC12LPEN` writer - ADC1/2 Peripheral Clocks Enable During CSleep Mode
pub use DMA1LPEN_W as ADC12LPEN_W;
///Field `ARTLPEN` writer - ART Clock Enable During CSleep Mode
pub use DMA1LPEN_W as ARTLPEN_W;
///Field `ETH1MACLPEN` writer - Ethernet MAC bus interface Clock Enable During CSleep Mode
pub use DMA1LPEN_W as ETH1MACLPEN_W;
///Field `ETH1TXLPEN` writer - Ethernet Transmission Clock Enable During CSleep Mode
pub use DMA1LPEN_W as ETH1TXLPEN_W;
///Field `ETH1RXLPEN` writer - Ethernet Reception Clock Enable During CSleep Mode
pub use DMA1LPEN_W as ETH1RXLPEN_W;
///Field `USB1OTGLPEN` writer - USB1OTG peripheral clock enable during CSleep mode
pub use DMA1LPEN_W as USB1OTGLPEN_W;
///Field `USB1OTGULPILPEN` writer - USB_PHY1 clock enable during CSleep mode
pub use DMA1LPEN_W as USB1OTGULPILPEN_W;
///Field `USB2OTGLPEN` writer - USB2OTG peripheral clock enable during CSleep mode
pub use DMA1LPEN_W as USB2OTGLPEN_W;
///Field `USB2ULPILPEN` writer - USB_PHY2 clocks enable during CSleep mode
pub use DMA1LPEN_W as USB2ULPILPEN_W;
impl R {
    ///Bit 0 - DMA1 Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn dma1lpen(&self) -> DMA1LPEN_R {
        DMA1LPEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMA2 Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn dma2lpen(&self) -> DMA2LPEN_R {
        DMA2LPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 5 - ADC1/2 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn adc12lpen(&self) -> ADC12LPEN_R {
        ADC12LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 14 - ART Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn artlpen(&self) -> ARTLPEN_R {
        ARTLPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Ethernet MAC bus interface Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn eth1maclpen(&self) -> ETH1MACLPEN_R {
        ETH1MACLPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Ethernet Transmission Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn eth1txlpen(&self) -> ETH1TXLPEN_R {
        ETH1TXLPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Ethernet Reception Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn eth1rxlpen(&self) -> ETH1RXLPEN_R {
        ETH1RXLPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 25 - USB1OTG peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn usb1otglpen(&self) -> USB1OTGLPEN_R {
        USB1OTGLPEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - USB_PHY1 clock enable during CSleep mode
    #[inline(always)]
    pub fn usb1otgulpilpen(&self) -> USB1OTGULPILPEN_R {
        USB1OTGULPILPEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - USB2OTG peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn usb2otglpen(&self) -> USB2OTGLPEN_R {
        USB2OTGLPEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - USB_PHY2 clocks enable during CSleep mode
    #[inline(always)]
    pub fn usb2ulpilpen(&self) -> USB2ULPILPEN_R {
        USB2ULPILPEN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C1_AHB1LPENR")
            .field("dma1lpen", &self.dma1lpen())
            .field("dma2lpen", &self.dma2lpen())
            .field("adc12lpen", &self.adc12lpen())
            .field("eth1maclpen", &self.eth1maclpen())
            .field("eth1txlpen", &self.eth1txlpen())
            .field("eth1rxlpen", &self.eth1rxlpen())
            .field("usb1otglpen", &self.usb1otglpen())
            .field("usb1otgulpilpen", &self.usb1otgulpilpen())
            .field("usb2otglpen", &self.usb2otglpen())
            .field("usb2ulpilpen", &self.usb2ulpilpen())
            .field("artlpen", &self.artlpen())
            .finish()
    }
}
impl W {
    ///Bit 0 - DMA1 Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn dma1lpen(&mut self) -> DMA1LPEN_W<'_, C1_AHB1LPENRrs> {
        DMA1LPEN_W::new(self, 0)
    }
    ///Bit 1 - DMA2 Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn dma2lpen(&mut self) -> DMA2LPEN_W<'_, C1_AHB1LPENRrs> {
        DMA2LPEN_W::new(self, 1)
    }
    ///Bit 5 - ADC1/2 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn adc12lpen(&mut self) -> ADC12LPEN_W<'_, C1_AHB1LPENRrs> {
        ADC12LPEN_W::new(self, 5)
    }
    ///Bit 14 - ART Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn artlpen(&mut self) -> ARTLPEN_W<'_, C1_AHB1LPENRrs> {
        ARTLPEN_W::new(self, 14)
    }
    ///Bit 15 - Ethernet MAC bus interface Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn eth1maclpen(&mut self) -> ETH1MACLPEN_W<'_, C1_AHB1LPENRrs> {
        ETH1MACLPEN_W::new(self, 15)
    }
    ///Bit 16 - Ethernet Transmission Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn eth1txlpen(&mut self) -> ETH1TXLPEN_W<'_, C1_AHB1LPENRrs> {
        ETH1TXLPEN_W::new(self, 16)
    }
    ///Bit 17 - Ethernet Reception Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn eth1rxlpen(&mut self) -> ETH1RXLPEN_W<'_, C1_AHB1LPENRrs> {
        ETH1RXLPEN_W::new(self, 17)
    }
    ///Bit 25 - USB1OTG peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn usb1otglpen(&mut self) -> USB1OTGLPEN_W<'_, C1_AHB1LPENRrs> {
        USB1OTGLPEN_W::new(self, 25)
    }
    ///Bit 26 - USB_PHY1 clock enable during CSleep mode
    #[inline(always)]
    pub fn usb1otgulpilpen(&mut self) -> USB1OTGULPILPEN_W<'_, C1_AHB1LPENRrs> {
        USB1OTGULPILPEN_W::new(self, 26)
    }
    ///Bit 27 - USB2OTG peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn usb2otglpen(&mut self) -> USB2OTGLPEN_W<'_, C1_AHB1LPENRrs> {
        USB2OTGLPEN_W::new(self, 27)
    }
    ///Bit 28 - USB_PHY2 clocks enable during CSleep mode
    #[inline(always)]
    pub fn usb2ulpilpen(&mut self) -> USB2ULPILPEN_W<'_, C1_AHB1LPENRrs> {
        USB2ULPILPEN_W::new(self, 28)
    }
}
/**RCC AHB1 Sleep Clock Register

You can [`read`](crate::Reg::read) this register and get [`c1_ahb1lpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1_ahb1lpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#RCC:C1_AHB1LPENR)*/
pub struct C1_AHB1LPENRrs;
impl crate::RegisterSpec for C1_AHB1LPENRrs {
    type Ux = u32;
}
///`read()` method returns [`c1_ahb1lpenr::R`](R) reader structure
impl crate::Readable for C1_AHB1LPENRrs {}
///`write(|w| ..)` method takes [`c1_ahb1lpenr::W`](W) writer structure
impl crate::Writable for C1_AHB1LPENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C1_AHB1LPENR to value 0
impl crate::Resettable for C1_AHB1LPENRrs {}

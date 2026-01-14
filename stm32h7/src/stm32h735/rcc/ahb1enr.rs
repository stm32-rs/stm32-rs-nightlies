///Register `AHB1ENR` reader
pub type R = crate::R<AHB1ENRrs>;
///Register `AHB1ENR` writer
pub type W = crate::W<AHB1ENRrs>;
/**DMA1 Clock Enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA1EN {
    ///0: The selected clock is disabled
    Disabled = 0,
    ///1: The selected clock is enabled
    Enabled = 1,
}
impl From<DMA1EN> for bool {
    #[inline(always)]
    fn from(variant: DMA1EN) -> Self {
        variant as u8 != 0
    }
}
///Field `DMA1EN` reader - DMA1 Clock Enable
pub type DMA1EN_R = crate::BitReader<DMA1EN>;
impl DMA1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMA1EN {
        match self.bits {
            false => DMA1EN::Disabled,
            true => DMA1EN::Enabled,
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMA1EN::Disabled
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMA1EN::Enabled
    }
}
///Field `DMA1EN` writer - DMA1 Clock Enable
pub type DMA1EN_W<'a, REG> = crate::BitWriter<'a, REG, DMA1EN>;
impl<'a, REG> DMA1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1EN::Disabled)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1EN::Enabled)
    }
}
///Field `DMA2EN` reader - DMA2 Clock Enable
pub use DMA1EN_R as DMA2EN_R;
///Field `ADC12EN` reader - ADC1/2 Peripheral Clocks Enable
pub use DMA1EN_R as ADC12EN_R;
///Field `ETH1MACEN` reader - Ethernet MAC bus interface Clock Enable
pub use DMA1EN_R as ETH1MACEN_R;
///Field `ETH1TXEN` reader - Ethernet Transmission Clock Enable
pub use DMA1EN_R as ETH1TXEN_R;
///Field `ETH1RXEN` reader - Ethernet Reception Clock Enable
pub use DMA1EN_R as ETH1RXEN_R;
///Field `USB1OTGHSEN` reader - USB1OTG Peripheral Clocks Enable
pub use DMA1EN_R as USB1OTGHSEN_R;
///Field `USB1OTGULPIEN` reader - USB_PHY1 Clocks Enable
pub use DMA1EN_R as USB1OTGULPIEN_R;
///Field `DMA2EN` writer - DMA2 Clock Enable
pub use DMA1EN_W as DMA2EN_W;
///Field `ADC12EN` writer - ADC1/2 Peripheral Clocks Enable
pub use DMA1EN_W as ADC12EN_W;
///Field `ETH1MACEN` writer - Ethernet MAC bus interface Clock Enable
pub use DMA1EN_W as ETH1MACEN_W;
///Field `ETH1TXEN` writer - Ethernet Transmission Clock Enable
pub use DMA1EN_W as ETH1TXEN_W;
///Field `ETH1RXEN` writer - Ethernet Reception Clock Enable
pub use DMA1EN_W as ETH1RXEN_W;
///Field `USB1OTGHSEN` writer - USB1OTG Peripheral Clocks Enable
pub use DMA1EN_W as USB1OTGHSEN_W;
///Field `USB1OTGULPIEN` writer - USB_PHY1 Clocks Enable
pub use DMA1EN_W as USB1OTGULPIEN_W;
impl R {
    ///Bit 0 - DMA1 Clock Enable
    #[inline(always)]
    pub fn dma1en(&self) -> DMA1EN_R {
        DMA1EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMA2 Clock Enable
    #[inline(always)]
    pub fn dma2en(&self) -> DMA2EN_R {
        DMA2EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 5 - ADC1/2 Peripheral Clocks Enable
    #[inline(always)]
    pub fn adc12en(&self) -> ADC12EN_R {
        ADC12EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 15 - Ethernet MAC bus interface Clock Enable
    #[inline(always)]
    pub fn eth1macen(&self) -> ETH1MACEN_R {
        ETH1MACEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Ethernet Transmission Clock Enable
    #[inline(always)]
    pub fn eth1txen(&self) -> ETH1TXEN_R {
        ETH1TXEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Ethernet Reception Clock Enable
    #[inline(always)]
    pub fn eth1rxen(&self) -> ETH1RXEN_R {
        ETH1RXEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 25 - USB1OTG Peripheral Clocks Enable
    #[inline(always)]
    pub fn usb1otghsen(&self) -> USB1OTGHSEN_R {
        USB1OTGHSEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - USB_PHY1 Clocks Enable
    #[inline(always)]
    pub fn usb1otgulpien(&self) -> USB1OTGULPIEN_R {
        USB1OTGULPIEN_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB1ENR")
            .field("dma1en", &self.dma1en())
            .field("dma2en", &self.dma2en())
            .field("adc12en", &self.adc12en())
            .field("eth1macen", &self.eth1macen())
            .field("eth1txen", &self.eth1txen())
            .field("eth1rxen", &self.eth1rxen())
            .field("usb1otghsen", &self.usb1otghsen())
            .field("usb1otgulpien", &self.usb1otgulpien())
            .finish()
    }
}
impl W {
    ///Bit 0 - DMA1 Clock Enable
    #[inline(always)]
    pub fn dma1en(&mut self) -> DMA1EN_W<'_, AHB1ENRrs> {
        DMA1EN_W::new(self, 0)
    }
    ///Bit 1 - DMA2 Clock Enable
    #[inline(always)]
    pub fn dma2en(&mut self) -> DMA2EN_W<'_, AHB1ENRrs> {
        DMA2EN_W::new(self, 1)
    }
    ///Bit 5 - ADC1/2 Peripheral Clocks Enable
    #[inline(always)]
    pub fn adc12en(&mut self) -> ADC12EN_W<'_, AHB1ENRrs> {
        ADC12EN_W::new(self, 5)
    }
    ///Bit 15 - Ethernet MAC bus interface Clock Enable
    #[inline(always)]
    pub fn eth1macen(&mut self) -> ETH1MACEN_W<'_, AHB1ENRrs> {
        ETH1MACEN_W::new(self, 15)
    }
    ///Bit 16 - Ethernet Transmission Clock Enable
    #[inline(always)]
    pub fn eth1txen(&mut self) -> ETH1TXEN_W<'_, AHB1ENRrs> {
        ETH1TXEN_W::new(self, 16)
    }
    ///Bit 17 - Ethernet Reception Clock Enable
    #[inline(always)]
    pub fn eth1rxen(&mut self) -> ETH1RXEN_W<'_, AHB1ENRrs> {
        ETH1RXEN_W::new(self, 17)
    }
    ///Bit 25 - USB1OTG Peripheral Clocks Enable
    #[inline(always)]
    pub fn usb1otghsen(&mut self) -> USB1OTGHSEN_W<'_, AHB1ENRrs> {
        USB1OTGHSEN_W::new(self, 25)
    }
    ///Bit 26 - USB_PHY1 Clocks Enable
    #[inline(always)]
    pub fn usb1otgulpien(&mut self) -> USB1OTGULPIEN_W<'_, AHB1ENRrs> {
        USB1OTGULPIEN_W::new(self, 26)
    }
}
/**RCC AHB1 Clock Register

You can [`read`](crate::Reg::read) this register and get [`ahb1enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#RCC:AHB1ENR)*/
pub struct AHB1ENRrs;
impl crate::RegisterSpec for AHB1ENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb1enr::R`](R) reader structure
impl crate::Readable for AHB1ENRrs {}
///`write(|w| ..)` method takes [`ahb1enr::W`](W) writer structure
impl crate::Writable for AHB1ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB1ENR to value 0
impl crate::Resettable for AHB1ENRrs {}

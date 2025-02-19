///Register `C1_AHB1ENR` reader
pub type R = crate::R<C1_AHB1ENRrs>;
///Register `C1_AHB1ENR` writer
pub type W = crate::W<C1_AHB1ENRrs>;
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
///Field `USB1OTGEN` reader - USB1OTG Peripheral Clocks Enable
pub use DMA1EN_R as USB1OTGEN_R;
///Field `USB1ULPIEN` reader - USB_PHY1 Clocks Enable
pub use DMA1EN_R as USB1ULPIEN_R;
///Field `USB2OTGEN` reader - USB2OTG Peripheral Clocks Enable
pub use DMA1EN_R as USB2OTGEN_R;
///Field `USB2ULPIEN` reader - USB_PHY2 Clocks Enable
pub use DMA1EN_R as USB2ULPIEN_R;
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
///Field `USB1OTGEN` writer - USB1OTG Peripheral Clocks Enable
pub use DMA1EN_W as USB1OTGEN_W;
///Field `USB1ULPIEN` writer - USB_PHY1 Clocks Enable
pub use DMA1EN_W as USB1ULPIEN_W;
///Field `USB2OTGEN` writer - USB2OTG Peripheral Clocks Enable
pub use DMA1EN_W as USB2OTGEN_W;
///Field `USB2ULPIEN` writer - USB_PHY2 Clocks Enable
pub use DMA1EN_W as USB2ULPIEN_W;
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
    pub fn usb1otgen(&self) -> USB1OTGEN_R {
        USB1OTGEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - USB_PHY1 Clocks Enable
    #[inline(always)]
    pub fn usb1ulpien(&self) -> USB1ULPIEN_R {
        USB1ULPIEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - USB2OTG Peripheral Clocks Enable
    #[inline(always)]
    pub fn usb2otgen(&self) -> USB2OTGEN_R {
        USB2OTGEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - USB_PHY2 Clocks Enable
    #[inline(always)]
    pub fn usb2ulpien(&self) -> USB2ULPIEN_R {
        USB2ULPIEN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C1_AHB1ENR")
            .field("dma1en", &self.dma1en())
            .field("dma2en", &self.dma2en())
            .field("adc12en", &self.adc12en())
            .field("eth1macen", &self.eth1macen())
            .field("eth1txen", &self.eth1txen())
            .field("eth1rxen", &self.eth1rxen())
            .field("usb1otgen", &self.usb1otgen())
            .field("usb1ulpien", &self.usb1ulpien())
            .field("usb2otgen", &self.usb2otgen())
            .field("usb2ulpien", &self.usb2ulpien())
            .finish()
    }
}
impl W {
    ///Bit 0 - DMA1 Clock Enable
    #[inline(always)]
    pub fn dma1en(&mut self) -> DMA1EN_W<C1_AHB1ENRrs> {
        DMA1EN_W::new(self, 0)
    }
    ///Bit 1 - DMA2 Clock Enable
    #[inline(always)]
    pub fn dma2en(&mut self) -> DMA2EN_W<C1_AHB1ENRrs> {
        DMA2EN_W::new(self, 1)
    }
    ///Bit 5 - ADC1/2 Peripheral Clocks Enable
    #[inline(always)]
    pub fn adc12en(&mut self) -> ADC12EN_W<C1_AHB1ENRrs> {
        ADC12EN_W::new(self, 5)
    }
    ///Bit 15 - Ethernet MAC bus interface Clock Enable
    #[inline(always)]
    pub fn eth1macen(&mut self) -> ETH1MACEN_W<C1_AHB1ENRrs> {
        ETH1MACEN_W::new(self, 15)
    }
    ///Bit 16 - Ethernet Transmission Clock Enable
    #[inline(always)]
    pub fn eth1txen(&mut self) -> ETH1TXEN_W<C1_AHB1ENRrs> {
        ETH1TXEN_W::new(self, 16)
    }
    ///Bit 17 - Ethernet Reception Clock Enable
    #[inline(always)]
    pub fn eth1rxen(&mut self) -> ETH1RXEN_W<C1_AHB1ENRrs> {
        ETH1RXEN_W::new(self, 17)
    }
    ///Bit 25 - USB1OTG Peripheral Clocks Enable
    #[inline(always)]
    pub fn usb1otgen(&mut self) -> USB1OTGEN_W<C1_AHB1ENRrs> {
        USB1OTGEN_W::new(self, 25)
    }
    ///Bit 26 - USB_PHY1 Clocks Enable
    #[inline(always)]
    pub fn usb1ulpien(&mut self) -> USB1ULPIEN_W<C1_AHB1ENRrs> {
        USB1ULPIEN_W::new(self, 26)
    }
    ///Bit 27 - USB2OTG Peripheral Clocks Enable
    #[inline(always)]
    pub fn usb2otgen(&mut self) -> USB2OTGEN_W<C1_AHB1ENRrs> {
        USB2OTGEN_W::new(self, 27)
    }
    ///Bit 28 - USB_PHY2 Clocks Enable
    #[inline(always)]
    pub fn usb2ulpien(&mut self) -> USB2ULPIEN_W<C1_AHB1ENRrs> {
        USB2ULPIEN_W::new(self, 28)
    }
}
/**RCC AHB1 Clock Register

You can [`read`](crate::Reg::read) this register and get [`c1_ahb1enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1_ahb1enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#RCC:C1_AHB1ENR)*/
pub struct C1_AHB1ENRrs;
impl crate::RegisterSpec for C1_AHB1ENRrs {
    type Ux = u32;
}
///`read()` method returns [`c1_ahb1enr::R`](R) reader structure
impl crate::Readable for C1_AHB1ENRrs {}
///`write(|w| ..)` method takes [`c1_ahb1enr::W`](W) writer structure
impl crate::Writable for C1_AHB1ENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets C1_AHB1ENR to value 0
impl crate::Resettable for C1_AHB1ENRrs {
    const RESET_VALUE: u32 = 0;
}

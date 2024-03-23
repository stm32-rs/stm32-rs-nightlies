#[doc = "Register `AHB1ENR` reader"]
pub type R = crate::R<AHB1ENRrs>;
#[doc = "Register `AHB1ENR` writer"]
pub type W = crate::W<AHB1ENRrs>;
#[doc = "DMA1 clock enable Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA1EN {
    #[doc = "0: The selected clock is disabled"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled"]
    Enabled = 1,
}
impl From<DMA1EN> for bool {
    #[inline(always)]
    fn from(variant: DMA1EN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA1EN` reader - DMA1 clock enable Set and reset by software."]
pub type DMA1EN_R = crate::BitReader<DMA1EN>;
impl DMA1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA1EN {
        match self.bits {
            false => DMA1EN::Disabled,
            true => DMA1EN::Enabled,
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMA1EN::Disabled
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMA1EN::Enabled
    }
}
#[doc = "Field `DMA1EN` writer - DMA1 clock enable Set and reset by software."]
pub type DMA1EN_W<'a, REG> = crate::BitWriter<'a, REG, DMA1EN>;
impl<'a, REG> DMA1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1EN::Disabled)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1EN::Enabled)
    }
}
#[doc = "Field `DMA2EN` reader - DMA2 clock enable Set and reset by software."]
pub use DMA1EN_R as DMA2EN_R;
#[doc = "Field `ADC12EN` reader - ADC1 and 2 peripheral clocks enable Set and reset by software. The peripheral clocks of the ADC1 and 2 are the kernel clock selected by ADCSEL and provided to adc_ker_ck input, and the rcc_hclk1 bus interface clock."]
pub use DMA1EN_R as ADC12EN_R;
#[doc = "Field `CRCEN` reader - CRC peripheral clock enable Set and reset by software."]
pub use DMA1EN_R as CRCEN_R;
#[doc = "Field `USB1OTGEN` reader - USB1OTG peripheral clocks enable Set and reset by software. The peripheral clocks of the USB1OTG are the kernel clock selected by USBSEL and the rcc_hclk1 bus interface clock."]
pub use DMA1EN_R as USB1OTGEN_R;
#[doc = "Field `USB1ULPIEN` reader - USB_PHY1 clocks enable Set and reset by software."]
pub use DMA1EN_R as USB1ULPIEN_R;
#[doc = "Field `DMA2EN` writer - DMA2 clock enable Set and reset by software."]
pub use DMA1EN_W as DMA2EN_W;
#[doc = "Field `ADC12EN` writer - ADC1 and 2 peripheral clocks enable Set and reset by software. The peripheral clocks of the ADC1 and 2 are the kernel clock selected by ADCSEL and provided to adc_ker_ck input, and the rcc_hclk1 bus interface clock."]
pub use DMA1EN_W as ADC12EN_W;
#[doc = "Field `CRCEN` writer - CRC peripheral clock enable Set and reset by software."]
pub use DMA1EN_W as CRCEN_W;
#[doc = "Field `USB1OTGEN` writer - USB1OTG peripheral clocks enable Set and reset by software. The peripheral clocks of the USB1OTG are the kernel clock selected by USBSEL and the rcc_hclk1 bus interface clock."]
pub use DMA1EN_W as USB1OTGEN_W;
#[doc = "Field `USB1ULPIEN` writer - USB_PHY1 clocks enable Set and reset by software."]
pub use DMA1EN_W as USB1ULPIEN_W;
impl R {
    #[doc = "Bit 0 - DMA1 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn dma1en(&self) -> DMA1EN_R {
        DMA1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA2 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn dma2en(&self) -> DMA2EN_R {
        DMA2EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC1 and 2 peripheral clocks enable Set and reset by software. The peripheral clocks of the ADC1 and 2 are the kernel clock selected by ADCSEL and provided to adc_ker_ck input, and the rcc_hclk1 bus interface clock."]
    #[inline(always)]
    pub fn adc12en(&self) -> ADC12EN_R {
        ADC12EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - CRC peripheral clock enable Set and reset by software."]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 25 - USB1OTG peripheral clocks enable Set and reset by software. The peripheral clocks of the USB1OTG are the kernel clock selected by USBSEL and the rcc_hclk1 bus interface clock."]
    #[inline(always)]
    pub fn usb1otgen(&self) -> USB1OTGEN_R {
        USB1OTGEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - USB_PHY1 clocks enable Set and reset by software."]
    #[inline(always)]
    pub fn usb1ulpien(&self) -> USB1ULPIEN_R {
        USB1ULPIEN_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn dma1en(&mut self) -> DMA1EN_W<AHB1ENRrs> {
        DMA1EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA2 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn dma2en(&mut self) -> DMA2EN_W<AHB1ENRrs> {
        DMA2EN_W::new(self, 1)
    }
    #[doc = "Bit 5 - ADC1 and 2 peripheral clocks enable Set and reset by software. The peripheral clocks of the ADC1 and 2 are the kernel clock selected by ADCSEL and provided to adc_ker_ck input, and the rcc_hclk1 bus interface clock."]
    #[inline(always)]
    #[must_use]
    pub fn adc12en(&mut self) -> ADC12EN_W<AHB1ENRrs> {
        ADC12EN_W::new(self, 5)
    }
    #[doc = "Bit 9 - CRC peripheral clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CRCEN_W<AHB1ENRrs> {
        CRCEN_W::new(self, 9)
    }
    #[doc = "Bit 25 - USB1OTG peripheral clocks enable Set and reset by software. The peripheral clocks of the USB1OTG are the kernel clock selected by USBSEL and the rcc_hclk1 bus interface clock."]
    #[inline(always)]
    #[must_use]
    pub fn usb1otgen(&mut self) -> USB1OTGEN_W<AHB1ENRrs> {
        USB1OTGEN_W::new(self, 25)
    }
    #[doc = "Bit 26 - USB_PHY1 clocks enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn usb1ulpien(&mut self) -> USB1ULPIEN_W<AHB1ENRrs> {
        USB1ULPIEN_W::new(self, 26)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb1enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb1enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB1ENRrs;
impl crate::RegisterSpec for AHB1ENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb1enr::R`](R) reader structure"]
impl crate::Readable for AHB1ENRrs {}
#[doc = "`write(|w| ..)` method takes [`ahb1enr::W`](W) writer structure"]
impl crate::Writable for AHB1ENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB1ENR to value 0"]
impl crate::Resettable for AHB1ENRrs {
    const RESET_VALUE: u32 = 0;
}

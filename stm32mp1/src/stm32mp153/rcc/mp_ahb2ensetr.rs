///Register `MP_AHB2ENSETR` reader
pub type R = crate::R<MP_AHB2ENSETRrs>;
///Register `MP_AHB2ENSETR` writer
pub type W = crate::W<MP_AHB2ENSETRrs>;
///Field `DMA1EN` reader - DMA1EN
pub type DMA1EN_R = crate::BitReader;
///Field `DMA1EN` writer - DMA1EN
pub type DMA1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA2EN` reader - DMA2EN
pub type DMA2EN_R = crate::BitReader;
///Field `DMA2EN` writer - DMA2EN
pub type DMA2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMAMUXEN` reader - DMAMUXEN
pub type DMAMUXEN_R = crate::BitReader;
///Field `DMAMUXEN` writer - DMAMUXEN
pub type DMAMUXEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC12EN` reader - ADC12EN
pub type ADC12EN_R = crate::BitReader;
///Field `ADC12EN` writer - ADC12EN
pub type ADC12EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USBOEN` reader - USBOEN
pub type USBOEN_R = crate::BitReader;
///Field `USBOEN` writer - USBOEN
pub type USBOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMMC3EN` reader - SDMMC3EN
pub type SDMMC3EN_R = crate::BitReader;
///Field `SDMMC3EN` writer - SDMMC3EN
pub type SDMMC3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - DMA1EN
    #[inline(always)]
    pub fn dma1en(&self) -> DMA1EN_R {
        DMA1EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMA2EN
    #[inline(always)]
    pub fn dma2en(&self) -> DMA2EN_R {
        DMA2EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DMAMUXEN
    #[inline(always)]
    pub fn dmamuxen(&self) -> DMAMUXEN_R {
        DMAMUXEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 5 - ADC12EN
    #[inline(always)]
    pub fn adc12en(&self) -> ADC12EN_R {
        ADC12EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - USBOEN
    #[inline(always)]
    pub fn usboen(&self) -> USBOEN_R {
        USBOEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 16 - SDMMC3EN
    #[inline(always)]
    pub fn sdmmc3en(&self) -> SDMMC3EN_R {
        SDMMC3EN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MP_AHB2ENSETR")
            .field("dma1en", &self.dma1en())
            .field("dma2en", &self.dma2en())
            .field("dmamuxen", &self.dmamuxen())
            .field("adc12en", &self.adc12en())
            .field("usboen", &self.usboen())
            .field("sdmmc3en", &self.sdmmc3en())
            .finish()
    }
}
impl W {
    ///Bit 0 - DMA1EN
    #[inline(always)]
    pub fn dma1en(&mut self) -> DMA1EN_W<'_, MP_AHB2ENSETRrs> {
        DMA1EN_W::new(self, 0)
    }
    ///Bit 1 - DMA2EN
    #[inline(always)]
    pub fn dma2en(&mut self) -> DMA2EN_W<'_, MP_AHB2ENSETRrs> {
        DMA2EN_W::new(self, 1)
    }
    ///Bit 2 - DMAMUXEN
    #[inline(always)]
    pub fn dmamuxen(&mut self) -> DMAMUXEN_W<'_, MP_AHB2ENSETRrs> {
        DMAMUXEN_W::new(self, 2)
    }
    ///Bit 5 - ADC12EN
    #[inline(always)]
    pub fn adc12en(&mut self) -> ADC12EN_W<'_, MP_AHB2ENSETRrs> {
        ADC12EN_W::new(self, 5)
    }
    ///Bit 8 - USBOEN
    #[inline(always)]
    pub fn usboen(&mut self) -> USBOEN_W<'_, MP_AHB2ENSETRrs> {
        USBOEN_W::new(self, 8)
    }
    ///Bit 16 - SDMMC3EN
    #[inline(always)]
    pub fn sdmmc3en(&mut self) -> SDMMC3EN_W<'_, MP_AHB2ENSETRrs> {
        SDMMC3EN_W::new(self, 16)
    }
}
/**This register is used to set the peripheral clock enable bit of the corresponding peripheral

You can [`read`](crate::Reg::read) this register and get [`mp_ahb2ensetr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_ahb2ensetr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_AHB2ENSETR)*/
pub struct MP_AHB2ENSETRrs;
impl crate::RegisterSpec for MP_AHB2ENSETRrs {
    type Ux = u32;
}
///`read()` method returns [`mp_ahb2ensetr::R`](R) reader structure
impl crate::Readable for MP_AHB2ENSETRrs {}
///`write(|w| ..)` method takes [`mp_ahb2ensetr::W`](W) writer structure
impl crate::Writable for MP_AHB2ENSETRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MP_AHB2ENSETR to value 0
impl crate::Resettable for MP_AHB2ENSETRrs {}

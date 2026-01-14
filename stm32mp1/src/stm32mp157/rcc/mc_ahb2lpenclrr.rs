///Register `MC_AHB2LPENCLRR` reader
pub type R = crate::R<MC_AHB2LPENCLRRrs>;
///Register `MC_AHB2LPENCLRR` writer
pub type W = crate::W<MC_AHB2LPENCLRRrs>;
///Field `DMA1LPEN` reader - DMA1LPEN
pub type DMA1LPEN_R = crate::BitReader;
///Field `DMA1LPEN` writer - DMA1LPEN
pub type DMA1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA2LPEN` reader - DMA2LPEN
pub type DMA2LPEN_R = crate::BitReader;
///Field `DMA2LPEN` writer - DMA2LPEN
pub type DMA2LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMAMUXLPEN` reader - DMAMUXLPEN
pub type DMAMUXLPEN_R = crate::BitReader;
///Field `DMAMUXLPEN` writer - DMAMUXLPEN
pub type DMAMUXLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC12LPEN` reader - ADC12LPEN
pub type ADC12LPEN_R = crate::BitReader;
///Field `ADC12LPEN` writer - ADC12LPEN
pub type ADC12LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USBOLPEN` reader - USBOLPEN
pub type USBOLPEN_R = crate::BitReader;
///Field `USBOLPEN` writer - USBOLPEN
pub type USBOLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMMC3LPEN` reader - SDMMC3LPEN
pub type SDMMC3LPEN_R = crate::BitReader;
///Field `SDMMC3LPEN` writer - SDMMC3LPEN
pub type SDMMC3LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - DMA1LPEN
    #[inline(always)]
    pub fn dma1lpen(&self) -> DMA1LPEN_R {
        DMA1LPEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMA2LPEN
    #[inline(always)]
    pub fn dma2lpen(&self) -> DMA2LPEN_R {
        DMA2LPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DMAMUXLPEN
    #[inline(always)]
    pub fn dmamuxlpen(&self) -> DMAMUXLPEN_R {
        DMAMUXLPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 5 - ADC12LPEN
    #[inline(always)]
    pub fn adc12lpen(&self) -> ADC12LPEN_R {
        ADC12LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - USBOLPEN
    #[inline(always)]
    pub fn usbolpen(&self) -> USBOLPEN_R {
        USBOLPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 16 - SDMMC3LPEN
    #[inline(always)]
    pub fn sdmmc3lpen(&self) -> SDMMC3LPEN_R {
        SDMMC3LPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MC_AHB2LPENCLRR")
            .field("dma1lpen", &self.dma1lpen())
            .field("dma2lpen", &self.dma2lpen())
            .field("dmamuxlpen", &self.dmamuxlpen())
            .field("adc12lpen", &self.adc12lpen())
            .field("usbolpen", &self.usbolpen())
            .field("sdmmc3lpen", &self.sdmmc3lpen())
            .finish()
    }
}
impl W {
    ///Bit 0 - DMA1LPEN
    #[inline(always)]
    pub fn dma1lpen(&mut self) -> DMA1LPEN_W<'_, MC_AHB2LPENCLRRrs> {
        DMA1LPEN_W::new(self, 0)
    }
    ///Bit 1 - DMA2LPEN
    #[inline(always)]
    pub fn dma2lpen(&mut self) -> DMA2LPEN_W<'_, MC_AHB2LPENCLRRrs> {
        DMA2LPEN_W::new(self, 1)
    }
    ///Bit 2 - DMAMUXLPEN
    #[inline(always)]
    pub fn dmamuxlpen(&mut self) -> DMAMUXLPEN_W<'_, MC_AHB2LPENCLRRrs> {
        DMAMUXLPEN_W::new(self, 2)
    }
    ///Bit 5 - ADC12LPEN
    #[inline(always)]
    pub fn adc12lpen(&mut self) -> ADC12LPEN_W<'_, MC_AHB2LPENCLRRrs> {
        ADC12LPEN_W::new(self, 5)
    }
    ///Bit 8 - USBOLPEN
    #[inline(always)]
    pub fn usbolpen(&mut self) -> USBOLPEN_W<'_, MC_AHB2LPENCLRRrs> {
        USBOLPEN_W::new(self, 8)
    }
    ///Bit 16 - SDMMC3LPEN
    #[inline(always)]
    pub fn sdmmc3lpen(&mut self) -> SDMMC3LPEN_W<'_, MC_AHB2LPENCLRRrs> {
        SDMMC3LPEN_W::new(self, 16)
    }
}
/**This register is used by the MCU in order to clear the PERxLPEN bit

You can [`read`](crate::Reg::read) this register and get [`mc_ahb2lpenclrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_ahb2lpenclrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:MC_AHB2LPENCLRR)*/
pub struct MC_AHB2LPENCLRRrs;
impl crate::RegisterSpec for MC_AHB2LPENCLRRrs {
    type Ux = u32;
}
///`read()` method returns [`mc_ahb2lpenclrr::R`](R) reader structure
impl crate::Readable for MC_AHB2LPENCLRRrs {}
///`write(|w| ..)` method takes [`mc_ahb2lpenclrr::W`](W) writer structure
impl crate::Writable for MC_AHB2LPENCLRRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MC_AHB2LPENCLRR to value 0x0001_0127
impl crate::Resettable for MC_AHB2LPENCLRRrs {
    const RESET_VALUE: u32 = 0x0001_0127;
}

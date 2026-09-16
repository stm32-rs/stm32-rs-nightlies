///Register `AHB2RSTSETR` reader
pub type R = crate::R<AHB2RSTSETRrs>;
///Register `AHB2RSTSETR` writer
pub type W = crate::W<AHB2RSTSETRrs>;
///Field `DMA1RST` reader - DMA1RST
pub type DMA1RST_R = crate::BitReader;
///Field `DMA1RST` writer - DMA1RST
pub type DMA1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA2RST` reader - DMA2RST
pub type DMA2RST_R = crate::BitReader;
///Field `DMA2RST` writer - DMA2RST
pub type DMA2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMAMUXRST` reader - DMAMUXRST
pub type DMAMUXRST_R = crate::BitReader;
///Field `DMAMUXRST` writer - DMAMUXRST
pub type DMAMUXRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC12RST` reader - ADC12RST
pub type ADC12RST_R = crate::BitReader;
///Field `ADC12RST` writer - ADC12RST
pub type ADC12RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USBORST` reader - USBORST
pub type USBORST_R = crate::BitReader;
///Field `USBORST` writer - USBORST
pub type USBORST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMMC3RST` reader - SDMMC3RST
pub type SDMMC3RST_R = crate::BitReader;
///Field `SDMMC3RST` writer - SDMMC3RST
pub type SDMMC3RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - DMA1RST
    #[inline(always)]
    pub fn dma1rst(&self) -> DMA1RST_R {
        DMA1RST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMA2RST
    #[inline(always)]
    pub fn dma2rst(&self) -> DMA2RST_R {
        DMA2RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DMAMUXRST
    #[inline(always)]
    pub fn dmamuxrst(&self) -> DMAMUXRST_R {
        DMAMUXRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 5 - ADC12RST
    #[inline(always)]
    pub fn adc12rst(&self) -> ADC12RST_R {
        ADC12RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - USBORST
    #[inline(always)]
    pub fn usborst(&self) -> USBORST_R {
        USBORST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 16 - SDMMC3RST
    #[inline(always)]
    pub fn sdmmc3rst(&self) -> SDMMC3RST_R {
        SDMMC3RST_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB2RSTSETR")
            .field("dma1rst", &self.dma1rst())
            .field("dma2rst", &self.dma2rst())
            .field("dmamuxrst", &self.dmamuxrst())
            .field("adc12rst", &self.adc12rst())
            .field("usborst", &self.usborst())
            .field("sdmmc3rst", &self.sdmmc3rst())
            .finish()
    }
}
impl W {
    ///Bit 0 - DMA1RST
    #[inline(always)]
    pub fn dma1rst(&mut self) -> DMA1RST_W<'_, AHB2RSTSETRrs> {
        DMA1RST_W::new(self, 0)
    }
    ///Bit 1 - DMA2RST
    #[inline(always)]
    pub fn dma2rst(&mut self) -> DMA2RST_W<'_, AHB2RSTSETRrs> {
        DMA2RST_W::new(self, 1)
    }
    ///Bit 2 - DMAMUXRST
    #[inline(always)]
    pub fn dmamuxrst(&mut self) -> DMAMUXRST_W<'_, AHB2RSTSETRrs> {
        DMAMUXRST_W::new(self, 2)
    }
    ///Bit 5 - ADC12RST
    #[inline(always)]
    pub fn adc12rst(&mut self) -> ADC12RST_W<'_, AHB2RSTSETRrs> {
        ADC12RST_W::new(self, 5)
    }
    ///Bit 8 - USBORST
    #[inline(always)]
    pub fn usborst(&mut self) -> USBORST_W<'_, AHB2RSTSETRrs> {
        USBORST_W::new(self, 8)
    }
    ///Bit 16 - SDMMC3RST
    #[inline(always)]
    pub fn sdmmc3rst(&mut self) -> SDMMC3RST_W<'_, AHB2RSTSETRrs> {
        SDMMC3RST_W::new(self, 16)
    }
}
/**This register is used to activate the reset of the corresponding peripheral.

You can [`read`](crate::Reg::read) this register and get [`ahb2rstsetr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2rstsetr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:AHB2RSTSETR)*/
pub struct AHB2RSTSETRrs;
impl crate::RegisterSpec for AHB2RSTSETRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb2rstsetr::R`](R) reader structure
impl crate::Readable for AHB2RSTSETRrs {}
///`write(|w| ..)` method takes [`ahb2rstsetr::W`](W) writer structure
impl crate::Writable for AHB2RSTSETRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB2RSTSETR to value 0
impl crate::Resettable for AHB2RSTSETRrs {}

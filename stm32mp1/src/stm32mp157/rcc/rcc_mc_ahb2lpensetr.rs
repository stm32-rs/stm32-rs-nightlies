#[doc = "Register `RCC_MC_AHB2LPENSETR` reader"]
pub type R = crate::R<RCC_MC_AHB2LPENSETRrs>;
#[doc = "Register `RCC_MC_AHB2LPENSETR` writer"]
pub type W = crate::W<RCC_MC_AHB2LPENSETRrs>;
#[doc = "Field `DMA1LPEN` reader - DMA1LPEN"]
pub type DMA1LPEN_R = crate::BitReader;
#[doc = "Field `DMA1LPEN` writer - DMA1LPEN"]
pub type DMA1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2LPEN` reader - DMA2LPEN"]
pub type DMA2LPEN_R = crate::BitReader;
#[doc = "Field `DMA2LPEN` writer - DMA2LPEN"]
pub type DMA2LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAMUXLPEN` reader - DMAMUXLPEN"]
pub type DMAMUXLPEN_R = crate::BitReader;
#[doc = "Field `DMAMUXLPEN` writer - DMAMUXLPEN"]
pub type DMAMUXLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12LPEN` reader - ADC12LPEN"]
pub type ADC12LPEN_R = crate::BitReader;
#[doc = "Field `ADC12LPEN` writer - ADC12LPEN"]
pub type ADC12LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBOLPEN` reader - USBOLPEN"]
pub type USBOLPEN_R = crate::BitReader;
#[doc = "Field `USBOLPEN` writer - USBOLPEN"]
pub type USBOLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDMMC3LPEN` reader - SDMMC3LPEN"]
pub type SDMMC3LPEN_R = crate::BitReader;
#[doc = "Field `SDMMC3LPEN` writer - SDMMC3LPEN"]
pub type SDMMC3LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA1LPEN"]
    #[inline(always)]
    pub fn dma1lpen(&self) -> DMA1LPEN_R {
        DMA1LPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA2LPEN"]
    #[inline(always)]
    pub fn dma2lpen(&self) -> DMA2LPEN_R {
        DMA2LPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMAMUXLPEN"]
    #[inline(always)]
    pub fn dmamuxlpen(&self) -> DMAMUXLPEN_R {
        DMAMUXLPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC12LPEN"]
    #[inline(always)]
    pub fn adc12lpen(&self) -> ADC12LPEN_R {
        ADC12LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - USBOLPEN"]
    #[inline(always)]
    pub fn usbolpen(&self) -> USBOLPEN_R {
        USBOLPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - SDMMC3LPEN"]
    #[inline(always)]
    pub fn sdmmc3lpen(&self) -> SDMMC3LPEN_R {
        SDMMC3LPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn dma1lpen(&mut self) -> DMA1LPEN_W<RCC_MC_AHB2LPENSETRrs> {
        DMA1LPEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA2LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn dma2lpen(&mut self) -> DMA2LPEN_W<RCC_MC_AHB2LPENSETRrs> {
        DMA2LPEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - DMAMUXLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn dmamuxlpen(&mut self) -> DMAMUXLPEN_W<RCC_MC_AHB2LPENSETRrs> {
        DMAMUXLPEN_W::new(self, 2)
    }
    #[doc = "Bit 5 - ADC12LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn adc12lpen(&mut self) -> ADC12LPEN_W<RCC_MC_AHB2LPENSETRrs> {
        ADC12LPEN_W::new(self, 5)
    }
    #[doc = "Bit 8 - USBOLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn usbolpen(&mut self) -> USBOLPEN_W<RCC_MC_AHB2LPENSETRrs> {
        USBOLPEN_W::new(self, 8)
    }
    #[doc = "Bit 16 - SDMMC3LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc3lpen(&mut self) -> SDMMC3LPEN_W<RCC_MC_AHB2LPENSETRrs> {
        SDMMC3LPEN_W::new(self, 16)
    }
}
#[doc = "This register is used by the MCU in order to set the PERxLPEN bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_ahb2lpensetr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_ahb2lpensetr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_MC_AHB2LPENSETRrs;
impl crate::RegisterSpec for RCC_MC_AHB2LPENSETRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_mc_ahb2lpensetr::R`](R) reader structure"]
impl crate::Readable for RCC_MC_AHB2LPENSETRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_mc_ahb2lpensetr::W`](W) writer structure"]
impl crate::Writable for RCC_MC_AHB2LPENSETRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_MC_AHB2LPENSETR to value 0x0001_0127"]
impl crate::Resettable for RCC_MC_AHB2LPENSETRrs {
    const RESET_VALUE: u32 = 0x0001_0127;
}

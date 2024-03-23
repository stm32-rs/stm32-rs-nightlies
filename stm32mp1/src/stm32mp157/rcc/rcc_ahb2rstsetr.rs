#[doc = "Register `RCC_AHB2RSTSETR` reader"]
pub type R = crate::R<RCC_AHB2RSTSETRrs>;
#[doc = "Register `RCC_AHB2RSTSETR` writer"]
pub type W = crate::W<RCC_AHB2RSTSETRrs>;
#[doc = "Field `DMA1RST` reader - DMA1RST"]
pub type DMA1RST_R = crate::BitReader;
#[doc = "Field `DMA1RST` writer - DMA1RST"]
pub type DMA1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2RST` reader - DMA2RST"]
pub type DMA2RST_R = crate::BitReader;
#[doc = "Field `DMA2RST` writer - DMA2RST"]
pub type DMA2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAMUXRST` reader - DMAMUXRST"]
pub type DMAMUXRST_R = crate::BitReader;
#[doc = "Field `DMAMUXRST` writer - DMAMUXRST"]
pub type DMAMUXRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12RST` reader - ADC12RST"]
pub type ADC12RST_R = crate::BitReader;
#[doc = "Field `ADC12RST` writer - ADC12RST"]
pub type ADC12RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBORST` reader - USBORST"]
pub type USBORST_R = crate::BitReader;
#[doc = "Field `USBORST` writer - USBORST"]
pub type USBORST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDMMC3RST` reader - SDMMC3RST"]
pub type SDMMC3RST_R = crate::BitReader;
#[doc = "Field `SDMMC3RST` writer - SDMMC3RST"]
pub type SDMMC3RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA1RST"]
    #[inline(always)]
    pub fn dma1rst(&self) -> DMA1RST_R {
        DMA1RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA2RST"]
    #[inline(always)]
    pub fn dma2rst(&self) -> DMA2RST_R {
        DMA2RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMAMUXRST"]
    #[inline(always)]
    pub fn dmamuxrst(&self) -> DMAMUXRST_R {
        DMAMUXRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC12RST"]
    #[inline(always)]
    pub fn adc12rst(&self) -> ADC12RST_R {
        ADC12RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - USBORST"]
    #[inline(always)]
    pub fn usborst(&self) -> USBORST_R {
        USBORST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - SDMMC3RST"]
    #[inline(always)]
    pub fn sdmmc3rst(&self) -> SDMMC3RST_R {
        SDMMC3RST_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1RST"]
    #[inline(always)]
    #[must_use]
    pub fn dma1rst(&mut self) -> DMA1RST_W<RCC_AHB2RSTSETRrs> {
        DMA1RST_W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA2RST"]
    #[inline(always)]
    #[must_use]
    pub fn dma2rst(&mut self) -> DMA2RST_W<RCC_AHB2RSTSETRrs> {
        DMA2RST_W::new(self, 1)
    }
    #[doc = "Bit 2 - DMAMUXRST"]
    #[inline(always)]
    #[must_use]
    pub fn dmamuxrst(&mut self) -> DMAMUXRST_W<RCC_AHB2RSTSETRrs> {
        DMAMUXRST_W::new(self, 2)
    }
    #[doc = "Bit 5 - ADC12RST"]
    #[inline(always)]
    #[must_use]
    pub fn adc12rst(&mut self) -> ADC12RST_W<RCC_AHB2RSTSETRrs> {
        ADC12RST_W::new(self, 5)
    }
    #[doc = "Bit 8 - USBORST"]
    #[inline(always)]
    #[must_use]
    pub fn usborst(&mut self) -> USBORST_W<RCC_AHB2RSTSETRrs> {
        USBORST_W::new(self, 8)
    }
    #[doc = "Bit 16 - SDMMC3RST"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc3rst(&mut self) -> SDMMC3RST_W<RCC_AHB2RSTSETRrs> {
        SDMMC3RST_W::new(self, 16)
    }
}
#[doc = "This register is used to activate the reset of the corresponding peripheral.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ahb2rstsetr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ahb2rstsetr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_AHB2RSTSETRrs;
impl crate::RegisterSpec for RCC_AHB2RSTSETRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_ahb2rstsetr::R`](R) reader structure"]
impl crate::Readable for RCC_AHB2RSTSETRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_ahb2rstsetr::W`](W) writer structure"]
impl crate::Writable for RCC_AHB2RSTSETRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_AHB2RSTSETR to value 0"]
impl crate::Resettable for RCC_AHB2RSTSETRrs {
    const RESET_VALUE: u32 = 0;
}

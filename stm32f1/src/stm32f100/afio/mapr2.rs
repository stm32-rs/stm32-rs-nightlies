#[doc = "Register `MAPR2` reader"]
pub type R = crate::R<MAPR2rs>;
#[doc = "Register `MAPR2` writer"]
pub type W = crate::W<MAPR2rs>;
#[doc = "Field `TIM15_REMAP` reader - TIM15 remapping"]
pub type TIM15_REMAP_R = crate::BitReader;
#[doc = "Field `TIM15_REMAP` writer - TIM15 remapping"]
pub type TIM15_REMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM16_REMAP` reader - TIM16 remapping"]
pub type TIM16_REMAP_R = crate::BitReader;
#[doc = "Field `TIM16_REMAP` writer - TIM16 remapping"]
pub type TIM16_REMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM17_REMAP` reader - TIM17 remapping"]
pub type TIM17_REMAP_R = crate::BitReader;
#[doc = "Field `TIM17_REMAP` writer - TIM17 remapping"]
pub type TIM17_REMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEC_REMAP` reader - CEC remapping"]
pub type CEC_REMAP_R = crate::BitReader;
#[doc = "Field `CEC_REMAP` writer - CEC remapping"]
pub type CEC_REMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM1_DMA_REMAP` reader - TIM1 DMA remapping"]
pub type TIM1_DMA_REMAP_R = crate::BitReader;
#[doc = "Field `TIM1_DMA_REMAP` writer - TIM1 DMA remapping"]
pub type TIM1_DMA_REMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM13_REMAP` reader - TIM13 remapping"]
pub type TIM13_REMAP_R = crate::BitReader;
#[doc = "Field `TIM13_REMAP` writer - TIM13 remapping"]
pub type TIM13_REMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM14_REMAP` reader - TIM14 remapping"]
pub type TIM14_REMAP_R = crate::BitReader;
#[doc = "Field `TIM14_REMAP` writer - TIM14 remapping"]
pub type TIM14_REMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSMC_NADV` reader - NADV connect/disconnect"]
pub type FSMC_NADV_R = crate::BitReader;
#[doc = "Field `FSMC_NADV` writer - NADV connect/disconnect"]
pub type FSMC_NADV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM67_DAC_DMA_REMAP` reader - TIM67_DAC DMA remapping"]
pub type TIM67_DAC_DMA_REMAP_R = crate::BitReader;
#[doc = "Field `TIM67_DAC_DMA_REMAP` writer - TIM67_DAC DMA remapping"]
pub type TIM67_DAC_DMA_REMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM12_REMAP` reader - TIM12 remapping"]
pub type TIM12_REMAP_R = crate::BitReader;
#[doc = "Field `TIM12_REMAP` writer - TIM12 remapping"]
pub type TIM12_REMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MISC_REMAP` reader - Miscellaneous features remapping"]
pub type MISC_REMAP_R = crate::BitReader;
#[doc = "Field `MISC_REMAP` writer - Miscellaneous features remapping"]
pub type MISC_REMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TIM15 remapping"]
    #[inline(always)]
    pub fn tim15_remap(&self) -> TIM15_REMAP_R {
        TIM15_REMAP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM16 remapping"]
    #[inline(always)]
    pub fn tim16_remap(&self) -> TIM16_REMAP_R {
        TIM16_REMAP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM17 remapping"]
    #[inline(always)]
    pub fn tim17_remap(&self) -> TIM17_REMAP_R {
        TIM17_REMAP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CEC remapping"]
    #[inline(always)]
    pub fn cec_remap(&self) -> CEC_REMAP_R {
        CEC_REMAP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM1 DMA remapping"]
    #[inline(always)]
    pub fn tim1_dma_remap(&self) -> TIM1_DMA_REMAP_R {
        TIM1_DMA_REMAP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - TIM13 remapping"]
    #[inline(always)]
    pub fn tim13_remap(&self) -> TIM13_REMAP_R {
        TIM13_REMAP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TIM14 remapping"]
    #[inline(always)]
    pub fn tim14_remap(&self) -> TIM14_REMAP_R {
        TIM14_REMAP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - NADV connect/disconnect"]
    #[inline(always)]
    pub fn fsmc_nadv(&self) -> FSMC_NADV_R {
        FSMC_NADV_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TIM67_DAC DMA remapping"]
    #[inline(always)]
    pub fn tim67_dac_dma_remap(&self) -> TIM67_DAC_DMA_REMAP_R {
        TIM67_DAC_DMA_REMAP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TIM12 remapping"]
    #[inline(always)]
    pub fn tim12_remap(&self) -> TIM12_REMAP_R {
        TIM12_REMAP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Miscellaneous features remapping"]
    #[inline(always)]
    pub fn misc_remap(&self) -> MISC_REMAP_R {
        MISC_REMAP_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM15 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn tim15_remap(&mut self) -> TIM15_REMAP_W<MAPR2rs> {
        TIM15_REMAP_W::new(self, 0)
    }
    #[doc = "Bit 1 - TIM16 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn tim16_remap(&mut self) -> TIM16_REMAP_W<MAPR2rs> {
        TIM16_REMAP_W::new(self, 1)
    }
    #[doc = "Bit 2 - TIM17 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn tim17_remap(&mut self) -> TIM17_REMAP_W<MAPR2rs> {
        TIM17_REMAP_W::new(self, 2)
    }
    #[doc = "Bit 3 - CEC remapping"]
    #[inline(always)]
    #[must_use]
    pub fn cec_remap(&mut self) -> CEC_REMAP_W<MAPR2rs> {
        CEC_REMAP_W::new(self, 3)
    }
    #[doc = "Bit 4 - TIM1 DMA remapping"]
    #[inline(always)]
    #[must_use]
    pub fn tim1_dma_remap(&mut self) -> TIM1_DMA_REMAP_W<MAPR2rs> {
        TIM1_DMA_REMAP_W::new(self, 4)
    }
    #[doc = "Bit 8 - TIM13 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn tim13_remap(&mut self) -> TIM13_REMAP_W<MAPR2rs> {
        TIM13_REMAP_W::new(self, 8)
    }
    #[doc = "Bit 9 - TIM14 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn tim14_remap(&mut self) -> TIM14_REMAP_W<MAPR2rs> {
        TIM14_REMAP_W::new(self, 9)
    }
    #[doc = "Bit 10 - NADV connect/disconnect"]
    #[inline(always)]
    #[must_use]
    pub fn fsmc_nadv(&mut self) -> FSMC_NADV_W<MAPR2rs> {
        FSMC_NADV_W::new(self, 10)
    }
    #[doc = "Bit 11 - TIM67_DAC DMA remapping"]
    #[inline(always)]
    #[must_use]
    pub fn tim67_dac_dma_remap(&mut self) -> TIM67_DAC_DMA_REMAP_W<MAPR2rs> {
        TIM67_DAC_DMA_REMAP_W::new(self, 11)
    }
    #[doc = "Bit 12 - TIM12 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn tim12_remap(&mut self) -> TIM12_REMAP_W<MAPR2rs> {
        TIM12_REMAP_W::new(self, 12)
    }
    #[doc = "Bit 13 - Miscellaneous features remapping"]
    #[inline(always)]
    #[must_use]
    pub fn misc_remap(&mut self) -> MISC_REMAP_W<MAPR2rs> {
        MISC_REMAP_W::new(self, 13)
    }
}
#[doc = "AF remap and debug I/O configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mapr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mapr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAPR2rs;
impl crate::RegisterSpec for MAPR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mapr2::R`](R) reader structure"]
impl crate::Readable for MAPR2rs {}
#[doc = "`write(|w| ..)` method takes [`mapr2::W`](W) writer structure"]
impl crate::Writable for MAPR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAPR2 to value 0"]
impl crate::Resettable for MAPR2rs {
    const RESET_VALUE: u32 = 0;
}

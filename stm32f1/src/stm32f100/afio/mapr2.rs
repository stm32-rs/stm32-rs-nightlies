///Register `MAPR2` reader
pub type R = crate::R<MAPR2rs>;
///Register `MAPR2` writer
pub type W = crate::W<MAPR2rs>;
///Field `TIM15_REMAP` reader - TIM15 remapping
pub type TIM15_REMAP_R = crate::BitReader;
///Field `TIM15_REMAP` writer - TIM15 remapping
pub type TIM15_REMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM16_REMAP` reader - TIM16 remapping
pub type TIM16_REMAP_R = crate::BitReader;
///Field `TIM16_REMAP` writer - TIM16 remapping
pub type TIM16_REMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM17_REMAP` reader - TIM17 remapping
pub type TIM17_REMAP_R = crate::BitReader;
///Field `TIM17_REMAP` writer - TIM17 remapping
pub type TIM17_REMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CEC_REMAP` reader - CEC remapping
pub type CEC_REMAP_R = crate::BitReader;
///Field `CEC_REMAP` writer - CEC remapping
pub type CEC_REMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM1_DMA_REMAP` reader - TIM1 DMA remapping
pub type TIM1_DMA_REMAP_R = crate::BitReader;
///Field `TIM1_DMA_REMAP` writer - TIM1 DMA remapping
pub type TIM1_DMA_REMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM13_REMAP` reader - TIM13 remapping
pub type TIM13_REMAP_R = crate::BitReader;
///Field `TIM13_REMAP` writer - TIM13 remapping
pub type TIM13_REMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM14_REMAP` reader - TIM14 remapping
pub type TIM14_REMAP_R = crate::BitReader;
///Field `TIM14_REMAP` writer - TIM14 remapping
pub type TIM14_REMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FSMC_NADV` reader - NADV connect/disconnect
pub type FSMC_NADV_R = crate::BitReader;
///Field `FSMC_NADV` writer - NADV connect/disconnect
pub type FSMC_NADV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM67_DAC_DMA_REMAP` reader - TIM67_DAC DMA remapping
pub type TIM67_DAC_DMA_REMAP_R = crate::BitReader;
///Field `TIM67_DAC_DMA_REMAP` writer - TIM67_DAC DMA remapping
pub type TIM67_DAC_DMA_REMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM12_REMAP` reader - TIM12 remapping
pub type TIM12_REMAP_R = crate::BitReader;
///Field `TIM12_REMAP` writer - TIM12 remapping
pub type TIM12_REMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MISC_REMAP` reader - Miscellaneous features remapping
pub type MISC_REMAP_R = crate::BitReader;
///Field `MISC_REMAP` writer - Miscellaneous features remapping
pub type MISC_REMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TIM15 remapping
    #[inline(always)]
    pub fn tim15_remap(&self) -> TIM15_REMAP_R {
        TIM15_REMAP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM16 remapping
    #[inline(always)]
    pub fn tim16_remap(&self) -> TIM16_REMAP_R {
        TIM16_REMAP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TIM17 remapping
    #[inline(always)]
    pub fn tim17_remap(&self) -> TIM17_REMAP_R {
        TIM17_REMAP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CEC remapping
    #[inline(always)]
    pub fn cec_remap(&self) -> CEC_REMAP_R {
        CEC_REMAP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TIM1 DMA remapping
    #[inline(always)]
    pub fn tim1_dma_remap(&self) -> TIM1_DMA_REMAP_R {
        TIM1_DMA_REMAP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - TIM13 remapping
    #[inline(always)]
    pub fn tim13_remap(&self) -> TIM13_REMAP_R {
        TIM13_REMAP_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - TIM14 remapping
    #[inline(always)]
    pub fn tim14_remap(&self) -> TIM14_REMAP_R {
        TIM14_REMAP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - NADV connect/disconnect
    #[inline(always)]
    pub fn fsmc_nadv(&self) -> FSMC_NADV_R {
        FSMC_NADV_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - TIM67_DAC DMA remapping
    #[inline(always)]
    pub fn tim67_dac_dma_remap(&self) -> TIM67_DAC_DMA_REMAP_R {
        TIM67_DAC_DMA_REMAP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - TIM12 remapping
    #[inline(always)]
    pub fn tim12_remap(&self) -> TIM12_REMAP_R {
        TIM12_REMAP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Miscellaneous features remapping
    #[inline(always)]
    pub fn misc_remap(&self) -> MISC_REMAP_R {
        MISC_REMAP_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MAPR2")
            .field("tim15_remap", &self.tim15_remap())
            .field("tim16_remap", &self.tim16_remap())
            .field("tim17_remap", &self.tim17_remap())
            .field("tim13_remap", &self.tim13_remap())
            .field("tim14_remap", &self.tim14_remap())
            .field("fsmc_nadv", &self.fsmc_nadv())
            .field("cec_remap", &self.cec_remap())
            .field("tim1_dma_remap", &self.tim1_dma_remap())
            .field("tim67_dac_dma_remap", &self.tim67_dac_dma_remap())
            .field("tim12_remap", &self.tim12_remap())
            .field("misc_remap", &self.misc_remap())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM15 remapping
    #[inline(always)]
    pub fn tim15_remap(&mut self) -> TIM15_REMAP_W<'_, MAPR2rs> {
        TIM15_REMAP_W::new(self, 0)
    }
    ///Bit 1 - TIM16 remapping
    #[inline(always)]
    pub fn tim16_remap(&mut self) -> TIM16_REMAP_W<'_, MAPR2rs> {
        TIM16_REMAP_W::new(self, 1)
    }
    ///Bit 2 - TIM17 remapping
    #[inline(always)]
    pub fn tim17_remap(&mut self) -> TIM17_REMAP_W<'_, MAPR2rs> {
        TIM17_REMAP_W::new(self, 2)
    }
    ///Bit 3 - CEC remapping
    #[inline(always)]
    pub fn cec_remap(&mut self) -> CEC_REMAP_W<'_, MAPR2rs> {
        CEC_REMAP_W::new(self, 3)
    }
    ///Bit 4 - TIM1 DMA remapping
    #[inline(always)]
    pub fn tim1_dma_remap(&mut self) -> TIM1_DMA_REMAP_W<'_, MAPR2rs> {
        TIM1_DMA_REMAP_W::new(self, 4)
    }
    ///Bit 8 - TIM13 remapping
    #[inline(always)]
    pub fn tim13_remap(&mut self) -> TIM13_REMAP_W<'_, MAPR2rs> {
        TIM13_REMAP_W::new(self, 8)
    }
    ///Bit 9 - TIM14 remapping
    #[inline(always)]
    pub fn tim14_remap(&mut self) -> TIM14_REMAP_W<'_, MAPR2rs> {
        TIM14_REMAP_W::new(self, 9)
    }
    ///Bit 10 - NADV connect/disconnect
    #[inline(always)]
    pub fn fsmc_nadv(&mut self) -> FSMC_NADV_W<'_, MAPR2rs> {
        FSMC_NADV_W::new(self, 10)
    }
    ///Bit 11 - TIM67_DAC DMA remapping
    #[inline(always)]
    pub fn tim67_dac_dma_remap(&mut self) -> TIM67_DAC_DMA_REMAP_W<'_, MAPR2rs> {
        TIM67_DAC_DMA_REMAP_W::new(self, 11)
    }
    ///Bit 12 - TIM12 remapping
    #[inline(always)]
    pub fn tim12_remap(&mut self) -> TIM12_REMAP_W<'_, MAPR2rs> {
        TIM12_REMAP_W::new(self, 12)
    }
    ///Bit 13 - Miscellaneous features remapping
    #[inline(always)]
    pub fn misc_remap(&mut self) -> MISC_REMAP_W<'_, MAPR2rs> {
        MISC_REMAP_W::new(self, 13)
    }
}
/**AF remap and debug I/O configuration register

You can [`read`](crate::Reg::read) this register and get [`mapr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mapr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F100.html#AFIO:MAPR2)*/
pub struct MAPR2rs;
impl crate::RegisterSpec for MAPR2rs {
    type Ux = u32;
}
///`read()` method returns [`mapr2::R`](R) reader structure
impl crate::Readable for MAPR2rs {}
///`write(|w| ..)` method takes [`mapr2::W`](W) writer structure
impl crate::Writable for MAPR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MAPR2 to value 0
impl crate::Resettable for MAPR2rs {}

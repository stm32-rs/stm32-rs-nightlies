///Register `TIM16_CR2` reader
pub type R = crate::R<TIM16_CR2rs>;
///Register `TIM16_CR2` writer
pub type W = crate::W<TIM16_CR2rs>;
///Field `CCPC` reader - Capture/compare preloaded control
pub type CCPC_R = crate::BitReader;
///Field `CCPC` writer - Capture/compare preloaded control
pub type CCPC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCUS` reader - Capture/compare control update selection
pub type CCUS_R = crate::BitReader;
///Field `CCUS` writer - Capture/compare control update selection
pub type CCUS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCDS` reader - Capture/compare DMA selection
pub type CCDS_R = crate::BitReader;
///Field `CCDS` writer - Capture/compare DMA selection
pub type CCDS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OIS1` reader - Output Idle state 1 (OC1 output)
pub type OIS1_R = crate::BitReader;
///Field `OIS1` writer - Output Idle state 1 (OC1 output)
pub type OIS1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OIS1N` reader - Output Idle state 1 (OC1N output)
pub type OIS1N_R = crate::BitReader;
///Field `OIS1N` writer - Output Idle state 1 (OC1N output)
pub type OIS1N_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Capture/compare preloaded control
    #[inline(always)]
    pub fn ccpc(&self) -> CCPC_R {
        CCPC_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - Capture/compare control update selection
    #[inline(always)]
    pub fn ccus(&self) -> CCUS_R {
        CCUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Capture/compare DMA selection
    #[inline(always)]
    pub fn ccds(&self) -> CCDS_R {
        CCDS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 8 - Output Idle state 1 (OC1 output)
    #[inline(always)]
    pub fn ois1(&self) -> OIS1_R {
        OIS1_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Output Idle state 1 (OC1N output)
    #[inline(always)]
    pub fn ois1n(&self) -> OIS1N_R {
        OIS1N_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM16_CR2")
            .field("ccpc", &self.ccpc())
            .field("ccus", &self.ccus())
            .field("ccds", &self.ccds())
            .field("ois1", &self.ois1())
            .field("ois1n", &self.ois1n())
            .finish()
    }
}
impl W {
    ///Bit 0 - Capture/compare preloaded control
    #[inline(always)]
    pub fn ccpc(&mut self) -> CCPC_W<TIM16_CR2rs> {
        CCPC_W::new(self, 0)
    }
    ///Bit 2 - Capture/compare control update selection
    #[inline(always)]
    pub fn ccus(&mut self) -> CCUS_W<TIM16_CR2rs> {
        CCUS_W::new(self, 2)
    }
    ///Bit 3 - Capture/compare DMA selection
    #[inline(always)]
    pub fn ccds(&mut self) -> CCDS_W<TIM16_CR2rs> {
        CCDS_W::new(self, 3)
    }
    ///Bit 8 - Output Idle state 1 (OC1 output)
    #[inline(always)]
    pub fn ois1(&mut self) -> OIS1_W<TIM16_CR2rs> {
        OIS1_W::new(self, 8)
    }
    ///Bit 9 - Output Idle state 1 (OC1N output)
    #[inline(always)]
    pub fn ois1n(&mut self) -> OIS1N_W<TIM16_CR2rs> {
        OIS1N_W::new(self, 9)
    }
}
/**TIM16 control register 2

You can [`read`](crate::Reg::read) this register and get [`tim16_cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim16_cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#TIM16:TIM16_CR2)*/
pub struct TIM16_CR2rs;
impl crate::RegisterSpec for TIM16_CR2rs {
    type Ux = u16;
}
///`read()` method returns [`tim16_cr2::R`](R) reader structure
impl crate::Readable for TIM16_CR2rs {}
///`write(|w| ..)` method takes [`tim16_cr2::W`](W) writer structure
impl crate::Writable for TIM16_CR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
///`reset()` method sets TIM16_CR2 to value 0
impl crate::Resettable for TIM16_CR2rs {
    const RESET_VALUE: u16 = 0;
}

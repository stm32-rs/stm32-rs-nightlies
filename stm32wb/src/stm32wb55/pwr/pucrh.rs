///Register `PUCRH` reader
pub type R = crate::R<PUCRHrs>;
///Register `PUCRH` writer
pub type W = crate::W<PUCRHrs>;
///Field `PU0` reader - Port H pull-up bit y (y=0..1)
pub type PU0_R = crate::BitReader;
///Field `PU0` writer - Port H pull-up bit y (y=0..1)
pub type PU0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PU1` reader - Port H pull-up bit y (y=0..1)
pub type PU1_R = crate::BitReader;
///Field `PU1` writer - Port H pull-up bit y (y=0..1)
pub type PU1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PU3` reader - Port H pull-up bit y (y=0..1)
pub type PU3_R = crate::BitReader;
///Field `PU3` writer - Port H pull-up bit y (y=0..1)
pub type PU3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Port H pull-up bit y (y=0..1)
    #[inline(always)]
    pub fn pu0(&self) -> PU0_R {
        PU0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port H pull-up bit y (y=0..1)
    #[inline(always)]
    pub fn pu1(&self) -> PU1_R {
        PU1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - Port H pull-up bit y (y=0..1)
    #[inline(always)]
    pub fn pu3(&self) -> PU3_R {
        PU3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PUCRH")
            .field("pu3", &self.pu3())
            .field("pu1", &self.pu1())
            .field("pu0", &self.pu0())
            .finish()
    }
}
impl W {
    ///Bit 0 - Port H pull-up bit y (y=0..1)
    #[inline(always)]
    pub fn pu0(&mut self) -> PU0_W<'_, PUCRHrs> {
        PU0_W::new(self, 0)
    }
    ///Bit 1 - Port H pull-up bit y (y=0..1)
    #[inline(always)]
    pub fn pu1(&mut self) -> PU1_W<'_, PUCRHrs> {
        PU1_W::new(self, 1)
    }
    ///Bit 3 - Port H pull-up bit y (y=0..1)
    #[inline(always)]
    pub fn pu3(&mut self) -> PU3_W<'_, PUCRHrs> {
        PU3_W::new(self, 3)
    }
}
/**Power Port H pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pucrh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucrh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#PWR:PUCRH)*/
pub struct PUCRHrs;
impl crate::RegisterSpec for PUCRHrs {
    type Ux = u32;
}
///`read()` method returns [`pucrh::R`](R) reader structure
impl crate::Readable for PUCRHrs {}
///`write(|w| ..)` method takes [`pucrh::W`](W) writer structure
impl crate::Writable for PUCRHrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PUCRH to value 0
impl crate::Resettable for PUCRHrs {}

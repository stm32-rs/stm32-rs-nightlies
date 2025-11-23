///Register `PUCRE` reader
pub type R = crate::R<PUCRErs>;
///Register `PUCRE` writer
pub type W = crate::W<PUCRErs>;
///Field `PU0` reader - Port E pull-up bit y (y=0..15)
pub type PU0_R = crate::BitReader;
///Field `PU0` writer - Port E pull-up bit y (y=0..15)
pub type PU0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PU1` reader - Port E pull-up bit y (y=0..15)
pub type PU1_R = crate::BitReader;
///Field `PU1` writer - Port E pull-up bit y (y=0..15)
pub type PU1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PU2` reader - Port E pull-up bit y (y=0..15)
pub type PU2_R = crate::BitReader;
///Field `PU2` writer - Port E pull-up bit y (y=0..15)
pub type PU2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PU3` reader - Port E pull-up bit y (y=0..15)
pub type PU3_R = crate::BitReader;
///Field `PU3` writer - Port E pull-up bit y (y=0..15)
pub type PU3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PU4` reader - Port E pull-up bit y (y=0..15)
pub type PU4_R = crate::BitReader;
///Field `PU4` writer - Port E pull-up bit y (y=0..15)
pub type PU4_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Port E pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu0(&self) -> PU0_R {
        PU0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port E pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu1(&self) -> PU1_R {
        PU1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Port E pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu2(&self) -> PU2_R {
        PU2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Port E pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu3(&self) -> PU3_R {
        PU3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Port E pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu4(&self) -> PU4_R {
        PU4_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PUCRE")
            .field("pu4", &self.pu4())
            .field("pu3", &self.pu3())
            .field("pu2", &self.pu2())
            .field("pu1", &self.pu1())
            .field("pu0", &self.pu0())
            .finish()
    }
}
impl W {
    ///Bit 0 - Port E pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu0(&mut self) -> PU0_W<'_, PUCRErs> {
        PU0_W::new(self, 0)
    }
    ///Bit 1 - Port E pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu1(&mut self) -> PU1_W<'_, PUCRErs> {
        PU1_W::new(self, 1)
    }
    ///Bit 2 - Port E pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu2(&mut self) -> PU2_W<'_, PUCRErs> {
        PU2_W::new(self, 2)
    }
    ///Bit 3 - Port E pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu3(&mut self) -> PU3_W<'_, PUCRErs> {
        PU3_W::new(self, 3)
    }
    ///Bit 4 - Port E pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu4(&mut self) -> PU4_W<'_, PUCRErs> {
        PU4_W::new(self, 4)
    }
}
/**Power Port E pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pucre::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucre::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#PWR:PUCRE)*/
pub struct PUCRErs;
impl crate::RegisterSpec for PUCRErs {
    type Ux = u32;
}
///`read()` method returns [`pucre::R`](R) reader structure
impl crate::Readable for PUCRErs {}
///`write(|w| ..)` method takes [`pucre::W`](W) writer structure
impl crate::Writable for PUCRErs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PUCRE to value 0
impl crate::Resettable for PUCRErs {}

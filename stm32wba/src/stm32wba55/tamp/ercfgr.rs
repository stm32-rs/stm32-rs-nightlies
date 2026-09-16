///Register `ERCFGR` reader
pub type R = crate::R<ERCFGRrs>;
///Register `ERCFGR` writer
pub type W = crate::W<ERCFGRrs>;
///Field `ERCFG1` reader - Configurable device secrets configuration
pub type ERCFG1_R = crate::BitReader;
///Field `ERCFG1` writer - Configurable device secrets configuration
pub type ERCFG1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERCFG2` reader - Configurable device secrets configuration
pub type ERCFG2_R = crate::BitReader;
///Field `ERCFG2` writer - Configurable device secrets configuration
pub type ERCFG2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERCFG3` reader - Configurable device secrets configuration
pub type ERCFG3_R = crate::BitReader;
///Field `ERCFG3` writer - Configurable device secrets configuration
pub type ERCFG3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERCFG4` reader - Configurable device secrets configuration
pub type ERCFG4_R = crate::BitReader;
///Field `ERCFG4` writer - Configurable device secrets configuration
pub type ERCFG4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERCFG5` reader - Configurable device secrets configuration
pub type ERCFG5_R = crate::BitReader;
///Field `ERCFG5` writer - Configurable device secrets configuration
pub type ERCFG5_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - Configurable device secrets configuration
    #[inline(always)]
    pub fn ercfg1(&self) -> ERCFG1_R {
        ERCFG1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Configurable device secrets configuration
    #[inline(always)]
    pub fn ercfg2(&self) -> ERCFG2_R {
        ERCFG2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Configurable device secrets configuration
    #[inline(always)]
    pub fn ercfg3(&self) -> ERCFG3_R {
        ERCFG3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Configurable device secrets configuration
    #[inline(always)]
    pub fn ercfg4(&self) -> ERCFG4_R {
        ERCFG4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Configurable device secrets configuration
    #[inline(always)]
    pub fn ercfg5(&self) -> ERCFG5_R {
        ERCFG5_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ERCFGR")
            .field("ercfg1", &self.ercfg1())
            .field("ercfg2", &self.ercfg2())
            .field("ercfg3", &self.ercfg3())
            .field("ercfg4", &self.ercfg4())
            .field("ercfg5", &self.ercfg5())
            .finish()
    }
}
impl W {
    ///Bit 1 - Configurable device secrets configuration
    #[inline(always)]
    pub fn ercfg1(&mut self) -> ERCFG1_W<'_, ERCFGRrs> {
        ERCFG1_W::new(self, 1)
    }
    ///Bit 2 - Configurable device secrets configuration
    #[inline(always)]
    pub fn ercfg2(&mut self) -> ERCFG2_W<'_, ERCFGRrs> {
        ERCFG2_W::new(self, 2)
    }
    ///Bit 3 - Configurable device secrets configuration
    #[inline(always)]
    pub fn ercfg3(&mut self) -> ERCFG3_W<'_, ERCFGRrs> {
        ERCFG3_W::new(self, 3)
    }
    ///Bit 4 - Configurable device secrets configuration
    #[inline(always)]
    pub fn ercfg4(&mut self) -> ERCFG4_W<'_, ERCFGRrs> {
        ERCFG4_W::new(self, 4)
    }
    ///Bit 5 - Configurable device secrets configuration
    #[inline(always)]
    pub fn ercfg5(&mut self) -> ERCFG5_W<'_, ERCFGRrs> {
        ERCFG5_W::new(self, 5)
    }
}
/**TAMP erase configuration register

You can [`read`](crate::Reg::read) this register and get [`ercfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ercfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#TAMP:ERCFGR)*/
pub struct ERCFGRrs;
impl crate::RegisterSpec for ERCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`ercfgr::R`](R) reader structure
impl crate::Readable for ERCFGRrs {}
///`write(|w| ..)` method takes [`ercfgr::W`](W) writer structure
impl crate::Writable for ERCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ERCFGR to value 0
impl crate::Resettable for ERCFGRrs {}

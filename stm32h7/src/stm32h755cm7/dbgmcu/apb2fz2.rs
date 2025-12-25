///Register `APB2FZ2` reader
pub type R = crate::R<APB2FZ2rs>;
///Register `APB2FZ2` writer
pub type W = crate::W<APB2FZ2rs>;
///Field `DBG_TIM1` reader - TIM1 stop in debug
pub type DBG_TIM1_R = crate::BitReader;
///Field `DBG_TIM1` writer - TIM1 stop in debug
pub type DBG_TIM1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM8` reader - TIM8 stop in debug
pub type DBG_TIM8_R = crate::BitReader;
///Field `DBG_TIM8` writer - TIM8 stop in debug
pub type DBG_TIM8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM15` reader - TIM15 stop in debug
pub type DBG_TIM15_R = crate::BitReader;
///Field `DBG_TIM15` writer - TIM15 stop in debug
pub type DBG_TIM15_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM16` reader - TIM16 stop in debug
pub type DBG_TIM16_R = crate::BitReader;
///Field `DBG_TIM16` writer - TIM16 stop in debug
pub type DBG_TIM16_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM17` reader - TIM17 stop in debug
pub type DBG_TIM17_R = crate::BitReader;
///Field `DBG_TIM17` writer - TIM17 stop in debug
pub type DBG_TIM17_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_HRTIM` reader - HRTIM stop in debug
pub type DBG_HRTIM_R = crate::BitReader;
///Field `DBG_HRTIM` writer - HRTIM stop in debug
pub type DBG_HRTIM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TIM1 stop in debug
    #[inline(always)]
    pub fn dbg_tim1(&self) -> DBG_TIM1_R {
        DBG_TIM1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM8 stop in debug
    #[inline(always)]
    pub fn dbg_tim8(&self) -> DBG_TIM8_R {
        DBG_TIM8_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 16 - TIM15 stop in debug
    #[inline(always)]
    pub fn dbg_tim15(&self) -> DBG_TIM15_R {
        DBG_TIM15_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TIM16 stop in debug
    #[inline(always)]
    pub fn dbg_tim16(&self) -> DBG_TIM16_R {
        DBG_TIM16_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TIM17 stop in debug
    #[inline(always)]
    pub fn dbg_tim17(&self) -> DBG_TIM17_R {
        DBG_TIM17_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 29 - HRTIM stop in debug
    #[inline(always)]
    pub fn dbg_hrtim(&self) -> DBG_HRTIM_R {
        DBG_HRTIM_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2FZ2")
            .field("dbg_tim1", &self.dbg_tim1())
            .field("dbg_tim8", &self.dbg_tim8())
            .field("dbg_tim15", &self.dbg_tim15())
            .field("dbg_tim16", &self.dbg_tim16())
            .field("dbg_tim17", &self.dbg_tim17())
            .field("dbg_hrtim", &self.dbg_hrtim())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM1 stop in debug
    #[inline(always)]
    pub fn dbg_tim1(&mut self) -> DBG_TIM1_W<'_, APB2FZ2rs> {
        DBG_TIM1_W::new(self, 0)
    }
    ///Bit 1 - TIM8 stop in debug
    #[inline(always)]
    pub fn dbg_tim8(&mut self) -> DBG_TIM8_W<'_, APB2FZ2rs> {
        DBG_TIM8_W::new(self, 1)
    }
    ///Bit 16 - TIM15 stop in debug
    #[inline(always)]
    pub fn dbg_tim15(&mut self) -> DBG_TIM15_W<'_, APB2FZ2rs> {
        DBG_TIM15_W::new(self, 16)
    }
    ///Bit 17 - TIM16 stop in debug
    #[inline(always)]
    pub fn dbg_tim16(&mut self) -> DBG_TIM16_W<'_, APB2FZ2rs> {
        DBG_TIM16_W::new(self, 17)
    }
    ///Bit 18 - TIM17 stop in debug
    #[inline(always)]
    pub fn dbg_tim17(&mut self) -> DBG_TIM17_W<'_, APB2FZ2rs> {
        DBG_TIM17_W::new(self, 18)
    }
    ///Bit 29 - HRTIM stop in debug
    #[inline(always)]
    pub fn dbg_hrtim(&mut self) -> DBG_HRTIM_W<'_, APB2FZ2rs> {
        DBG_HRTIM_W::new(self, 29)
    }
}
/**DBGMCU APB2 peripheral freeze register CPU2

You can [`read`](crate::Reg::read) this register and get [`apb2fz2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2fz2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM7.html#DBGMCU:APB2FZ2)*/
pub struct APB2FZ2rs;
impl crate::RegisterSpec for APB2FZ2rs {
    type Ux = u32;
}
///`read()` method returns [`apb2fz2::R`](R) reader structure
impl crate::Readable for APB2FZ2rs {}
///`write(|w| ..)` method takes [`apb2fz2::W`](W) writer structure
impl crate::Writable for APB2FZ2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB2FZ2 to value 0
impl crate::Resettable for APB2FZ2rs {}

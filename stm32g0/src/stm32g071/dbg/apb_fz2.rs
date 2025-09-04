///Register `APB_FZ2` reader
pub type R = crate::R<APB_FZ2rs>;
///Register `APB_FZ2` writer
pub type W = crate::W<APB_FZ2rs>;
///Field `DBG_TIM1_STOP` reader - DBG_TIM1_STOP
pub type DBG_TIM1_STOP_R = crate::BitReader;
///Field `DBG_TIM1_STOP` writer - DBG_TIM1_STOP
pub type DBG_TIM1_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM14_STOP` reader - DBG_TIM14_STOP
pub type DBG_TIM14_STOP_R = crate::BitReader;
///Field `DBG_TIM14_STOP` writer - DBG_TIM14_STOP
pub type DBG_TIM14_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM15_STOP` reader - DBG_TIM15_STOP
pub type DBG_TIM15_STOP_R = crate::BitReader;
///Field `DBG_TIM15_STOP` writer - DBG_TIM15_STOP
pub type DBG_TIM15_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM16_STOP` reader - DBG_TIM16_STOP
pub type DBG_TIM16_STOP_R = crate::BitReader;
///Field `DBG_TIM16_STOP` writer - DBG_TIM16_STOP
pub type DBG_TIM16_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM17_STOP` reader - DBG_TIM17_STOP
pub type DBG_TIM17_STOP_R = crate::BitReader;
///Field `DBG_TIM17_STOP` writer - DBG_TIM17_STOP
pub type DBG_TIM17_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 11 - DBG_TIM1_STOP
    #[inline(always)]
    pub fn dbg_tim1_stop(&self) -> DBG_TIM1_STOP_R {
        DBG_TIM1_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 15 - DBG_TIM14_STOP
    #[inline(always)]
    pub fn dbg_tim14_stop(&self) -> DBG_TIM14_STOP_R {
        DBG_TIM14_STOP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - DBG_TIM15_STOP
    #[inline(always)]
    pub fn dbg_tim15_stop(&self) -> DBG_TIM15_STOP_R {
        DBG_TIM15_STOP_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - DBG_TIM16_STOP
    #[inline(always)]
    pub fn dbg_tim16_stop(&self) -> DBG_TIM16_STOP_R {
        DBG_TIM16_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - DBG_TIM17_STOP
    #[inline(always)]
    pub fn dbg_tim17_stop(&self) -> DBG_TIM17_STOP_R {
        DBG_TIM17_STOP_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_FZ2")
            .field("dbg_tim1_stop", &self.dbg_tim1_stop())
            .field("dbg_tim14_stop", &self.dbg_tim14_stop())
            .field("dbg_tim15_stop", &self.dbg_tim15_stop())
            .field("dbg_tim16_stop", &self.dbg_tim16_stop())
            .field("dbg_tim17_stop", &self.dbg_tim17_stop())
            .finish()
    }
}
impl W {
    ///Bit 11 - DBG_TIM1_STOP
    #[inline(always)]
    pub fn dbg_tim1_stop(&mut self) -> DBG_TIM1_STOP_W<APB_FZ2rs> {
        DBG_TIM1_STOP_W::new(self, 11)
    }
    ///Bit 15 - DBG_TIM14_STOP
    #[inline(always)]
    pub fn dbg_tim14_stop(&mut self) -> DBG_TIM14_STOP_W<APB_FZ2rs> {
        DBG_TIM14_STOP_W::new(self, 15)
    }
    ///Bit 16 - DBG_TIM15_STOP
    #[inline(always)]
    pub fn dbg_tim15_stop(&mut self) -> DBG_TIM15_STOP_W<APB_FZ2rs> {
        DBG_TIM15_STOP_W::new(self, 16)
    }
    ///Bit 17 - DBG_TIM16_STOP
    #[inline(always)]
    pub fn dbg_tim16_stop(&mut self) -> DBG_TIM16_STOP_W<APB_FZ2rs> {
        DBG_TIM16_STOP_W::new(self, 17)
    }
    ///Bit 18 - DBG_TIM17_STOP
    #[inline(always)]
    pub fn dbg_tim17_stop(&mut self) -> DBG_TIM17_STOP_W<APB_FZ2rs> {
        DBG_TIM17_STOP_W::new(self, 18)
    }
}
/**DBG APB freeze register 2

You can [`read`](crate::Reg::read) this register and get [`apb_fz2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb_fz2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G071.html#DBG:APB_FZ2)*/
pub struct APB_FZ2rs;
impl crate::RegisterSpec for APB_FZ2rs {
    type Ux = u32;
}
///`read()` method returns [`apb_fz2::R`](R) reader structure
impl crate::Readable for APB_FZ2rs {}
///`write(|w| ..)` method takes [`apb_fz2::W`](W) writer structure
impl crate::Writable for APB_FZ2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB_FZ2 to value 0
impl crate::Resettable for APB_FZ2rs {}

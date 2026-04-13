///Register `APB2FZ1` reader
pub type R = crate::R<APB2FZ1rs>;
///Register `APB2FZ1` writer
pub type W = crate::W<APB2FZ1rs>;
///Field `DBG_TIM1_STOP` reader - TIM1 stop in debug
pub type DBG_TIM1_STOP_R = crate::BitReader;
///Field `DBG_TIM1_STOP` writer - TIM1 stop in debug
pub type DBG_TIM1_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM8_STOP` reader - TIM8 stop in debug
pub type DBG_TIM8_STOP_R = crate::BitReader;
///Field `DBG_TIM8_STOP` writer - TIM8 stop in debug
pub type DBG_TIM8_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM18_STOP` reader - TIM18 stop in debug
pub type DBG_TIM18_STOP_R = crate::BitReader;
///Field `DBG_TIM18_STOP` writer - TIM18 stop in debug
pub type DBG_TIM18_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM15_STOP` reader - TIM15 stop in debug
pub type DBG_TIM15_STOP_R = crate::BitReader;
///Field `DBG_TIM15_STOP` writer - TIM15 stop in debug
pub type DBG_TIM15_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM16_STOP` reader - TIM16 stop in debug
pub type DBG_TIM16_STOP_R = crate::BitReader;
///Field `DBG_TIM16_STOP` writer - TIM16 stop in debug
pub type DBG_TIM16_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM17_STOP` reader - TIM17 stop in debug
pub type DBG_TIM17_STOP_R = crate::BitReader;
///Field `DBG_TIM17_STOP` writer - TIM17 stop in debug
pub type DBG_TIM17_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM9_STOP` reader - TIM9 stop in debug
pub type DBG_TIM9_STOP_R = crate::BitReader;
///Field `DBG_TIM9_STOP` writer - TIM9 stop in debug
pub type DBG_TIM9_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TIM1 stop in debug
    #[inline(always)]
    pub fn dbg_tim1_stop(&self) -> DBG_TIM1_STOP_R {
        DBG_TIM1_STOP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM8 stop in debug
    #[inline(always)]
    pub fn dbg_tim8_stop(&self) -> DBG_TIM8_STOP_R {
        DBG_TIM8_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 15 - TIM18 stop in debug
    #[inline(always)]
    pub fn dbg_tim18_stop(&self) -> DBG_TIM18_STOP_R {
        DBG_TIM18_STOP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - TIM15 stop in debug
    #[inline(always)]
    pub fn dbg_tim15_stop(&self) -> DBG_TIM15_STOP_R {
        DBG_TIM15_STOP_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TIM16 stop in debug
    #[inline(always)]
    pub fn dbg_tim16_stop(&self) -> DBG_TIM16_STOP_R {
        DBG_TIM16_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TIM17 stop in debug
    #[inline(always)]
    pub fn dbg_tim17_stop(&self) -> DBG_TIM17_STOP_R {
        DBG_TIM17_STOP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - TIM9 stop in debug
    #[inline(always)]
    pub fn dbg_tim9_stop(&self) -> DBG_TIM9_STOP_R {
        DBG_TIM9_STOP_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2FZ1")
            .field("dbg_tim1_stop", &self.dbg_tim1_stop())
            .field("dbg_tim8_stop", &self.dbg_tim8_stop())
            .field("dbg_tim18_stop", &self.dbg_tim18_stop())
            .field("dbg_tim15_stop", &self.dbg_tim15_stop())
            .field("dbg_tim16_stop", &self.dbg_tim16_stop())
            .field("dbg_tim17_stop", &self.dbg_tim17_stop())
            .field("dbg_tim9_stop", &self.dbg_tim9_stop())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM1 stop in debug
    #[inline(always)]
    pub fn dbg_tim1_stop(&mut self) -> DBG_TIM1_STOP_W<'_, APB2FZ1rs> {
        DBG_TIM1_STOP_W::new(self, 0)
    }
    ///Bit 1 - TIM8 stop in debug
    #[inline(always)]
    pub fn dbg_tim8_stop(&mut self) -> DBG_TIM8_STOP_W<'_, APB2FZ1rs> {
        DBG_TIM8_STOP_W::new(self, 1)
    }
    ///Bit 15 - TIM18 stop in debug
    #[inline(always)]
    pub fn dbg_tim18_stop(&mut self) -> DBG_TIM18_STOP_W<'_, APB2FZ1rs> {
        DBG_TIM18_STOP_W::new(self, 15)
    }
    ///Bit 16 - TIM15 stop in debug
    #[inline(always)]
    pub fn dbg_tim15_stop(&mut self) -> DBG_TIM15_STOP_W<'_, APB2FZ1rs> {
        DBG_TIM15_STOP_W::new(self, 16)
    }
    ///Bit 17 - TIM16 stop in debug
    #[inline(always)]
    pub fn dbg_tim16_stop(&mut self) -> DBG_TIM16_STOP_W<'_, APB2FZ1rs> {
        DBG_TIM16_STOP_W::new(self, 17)
    }
    ///Bit 18 - TIM17 stop in debug
    #[inline(always)]
    pub fn dbg_tim17_stop(&mut self) -> DBG_TIM17_STOP_W<'_, APB2FZ1rs> {
        DBG_TIM17_STOP_W::new(self, 18)
    }
    ///Bit 19 - TIM9 stop in debug
    #[inline(always)]
    pub fn dbg_tim9_stop(&mut self) -> DBG_TIM9_STOP_W<'_, APB2FZ1rs> {
        DBG_TIM9_STOP_W::new(self, 19)
    }
}
/**DBGMCU APB2 peripheral freeze register

You can [`read`](crate::Reg::read) this register and get [`apb2fz1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2fz1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#DBGMCU:APB2FZ1)*/
pub struct APB2FZ1rs;
impl crate::RegisterSpec for APB2FZ1rs {
    type Ux = u32;
}
///`read()` method returns [`apb2fz1::R`](R) reader structure
impl crate::Readable for APB2FZ1rs {}
///`write(|w| ..)` method takes [`apb2fz1::W`](W) writer structure
impl crate::Writable for APB2FZ1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB2FZ1 to value 0
impl crate::Resettable for APB2FZ1rs {}

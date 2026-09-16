///Register `DBG_APB0_FZ` reader
pub type R = crate::R<DBG_APB0_FZrs>;
///Register `DBG_APB0_FZ` writer
pub type W = crate::W<DBG_APB0_FZrs>;
///Field `DBG_TIM2_STOP` reader - TIM2 stop in the CPU debug - 0: Normal operation. TIM2 continues to operate while the CPU is in debug mode - 1: Stop in debug. TIM2 is frozen while the CPU is in debug mode.
pub type DBG_TIM2_STOP_R = crate::BitReader;
///Field `DBG_TIM2_STOP` writer - TIM2 stop in the CPU debug - 0: Normal operation. TIM2 continues to operate while the CPU is in debug mode - 1: Stop in debug. TIM2 is frozen while the CPU is in debug mode.
pub type DBG_TIM2_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM16_STOP` reader - TIM16 stop in the CPU debug - 0: Normal operation. TIM16 continues to operate while the CPU is in debug mode - 1: Stop in debug. TIM16 is frozen while the CPU is in debug mode.
pub type DBG_TIM16_STOP_R = crate::BitReader;
///Field `DBG_TIM16_STOP` writer - TIM16 stop in the CPU debug - 0: Normal operation. TIM16 continues to operate while the CPU is in debug mode - 1: Stop in debug. TIM16 is frozen while the CPU is in debug mode.
pub type DBG_TIM16_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_RTC_STOP` reader - RTC stop in CPU debug - 0: Normal operation. RTC continues to operate while the CPU is in debug mode - 1: Stop in debug. RTC is frozen while the CPU is in debug mode.
pub type DBG_RTC_STOP_R = crate::BitReader;
///Field `DBG_RTC_STOP` writer - RTC stop in CPU debug - 0: Normal operation. RTC continues to operate while the CPU is in debug mode - 1: Stop in debug. RTC is frozen while the CPU is in debug mode.
pub type DBG_RTC_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_IWDG_STOP` reader - IWDG stop in the CPU debug - 0: Normal operation. IWDG continues to operate while the CPU is in debug mode - 1: Stop in debug. IWDG is frozen while the CPU is in debug mode.
pub type DBG_IWDG_STOP_R = crate::BitReader;
///Field `DBG_IWDG_STOP` writer - IWDG stop in the CPU debug - 0: Normal operation. IWDG continues to operate while the CPU is in debug mode - 1: Stop in debug. IWDG is frozen while the CPU is in debug mode.
pub type DBG_IWDG_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TIM2 stop in the CPU debug - 0: Normal operation. TIM2 continues to operate while the CPU is in debug mode - 1: Stop in debug. TIM2 is frozen while the CPU is in debug mode.
    #[inline(always)]
    pub fn dbg_tim2_stop(&self) -> DBG_TIM2_STOP_R {
        DBG_TIM2_STOP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM16 stop in the CPU debug - 0: Normal operation. TIM16 continues to operate while the CPU is in debug mode - 1: Stop in debug. TIM16 is frozen while the CPU is in debug mode.
    #[inline(always)]
    pub fn dbg_tim16_stop(&self) -> DBG_TIM16_STOP_R {
        DBG_TIM16_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 12 - RTC stop in CPU debug - 0: Normal operation. RTC continues to operate while the CPU is in debug mode - 1: Stop in debug. RTC is frozen while the CPU is in debug mode.
    #[inline(always)]
    pub fn dbg_rtc_stop(&self) -> DBG_RTC_STOP_R {
        DBG_RTC_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - IWDG stop in the CPU debug - 0: Normal operation. IWDG continues to operate while the CPU is in debug mode - 1: Stop in debug. IWDG is frozen while the CPU is in debug mode.
    #[inline(always)]
    pub fn dbg_iwdg_stop(&self) -> DBG_IWDG_STOP_R {
        DBG_IWDG_STOP_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBG_APB0_FZ")
            .field("dbg_tim2_stop", &self.dbg_tim2_stop())
            .field("dbg_tim16_stop", &self.dbg_tim16_stop())
            .field("dbg_rtc_stop", &self.dbg_rtc_stop())
            .field("dbg_iwdg_stop", &self.dbg_iwdg_stop())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM2 stop in the CPU debug - 0: Normal operation. TIM2 continues to operate while the CPU is in debug mode - 1: Stop in debug. TIM2 is frozen while the CPU is in debug mode.
    #[inline(always)]
    pub fn dbg_tim2_stop(&mut self) -> DBG_TIM2_STOP_W<'_, DBG_APB0_FZrs> {
        DBG_TIM2_STOP_W::new(self, 0)
    }
    ///Bit 1 - TIM16 stop in the CPU debug - 0: Normal operation. TIM16 continues to operate while the CPU is in debug mode - 1: Stop in debug. TIM16 is frozen while the CPU is in debug mode.
    #[inline(always)]
    pub fn dbg_tim16_stop(&mut self) -> DBG_TIM16_STOP_W<'_, DBG_APB0_FZrs> {
        DBG_TIM16_STOP_W::new(self, 1)
    }
    ///Bit 12 - RTC stop in CPU debug - 0: Normal operation. RTC continues to operate while the CPU is in debug mode - 1: Stop in debug. RTC is frozen while the CPU is in debug mode.
    #[inline(always)]
    pub fn dbg_rtc_stop(&mut self) -> DBG_RTC_STOP_W<'_, DBG_APB0_FZrs> {
        DBG_RTC_STOP_W::new(self, 12)
    }
    ///Bit 14 - IWDG stop in the CPU debug - 0: Normal operation. IWDG continues to operate while the CPU is in debug mode - 1: Stop in debug. IWDG is frozen while the CPU is in debug mode.
    #[inline(always)]
    pub fn dbg_iwdg_stop(&mut self) -> DBG_IWDG_STOP_W<'_, DBG_APB0_FZrs> {
        DBG_IWDG_STOP_W::new(self, 14)
    }
}
/**DBG_APB0_FZ register

You can [`read`](crate::Reg::read) this register and get [`dbg_apb0_fz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_apb0_fz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DBGMCU:DBG_APB0_FZ)*/
pub struct DBG_APB0_FZrs;
impl crate::RegisterSpec for DBG_APB0_FZrs {
    type Ux = u32;
}
///`read()` method returns [`dbg_apb0_fz::R`](R) reader structure
impl crate::Readable for DBG_APB0_FZrs {}
///`write(|w| ..)` method takes [`dbg_apb0_fz::W`](W) writer structure
impl crate::Writable for DBG_APB0_FZrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DBG_APB0_FZ to value 0
impl crate::Resettable for DBG_APB0_FZrs {}

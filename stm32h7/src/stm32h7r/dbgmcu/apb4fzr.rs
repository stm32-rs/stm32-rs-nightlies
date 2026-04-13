///Register `APB4FZR` reader
pub type R = crate::R<APB4FZRrs>;
///Register `APB4FZR` writer
pub type W = crate::W<APB4FZRrs>;
///Field `LPTIM2` reader - LPTIM2 stop in debug
pub type LPTIM2_R = crate::BitReader;
///Field `LPTIM2` writer - LPTIM2 stop in debug
pub type LPTIM2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM3` reader - LPTIM2 stop in debug
pub type LPTIM3_R = crate::BitReader;
///Field `LPTIM3` writer - LPTIM2 stop in debug
pub type LPTIM3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM4` reader - LPTIM4 stop in debug
pub type LPTIM4_R = crate::BitReader;
///Field `LPTIM4` writer - LPTIM4 stop in debug
pub type LPTIM4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM5` reader - LPTIM5 stop in debug
pub type LPTIM5_R = crate::BitReader;
///Field `LPTIM5` writer - LPTIM5 stop in debug
pub type LPTIM5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTC` reader - RTC stop in debug
pub type RTC_R = crate::BitReader;
///Field `RTC` writer - RTC stop in debug
pub type RTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWDG` reader - Independent watchdog for stop in debug
pub type IWDG_R = crate::BitReader;
///Field `IWDG` writer - Independent watchdog for stop in debug
pub type IWDG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 9 - LPTIM2 stop in debug
    #[inline(always)]
    pub fn lptim2(&self) -> LPTIM2_R {
        LPTIM2_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - LPTIM2 stop in debug
    #[inline(always)]
    pub fn lptim3(&self) -> LPTIM3_R {
        LPTIM3_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - LPTIM4 stop in debug
    #[inline(always)]
    pub fn lptim4(&self) -> LPTIM4_R {
        LPTIM4_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - LPTIM5 stop in debug
    #[inline(always)]
    pub fn lptim5(&self) -> LPTIM5_R {
        LPTIM5_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - RTC stop in debug
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - Independent watchdog for stop in debug
    #[inline(always)]
    pub fn iwdg(&self) -> IWDG_R {
        IWDG_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB4FZR")
            .field("lptim2", &self.lptim2())
            .field("lptim3", &self.lptim3())
            .field("lptim4", &self.lptim4())
            .field("lptim5", &self.lptim5())
            .field("rtc", &self.rtc())
            .field("iwdg", &self.iwdg())
            .finish()
    }
}
impl W {
    ///Bit 9 - LPTIM2 stop in debug
    #[inline(always)]
    pub fn lptim2(&mut self) -> LPTIM2_W<'_, APB4FZRrs> {
        LPTIM2_W::new(self, 9)
    }
    ///Bit 10 - LPTIM2 stop in debug
    #[inline(always)]
    pub fn lptim3(&mut self) -> LPTIM3_W<'_, APB4FZRrs> {
        LPTIM3_W::new(self, 10)
    }
    ///Bit 11 - LPTIM4 stop in debug
    #[inline(always)]
    pub fn lptim4(&mut self) -> LPTIM4_W<'_, APB4FZRrs> {
        LPTIM4_W::new(self, 11)
    }
    ///Bit 12 - LPTIM5 stop in debug
    #[inline(always)]
    pub fn lptim5(&mut self) -> LPTIM5_W<'_, APB4FZRrs> {
        LPTIM5_W::new(self, 12)
    }
    ///Bit 16 - RTC stop in debug
    #[inline(always)]
    pub fn rtc(&mut self) -> RTC_W<'_, APB4FZRrs> {
        RTC_W::new(self, 16)
    }
    ///Bit 18 - Independent watchdog for stop in debug
    #[inline(always)]
    pub fn iwdg(&mut self) -> IWDG_W<'_, APB4FZRrs> {
        IWDG_W::new(self, 18)
    }
}
/**DBGMCU APB4 peripheral freeze register

You can [`read`](crate::Reg::read) this register and get [`apb4fzr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4fzr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#DBGMCU:APB4FZR)*/
pub struct APB4FZRrs;
impl crate::RegisterSpec for APB4FZRrs {
    type Ux = u32;
}
///`read()` method returns [`apb4fzr::R`](R) reader structure
impl crate::Readable for APB4FZRrs {}
///`write(|w| ..)` method takes [`apb4fzr::W`](W) writer structure
impl crate::Writable for APB4FZRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB4FZR to value 0
impl crate::Resettable for APB4FZRrs {}

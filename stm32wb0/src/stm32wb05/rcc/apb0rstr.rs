///Register `APB0RSTR` reader
pub type R = crate::R<APB0RSTRrs>;
///Register `APB0RSTR` writer
pub type W = crate::W<APB0RSTRrs>;
///Field `TIM1RST` reader - TIM1: Advanced Timer reset Set and reset by software.
pub type TIM1RST_R = crate::BitReader;
///Field `TIM1RST` writer - TIM1: Advanced Timer reset Set and reset by software.
pub type TIM1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM16RST` reader - TIM16 reset
pub type TIM16RST_R = crate::BitReader;
///Field `TIM16RST` writer - TIM16 reset
pub type TIM16RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM17RST` reader - TIM17 reset
pub type TIM17RST_R = crate::BitReader;
///Field `TIM17RST` writer - TIM17 reset
pub type TIM17RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYSCFGRST` reader - SYSTEM CONFIG reset Set and reset by software.
pub type SYSCFGRST_R = crate::BitReader;
///Field `SYSCFGRST` writer - SYSTEM CONFIG reset Set and reset by software.
pub type SYSCFGRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTCRST` reader - RTC reset Set and reset by software.
pub type RTCRST_R = crate::BitReader;
///Field `RTCRST` writer - RTC reset Set and reset by software.
pub type RTCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WDRST` reader - WATCHDOG reset Set and reset by software.
pub type WDRST_R = crate::BitReader;
///Field `WDRST` writer - WATCHDOG reset Set and reset by software.
pub type WDRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TIM1: Advanced Timer reset Set and reset by software.
    #[inline(always)]
    pub fn tim1rst(&self) -> TIM1RST_R {
        TIM1RST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM16 reset
    #[inline(always)]
    pub fn tim16rst(&self) -> TIM16RST_R {
        TIM16RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TIM17 reset
    #[inline(always)]
    pub fn tim17rst(&self) -> TIM17RST_R {
        TIM17RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 8 - SYSTEM CONFIG reset Set and reset by software.
    #[inline(always)]
    pub fn syscfgrst(&self) -> SYSCFGRST_R {
        SYSCFGRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - RTC reset Set and reset by software.
    #[inline(always)]
    pub fn rtcrst(&self) -> RTCRST_R {
        RTCRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - WATCHDOG reset Set and reset by software.
    #[inline(always)]
    pub fn wdrst(&self) -> WDRST_R {
        WDRST_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB0RSTR")
            .field("tim1rst", &self.tim1rst())
            .field("tim16rst", &self.tim16rst())
            .field("tim17rst", &self.tim17rst())
            .field("syscfgrst", &self.syscfgrst())
            .field("rtcrst", &self.rtcrst())
            .field("wdrst", &self.wdrst())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM1: Advanced Timer reset Set and reset by software.
    #[inline(always)]
    pub fn tim1rst(&mut self) -> TIM1RST_W<'_, APB0RSTRrs> {
        TIM1RST_W::new(self, 0)
    }
    ///Bit 1 - TIM16 reset
    #[inline(always)]
    pub fn tim16rst(&mut self) -> TIM16RST_W<'_, APB0RSTRrs> {
        TIM16RST_W::new(self, 1)
    }
    ///Bit 2 - TIM17 reset
    #[inline(always)]
    pub fn tim17rst(&mut self) -> TIM17RST_W<'_, APB0RSTRrs> {
        TIM17RST_W::new(self, 2)
    }
    ///Bit 8 - SYSTEM CONFIG reset Set and reset by software.
    #[inline(always)]
    pub fn syscfgrst(&mut self) -> SYSCFGRST_W<'_, APB0RSTRrs> {
        SYSCFGRST_W::new(self, 8)
    }
    ///Bit 12 - RTC reset Set and reset by software.
    #[inline(always)]
    pub fn rtcrst(&mut self) -> RTCRST_W<'_, APB0RSTRrs> {
        RTCRST_W::new(self, 12)
    }
    ///Bit 14 - WATCHDOG reset Set and reset by software.
    #[inline(always)]
    pub fn wdrst(&mut self) -> WDRST_W<'_, APB0RSTRrs> {
        WDRST_W::new(self, 14)
    }
}
/**APB0RSTR register

You can [`read`](crate::Reg::read) this register and get [`apb0rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb0rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#RCC:APB0RSTR)*/
pub struct APB0RSTRrs;
impl crate::RegisterSpec for APB0RSTRrs {
    type Ux = u32;
}
///`read()` method returns [`apb0rstr::R`](R) reader structure
impl crate::Readable for APB0RSTRrs {}
///`write(|w| ..)` method takes [`apb0rstr::W`](W) writer structure
impl crate::Writable for APB0RSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB0RSTR to value 0
impl crate::Resettable for APB0RSTRrs {}

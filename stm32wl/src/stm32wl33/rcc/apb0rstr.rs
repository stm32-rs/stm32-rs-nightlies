///Register `APB0RSTR` reader
pub type R = crate::R<APB0RSTRrs>;
///Register `APB0RSTR` writer
pub type W = crate::W<APB0RSTRrs>;
///Field `TIM2RST` reader - TIM2RST: TIM2 reset. 0: TIM2 IP is not under reset. 1: TIM2 IP is under reset.
pub type TIM2RST_R = crate::BitReader;
///Field `TIM2RST` writer - TIM2RST: TIM2 reset. 0: TIM2 IP is not under reset. 1: TIM2 IP is under reset.
pub type TIM2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM16RST` reader - TIM16RST: TIM16 reset. 0: TIM16 IP is not under reset. 1: TIM16 IP is under reset.
pub type TIM16RST_R = crate::BitReader;
///Field `TIM16RST` writer - TIM16RST: TIM16 reset. 0: TIM16 IP is not under reset. 1: TIM16 IP is under reset.
pub type TIM16RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYSCFGRST` reader - SYSCFGRST: system controller reset. 0: system controller IP is not under reset. 1: system controller IP is under reset.
pub type SYSCFGRST_R = crate::BitReader;
///Field `SYSCFGRST` writer - SYSCFGRST: system controller reset. 0: system controller IP is not under reset. 1: system controller IP is under reset.
pub type SYSCFGRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LCDCRST` reader - LCDCRST: LCD controller reset. 0: LCD controller IP is not under reset. 1: LCD controller IP is under reset.
pub type LCDCRST_R = crate::BitReader;
///Field `LCDCRST` writer - LCDCRST: LCD controller reset. 0: LCD controller IP is not under reset. 1: LCD controller IP is under reset.
pub type LCDCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMPRST` reader - COMPRST: COMP reset. 0: COMP IP is not under reset. 1: COMP IP is under reset.
pub type COMPRST_R = crate::BitReader;
///Field `COMPRST` writer - COMPRST: COMP reset. 0: COMP IP is not under reset. 1: COMP IP is under reset.
pub type COMPRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DACRST` reader - DACRST: DAC reset. 0: DAC IP is not under reset. 1: DAC IP is under reset.
pub type DACRST_R = crate::BitReader;
///Field `DACRST` writer - DACRST: DAC reset. 0: DAC IP is not under reset. 1: DAC IP is under reset.
pub type DACRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTCRST` reader - RTCRST: RTC reset. 0: RTC IP is not under reset. 1: RTC IP is under reset.
pub type RTCRST_R = crate::BitReader;
///Field `RTCRST` writer - RTCRST: RTC reset. 0: RTC IP is not under reset. 1: RTC IP is under reset.
pub type RTCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LCSCRST` reader - LCSCRST: LCSC reset. 0: LCSC IP is not under reset. 1: LCSC IP is under reset.
pub type LCSCRST_R = crate::BitReader;
///Field `LCSCRST` writer - LCSCRST: LCSC reset. 0: LCSC IP is not under reset. 1: LCSC IP is under reset.
pub type LCSCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WDGRST` reader - WDGRST: Watchdog reset. 0: Watchdog IP is not under reset. 1: Watchdog IP is under reset.
pub type WDGRST_R = crate::BitReader;
///Field `WDGRST` writer - WDGRST: Watchdog reset. 0: Watchdog IP is not under reset. 1: Watchdog IP is under reset.
pub type WDGRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBGMCURST` reader - DBGMCURST: DBGMCU reset. 0: DBGMCU IP is not under reset. 1: DBGMCU IP is under reset.
pub type DBGMCURST_R = crate::BitReader;
///Field `DBGMCURST` writer - DBGMCURST: DBGMCU reset. 0: DBGMCU IP is not under reset. 1: DBGMCU IP is under reset.
pub type DBGMCURST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TIM2RST: TIM2 reset. 0: TIM2 IP is not under reset. 1: TIM2 IP is under reset.
    #[inline(always)]
    pub fn tim2rst(&self) -> TIM2RST_R {
        TIM2RST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM16RST: TIM16 reset. 0: TIM16 IP is not under reset. 1: TIM16 IP is under reset.
    #[inline(always)]
    pub fn tim16rst(&self) -> TIM16RST_R {
        TIM16RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - SYSCFGRST: system controller reset. 0: system controller IP is not under reset. 1: system controller IP is under reset.
    #[inline(always)]
    pub fn syscfgrst(&self) -> SYSCFGRST_R {
        SYSCFGRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - LCDCRST: LCD controller reset. 0: LCD controller IP is not under reset. 1: LCD controller IP is under reset.
    #[inline(always)]
    pub fn lcdcrst(&self) -> LCDCRST_R {
        LCDCRST_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - COMPRST: COMP reset. 0: COMP IP is not under reset. 1: COMP IP is under reset.
    #[inline(always)]
    pub fn comprst(&self) -> COMPRST_R {
        COMPRST_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - DACRST: DAC reset. 0: DAC IP is not under reset. 1: DAC IP is under reset.
    #[inline(always)]
    pub fn dacrst(&self) -> DACRST_R {
        DACRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - RTCRST: RTC reset. 0: RTC IP is not under reset. 1: RTC IP is under reset.
    #[inline(always)]
    pub fn rtcrst(&self) -> RTCRST_R {
        RTCRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - LCSCRST: LCSC reset. 0: LCSC IP is not under reset. 1: LCSC IP is under reset.
    #[inline(always)]
    pub fn lcscrst(&self) -> LCSCRST_R {
        LCSCRST_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - WDGRST: Watchdog reset. 0: Watchdog IP is not under reset. 1: Watchdog IP is under reset.
    #[inline(always)]
    pub fn wdgrst(&self) -> WDGRST_R {
        WDGRST_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - DBGMCURST: DBGMCU reset. 0: DBGMCU IP is not under reset. 1: DBGMCU IP is under reset.
    #[inline(always)]
    pub fn dbgmcurst(&self) -> DBGMCURST_R {
        DBGMCURST_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB0RSTR")
            .field("tim2rst", &self.tim2rst())
            .field("tim16rst", &self.tim16rst())
            .field("syscfgrst", &self.syscfgrst())
            .field("lcdcrst", &self.lcdcrst())
            .field("comprst", &self.comprst())
            .field("dacrst", &self.dacrst())
            .field("rtcrst", &self.rtcrst())
            .field("lcscrst", &self.lcscrst())
            .field("wdgrst", &self.wdgrst())
            .field("dbgmcurst", &self.dbgmcurst())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM2RST: TIM2 reset. 0: TIM2 IP is not under reset. 1: TIM2 IP is under reset.
    #[inline(always)]
    pub fn tim2rst(&mut self) -> TIM2RST_W<'_, APB0RSTRrs> {
        TIM2RST_W::new(self, 0)
    }
    ///Bit 1 - TIM16RST: TIM16 reset. 0: TIM16 IP is not under reset. 1: TIM16 IP is under reset.
    #[inline(always)]
    pub fn tim16rst(&mut self) -> TIM16RST_W<'_, APB0RSTRrs> {
        TIM16RST_W::new(self, 1)
    }
    ///Bit 8 - SYSCFGRST: system controller reset. 0: system controller IP is not under reset. 1: system controller IP is under reset.
    #[inline(always)]
    pub fn syscfgrst(&mut self) -> SYSCFGRST_W<'_, APB0RSTRrs> {
        SYSCFGRST_W::new(self, 8)
    }
    ///Bit 9 - LCDCRST: LCD controller reset. 0: LCD controller IP is not under reset. 1: LCD controller IP is under reset.
    #[inline(always)]
    pub fn lcdcrst(&mut self) -> LCDCRST_W<'_, APB0RSTRrs> {
        LCDCRST_W::new(self, 9)
    }
    ///Bit 10 - COMPRST: COMP reset. 0: COMP IP is not under reset. 1: COMP IP is under reset.
    #[inline(always)]
    pub fn comprst(&mut self) -> COMPRST_W<'_, APB0RSTRrs> {
        COMPRST_W::new(self, 10)
    }
    ///Bit 11 - DACRST: DAC reset. 0: DAC IP is not under reset. 1: DAC IP is under reset.
    #[inline(always)]
    pub fn dacrst(&mut self) -> DACRST_W<'_, APB0RSTRrs> {
        DACRST_W::new(self, 11)
    }
    ///Bit 12 - RTCRST: RTC reset. 0: RTC IP is not under reset. 1: RTC IP is under reset.
    #[inline(always)]
    pub fn rtcrst(&mut self) -> RTCRST_W<'_, APB0RSTRrs> {
        RTCRST_W::new(self, 12)
    }
    ///Bit 13 - LCSCRST: LCSC reset. 0: LCSC IP is not under reset. 1: LCSC IP is under reset.
    #[inline(always)]
    pub fn lcscrst(&mut self) -> LCSCRST_W<'_, APB0RSTRrs> {
        LCSCRST_W::new(self, 13)
    }
    ///Bit 14 - WDGRST: Watchdog reset. 0: Watchdog IP is not under reset. 1: Watchdog IP is under reset.
    #[inline(always)]
    pub fn wdgrst(&mut self) -> WDGRST_W<'_, APB0RSTRrs> {
        WDGRST_W::new(self, 14)
    }
    ///Bit 15 - DBGMCURST: DBGMCU reset. 0: DBGMCU IP is not under reset. 1: DBGMCU IP is under reset.
    #[inline(always)]
    pub fn dbgmcurst(&mut self) -> DBGMCURST_W<'_, APB0RSTRrs> {
        DBGMCURST_W::new(self, 15)
    }
}
/**APB0RSTR register

You can [`read`](crate::Reg::read) this register and get [`apb0rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb0rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#RCC:APB0RSTR)*/
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

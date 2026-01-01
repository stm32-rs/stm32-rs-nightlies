///Register `APB0ENR` reader
pub type R = crate::R<APB0ENRrs>;
///Register `APB0ENR` writer
pub type W = crate::W<APB0ENRrs>;
///Field `TIM2EN` reader - TIM2: Advanced Timer clock enable Set and enable by software. 0: clock disable 1: clock enable
pub type TIM2EN_R = crate::BitReader;
///Field `TIM2EN` writer - TIM2: Advanced Timer clock enable Set and enable by software. 0: clock disable 1: clock enable
pub type TIM2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM16EN` reader - TIM16: Advanced Timer clock enable Set and enable by software. 0: clock disable 1: clock enable
pub type TIM16EN_R = crate::BitReader;
///Field `TIM16EN` writer - TIM16: Advanced Timer clock enable Set and enable by software. 0: clock disable 1: clock enable
pub type TIM16EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYSCFGEN` reader - SYSTEM CONFIG clock enable Set and enable by software. 0: clock disable 1: clock enable
pub type SYSCFGEN_R = crate::BitReader;
///Field `SYSCFGEN` writer - SYSTEM CONFIG clock enable Set and enable by software. 0: clock disable 1: clock enable
pub type SYSCFGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LCDEN` reader - LCD clock enable Set and enable by software. 0: clock disable 1: clock enable
pub type LCDEN_R = crate::BitReader;
///Field `LCDEN` writer - LCD clock enable Set and enable by software. 0: clock disable 1: clock enable
pub type LCDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMPEN` reader - COMP clock enable Set and enable by software. 0: clock disable 1: clock enable
pub type COMPEN_R = crate::BitReader;
///Field `COMPEN` writer - COMP clock enable Set and enable by software. 0: clock disable 1: clock enable
pub type COMPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DACEN` reader - DAC clock enable Set and enable by software. 0: clock disable 1: clock enable
pub type DACEN_R = crate::BitReader;
///Field `DACEN` writer - DAC clock enable Set and enable by software. 0: clock disable 1: clock enable
pub type DACEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTCEN` reader - RTC clock enable Set and enable by software. Reset source only for this field: PORESETn 0: clock disable 1: clock enable
pub type RTCEN_R = crate::BitReader;
///Field `RTCEN` writer - RTC clock enable Set and enable by software. Reset source only for this field: PORESETn 0: clock disable 1: clock enable
pub type RTCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LCSCEN` reader - LCSC clock enable. Set and enable by software. 0: clock disable 1: clock enable
pub type LCSCEN_R = crate::BitReader;
///Field `LCSCEN` writer - LCSC clock enable. Set and enable by software. 0: clock disable 1: clock enable
pub type LCSCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WDGEN` reader - Watchdog clock enable. Set and enable by software. 0: clock disable 1: clock enable
pub type WDGEN_R = crate::BitReader;
///Field `WDGEN` writer - Watchdog clock enable. Set and enable by software. 0: clock disable 1: clock enable
pub type WDGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBGMCUEN` reader - DBG MCU clock enable. Set and enable by software. 0: clock disable 1: clock enable
pub type DBGMCUEN_R = crate::BitReader;
///Field `DBGMCUEN` writer - DBG MCU clock enable. Set and enable by software. 0: clock disable 1: clock enable
pub type DBGMCUEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TIM2: Advanced Timer clock enable Set and enable by software. 0: clock disable 1: clock enable
    #[inline(always)]
    pub fn tim2en(&self) -> TIM2EN_R {
        TIM2EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM16: Advanced Timer clock enable Set and enable by software. 0: clock disable 1: clock enable
    #[inline(always)]
    pub fn tim16en(&self) -> TIM16EN_R {
        TIM16EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - SYSTEM CONFIG clock enable Set and enable by software. 0: clock disable 1: clock enable
    #[inline(always)]
    pub fn syscfgen(&self) -> SYSCFGEN_R {
        SYSCFGEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - LCD clock enable Set and enable by software. 0: clock disable 1: clock enable
    #[inline(always)]
    pub fn lcden(&self) -> LCDEN_R {
        LCDEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - COMP clock enable Set and enable by software. 0: clock disable 1: clock enable
    #[inline(always)]
    pub fn compen(&self) -> COMPEN_R {
        COMPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - DAC clock enable Set and enable by software. 0: clock disable 1: clock enable
    #[inline(always)]
    pub fn dacen(&self) -> DACEN_R {
        DACEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - RTC clock enable Set and enable by software. Reset source only for this field: PORESETn 0: clock disable 1: clock enable
    #[inline(always)]
    pub fn rtcen(&self) -> RTCEN_R {
        RTCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - LCSC clock enable. Set and enable by software. 0: clock disable 1: clock enable
    #[inline(always)]
    pub fn lcscen(&self) -> LCSCEN_R {
        LCSCEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Watchdog clock enable. Set and enable by software. 0: clock disable 1: clock enable
    #[inline(always)]
    pub fn wdgen(&self) -> WDGEN_R {
        WDGEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - DBG MCU clock enable. Set and enable by software. 0: clock disable 1: clock enable
    #[inline(always)]
    pub fn dbgmcuen(&self) -> DBGMCUEN_R {
        DBGMCUEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB0ENR")
            .field("tim2en", &self.tim2en())
            .field("tim16en", &self.tim16en())
            .field("syscfgen", &self.syscfgen())
            .field("lcden", &self.lcden())
            .field("compen", &self.compen())
            .field("dacen", &self.dacen())
            .field("rtcen", &self.rtcen())
            .field("lcscen", &self.lcscen())
            .field("wdgen", &self.wdgen())
            .field("dbgmcuen", &self.dbgmcuen())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM2: Advanced Timer clock enable Set and enable by software. 0: clock disable 1: clock enable
    #[inline(always)]
    pub fn tim2en(&mut self) -> TIM2EN_W<'_, APB0ENRrs> {
        TIM2EN_W::new(self, 0)
    }
    ///Bit 1 - TIM16: Advanced Timer clock enable Set and enable by software. 0: clock disable 1: clock enable
    #[inline(always)]
    pub fn tim16en(&mut self) -> TIM16EN_W<'_, APB0ENRrs> {
        TIM16EN_W::new(self, 1)
    }
    ///Bit 8 - SYSTEM CONFIG clock enable Set and enable by software. 0: clock disable 1: clock enable
    #[inline(always)]
    pub fn syscfgen(&mut self) -> SYSCFGEN_W<'_, APB0ENRrs> {
        SYSCFGEN_W::new(self, 8)
    }
    ///Bit 9 - LCD clock enable Set and enable by software. 0: clock disable 1: clock enable
    #[inline(always)]
    pub fn lcden(&mut self) -> LCDEN_W<'_, APB0ENRrs> {
        LCDEN_W::new(self, 9)
    }
    ///Bit 10 - COMP clock enable Set and enable by software. 0: clock disable 1: clock enable
    #[inline(always)]
    pub fn compen(&mut self) -> COMPEN_W<'_, APB0ENRrs> {
        COMPEN_W::new(self, 10)
    }
    ///Bit 11 - DAC clock enable Set and enable by software. 0: clock disable 1: clock enable
    #[inline(always)]
    pub fn dacen(&mut self) -> DACEN_W<'_, APB0ENRrs> {
        DACEN_W::new(self, 11)
    }
    ///Bit 12 - RTC clock enable Set and enable by software. Reset source only for this field: PORESETn 0: clock disable 1: clock enable
    #[inline(always)]
    pub fn rtcen(&mut self) -> RTCEN_W<'_, APB0ENRrs> {
        RTCEN_W::new(self, 12)
    }
    ///Bit 13 - LCSC clock enable. Set and enable by software. 0: clock disable 1: clock enable
    #[inline(always)]
    pub fn lcscen(&mut self) -> LCSCEN_W<'_, APB0ENRrs> {
        LCSCEN_W::new(self, 13)
    }
    ///Bit 14 - Watchdog clock enable. Set and enable by software. 0: clock disable 1: clock enable
    #[inline(always)]
    pub fn wdgen(&mut self) -> WDGEN_W<'_, APB0ENRrs> {
        WDGEN_W::new(self, 14)
    }
    ///Bit 15 - DBG MCU clock enable. Set and enable by software. 0: clock disable 1: clock enable
    #[inline(always)]
    pub fn dbgmcuen(&mut self) -> DBGMCUEN_W<'_, APB0ENRrs> {
        DBGMCUEN_W::new(self, 15)
    }
}
/**APB0ENR register

You can [`read`](crate::Reg::read) this register and get [`apb0enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb0enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#RCC:APB0ENR)*/
pub struct APB0ENRrs;
impl crate::RegisterSpec for APB0ENRrs {
    type Ux = u32;
}
///`read()` method returns [`apb0enr::R`](R) reader structure
impl crate::Readable for APB0ENRrs {}
///`write(|w| ..)` method takes [`apb0enr::W`](W) writer structure
impl crate::Writable for APB0ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB0ENR to value 0
impl crate::Resettable for APB0ENRrs {}

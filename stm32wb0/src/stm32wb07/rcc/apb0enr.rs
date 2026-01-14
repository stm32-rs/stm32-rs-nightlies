///Register `APB0ENR` reader
pub type R = crate::R<APB0ENRrs>;
///Register `APB0ENR` writer
pub type W = crate::W<APB0ENRrs>;
///Field `TIM1EN` reader - TIM1 enable
pub type TIM1EN_R = crate::BitReader;
///Field `TIM1EN` writer - TIM1 enable
pub type TIM1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYSCFGEN` reader - SYSTEM CONFIG enable Set and enable by software.
pub type SYSCFGEN_R = crate::BitReader;
///Field `SYSCFGEN` writer - SYSTEM CONFIG enable Set and enable by software.
pub type SYSCFGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTCEN` reader - RTC clock enable Set and enable by software. Reset source only for this field: PORESETn
pub type RTCEN_R = crate::BitReader;
///Field `RTCEN` writer - RTC clock enable Set and enable by software. Reset source only for this field: PORESETn
pub type RTCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WDGEN` reader - Watchdog clock enable. Set and enable by software.
pub type WDGEN_R = crate::BitReader;
///Field `WDGEN` writer - Watchdog clock enable. Set and enable by software.
pub type WDGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TIM1 enable
    #[inline(always)]
    pub fn tim1en(&self) -> TIM1EN_R {
        TIM1EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - SYSTEM CONFIG enable Set and enable by software.
    #[inline(always)]
    pub fn syscfgen(&self) -> SYSCFGEN_R {
        SYSCFGEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - RTC clock enable Set and enable by software. Reset source only for this field: PORESETn
    #[inline(always)]
    pub fn rtcen(&self) -> RTCEN_R {
        RTCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - Watchdog clock enable. Set and enable by software.
    #[inline(always)]
    pub fn wdgen(&self) -> WDGEN_R {
        WDGEN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB0ENR")
            .field("tim1en", &self.tim1en())
            .field("syscfgen", &self.syscfgen())
            .field("rtcen", &self.rtcen())
            .field("wdgen", &self.wdgen())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM1 enable
    #[inline(always)]
    pub fn tim1en(&mut self) -> TIM1EN_W<'_, APB0ENRrs> {
        TIM1EN_W::new(self, 0)
    }
    ///Bit 8 - SYSTEM CONFIG enable Set and enable by software.
    #[inline(always)]
    pub fn syscfgen(&mut self) -> SYSCFGEN_W<'_, APB0ENRrs> {
        SYSCFGEN_W::new(self, 8)
    }
    ///Bit 12 - RTC clock enable Set and enable by software. Reset source only for this field: PORESETn
    #[inline(always)]
    pub fn rtcen(&mut self) -> RTCEN_W<'_, APB0ENRrs> {
        RTCEN_W::new(self, 12)
    }
    ///Bit 14 - Watchdog clock enable. Set and enable by software.
    #[inline(always)]
    pub fn wdgen(&mut self) -> WDGEN_W<'_, APB0ENRrs> {
        WDGEN_W::new(self, 14)
    }
}
/**APB0ENR register

You can [`read`](crate::Reg::read) this register and get [`apb0enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb0enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#RCC:APB0ENR)*/
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

///Register `CR3` reader
pub type R = crate::R<CR3rs>;
///Register `CR3` writer
pub type W = crate::W<CR3rs>;
///Field `EWU0` reader - EWU0 Enable WakeUp line 0 (PB0) When this bit is set the wakeup line 0 is enabled and a rising or falling edge on wakeup line 0 will trigger a CPU wakeup event depending on CR4.WP0 bit.
pub type EWU0_R = crate::BitReader;
///Field `EWU0` writer - EWU0 Enable WakeUp line 0 (PB0) When this bit is set the wakeup line 0 is enabled and a rising or falling edge on wakeup line 0 will trigger a CPU wakeup event depending on CR4.WP0 bit.
pub type EWU0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWU1` reader - EWU1 Enable WakeUp line 1 (PB1) When this bit is set the wakeup line 1 is enabled and a rising or falling edge on wakeup line 1 will trigger a CPU wakeup event depending on CR4.WP1 bit.
pub type EWU1_R = crate::BitReader;
///Field `EWU1` writer - EWU1 Enable WakeUp line 1 (PB1) When this bit is set the wakeup line 1 is enabled and a rising or falling edge on wakeup line 1 will trigger a CPU wakeup event depending on CR4.WP1 bit.
pub type EWU1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWU2` reader - EWU2 Enable WakeUp line 2 (PB2) When this bit is set the wakeup line 2 is enabled and a rising or falling edge on wakeup line 2 will trigger a CPU wakeup event depending on CR4.WP2 bit.
pub type EWU2_R = crate::BitReader;
///Field `EWU2` writer - EWU2 Enable WakeUp line 2 (PB2) When this bit is set the wakeup line 2 is enabled and a rising or falling edge on wakeup line 2 will trigger a CPU wakeup event depending on CR4.WP2 bit.
pub type EWU2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWU3` reader - EWU3 Enable WakeUp line 3 (PB3) When this bit is set the wakeup line 3 is enabled and a rising or falling edge on wakeup line 3 will trigger a CPU wakeup event depending on CR4.WP3 bit.
pub type EWU3_R = crate::BitReader;
///Field `EWU3` writer - EWU3 Enable WakeUp line 3 (PB3) When this bit is set the wakeup line 3 is enabled and a rising or falling edge on wakeup line 3 will trigger a CPU wakeup event depending on CR4.WP3 bit.
pub type EWU3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWU4` reader - EWU4 Enable WakeUp line 4 (PB4) When this bit is set the wakeup line 4 is enabled and a rising or falling edge on wakeup line 4 will trigger a CPU wakeup event depending on CR4.WP4 bit.
pub type EWU4_R = crate::BitReader;
///Field `EWU4` writer - EWU4 Enable WakeUp line 4 (PB4) When this bit is set the wakeup line 4 is enabled and a rising or falling edge on wakeup line 4 will trigger a CPU wakeup event depending on CR4.WP4 bit.
pub type EWU4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWU5` reader - EWU5 Enable WakeUp line 5 (PB5) When this bit is set the wakeup line 5 is enabled and a rising or falling edge on wakeup line 5 will trigger a CPU wakeup event depending on CR4.WP5 bit.
pub type EWU5_R = crate::BitReader;
///Field `EWU5` writer - EWU5 Enable WakeUp line 5 (PB5) When this bit is set the wakeup line 5 is enabled and a rising or falling edge on wakeup line 5 will trigger a CPU wakeup event depending on CR4.WP5 bit.
pub type EWU5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWU6` reader - EWU6 Enable WakeUp line 6 (PB6) When this bit is set the wakeup line 6 is enabled and a rising or falling edge on wakeup line 6 will trigger a CPU wakeup event depending on CR4.WP6 bit.
pub type EWU6_R = crate::BitReader;
///Field `EWU6` writer - EWU6 Enable WakeUp line 6 (PB6) When this bit is set the wakeup line 6 is enabled and a rising or falling edge on wakeup line 6 will trigger a CPU wakeup event depending on CR4.WP6 bit.
pub type EWU6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWU7` reader - EWU7 Enable WakeUp line 7 (PB7) When this bit is set the wakeup line 7 is enabled and a rising or falling edge on wakeup line 7 will trigger a CPU wakeup event depending on CR4.WP7 bit.
pub type EWU7_R = crate::BitReader;
///Field `EWU7` writer - EWU7 Enable WakeUp line 7 (PB7) When this bit is set the wakeup line 7 is enabled and a rising or falling edge on wakeup line 7 will trigger a CPU wakeup event depending on CR4.WP7 bit.
pub type EWU7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWU8` reader - EWU8 Enable WakeUp line 8 (PA8) When this bit is set the wakeup line 8 is enabled and a rising or falling edge on wakeup line 8 will trigger a CPU wakeup event depending on CR4.WP8 bit.
pub type EWU8_R = crate::BitReader;
///Field `EWU8` writer - EWU8 Enable WakeUp line 8 (PA8) When this bit is set the wakeup line 8 is enabled and a rising or falling edge on wakeup line 8 will trigger a CPU wakeup event depending on CR4.WP8 bit.
pub type EWU8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWU9` reader - EWU9 Enable WakeUp line 9 (PA9) When this bit is set the wakeup line 9 is enabled and a rising or falling edge on wakeup line 9 will trigger a CPU wakeup event depending on CR4.WP9 bit.
pub type EWU9_R = crate::BitReader;
///Field `EWU9` writer - EWU9 Enable WakeUp line 9 (PA9) When this bit is set the wakeup line 9 is enabled and a rising or falling edge on wakeup line 9 will trigger a CPU wakeup event depending on CR4.WP9 bit.
pub type EWU9_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWU10` reader - EWU10 Enable WakeUp line 10 (PA10) When this bit is set the wakeup line 10 is enabled and a rising or falling edge on wakeup line 10 will trigger a CPU wakeup event depending on CR4.WP10 bit.
pub type EWU10_R = crate::BitReader;
///Field `EWU10` writer - EWU10 Enable WakeUp line 10 (PA10) When this bit is set the wakeup line 10 is enabled and a rising or falling edge on wakeup line 10 will trigger a CPU wakeup event depending on CR4.WP10 bit.
pub type EWU10_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWU11` reader - EWU11 Enable WakeUp line 11 (PA11) When this bit is set the wakeup line 11 is enabled and a rising or falling edge on wakeup line 11 will trigger a CPU wakeup event depending on CR4.WP11 bit.
pub type EWU11_R = crate::BitReader;
///Field `EWU11` writer - EWU11 Enable WakeUp line 11 (PA11) When this bit is set the wakeup line 11 is enabled and a rising or falling edge on wakeup line 11 will trigger a CPU wakeup event depending on CR4.WP11 bit.
pub type EWU11_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWBLE` reader - EWBLE: Enable wakeup on BLE event. 0: Wakeup on BLE line is disabled (default). 1: Wakeup on BLE line is enabled.
pub type EWBLE_R = crate::BitReader;
///Field `EWBLE` writer - EWBLE: Enable wakeup on BLE event. 0: Wakeup on BLE line is disabled (default). 1: Wakeup on BLE line is enabled.
pub type EWBLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWBLEHCPU` reader - EWBLEHCPU: Enable wakeup on BLE Host CPU event. 0: Wakeup on BLE Host CPU line is disabled (default). 1: Wakeup on BLE Host CPU line is enabled.
pub type EWBLEHCPU_R = crate::BitReader;
///Field `EWBLEHCPU` writer - EWBLEHCPU: Enable wakeup on BLE Host CPU event. 0: Wakeup on BLE Host CPU line is disabled (default). 1: Wakeup on BLE Host CPU line is enabled.
pub type EWBLEHCPU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EIWL2` reader - EIWL2: Enable wakeup on Internal event (LPUART). 0: Wakeup on internal line is disabled (default). 1: Wakeup on internal line is enabled.
pub type EIWL2_R = crate::BitReader;
///Field `EIWL2` writer - EIWL2: Enable wakeup on Internal event (LPUART). 0: Wakeup on internal line is disabled (default). 1: Wakeup on internal line is enabled.
pub type EIWL2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EIWL` reader - EIWL: Enable wakeup on Internal event (RTC). 0: Wakeup on internal line is disabled (default). 1: Wakeup on internal line is enabled.
pub type EIWL_R = crate::BitReader;
///Field `EIWL` writer - EIWL: Enable wakeup on Internal event (RTC). 0: Wakeup on internal line is disabled (default). 1: Wakeup on internal line is enabled.
pub type EIWL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - EWU0 Enable WakeUp line 0 (PB0) When this bit is set the wakeup line 0 is enabled and a rising or falling edge on wakeup line 0 will trigger a CPU wakeup event depending on CR4.WP0 bit.
    #[inline(always)]
    pub fn ewu0(&self) -> EWU0_R {
        EWU0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - EWU1 Enable WakeUp line 1 (PB1) When this bit is set the wakeup line 1 is enabled and a rising or falling edge on wakeup line 1 will trigger a CPU wakeup event depending on CR4.WP1 bit.
    #[inline(always)]
    pub fn ewu1(&self) -> EWU1_R {
        EWU1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - EWU2 Enable WakeUp line 2 (PB2) When this bit is set the wakeup line 2 is enabled and a rising or falling edge on wakeup line 2 will trigger a CPU wakeup event depending on CR4.WP2 bit.
    #[inline(always)]
    pub fn ewu2(&self) -> EWU2_R {
        EWU2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - EWU3 Enable WakeUp line 3 (PB3) When this bit is set the wakeup line 3 is enabled and a rising or falling edge on wakeup line 3 will trigger a CPU wakeup event depending on CR4.WP3 bit.
    #[inline(always)]
    pub fn ewu3(&self) -> EWU3_R {
        EWU3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - EWU4 Enable WakeUp line 4 (PB4) When this bit is set the wakeup line 4 is enabled and a rising or falling edge on wakeup line 4 will trigger a CPU wakeup event depending on CR4.WP4 bit.
    #[inline(always)]
    pub fn ewu4(&self) -> EWU4_R {
        EWU4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - EWU5 Enable WakeUp line 5 (PB5) When this bit is set the wakeup line 5 is enabled and a rising or falling edge on wakeup line 5 will trigger a CPU wakeup event depending on CR4.WP5 bit.
    #[inline(always)]
    pub fn ewu5(&self) -> EWU5_R {
        EWU5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - EWU6 Enable WakeUp line 6 (PB6) When this bit is set the wakeup line 6 is enabled and a rising or falling edge on wakeup line 6 will trigger a CPU wakeup event depending on CR4.WP6 bit.
    #[inline(always)]
    pub fn ewu6(&self) -> EWU6_R {
        EWU6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - EWU7 Enable WakeUp line 7 (PB7) When this bit is set the wakeup line 7 is enabled and a rising or falling edge on wakeup line 7 will trigger a CPU wakeup event depending on CR4.WP7 bit.
    #[inline(always)]
    pub fn ewu7(&self) -> EWU7_R {
        EWU7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - EWU8 Enable WakeUp line 8 (PA8) When this bit is set the wakeup line 8 is enabled and a rising or falling edge on wakeup line 8 will trigger a CPU wakeup event depending on CR4.WP8 bit.
    #[inline(always)]
    pub fn ewu8(&self) -> EWU8_R {
        EWU8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - EWU9 Enable WakeUp line 9 (PA9) When this bit is set the wakeup line 9 is enabled and a rising or falling edge on wakeup line 9 will trigger a CPU wakeup event depending on CR4.WP9 bit.
    #[inline(always)]
    pub fn ewu9(&self) -> EWU9_R {
        EWU9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - EWU10 Enable WakeUp line 10 (PA10) When this bit is set the wakeup line 10 is enabled and a rising or falling edge on wakeup line 10 will trigger a CPU wakeup event depending on CR4.WP10 bit.
    #[inline(always)]
    pub fn ewu10(&self) -> EWU10_R {
        EWU10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - EWU11 Enable WakeUp line 11 (PA11) When this bit is set the wakeup line 11 is enabled and a rising or falling edge on wakeup line 11 will trigger a CPU wakeup event depending on CR4.WP11 bit.
    #[inline(always)]
    pub fn ewu11(&self) -> EWU11_R {
        EWU11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - EWBLE: Enable wakeup on BLE event. 0: Wakeup on BLE line is disabled (default). 1: Wakeup on BLE line is enabled.
    #[inline(always)]
    pub fn ewble(&self) -> EWBLE_R {
        EWBLE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - EWBLEHCPU: Enable wakeup on BLE Host CPU event. 0: Wakeup on BLE Host CPU line is disabled (default). 1: Wakeup on BLE Host CPU line is enabled.
    #[inline(always)]
    pub fn ewblehcpu(&self) -> EWBLEHCPU_R {
        EWBLEHCPU_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - EIWL2: Enable wakeup on Internal event (LPUART). 0: Wakeup on internal line is disabled (default). 1: Wakeup on internal line is enabled.
    #[inline(always)]
    pub fn eiwl2(&self) -> EIWL2_R {
        EIWL2_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - EIWL: Enable wakeup on Internal event (RTC). 0: Wakeup on internal line is disabled (default). 1: Wakeup on internal line is enabled.
    #[inline(always)]
    pub fn eiwl(&self) -> EIWL_R {
        EIWL_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR3")
            .field("ewu0", &self.ewu0())
            .field("ewu1", &self.ewu1())
            .field("ewu2", &self.ewu2())
            .field("ewu3", &self.ewu3())
            .field("ewu4", &self.ewu4())
            .field("ewu5", &self.ewu5())
            .field("ewu6", &self.ewu6())
            .field("ewu7", &self.ewu7())
            .field("ewu8", &self.ewu8())
            .field("ewu9", &self.ewu9())
            .field("ewu10", &self.ewu10())
            .field("ewu11", &self.ewu11())
            .field("ewble", &self.ewble())
            .field("ewblehcpu", &self.ewblehcpu())
            .field("eiwl2", &self.eiwl2())
            .field("eiwl", &self.eiwl())
            .finish()
    }
}
impl W {
    ///Bit 0 - EWU0 Enable WakeUp line 0 (PB0) When this bit is set the wakeup line 0 is enabled and a rising or falling edge on wakeup line 0 will trigger a CPU wakeup event depending on CR4.WP0 bit.
    #[inline(always)]
    pub fn ewu0(&mut self) -> EWU0_W<'_, CR3rs> {
        EWU0_W::new(self, 0)
    }
    ///Bit 1 - EWU1 Enable WakeUp line 1 (PB1) When this bit is set the wakeup line 1 is enabled and a rising or falling edge on wakeup line 1 will trigger a CPU wakeup event depending on CR4.WP1 bit.
    #[inline(always)]
    pub fn ewu1(&mut self) -> EWU1_W<'_, CR3rs> {
        EWU1_W::new(self, 1)
    }
    ///Bit 2 - EWU2 Enable WakeUp line 2 (PB2) When this bit is set the wakeup line 2 is enabled and a rising or falling edge on wakeup line 2 will trigger a CPU wakeup event depending on CR4.WP2 bit.
    #[inline(always)]
    pub fn ewu2(&mut self) -> EWU2_W<'_, CR3rs> {
        EWU2_W::new(self, 2)
    }
    ///Bit 3 - EWU3 Enable WakeUp line 3 (PB3) When this bit is set the wakeup line 3 is enabled and a rising or falling edge on wakeup line 3 will trigger a CPU wakeup event depending on CR4.WP3 bit.
    #[inline(always)]
    pub fn ewu3(&mut self) -> EWU3_W<'_, CR3rs> {
        EWU3_W::new(self, 3)
    }
    ///Bit 4 - EWU4 Enable WakeUp line 4 (PB4) When this bit is set the wakeup line 4 is enabled and a rising or falling edge on wakeup line 4 will trigger a CPU wakeup event depending on CR4.WP4 bit.
    #[inline(always)]
    pub fn ewu4(&mut self) -> EWU4_W<'_, CR3rs> {
        EWU4_W::new(self, 4)
    }
    ///Bit 5 - EWU5 Enable WakeUp line 5 (PB5) When this bit is set the wakeup line 5 is enabled and a rising or falling edge on wakeup line 5 will trigger a CPU wakeup event depending on CR4.WP5 bit.
    #[inline(always)]
    pub fn ewu5(&mut self) -> EWU5_W<'_, CR3rs> {
        EWU5_W::new(self, 5)
    }
    ///Bit 6 - EWU6 Enable WakeUp line 6 (PB6) When this bit is set the wakeup line 6 is enabled and a rising or falling edge on wakeup line 6 will trigger a CPU wakeup event depending on CR4.WP6 bit.
    #[inline(always)]
    pub fn ewu6(&mut self) -> EWU6_W<'_, CR3rs> {
        EWU6_W::new(self, 6)
    }
    ///Bit 7 - EWU7 Enable WakeUp line 7 (PB7) When this bit is set the wakeup line 7 is enabled and a rising or falling edge on wakeup line 7 will trigger a CPU wakeup event depending on CR4.WP7 bit.
    #[inline(always)]
    pub fn ewu7(&mut self) -> EWU7_W<'_, CR3rs> {
        EWU7_W::new(self, 7)
    }
    ///Bit 8 - EWU8 Enable WakeUp line 8 (PA8) When this bit is set the wakeup line 8 is enabled and a rising or falling edge on wakeup line 8 will trigger a CPU wakeup event depending on CR4.WP8 bit.
    #[inline(always)]
    pub fn ewu8(&mut self) -> EWU8_W<'_, CR3rs> {
        EWU8_W::new(self, 8)
    }
    ///Bit 9 - EWU9 Enable WakeUp line 9 (PA9) When this bit is set the wakeup line 9 is enabled and a rising or falling edge on wakeup line 9 will trigger a CPU wakeup event depending on CR4.WP9 bit.
    #[inline(always)]
    pub fn ewu9(&mut self) -> EWU9_W<'_, CR3rs> {
        EWU9_W::new(self, 9)
    }
    ///Bit 10 - EWU10 Enable WakeUp line 10 (PA10) When this bit is set the wakeup line 10 is enabled and a rising or falling edge on wakeup line 10 will trigger a CPU wakeup event depending on CR4.WP10 bit.
    #[inline(always)]
    pub fn ewu10(&mut self) -> EWU10_W<'_, CR3rs> {
        EWU10_W::new(self, 10)
    }
    ///Bit 11 - EWU11 Enable WakeUp line 11 (PA11) When this bit is set the wakeup line 11 is enabled and a rising or falling edge on wakeup line 11 will trigger a CPU wakeup event depending on CR4.WP11 bit.
    #[inline(always)]
    pub fn ewu11(&mut self) -> EWU11_W<'_, CR3rs> {
        EWU11_W::new(self, 11)
    }
    ///Bit 12 - EWBLE: Enable wakeup on BLE event. 0: Wakeup on BLE line is disabled (default). 1: Wakeup on BLE line is enabled.
    #[inline(always)]
    pub fn ewble(&mut self) -> EWBLE_W<'_, CR3rs> {
        EWBLE_W::new(self, 12)
    }
    ///Bit 13 - EWBLEHCPU: Enable wakeup on BLE Host CPU event. 0: Wakeup on BLE Host CPU line is disabled (default). 1: Wakeup on BLE Host CPU line is enabled.
    #[inline(always)]
    pub fn ewblehcpu(&mut self) -> EWBLEHCPU_W<'_, CR3rs> {
        EWBLEHCPU_W::new(self, 13)
    }
    ///Bit 14 - EIWL2: Enable wakeup on Internal event (LPUART). 0: Wakeup on internal line is disabled (default). 1: Wakeup on internal line is enabled.
    #[inline(always)]
    pub fn eiwl2(&mut self) -> EIWL2_W<'_, CR3rs> {
        EIWL2_W::new(self, 14)
    }
    ///Bit 15 - EIWL: Enable wakeup on Internal event (RTC). 0: Wakeup on internal line is disabled (default). 1: Wakeup on internal line is enabled.
    #[inline(always)]
    pub fn eiwl(&mut self) -> EIWL_W<'_, CR3rs> {
        EIWL_W::new(self, 15)
    }
}
/**CR3 register

You can [`read`](crate::Reg::read) this register and get [`cr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#PWRC:CR3)*/
pub struct CR3rs;
impl crate::RegisterSpec for CR3rs {
    type Ux = u32;
}
///`read()` method returns [`cr3::R`](R) reader structure
impl crate::Readable for CR3rs {}
///`write(|w| ..)` method takes [`cr3::W`](W) writer structure
impl crate::Writable for CR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR3 to value 0
impl crate::Resettable for CR3rs {}

///Register `CR6` reader
pub type R = crate::R<CR6rs>;
///Register `CR6` writer
pub type W = crate::W<CR6rs>;
///Field `EWU12` reader - EWU12 Enable WakeUp line 12 (PA0) When this bit is set the wakeup line 12 is enabled and a rising or falling edge on wakeup line 0 will trigger a CPU wakeup event depending on CR7.WP0 bit.
pub type EWU12_R = crate::BitReader;
///Field `EWU12` writer - EWU12 Enable WakeUp line 12 (PA0) When this bit is set the wakeup line 12 is enabled and a rising or falling edge on wakeup line 0 will trigger a CPU wakeup event depending on CR7.WP0 bit.
pub type EWU12_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWU13` reader - EWU13 Enable WakeUp line 13 (PA1) When this bit is set the wakeup line 13 is enabled and a rising or falling edge on wakeup line 1 will trigger a CPU wakeup event depending on CR7.WP1 bit.
pub type EWU13_R = crate::BitReader;
///Field `EWU13` writer - EWU13 Enable WakeUp line 13 (PA1) When this bit is set the wakeup line 13 is enabled and a rising or falling edge on wakeup line 1 will trigger a CPU wakeup event depending on CR7.WP1 bit.
pub type EWU13_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWU14` reader - EWU14 Enable WakeUp line 14 (PA2) When this bit is set the wakeup line 14 is enabled and a rising or falling edge on wakeup line 2 will trigger a CPU wakeup event depending on CR7.WP2 bit.
pub type EWU14_R = crate::BitReader;
///Field `EWU14` writer - EWU14 Enable WakeUp line 14 (PA2) When this bit is set the wakeup line 14 is enabled and a rising or falling edge on wakeup line 2 will trigger a CPU wakeup event depending on CR7.WP2 bit.
pub type EWU14_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWU15` reader - EWU15 Enable WakeUp line 15 (PA3) When this bit is set the wakeup line 15 is enabled and a rising or falling edge on wakeup line 3 will trigger a CPU wakeup event depending on CR7.WP3 bit.
pub type EWU15_R = crate::BitReader;
///Field `EWU15` writer - EWU15 Enable WakeUp line 15 (PA3) When this bit is set the wakeup line 15 is enabled and a rising or falling edge on wakeup line 3 will trigger a CPU wakeup event depending on CR7.WP3 bit.
pub type EWU15_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWU16` reader - EWU16 Enable WakeUp line 16 (PB12) When this bit is set the wakeup line 16 is enabled and a rising or falling edge on wakeup line 4 will trigger a CPU wakeup event depending on CR7.WP4 bit.
pub type EWU16_R = crate::BitReader;
///Field `EWU16` writer - EWU16 Enable WakeUp line 16 (PB12) When this bit is set the wakeup line 16 is enabled and a rising or falling edge on wakeup line 4 will trigger a CPU wakeup event depending on CR7.WP4 bit.
pub type EWU16_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWU17` reader - EWU17 Enable WakeUp line 17 (PB13) When this bit is set the wakeup line 17 is enabled and a rising or falling edge on wakeup line 5 will trigger a CPU wakeup event depending on CR7.WP5 bit.
pub type EWU17_R = crate::BitReader;
///Field `EWU17` writer - EWU17 Enable WakeUp line 17 (PB13) When this bit is set the wakeup line 17 is enabled and a rising or falling edge on wakeup line 5 will trigger a CPU wakeup event depending on CR7.WP5 bit.
pub type EWU17_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWU18` reader - EWU18 Enable WakeUp line 18 (PB14) When this bit is set the wakeup line 18 is enabled and a rising or falling edge on wakeup line 6 will trigger a CPU wakeup event depending on CR7.WP6 bit.
pub type EWU18_R = crate::BitReader;
///Field `EWU18` writer - EWU18 Enable WakeUp line 18 (PB14) When this bit is set the wakeup line 18 is enabled and a rising or falling edge on wakeup line 6 will trigger a CPU wakeup event depending on CR7.WP6 bit.
pub type EWU18_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWU19` reader - EWU19 Enable WakeUp line 19 (PB15) When this bit is set the wakeup line 19 is enabled and a rising or falling edge on wakeup line 7 will trigger a CPU wakeup event depending on CR7.WP7 bit.
pub type EWU19_R = crate::BitReader;
///Field `EWU19` writer - EWU19 Enable WakeUp line 19 (PB15) When this bit is set the wakeup line 19 is enabled and a rising or falling edge on wakeup line 7 will trigger a CPU wakeup event depending on CR7.WP7 bit.
pub type EWU19_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - EWU12 Enable WakeUp line 12 (PA0) When this bit is set the wakeup line 12 is enabled and a rising or falling edge on wakeup line 0 will trigger a CPU wakeup event depending on CR7.WP0 bit.
    #[inline(always)]
    pub fn ewu12(&self) -> EWU12_R {
        EWU12_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - EWU13 Enable WakeUp line 13 (PA1) When this bit is set the wakeup line 13 is enabled and a rising or falling edge on wakeup line 1 will trigger a CPU wakeup event depending on CR7.WP1 bit.
    #[inline(always)]
    pub fn ewu13(&self) -> EWU13_R {
        EWU13_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - EWU14 Enable WakeUp line 14 (PA2) When this bit is set the wakeup line 14 is enabled and a rising or falling edge on wakeup line 2 will trigger a CPU wakeup event depending on CR7.WP2 bit.
    #[inline(always)]
    pub fn ewu14(&self) -> EWU14_R {
        EWU14_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - EWU15 Enable WakeUp line 15 (PA3) When this bit is set the wakeup line 15 is enabled and a rising or falling edge on wakeup line 3 will trigger a CPU wakeup event depending on CR7.WP3 bit.
    #[inline(always)]
    pub fn ewu15(&self) -> EWU15_R {
        EWU15_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - EWU16 Enable WakeUp line 16 (PB12) When this bit is set the wakeup line 16 is enabled and a rising or falling edge on wakeup line 4 will trigger a CPU wakeup event depending on CR7.WP4 bit.
    #[inline(always)]
    pub fn ewu16(&self) -> EWU16_R {
        EWU16_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - EWU17 Enable WakeUp line 17 (PB13) When this bit is set the wakeup line 17 is enabled and a rising or falling edge on wakeup line 5 will trigger a CPU wakeup event depending on CR7.WP5 bit.
    #[inline(always)]
    pub fn ewu17(&self) -> EWU17_R {
        EWU17_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - EWU18 Enable WakeUp line 18 (PB14) When this bit is set the wakeup line 18 is enabled and a rising or falling edge on wakeup line 6 will trigger a CPU wakeup event depending on CR7.WP6 bit.
    #[inline(always)]
    pub fn ewu18(&self) -> EWU18_R {
        EWU18_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - EWU19 Enable WakeUp line 19 (PB15) When this bit is set the wakeup line 19 is enabled and a rising or falling edge on wakeup line 7 will trigger a CPU wakeup event depending on CR7.WP7 bit.
    #[inline(always)]
    pub fn ewu19(&self) -> EWU19_R {
        EWU19_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR6")
            .field("ewu12", &self.ewu12())
            .field("ewu13", &self.ewu13())
            .field("ewu14", &self.ewu14())
            .field("ewu15", &self.ewu15())
            .field("ewu16", &self.ewu16())
            .field("ewu17", &self.ewu17())
            .field("ewu18", &self.ewu18())
            .field("ewu19", &self.ewu19())
            .finish()
    }
}
impl W {
    ///Bit 0 - EWU12 Enable WakeUp line 12 (PA0) When this bit is set the wakeup line 12 is enabled and a rising or falling edge on wakeup line 0 will trigger a CPU wakeup event depending on CR7.WP0 bit.
    #[inline(always)]
    pub fn ewu12(&mut self) -> EWU12_W<'_, CR6rs> {
        EWU12_W::new(self, 0)
    }
    ///Bit 1 - EWU13 Enable WakeUp line 13 (PA1) When this bit is set the wakeup line 13 is enabled and a rising or falling edge on wakeup line 1 will trigger a CPU wakeup event depending on CR7.WP1 bit.
    #[inline(always)]
    pub fn ewu13(&mut self) -> EWU13_W<'_, CR6rs> {
        EWU13_W::new(self, 1)
    }
    ///Bit 2 - EWU14 Enable WakeUp line 14 (PA2) When this bit is set the wakeup line 14 is enabled and a rising or falling edge on wakeup line 2 will trigger a CPU wakeup event depending on CR7.WP2 bit.
    #[inline(always)]
    pub fn ewu14(&mut self) -> EWU14_W<'_, CR6rs> {
        EWU14_W::new(self, 2)
    }
    ///Bit 3 - EWU15 Enable WakeUp line 15 (PA3) When this bit is set the wakeup line 15 is enabled and a rising or falling edge on wakeup line 3 will trigger a CPU wakeup event depending on CR7.WP3 bit.
    #[inline(always)]
    pub fn ewu15(&mut self) -> EWU15_W<'_, CR6rs> {
        EWU15_W::new(self, 3)
    }
    ///Bit 4 - EWU16 Enable WakeUp line 16 (PB12) When this bit is set the wakeup line 16 is enabled and a rising or falling edge on wakeup line 4 will trigger a CPU wakeup event depending on CR7.WP4 bit.
    #[inline(always)]
    pub fn ewu16(&mut self) -> EWU16_W<'_, CR6rs> {
        EWU16_W::new(self, 4)
    }
    ///Bit 5 - EWU17 Enable WakeUp line 17 (PB13) When this bit is set the wakeup line 17 is enabled and a rising or falling edge on wakeup line 5 will trigger a CPU wakeup event depending on CR7.WP5 bit.
    #[inline(always)]
    pub fn ewu17(&mut self) -> EWU17_W<'_, CR6rs> {
        EWU17_W::new(self, 5)
    }
    ///Bit 6 - EWU18 Enable WakeUp line 18 (PB14) When this bit is set the wakeup line 18 is enabled and a rising or falling edge on wakeup line 6 will trigger a CPU wakeup event depending on CR7.WP6 bit.
    #[inline(always)]
    pub fn ewu18(&mut self) -> EWU18_W<'_, CR6rs> {
        EWU18_W::new(self, 6)
    }
    ///Bit 7 - EWU19 Enable WakeUp line 19 (PB15) When this bit is set the wakeup line 19 is enabled and a rising or falling edge on wakeup line 7 will trigger a CPU wakeup event depending on CR7.WP7 bit.
    #[inline(always)]
    pub fn ewu19(&mut self) -> EWU19_W<'_, CR6rs> {
        EWU19_W::new(self, 7)
    }
}
/**CR6 register

You can [`read`](crate::Reg::read) this register and get [`cr6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#PWRC:CR6)*/
pub struct CR6rs;
impl crate::RegisterSpec for CR6rs {
    type Ux = u32;
}
///`read()` method returns [`cr6::R`](R) reader structure
impl crate::Readable for CR6rs {}
///`write(|w| ..)` method takes [`cr6::W`](W) writer structure
impl crate::Writable for CR6rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR6 to value 0
impl crate::Resettable for CR6rs {}

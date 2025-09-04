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
///Field `EWU20` reader - Enable wakeup on PB8 I/O event.
pub type EWU20_R = crate::BitReader;
///Field `EWU20` writer - Enable wakeup on PB8 I/O event.
pub type EWU20_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWU21` reader - Enable wakeup on PB9 I/O event.
pub type EWU21_R = crate::BitReader;
///Field `EWU21` writer - Enable wakeup on PB9 I/O event.
pub type EWU21_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWU22` reader - Enable wakeup on PB10 I/O event.
pub type EWU22_R = crate::BitReader;
///Field `EWU22` writer - Enable wakeup on PB10 I/O event.
pub type EWU22_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWU23` reader - Enable wakeup on PB11 I/O event.
pub type EWU23_R = crate::BitReader;
///Field `EWU23` writer - Enable wakeup on PB11 I/O event.
pub type EWU23_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWU24` reader - Enable wakeup on PA12 I/O event.
pub type EWU24_R = crate::BitReader;
///Field `EWU24` writer - Enable wakeup on PA12 I/O event.
pub type EWU24_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWU25` reader - Enable wakeup on PA13 I/O event.
pub type EWU25_R = crate::BitReader;
///Field `EWU25` writer - Enable wakeup on PA13 I/O event.
pub type EWU25_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWU26` reader - Enable wakeup on PA14 I/O event.
pub type EWU26_R = crate::BitReader;
///Field `EWU26` writer - Enable wakeup on PA14 I/O event.
pub type EWU26_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWU27` reader - Enable wakeup on PA15 I/O event.
pub type EWU27_R = crate::BitReader;
///Field `EWU27` writer - Enable wakeup on PA15 I/O event.
pub type EWU27_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 8 - Enable wakeup on PB8 I/O event.
    #[inline(always)]
    pub fn ewu20(&self) -> EWU20_R {
        EWU20_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Enable wakeup on PB9 I/O event.
    #[inline(always)]
    pub fn ewu21(&self) -> EWU21_R {
        EWU21_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Enable wakeup on PB10 I/O event.
    #[inline(always)]
    pub fn ewu22(&self) -> EWU22_R {
        EWU22_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Enable wakeup on PB11 I/O event.
    #[inline(always)]
    pub fn ewu23(&self) -> EWU23_R {
        EWU23_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Enable wakeup on PA12 I/O event.
    #[inline(always)]
    pub fn ewu24(&self) -> EWU24_R {
        EWU24_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Enable wakeup on PA13 I/O event.
    #[inline(always)]
    pub fn ewu25(&self) -> EWU25_R {
        EWU25_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Enable wakeup on PA14 I/O event.
    #[inline(always)]
    pub fn ewu26(&self) -> EWU26_R {
        EWU26_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Enable wakeup on PA15 I/O event.
    #[inline(always)]
    pub fn ewu27(&self) -> EWU27_R {
        EWU27_R::new(((self.bits >> 15) & 1) != 0)
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
            .field("ewu20", &self.ewu20())
            .field("ewu21", &self.ewu21())
            .field("ewu22", &self.ewu22())
            .field("ewu23", &self.ewu23())
            .field("ewu24", &self.ewu24())
            .field("ewu25", &self.ewu25())
            .field("ewu26", &self.ewu26())
            .field("ewu27", &self.ewu27())
            .finish()
    }
}
impl W {
    ///Bit 0 - EWU12 Enable WakeUp line 12 (PA0) When this bit is set the wakeup line 12 is enabled and a rising or falling edge on wakeup line 0 will trigger a CPU wakeup event depending on CR7.WP0 bit.
    #[inline(always)]
    pub fn ewu12(&mut self) -> EWU12_W<CR6rs> {
        EWU12_W::new(self, 0)
    }
    ///Bit 1 - EWU13 Enable WakeUp line 13 (PA1) When this bit is set the wakeup line 13 is enabled and a rising or falling edge on wakeup line 1 will trigger a CPU wakeup event depending on CR7.WP1 bit.
    #[inline(always)]
    pub fn ewu13(&mut self) -> EWU13_W<CR6rs> {
        EWU13_W::new(self, 1)
    }
    ///Bit 2 - EWU14 Enable WakeUp line 14 (PA2) When this bit is set the wakeup line 14 is enabled and a rising or falling edge on wakeup line 2 will trigger a CPU wakeup event depending on CR7.WP2 bit.
    #[inline(always)]
    pub fn ewu14(&mut self) -> EWU14_W<CR6rs> {
        EWU14_W::new(self, 2)
    }
    ///Bit 3 - EWU15 Enable WakeUp line 15 (PA3) When this bit is set the wakeup line 15 is enabled and a rising or falling edge on wakeup line 3 will trigger a CPU wakeup event depending on CR7.WP3 bit.
    #[inline(always)]
    pub fn ewu15(&mut self) -> EWU15_W<CR6rs> {
        EWU15_W::new(self, 3)
    }
    ///Bit 4 - EWU16 Enable WakeUp line 16 (PB12) When this bit is set the wakeup line 16 is enabled and a rising or falling edge on wakeup line 4 will trigger a CPU wakeup event depending on CR7.WP4 bit.
    #[inline(always)]
    pub fn ewu16(&mut self) -> EWU16_W<CR6rs> {
        EWU16_W::new(self, 4)
    }
    ///Bit 5 - EWU17 Enable WakeUp line 17 (PB13) When this bit is set the wakeup line 17 is enabled and a rising or falling edge on wakeup line 5 will trigger a CPU wakeup event depending on CR7.WP5 bit.
    #[inline(always)]
    pub fn ewu17(&mut self) -> EWU17_W<CR6rs> {
        EWU17_W::new(self, 5)
    }
    ///Bit 6 - EWU18 Enable WakeUp line 18 (PB14) When this bit is set the wakeup line 18 is enabled and a rising or falling edge on wakeup line 6 will trigger a CPU wakeup event depending on CR7.WP6 bit.
    #[inline(always)]
    pub fn ewu18(&mut self) -> EWU18_W<CR6rs> {
        EWU18_W::new(self, 6)
    }
    ///Bit 7 - EWU19 Enable WakeUp line 19 (PB15) When this bit is set the wakeup line 19 is enabled and a rising or falling edge on wakeup line 7 will trigger a CPU wakeup event depending on CR7.WP7 bit.
    #[inline(always)]
    pub fn ewu19(&mut self) -> EWU19_W<CR6rs> {
        EWU19_W::new(self, 7)
    }
    ///Bit 8 - Enable wakeup on PB8 I/O event.
    #[inline(always)]
    pub fn ewu20(&mut self) -> EWU20_W<CR6rs> {
        EWU20_W::new(self, 8)
    }
    ///Bit 9 - Enable wakeup on PB9 I/O event.
    #[inline(always)]
    pub fn ewu21(&mut self) -> EWU21_W<CR6rs> {
        EWU21_W::new(self, 9)
    }
    ///Bit 10 - Enable wakeup on PB10 I/O event.
    #[inline(always)]
    pub fn ewu22(&mut self) -> EWU22_W<CR6rs> {
        EWU22_W::new(self, 10)
    }
    ///Bit 11 - Enable wakeup on PB11 I/O event.
    #[inline(always)]
    pub fn ewu23(&mut self) -> EWU23_W<CR6rs> {
        EWU23_W::new(self, 11)
    }
    ///Bit 12 - Enable wakeup on PA12 I/O event.
    #[inline(always)]
    pub fn ewu24(&mut self) -> EWU24_W<CR6rs> {
        EWU24_W::new(self, 12)
    }
    ///Bit 13 - Enable wakeup on PA13 I/O event.
    #[inline(always)]
    pub fn ewu25(&mut self) -> EWU25_W<CR6rs> {
        EWU25_W::new(self, 13)
    }
    ///Bit 14 - Enable wakeup on PA14 I/O event.
    #[inline(always)]
    pub fn ewu26(&mut self) -> EWU26_W<CR6rs> {
        EWU26_W::new(self, 14)
    }
    ///Bit 15 - Enable wakeup on PA15 I/O event.
    #[inline(always)]
    pub fn ewu27(&mut self) -> EWU27_W<CR6rs> {
        EWU27_W::new(self, 15)
    }
}
/**CR6 register

You can [`read`](crate::Reg::read) this register and get [`cr6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#PWRC:CR6)*/
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

///Register `CR4` reader
pub type R = crate::R<CR4rs>;
///Register `CR4` writer
pub type W = crate::W<CR4rs>;
///Field `WUP0` reader - WUP0 Wake-up Line Polarity 0 (PB0) This bit defines the polarity used for event detection on external wake-up line 0
pub type WUP0_R = crate::BitReader;
///Field `WUP0` writer - WUP0 Wake-up Line Polarity 0 (PB0) This bit defines the polarity used for event detection on external wake-up line 0
pub type WUP0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUP1` reader - WUP1 Wake-up Line Polarity 1 (PB1) This bit defines the polarity used for event detection on external wake-up line 1
pub type WUP1_R = crate::BitReader;
///Field `WUP1` writer - WUP1 Wake-up Line Polarity 1 (PB1) This bit defines the polarity used for event detection on external wake-up line 1
pub type WUP1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUP2` reader - WUP2 Wake-up Line Polarity 2 (PB2) This bit defines the polarity used for event detection on external wake-up line 2
pub type WUP2_R = crate::BitReader;
///Field `WUP2` writer - WUP2 Wake-up Line Polarity 2 (PB2) This bit defines the polarity used for event detection on external wake-up line 2
pub type WUP2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUP3` reader - WUP3 Wake-up Line Polarity 3 (PB3) This bit defines the polarity used for event detection on external wake-up line 3
pub type WUP3_R = crate::BitReader;
///Field `WUP3` writer - WUP3 Wake-up Line Polarity 3 (PB3) This bit defines the polarity used for event detection on external wake-up line 3
pub type WUP3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUP4` reader - WUP4 Wake-up Line Polarity 4 (PB4) This bit defines the polarity used for event detection on external wake-up line 4
pub type WUP4_R = crate::BitReader;
///Field `WUP4` writer - WUP4 Wake-up Line Polarity 4 (PB4) This bit defines the polarity used for event detection on external wake-up line 4
pub type WUP4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUP5` reader - WUP5 Wake-up Line Polarity 5 (PB5) This bit defines the polarity used for event detection on external wake-up line 5
pub type WUP5_R = crate::BitReader;
///Field `WUP5` writer - WUP5 Wake-up Line Polarity 5 (PB5) This bit defines the polarity used for event detection on external wake-up line 5
pub type WUP5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUP6` reader - WUP6 Wake-up Line Polarity 6 (PB6) This bit defines the polarity used for event detection on external wake-up line 6
pub type WUP6_R = crate::BitReader;
///Field `WUP6` writer - WUP6 Wake-up Line Polarity 6 (PB6) This bit defines the polarity used for event detection on external wake-up line 6
pub type WUP6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUP7` reader - WUP7 Wake-up Line Polarity 7 (PB7) This bit defines the polarity used for event detection on external wake-up line 7
pub type WUP7_R = crate::BitReader;
///Field `WUP7` writer - WUP7 Wake-up Line Polarity 7 (PB7) This bit defines the polarity used for event detection on external wake-up line 7
pub type WUP7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUP8` reader - WUP8 Wake-up Line Polarity 8 (PA8) This bit defines the polarity used for event detection on external wake-up line 8
pub type WUP8_R = crate::BitReader;
///Field `WUP8` writer - WUP8 Wake-up Line Polarity 8 (PA8) This bit defines the polarity used for event detection on external wake-up line 8
pub type WUP8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUP9` reader - WUP9 Wake-up Line Polarity 9 (PA9) This bit defines the polarity used for event detection on external wake-up line 9
pub type WUP9_R = crate::BitReader;
///Field `WUP9` writer - WUP9 Wake-up Line Polarity 9 (PA9) This bit defines the polarity used for event detection on external wake-up line 9
pub type WUP9_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUP10` reader - WUP10 Wake-up Line Polarity 10 (PA10) This bit defines the polarity used for event detection on external wake-up line 10
pub type WUP10_R = crate::BitReader;
///Field `WUP10` writer - WUP10 Wake-up Line Polarity 10 (PA10) This bit defines the polarity used for event detection on external wake-up line 10
pub type WUP10_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUP11` reader - WUP11 Wake-up Line Polarity 11 (PA11) This bit defines the polarity used for event detection on external wake-up line 11
pub type WUP11_R = crate::BitReader;
///Field `WUP11` writer - WUP11 Wake-up Line Polarity 11 (PA11) This bit defines the polarity used for event detection on external wake-up line 11
pub type WUP11_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - WUP0 Wake-up Line Polarity 0 (PB0) This bit defines the polarity used for event detection on external wake-up line 0
    #[inline(always)]
    pub fn wup0(&self) -> WUP0_R {
        WUP0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - WUP1 Wake-up Line Polarity 1 (PB1) This bit defines the polarity used for event detection on external wake-up line 1
    #[inline(always)]
    pub fn wup1(&self) -> WUP1_R {
        WUP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - WUP2 Wake-up Line Polarity 2 (PB2) This bit defines the polarity used for event detection on external wake-up line 2
    #[inline(always)]
    pub fn wup2(&self) -> WUP2_R {
        WUP2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - WUP3 Wake-up Line Polarity 3 (PB3) This bit defines the polarity used for event detection on external wake-up line 3
    #[inline(always)]
    pub fn wup3(&self) -> WUP3_R {
        WUP3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - WUP4 Wake-up Line Polarity 4 (PB4) This bit defines the polarity used for event detection on external wake-up line 4
    #[inline(always)]
    pub fn wup4(&self) -> WUP4_R {
        WUP4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - WUP5 Wake-up Line Polarity 5 (PB5) This bit defines the polarity used for event detection on external wake-up line 5
    #[inline(always)]
    pub fn wup5(&self) -> WUP5_R {
        WUP5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - WUP6 Wake-up Line Polarity 6 (PB6) This bit defines the polarity used for event detection on external wake-up line 6
    #[inline(always)]
    pub fn wup6(&self) -> WUP6_R {
        WUP6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - WUP7 Wake-up Line Polarity 7 (PB7) This bit defines the polarity used for event detection on external wake-up line 7
    #[inline(always)]
    pub fn wup7(&self) -> WUP7_R {
        WUP7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - WUP8 Wake-up Line Polarity 8 (PA8) This bit defines the polarity used for event detection on external wake-up line 8
    #[inline(always)]
    pub fn wup8(&self) -> WUP8_R {
        WUP8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - WUP9 Wake-up Line Polarity 9 (PA9) This bit defines the polarity used for event detection on external wake-up line 9
    #[inline(always)]
    pub fn wup9(&self) -> WUP9_R {
        WUP9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - WUP10 Wake-up Line Polarity 10 (PA10) This bit defines the polarity used for event detection on external wake-up line 10
    #[inline(always)]
    pub fn wup10(&self) -> WUP10_R {
        WUP10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - WUP11 Wake-up Line Polarity 11 (PA11) This bit defines the polarity used for event detection on external wake-up line 11
    #[inline(always)]
    pub fn wup11(&self) -> WUP11_R {
        WUP11_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR4")
            .field("wup0", &self.wup0())
            .field("wup1", &self.wup1())
            .field("wup2", &self.wup2())
            .field("wup3", &self.wup3())
            .field("wup4", &self.wup4())
            .field("wup5", &self.wup5())
            .field("wup6", &self.wup6())
            .field("wup7", &self.wup7())
            .field("wup8", &self.wup8())
            .field("wup9", &self.wup9())
            .field("wup10", &self.wup10())
            .field("wup11", &self.wup11())
            .finish()
    }
}
impl W {
    ///Bit 0 - WUP0 Wake-up Line Polarity 0 (PB0) This bit defines the polarity used for event detection on external wake-up line 0
    #[inline(always)]
    pub fn wup0(&mut self) -> WUP0_W<'_, CR4rs> {
        WUP0_W::new(self, 0)
    }
    ///Bit 1 - WUP1 Wake-up Line Polarity 1 (PB1) This bit defines the polarity used for event detection on external wake-up line 1
    #[inline(always)]
    pub fn wup1(&mut self) -> WUP1_W<'_, CR4rs> {
        WUP1_W::new(self, 1)
    }
    ///Bit 2 - WUP2 Wake-up Line Polarity 2 (PB2) This bit defines the polarity used for event detection on external wake-up line 2
    #[inline(always)]
    pub fn wup2(&mut self) -> WUP2_W<'_, CR4rs> {
        WUP2_W::new(self, 2)
    }
    ///Bit 3 - WUP3 Wake-up Line Polarity 3 (PB3) This bit defines the polarity used for event detection on external wake-up line 3
    #[inline(always)]
    pub fn wup3(&mut self) -> WUP3_W<'_, CR4rs> {
        WUP3_W::new(self, 3)
    }
    ///Bit 4 - WUP4 Wake-up Line Polarity 4 (PB4) This bit defines the polarity used for event detection on external wake-up line 4
    #[inline(always)]
    pub fn wup4(&mut self) -> WUP4_W<'_, CR4rs> {
        WUP4_W::new(self, 4)
    }
    ///Bit 5 - WUP5 Wake-up Line Polarity 5 (PB5) This bit defines the polarity used for event detection on external wake-up line 5
    #[inline(always)]
    pub fn wup5(&mut self) -> WUP5_W<'_, CR4rs> {
        WUP5_W::new(self, 5)
    }
    ///Bit 6 - WUP6 Wake-up Line Polarity 6 (PB6) This bit defines the polarity used for event detection on external wake-up line 6
    #[inline(always)]
    pub fn wup6(&mut self) -> WUP6_W<'_, CR4rs> {
        WUP6_W::new(self, 6)
    }
    ///Bit 7 - WUP7 Wake-up Line Polarity 7 (PB7) This bit defines the polarity used for event detection on external wake-up line 7
    #[inline(always)]
    pub fn wup7(&mut self) -> WUP7_W<'_, CR4rs> {
        WUP7_W::new(self, 7)
    }
    ///Bit 8 - WUP8 Wake-up Line Polarity 8 (PA8) This bit defines the polarity used for event detection on external wake-up line 8
    #[inline(always)]
    pub fn wup8(&mut self) -> WUP8_W<'_, CR4rs> {
        WUP8_W::new(self, 8)
    }
    ///Bit 9 - WUP9 Wake-up Line Polarity 9 (PA9) This bit defines the polarity used for event detection on external wake-up line 9
    #[inline(always)]
    pub fn wup9(&mut self) -> WUP9_W<'_, CR4rs> {
        WUP9_W::new(self, 9)
    }
    ///Bit 10 - WUP10 Wake-up Line Polarity 10 (PA10) This bit defines the polarity used for event detection on external wake-up line 10
    #[inline(always)]
    pub fn wup10(&mut self) -> WUP10_W<'_, CR4rs> {
        WUP10_W::new(self, 10)
    }
    ///Bit 11 - WUP11 Wake-up Line Polarity 11 (PA11) This bit defines the polarity used for event detection on external wake-up line 11
    #[inline(always)]
    pub fn wup11(&mut self) -> WUP11_W<'_, CR4rs> {
        WUP11_W::new(self, 11)
    }
}
/**CR4 register

You can [`read`](crate::Reg::read) this register and get [`cr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#PWRC:CR4)*/
pub struct CR4rs;
impl crate::RegisterSpec for CR4rs {
    type Ux = u32;
}
///`read()` method returns [`cr4::R`](R) reader structure
impl crate::Readable for CR4rs {}
///`write(|w| ..)` method takes [`cr4::W`](W) writer structure
impl crate::Writable for CR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR4 to value 0
impl crate::Resettable for CR4rs {}

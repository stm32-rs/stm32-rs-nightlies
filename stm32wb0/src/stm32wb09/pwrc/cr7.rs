///Register `CR7` reader
pub type R = crate::R<CR7rs>;
///Register `CR7` writer
pub type W = crate::W<CR7rs>;
///Field `WUP12` reader - WUP12 Wake-up Line Polarity 12 (PA0) This bit defines the polarity used for event detection on external wake-up line 12
pub type WUP12_R = crate::BitReader;
///Field `WUP12` writer - WUP12 Wake-up Line Polarity 12 (PA0) This bit defines the polarity used for event detection on external wake-up line 12
pub type WUP12_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUP13` reader - WUP13 Wake-up Line Polarity 13 (PA1) This bit defines the polarity used for event detection on external wake-up line 13
pub type WUP13_R = crate::BitReader;
///Field `WUP13` writer - WUP13 Wake-up Line Polarity 13 (PA1) This bit defines the polarity used for event detection on external wake-up line 13
pub type WUP13_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUP14` reader - WUP14 Wake-up Line Polarity 14 (PA2) This bit defines the polarity used for event detection on external wake-up line 14
pub type WUP14_R = crate::BitReader;
///Field `WUP14` writer - WUP14 Wake-up Line Polarity 14 (PA2) This bit defines the polarity used for event detection on external wake-up line 14
pub type WUP14_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUP15` reader - WUP15 Wake-up Line Polarity 15 (PA3) This bit defines the polarity used for event detection on external wake-up line 15
pub type WUP15_R = crate::BitReader;
///Field `WUP15` writer - WUP15 Wake-up Line Polarity 15 (PA3) This bit defines the polarity used for event detection on external wake-up line 15
pub type WUP15_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUP16` reader - WUP16 Wake-up Line Polarity 16 (PB12) This bit defines the polarity used for event detection on external wake-up line 16
pub type WUP16_R = crate::BitReader;
///Field `WUP16` writer - WUP16 Wake-up Line Polarity 16 (PB12) This bit defines the polarity used for event detection on external wake-up line 16
pub type WUP16_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUP17` reader - WUP17 Wake-up Line Polarity 17 (PB13) This bit defines the polarity used for event detection on external wake-up line 17
pub type WUP17_R = crate::BitReader;
///Field `WUP17` writer - WUP17 Wake-up Line Polarity 17 (PB13) This bit defines the polarity used for event detection on external wake-up line 17
pub type WUP17_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUP18` reader - WUP18 Wake-up Line Polarity 18 (PB14) This bit defines the polarity used for event detection on external wake-up line 18
pub type WUP18_R = crate::BitReader;
///Field `WUP18` writer - WUP18 Wake-up Line Polarity 18 (PB14) This bit defines the polarity used for event detection on external wake-up line 18
pub type WUP18_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUP19` reader - WUP19 Wake-up Line Polarity 19 (PB15) This bit defines the polarity used for event detection on external wake-up line 19
pub type WUP19_R = crate::BitReader;
///Field `WUP19` writer - WUP19 Wake-up Line Polarity 19 (PB15) This bit defines the polarity used for event detection on external wake-up line 19
pub type WUP19_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - WUP12 Wake-up Line Polarity 12 (PA0) This bit defines the polarity used for event detection on external wake-up line 12
    #[inline(always)]
    pub fn wup12(&self) -> WUP12_R {
        WUP12_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - WUP13 Wake-up Line Polarity 13 (PA1) This bit defines the polarity used for event detection on external wake-up line 13
    #[inline(always)]
    pub fn wup13(&self) -> WUP13_R {
        WUP13_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - WUP14 Wake-up Line Polarity 14 (PA2) This bit defines the polarity used for event detection on external wake-up line 14
    #[inline(always)]
    pub fn wup14(&self) -> WUP14_R {
        WUP14_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - WUP15 Wake-up Line Polarity 15 (PA3) This bit defines the polarity used for event detection on external wake-up line 15
    #[inline(always)]
    pub fn wup15(&self) -> WUP15_R {
        WUP15_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - WUP16 Wake-up Line Polarity 16 (PB12) This bit defines the polarity used for event detection on external wake-up line 16
    #[inline(always)]
    pub fn wup16(&self) -> WUP16_R {
        WUP16_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - WUP17 Wake-up Line Polarity 17 (PB13) This bit defines the polarity used for event detection on external wake-up line 17
    #[inline(always)]
    pub fn wup17(&self) -> WUP17_R {
        WUP17_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - WUP18 Wake-up Line Polarity 18 (PB14) This bit defines the polarity used for event detection on external wake-up line 18
    #[inline(always)]
    pub fn wup18(&self) -> WUP18_R {
        WUP18_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - WUP19 Wake-up Line Polarity 19 (PB15) This bit defines the polarity used for event detection on external wake-up line 19
    #[inline(always)]
    pub fn wup19(&self) -> WUP19_R {
        WUP19_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR7")
            .field("wup12", &self.wup12())
            .field("wup13", &self.wup13())
            .field("wup14", &self.wup14())
            .field("wup15", &self.wup15())
            .field("wup16", &self.wup16())
            .field("wup17", &self.wup17())
            .field("wup18", &self.wup18())
            .field("wup19", &self.wup19())
            .finish()
    }
}
impl W {
    ///Bit 0 - WUP12 Wake-up Line Polarity 12 (PA0) This bit defines the polarity used for event detection on external wake-up line 12
    #[inline(always)]
    pub fn wup12(&mut self) -> WUP12_W<'_, CR7rs> {
        WUP12_W::new(self, 0)
    }
    ///Bit 1 - WUP13 Wake-up Line Polarity 13 (PA1) This bit defines the polarity used for event detection on external wake-up line 13
    #[inline(always)]
    pub fn wup13(&mut self) -> WUP13_W<'_, CR7rs> {
        WUP13_W::new(self, 1)
    }
    ///Bit 2 - WUP14 Wake-up Line Polarity 14 (PA2) This bit defines the polarity used for event detection on external wake-up line 14
    #[inline(always)]
    pub fn wup14(&mut self) -> WUP14_W<'_, CR7rs> {
        WUP14_W::new(self, 2)
    }
    ///Bit 3 - WUP15 Wake-up Line Polarity 15 (PA3) This bit defines the polarity used for event detection on external wake-up line 15
    #[inline(always)]
    pub fn wup15(&mut self) -> WUP15_W<'_, CR7rs> {
        WUP15_W::new(self, 3)
    }
    ///Bit 4 - WUP16 Wake-up Line Polarity 16 (PB12) This bit defines the polarity used for event detection on external wake-up line 16
    #[inline(always)]
    pub fn wup16(&mut self) -> WUP16_W<'_, CR7rs> {
        WUP16_W::new(self, 4)
    }
    ///Bit 5 - WUP17 Wake-up Line Polarity 17 (PB13) This bit defines the polarity used for event detection on external wake-up line 17
    #[inline(always)]
    pub fn wup17(&mut self) -> WUP17_W<'_, CR7rs> {
        WUP17_W::new(self, 5)
    }
    ///Bit 6 - WUP18 Wake-up Line Polarity 18 (PB14) This bit defines the polarity used for event detection on external wake-up line 18
    #[inline(always)]
    pub fn wup18(&mut self) -> WUP18_W<'_, CR7rs> {
        WUP18_W::new(self, 6)
    }
    ///Bit 7 - WUP19 Wake-up Line Polarity 19 (PB15) This bit defines the polarity used for event detection on external wake-up line 19
    #[inline(always)]
    pub fn wup19(&mut self) -> WUP19_W<'_, CR7rs> {
        WUP19_W::new(self, 7)
    }
}
/**CR7 register

You can [`read`](crate::Reg::read) this register and get [`cr7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#PWRC:CR7)*/
pub struct CR7rs;
impl crate::RegisterSpec for CR7rs {
    type Ux = u32;
}
///`read()` method returns [`cr7::R`](R) reader structure
impl crate::Readable for CR7rs {}
///`write(|w| ..)` method takes [`cr7::W`](W) writer structure
impl crate::Writable for CR7rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR7 to value 0
impl crate::Resettable for CR7rs {}

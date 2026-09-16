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
///Field `WUP20` reader - Wake-up polarity for PB8 IO event.
pub type WUP20_R = crate::BitReader;
///Field `WUP20` writer - Wake-up polarity for PB8 IO event.
pub type WUP20_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUP21` reader - Wake-up polarity for PB9 IO event.
pub type WUP21_R = crate::BitReader;
///Field `WUP21` writer - Wake-up polarity for PB9 IO event.
pub type WUP21_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUP22` reader - Wake-up polarity for PB10 IO event.
pub type WUP22_R = crate::BitReader;
///Field `WUP22` writer - Wake-up polarity for PB10 IO event.
pub type WUP22_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUP23` reader - Wake-up polarity for PB11 IO event.
pub type WUP23_R = crate::BitReader;
///Field `WUP23` writer - Wake-up polarity for PB11 IO event.
pub type WUP23_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUP24` reader - Wake-up polarity for PB12 IO event.
pub type WUP24_R = crate::BitReader;
///Field `WUP24` writer - Wake-up polarity for PB12 IO event.
pub type WUP24_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUP25` reader - Wake-up polarity for PB13 IO event.
pub type WUP25_R = crate::BitReader;
///Field `WUP25` writer - Wake-up polarity for PB13 IO event.
pub type WUP25_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUP26` reader - Wake-up polarity for PB14 IO event.
pub type WUP26_R = crate::BitReader;
///Field `WUP26` writer - Wake-up polarity for PB14 IO event.
pub type WUP26_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUP27` reader - Wake-up polarity for PB15 IO event.
pub type WUP27_R = crate::BitReader;
///Field `WUP27` writer - Wake-up polarity for PB15 IO event.
pub type WUP27_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 8 - Wake-up polarity for PB8 IO event.
    #[inline(always)]
    pub fn wup20(&self) -> WUP20_R {
        WUP20_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Wake-up polarity for PB9 IO event.
    #[inline(always)]
    pub fn wup21(&self) -> WUP21_R {
        WUP21_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Wake-up polarity for PB10 IO event.
    #[inline(always)]
    pub fn wup22(&self) -> WUP22_R {
        WUP22_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Wake-up polarity for PB11 IO event.
    #[inline(always)]
    pub fn wup23(&self) -> WUP23_R {
        WUP23_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Wake-up polarity for PB12 IO event.
    #[inline(always)]
    pub fn wup24(&self) -> WUP24_R {
        WUP24_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Wake-up polarity for PB13 IO event.
    #[inline(always)]
    pub fn wup25(&self) -> WUP25_R {
        WUP25_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Wake-up polarity for PB14 IO event.
    #[inline(always)]
    pub fn wup26(&self) -> WUP26_R {
        WUP26_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Wake-up polarity for PB15 IO event.
    #[inline(always)]
    pub fn wup27(&self) -> WUP27_R {
        WUP27_R::new(((self.bits >> 15) & 1) != 0)
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
            .field("wup20", &self.wup20())
            .field("wup21", &self.wup21())
            .field("wup22", &self.wup22())
            .field("wup23", &self.wup23())
            .field("wup24", &self.wup24())
            .field("wup25", &self.wup25())
            .field("wup26", &self.wup26())
            .field("wup27", &self.wup27())
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
    ///Bit 8 - Wake-up polarity for PB8 IO event.
    #[inline(always)]
    pub fn wup20(&mut self) -> WUP20_W<'_, CR7rs> {
        WUP20_W::new(self, 8)
    }
    ///Bit 9 - Wake-up polarity for PB9 IO event.
    #[inline(always)]
    pub fn wup21(&mut self) -> WUP21_W<'_, CR7rs> {
        WUP21_W::new(self, 9)
    }
    ///Bit 10 - Wake-up polarity for PB10 IO event.
    #[inline(always)]
    pub fn wup22(&mut self) -> WUP22_W<'_, CR7rs> {
        WUP22_W::new(self, 10)
    }
    ///Bit 11 - Wake-up polarity for PB11 IO event.
    #[inline(always)]
    pub fn wup23(&mut self) -> WUP23_W<'_, CR7rs> {
        WUP23_W::new(self, 11)
    }
    ///Bit 12 - Wake-up polarity for PB12 IO event.
    #[inline(always)]
    pub fn wup24(&mut self) -> WUP24_W<'_, CR7rs> {
        WUP24_W::new(self, 12)
    }
    ///Bit 13 - Wake-up polarity for PB13 IO event.
    #[inline(always)]
    pub fn wup25(&mut self) -> WUP25_W<'_, CR7rs> {
        WUP25_W::new(self, 13)
    }
    ///Bit 14 - Wake-up polarity for PB14 IO event.
    #[inline(always)]
    pub fn wup26(&mut self) -> WUP26_W<'_, CR7rs> {
        WUP26_W::new(self, 14)
    }
    ///Bit 15 - Wake-up polarity for PB15 IO event.
    #[inline(always)]
    pub fn wup27(&mut self) -> WUP27_W<'_, CR7rs> {
        WUP27_W::new(self, 15)
    }
}
/**CR7 register

You can [`read`](crate::Reg::read) this register and get [`cr7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#PWRC:CR7)*/
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

///Register `IWUP` reader
pub type R = crate::R<IWUPrs>;
///Register `IWUP` writer
pub type W = crate::W<IWUPrs>;
///Field `IWUP0` reader - IWUP0: Wakeup polarity for internal wakeup line 0 event (LPUART). - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge.
pub type IWUP0_R = crate::BitReader;
///Field `IWUP0` writer - IWUP0: Wakeup polarity for internal wakeup line 0 event (LPUART). - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge.
pub type IWUP0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWUP1` reader - IWUP1: Wakeup polarity for internal wakeup line 1 event (RTC). - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge.
pub type IWUP1_R = crate::BitReader;
///Field `IWUP1` writer - IWUP1: Wakeup polarity for internal wakeup line 1 event (RTC). - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge.
pub type IWUP1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWUP2` reader - IWUP2: Wakeup polarity for internal wakeup line 2 event (LCD). - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge.
pub type IWUP2_R = crate::BitReader;
///Field `IWUP2` writer - IWUP2: Wakeup polarity for internal wakeup line 2 event (LCD). - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge.
pub type IWUP2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWUP3` reader - IWUP3: Wakeup polarity for internal wakeup line 3 event (COMP). - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge.
pub type IWUP3_R = crate::BitReader;
///Field `IWUP3` writer - IWUP3: Wakeup polarity for internal wakeup line 3 event (COMP). - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge.
pub type IWUP3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWUP4` reader - IWUP4: Wakeup polarity for internal wakeup line 4 event (LCSC). - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge.
pub type IWUP4_R = crate::BitReader;
///Field `IWUP4` writer - IWUP4: Wakeup polarity for internal wakeup line 4 event (LCSC). - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge.
pub type IWUP4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WMRSUBGHP` reader - WMRSUBGHP: Wakeup polarity for internal wakeup MRSUBG event - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge.
pub type WMRSUBGHP_R = crate::BitReader;
///Field `WMRSUBGHP` writer - WMRSUBGHP: Wakeup polarity for internal wakeup MRSUBG event - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge.
pub type WMRSUBGHP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WMRSUBGHCPUP` reader - WMRSUBGHCPUP: Wakeup polarity for internal wakeup MRSUBG Host CPU event - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge.
pub type WMRSUBGHCPUP_R = crate::BitReader;
///Field `WMRSUBGHCPUP` writer - WMRSUBGHCPUP: Wakeup polarity for internal wakeup MRSUBG Host CPU event - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge.
pub type WMRSUBGHCPUP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WLPAWURP` reader - WLPAWURP: Wakeup polarity for wakeup LPAWUR event. - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge.
pub type WLPAWURP_R = crate::BitReader;
///Field `WLPAWURP` writer - WLPAWURP: Wakeup polarity for wakeup LPAWUR event. - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge.
pub type WLPAWURP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - IWUP0: Wakeup polarity for internal wakeup line 0 event (LPUART). - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge.
    #[inline(always)]
    pub fn iwup0(&self) -> IWUP0_R {
        IWUP0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IWUP1: Wakeup polarity for internal wakeup line 1 event (RTC). - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge.
    #[inline(always)]
    pub fn iwup1(&self) -> IWUP1_R {
        IWUP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IWUP2: Wakeup polarity for internal wakeup line 2 event (LCD). - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge.
    #[inline(always)]
    pub fn iwup2(&self) -> IWUP2_R {
        IWUP2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - IWUP3: Wakeup polarity for internal wakeup line 3 event (COMP). - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge.
    #[inline(always)]
    pub fn iwup3(&self) -> IWUP3_R {
        IWUP3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IWUP4: Wakeup polarity for internal wakeup line 4 event (LCSC). - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge.
    #[inline(always)]
    pub fn iwup4(&self) -> IWUP4_R {
        IWUP4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - WMRSUBGHP: Wakeup polarity for internal wakeup MRSUBG event - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge.
    #[inline(always)]
    pub fn wmrsubghp(&self) -> WMRSUBGHP_R {
        WMRSUBGHP_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - WMRSUBGHCPUP: Wakeup polarity for internal wakeup MRSUBG Host CPU event - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge.
    #[inline(always)]
    pub fn wmrsubghcpup(&self) -> WMRSUBGHCPUP_R {
        WMRSUBGHCPUP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - WLPAWURP: Wakeup polarity for wakeup LPAWUR event. - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge.
    #[inline(always)]
    pub fn wlpawurp(&self) -> WLPAWURP_R {
        WLPAWURP_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IWUP")
            .field("iwup0", &self.iwup0())
            .field("iwup1", &self.iwup1())
            .field("iwup2", &self.iwup2())
            .field("iwup3", &self.iwup3())
            .field("iwup4", &self.iwup4())
            .field("wmrsubghp", &self.wmrsubghp())
            .field("wmrsubghcpup", &self.wmrsubghcpup())
            .field("wlpawurp", &self.wlpawurp())
            .finish()
    }
}
impl W {
    ///Bit 0 - IWUP0: Wakeup polarity for internal wakeup line 0 event (LPUART). - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge.
    #[inline(always)]
    pub fn iwup0(&mut self) -> IWUP0_W<'_, IWUPrs> {
        IWUP0_W::new(self, 0)
    }
    ///Bit 1 - IWUP1: Wakeup polarity for internal wakeup line 1 event (RTC). - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge.
    #[inline(always)]
    pub fn iwup1(&mut self) -> IWUP1_W<'_, IWUPrs> {
        IWUP1_W::new(self, 1)
    }
    ///Bit 2 - IWUP2: Wakeup polarity for internal wakeup line 2 event (LCD). - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge.
    #[inline(always)]
    pub fn iwup2(&mut self) -> IWUP2_W<'_, IWUPrs> {
        IWUP2_W::new(self, 2)
    }
    ///Bit 3 - IWUP3: Wakeup polarity for internal wakeup line 3 event (COMP). - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge.
    #[inline(always)]
    pub fn iwup3(&mut self) -> IWUP3_W<'_, IWUPrs> {
        IWUP3_W::new(self, 3)
    }
    ///Bit 4 - IWUP4: Wakeup polarity for internal wakeup line 4 event (LCSC). - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge.
    #[inline(always)]
    pub fn iwup4(&mut self) -> IWUP4_W<'_, IWUPrs> {
        IWUP4_W::new(self, 4)
    }
    ///Bit 8 - WMRSUBGHP: Wakeup polarity for internal wakeup MRSUBG event - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge.
    #[inline(always)]
    pub fn wmrsubghp(&mut self) -> WMRSUBGHP_W<'_, IWUPrs> {
        WMRSUBGHP_W::new(self, 8)
    }
    ///Bit 9 - WMRSUBGHCPUP: Wakeup polarity for internal wakeup MRSUBG Host CPU event - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge.
    #[inline(always)]
    pub fn wmrsubghcpup(&mut self) -> WMRSUBGHCPUP_W<'_, IWUPrs> {
        WMRSUBGHCPUP_W::new(self, 9)
    }
    ///Bit 10 - WLPAWURP: Wakeup polarity for wakeup LPAWUR event. - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge.
    #[inline(always)]
    pub fn wlpawurp(&mut self) -> WLPAWURP_W<'_, IWUPrs> {
        WLPAWURP_W::new(self, 10)
    }
}
/**IWUP register

You can [`read`](crate::Reg::read) this register and get [`iwup::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iwup::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#PWRC:IWUP)*/
pub struct IWUPrs;
impl crate::RegisterSpec for IWUPrs {
    type Ux = u32;
}
///`read()` method returns [`iwup::R`](R) reader structure
impl crate::Readable for IWUPrs {}
///`write(|w| ..)` method takes [`iwup::W`](W) writer structure
impl crate::Writable for IWUPrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IWUP to value 0
impl crate::Resettable for IWUPrs {}

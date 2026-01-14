///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `WUCKSEL` reader - Wakeup clock selection
pub type WUCKSEL_R = crate::FieldReader;
///Field `WUCKSEL` writer - Wakeup clock selection
pub type WUCKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TSEDGE` reader - Time-stamp event active edge TSE must be reset when TSEDGE is changed to avoid unwanted TSF setting.
pub type TSEDGE_R = crate::BitReader;
///Field `TSEDGE` writer - Time-stamp event active edge TSE must be reset when TSEDGE is changed to avoid unwanted TSF setting.
pub type TSEDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REFCKON` reader - RTC_REFIN reference clock detection enable (50 or 60Hz) Note: PREDIV_S must be 0x00FF.
pub type REFCKON_R = crate::BitReader;
///Field `REFCKON` writer - RTC_REFIN reference clock detection enable (50 or 60Hz) Note: PREDIV_S must be 0x00FF.
pub type REFCKON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BYPSHAD` reader - Bypass the shadow registers Note: If the frequency of the APB clock is less than seven times the frequency of RTCCLK, BYPSHAD must be set to 1.
pub type BYPSHAD_R = crate::BitReader;
///Field `BYPSHAD` writer - Bypass the shadow registers Note: If the frequency of the APB clock is less than seven times the frequency of RTCCLK, BYPSHAD must be set to 1.
pub type BYPSHAD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMT` reader - Hour format
pub type FMT_R = crate::BitReader;
///Field `FMT` writer - Hour format
pub type FMT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALRE(A,B)` reader - Alarm %s enable
pub type ALRE_R = crate::BitReader;
///Field `ALRE(A,B)` writer - Alarm %s enable
pub type ALRE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUTE` reader - Wakeup timer enable
pub type WUTE_R = crate::BitReader;
///Field `WUTE` writer - Wakeup timer enable
pub type WUTE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSE` reader - timestamp enable
pub type TSE_R = crate::BitReader;
///Field `TSE` writer - timestamp enable
pub type TSE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALRIE(A,B)` reader - Alarm %s interrupt enable
pub type ALRIE_R = crate::BitReader;
///Field `ALRIE(A,B)` writer - Alarm %s interrupt enable
pub type ALRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUTIE` reader - Wakeup timer interrupt enable
pub type WUTIE_R = crate::BitReader;
///Field `WUTIE` writer - Wakeup timer interrupt enable
pub type WUTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSIE` reader - Time-stamp interrupt enable
pub type TSIE_R = crate::BitReader;
///Field `TSIE` writer - Time-stamp interrupt enable
pub type TSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADD1H` writer - Add 1 hour (summer time change) When this bit is set outside initialization mode, 1 hour is added to the calendar time. This bit is always read as 0.
pub type ADD1H_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SUB1H` writer - Subtract 1 hour (winter time change) When this bit is set outside initialization mode, 1 hour is subtracted to the calendar time if the current hour is not 0. This bit is always read as 0. Setting this bit has no effect when current hour is 0.
pub type SUB1H_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKP` reader - Backup This bit can be written by the user to memorize whether the daylight saving time change has been performed or not.
pub type BKP_R = crate::BitReader;
///Field `BKP` writer - Backup This bit can be written by the user to memorize whether the daylight saving time change has been performed or not.
pub type BKP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COSEL` reader - Calibration output selection When COE=1, this bit selects which signal is output on RTC_CALIB. These frequencies are valid for RTCCLK at 32.768 kHz and prescalers at their default values (PREDIV_A=127 and PREDIV_S=255). Refer to Section24.3.15: Calibration clock output
pub type COSEL_R = crate::BitReader;
///Field `COSEL` writer - Calibration output selection When COE=1, this bit selects which signal is output on RTC_CALIB. These frequencies are valid for RTCCLK at 32.768 kHz and prescalers at their default values (PREDIV_A=127 and PREDIV_S=255). Refer to Section24.3.15: Calibration clock output
pub type COSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `POL` reader - Output polarity This bit is used to configure the polarity of RTC_ALARM output
pub type POL_R = crate::BitReader;
///Field `POL` writer - Output polarity This bit is used to configure the polarity of RTC_ALARM output
pub type POL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OSEL` reader - Output selection These bits are used to select the flag to be routed to RTC_ALARM output
pub type OSEL_R = crate::FieldReader;
///Field `OSEL` writer - Output selection These bits are used to select the flag to be routed to RTC_ALARM output
pub type OSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `COE` reader - Calibration output enable This bit enables the RTC_CALIB output
pub type COE_R = crate::BitReader;
///Field `COE` writer - Calibration output enable This bit enables the RTC_CALIB output
pub type COE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITSE` reader - timestamp on internal event enable
pub type ITSE_R = crate::BitReader;
///Field `ITSE` writer - timestamp on internal event enable
pub type ITSE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - Wakeup clock selection
    #[inline(always)]
    pub fn wucksel(&self) -> WUCKSEL_R {
        WUCKSEL_R::new((self.bits & 7) as u8)
    }
    ///Bit 3 - Time-stamp event active edge TSE must be reset when TSEDGE is changed to avoid unwanted TSF setting.
    #[inline(always)]
    pub fn tsedge(&self) -> TSEDGE_R {
        TSEDGE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - RTC_REFIN reference clock detection enable (50 or 60Hz) Note: PREDIV_S must be 0x00FF.
    #[inline(always)]
    pub fn refckon(&self) -> REFCKON_R {
        REFCKON_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Bypass the shadow registers Note: If the frequency of the APB clock is less than seven times the frequency of RTCCLK, BYPSHAD must be set to 1.
    #[inline(always)]
    pub fn bypshad(&self) -> BYPSHAD_R {
        BYPSHAD_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Hour format
    #[inline(always)]
    pub fn fmt(&self) -> FMT_R {
        FMT_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Alarm (A,B) enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `ALRAE` field.</div>
    #[inline(always)]
    pub fn alre(&self, n: u8) -> ALRE_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        ALRE_R::new(((self.bits >> (n + 8)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Alarm (A,B) enable
    #[inline(always)]
    pub fn alre_iter(&self) -> impl Iterator<Item = ALRE_R> + '_ {
        (0..2).map(move |n| ALRE_R::new(((self.bits >> (n + 8)) & 1) != 0))
    }
    ///Bit 8 - Alarm A enable
    #[inline(always)]
    pub fn alrae(&self) -> ALRE_R {
        ALRE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Alarm B enable
    #[inline(always)]
    pub fn alrbe(&self) -> ALRE_R {
        ALRE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Wakeup timer enable
    #[inline(always)]
    pub fn wute(&self) -> WUTE_R {
        WUTE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - timestamp enable
    #[inline(always)]
    pub fn tse(&self) -> TSE_R {
        TSE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Alarm (A,B) interrupt enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `ALRAIE` field.</div>
    #[inline(always)]
    pub fn alrie(&self, n: u8) -> ALRIE_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        ALRIE_R::new(((self.bits >> (n + 12)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Alarm (A,B) interrupt enable
    #[inline(always)]
    pub fn alrie_iter(&self) -> impl Iterator<Item = ALRIE_R> + '_ {
        (0..2).map(move |n| ALRIE_R::new(((self.bits >> (n + 12)) & 1) != 0))
    }
    ///Bit 12 - Alarm A interrupt enable
    #[inline(always)]
    pub fn alraie(&self) -> ALRIE_R {
        ALRIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Alarm B interrupt enable
    #[inline(always)]
    pub fn alrbie(&self) -> ALRIE_R {
        ALRIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Wakeup timer interrupt enable
    #[inline(always)]
    pub fn wutie(&self) -> WUTIE_R {
        WUTIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Time-stamp interrupt enable
    #[inline(always)]
    pub fn tsie(&self) -> TSIE_R {
        TSIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 18 - Backup This bit can be written by the user to memorize whether the daylight saving time change has been performed or not.
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Calibration output selection When COE=1, this bit selects which signal is output on RTC_CALIB. These frequencies are valid for RTCCLK at 32.768 kHz and prescalers at their default values (PREDIV_A=127 and PREDIV_S=255). Refer to Section24.3.15: Calibration clock output
    #[inline(always)]
    pub fn cosel(&self) -> COSEL_R {
        COSEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Output polarity This bit is used to configure the polarity of RTC_ALARM output
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bits 21:22 - Output selection These bits are used to select the flag to be routed to RTC_ALARM output
    #[inline(always)]
    pub fn osel(&self) -> OSEL_R {
        OSEL_R::new(((self.bits >> 21) & 3) as u8)
    }
    ///Bit 23 - Calibration output enable This bit enables the RTC_CALIB output
    #[inline(always)]
    pub fn coe(&self) -> COE_R {
        COE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - timestamp on internal event enable
    #[inline(always)]
    pub fn itse(&self) -> ITSE_R {
        ITSE_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("wucksel", &self.wucksel())
            .field("tsedge", &self.tsedge())
            .field("refckon", &self.refckon())
            .field("bypshad", &self.bypshad())
            .field("fmt", &self.fmt())
            .field("alrae", &self.alrae())
            .field("alrbe", &self.alrbe())
            .field("wute", &self.wute())
            .field("tse", &self.tse())
            .field("alraie", &self.alraie())
            .field("alrbie", &self.alrbie())
            .field("wutie", &self.wutie())
            .field("tsie", &self.tsie())
            .field("bkp", &self.bkp())
            .field("cosel", &self.cosel())
            .field("pol", &self.pol())
            .field("osel", &self.osel())
            .field("coe", &self.coe())
            .field("itse", &self.itse())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Wakeup clock selection
    #[inline(always)]
    pub fn wucksel(&mut self) -> WUCKSEL_W<'_, CRrs> {
        WUCKSEL_W::new(self, 0)
    }
    ///Bit 3 - Time-stamp event active edge TSE must be reset when TSEDGE is changed to avoid unwanted TSF setting.
    #[inline(always)]
    pub fn tsedge(&mut self) -> TSEDGE_W<'_, CRrs> {
        TSEDGE_W::new(self, 3)
    }
    ///Bit 4 - RTC_REFIN reference clock detection enable (50 or 60Hz) Note: PREDIV_S must be 0x00FF.
    #[inline(always)]
    pub fn refckon(&mut self) -> REFCKON_W<'_, CRrs> {
        REFCKON_W::new(self, 4)
    }
    ///Bit 5 - Bypass the shadow registers Note: If the frequency of the APB clock is less than seven times the frequency of RTCCLK, BYPSHAD must be set to 1.
    #[inline(always)]
    pub fn bypshad(&mut self) -> BYPSHAD_W<'_, CRrs> {
        BYPSHAD_W::new(self, 5)
    }
    ///Bit 6 - Hour format
    #[inline(always)]
    pub fn fmt(&mut self) -> FMT_W<'_, CRrs> {
        FMT_W::new(self, 6)
    }
    ///Alarm (A,B) enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `ALRAE` field.</div>
    #[inline(always)]
    pub fn alre(&mut self, n: u8) -> ALRE_W<'_, CRrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        ALRE_W::new(self, n + 8)
    }
    ///Bit 8 - Alarm A enable
    #[inline(always)]
    pub fn alrae(&mut self) -> ALRE_W<'_, CRrs> {
        ALRE_W::new(self, 8)
    }
    ///Bit 9 - Alarm B enable
    #[inline(always)]
    pub fn alrbe(&mut self) -> ALRE_W<'_, CRrs> {
        ALRE_W::new(self, 9)
    }
    ///Bit 10 - Wakeup timer enable
    #[inline(always)]
    pub fn wute(&mut self) -> WUTE_W<'_, CRrs> {
        WUTE_W::new(self, 10)
    }
    ///Bit 11 - timestamp enable
    #[inline(always)]
    pub fn tse(&mut self) -> TSE_W<'_, CRrs> {
        TSE_W::new(self, 11)
    }
    ///Alarm (A,B) interrupt enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `ALRAIE` field.</div>
    #[inline(always)]
    pub fn alrie(&mut self, n: u8) -> ALRIE_W<'_, CRrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        ALRIE_W::new(self, n + 12)
    }
    ///Bit 12 - Alarm A interrupt enable
    #[inline(always)]
    pub fn alraie(&mut self) -> ALRIE_W<'_, CRrs> {
        ALRIE_W::new(self, 12)
    }
    ///Bit 13 - Alarm B interrupt enable
    #[inline(always)]
    pub fn alrbie(&mut self) -> ALRIE_W<'_, CRrs> {
        ALRIE_W::new(self, 13)
    }
    ///Bit 14 - Wakeup timer interrupt enable
    #[inline(always)]
    pub fn wutie(&mut self) -> WUTIE_W<'_, CRrs> {
        WUTIE_W::new(self, 14)
    }
    ///Bit 15 - Time-stamp interrupt enable
    #[inline(always)]
    pub fn tsie(&mut self) -> TSIE_W<'_, CRrs> {
        TSIE_W::new(self, 15)
    }
    ///Bit 16 - Add 1 hour (summer time change) When this bit is set outside initialization mode, 1 hour is added to the calendar time. This bit is always read as 0.
    #[inline(always)]
    pub fn add1h(&mut self) -> ADD1H_W<'_, CRrs> {
        ADD1H_W::new(self, 16)
    }
    ///Bit 17 - Subtract 1 hour (winter time change) When this bit is set outside initialization mode, 1 hour is subtracted to the calendar time if the current hour is not 0. This bit is always read as 0. Setting this bit has no effect when current hour is 0.
    #[inline(always)]
    pub fn sub1h(&mut self) -> SUB1H_W<'_, CRrs> {
        SUB1H_W::new(self, 17)
    }
    ///Bit 18 - Backup This bit can be written by the user to memorize whether the daylight saving time change has been performed or not.
    #[inline(always)]
    pub fn bkp(&mut self) -> BKP_W<'_, CRrs> {
        BKP_W::new(self, 18)
    }
    ///Bit 19 - Calibration output selection When COE=1, this bit selects which signal is output on RTC_CALIB. These frequencies are valid for RTCCLK at 32.768 kHz and prescalers at their default values (PREDIV_A=127 and PREDIV_S=255). Refer to Section24.3.15: Calibration clock output
    #[inline(always)]
    pub fn cosel(&mut self) -> COSEL_W<'_, CRrs> {
        COSEL_W::new(self, 19)
    }
    ///Bit 20 - Output polarity This bit is used to configure the polarity of RTC_ALARM output
    #[inline(always)]
    pub fn pol(&mut self) -> POL_W<'_, CRrs> {
        POL_W::new(self, 20)
    }
    ///Bits 21:22 - Output selection These bits are used to select the flag to be routed to RTC_ALARM output
    #[inline(always)]
    pub fn osel(&mut self) -> OSEL_W<'_, CRrs> {
        OSEL_W::new(self, 21)
    }
    ///Bit 23 - Calibration output enable This bit enables the RTC_CALIB output
    #[inline(always)]
    pub fn coe(&mut self) -> COE_W<'_, CRrs> {
        COE_W::new(self, 23)
    }
    ///Bit 24 - timestamp on internal event enable
    #[inline(always)]
    pub fn itse(&mut self) -> ITSE_W<'_, CRrs> {
        ITSE_W::new(self, 24)
    }
}
/**RTC control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#RTC:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}

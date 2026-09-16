///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `WUCKSEL` reader - Wakeup clock selection 000: RTC/16 clock is selected 001: RTC/8 clock is selected 010: RTC/4 clock is selected 011: RTC/2 clock is selected 10x: ck_spre (usually 1 Hz) clock is selected 11x: ck_spre (usually 1 Hz) clock is selected and 216 is added to the WUT counter value
pub type WUCKSEL_R = crate::FieldReader;
///Field `WUCKSEL` writer - Wakeup clock selection 000: RTC/16 clock is selected 001: RTC/8 clock is selected 010: RTC/4 clock is selected 011: RTC/2 clock is selected 10x: ck_spre (usually 1 Hz) clock is selected 11x: ck_spre (usually 1 Hz) clock is selected and 216 is added to the WUT counter value
pub type WUCKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `BYPSHAD` reader - Bypass the shadow registers 0: Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken from the shadow registers, which are updated once every two RTCCLK cycles. 1: Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken directly from the calendar counters.
pub type BYPSHAD_R = crate::BitReader;
///Field `BYPSHAD` writer - Bypass the shadow registers 0: Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken from the shadow registers, which are updated once every two RTCCLK cycles. 1: Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken directly from the calendar counters.
pub type BYPSHAD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMT` reader - Hour format
pub type FMT_R = crate::BitReader;
///Field `FMT` writer - Hour format
pub type FMT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALRAE` reader - Alarm A enable 0: Alarm A disabled 1: Alarm A enabled
pub type ALRAE_R = crate::BitReader;
///Field `ALRAE` writer - Alarm A enable 0: Alarm A disabled 1: Alarm A enabled
pub type ALRAE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUTE` reader - Wakeup timer enable 0: Wakeup timer disabled 1: Wakeup timer enabled
pub type WUTE_R = crate::BitReader;
///Field `WUTE` writer - Wakeup timer enable 0: Wakeup timer disabled 1: Wakeup timer enabled
pub type WUTE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALRAIE` reader - Alarm A interrupt enable 0: Alarm A interrupt disabled 1: Alarm A interrupt enabled
pub type ALRAIE_R = crate::BitReader;
///Field `ALRAIE` writer - Alarm A interrupt enable 0: Alarm A interrupt disabled 1: Alarm A interrupt enabled
pub type ALRAIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUTIE` reader - Wakeup timer interrupt enable 0: Wakeup timer interrupt disabled 1: Wakeup timer interrupt enabled
pub type WUTIE_R = crate::BitReader;
///Field `WUTIE` writer - Wakeup timer interrupt enable 0: Wakeup timer interrupt disabled 1: Wakeup timer interrupt enabled
pub type WUTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADD1H` writer - Add 1 hour (summer time change) When this bit is set outside initialization mode, 1 hour is added to the calendar time. This bit is always read as 0. 0: No effect 1: Adds 1 hour to the current time. This can be used for summer time change
pub type ADD1H_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SUB1H` writer - Subtract 1 hour (winter time change) When this bit is set outside initialization mode, 1 hour is subtracted to the calendar time if the current hour is not 0. This bit is always read as 0. Setting this bit has no effect when current hour is 0. 0: No effect 1: Subtracts 1 hour to the current time. This can be used for winter time change.
pub type SUB1H_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKP` reader - Backup This bit can be written by the user to memorize whether the daylight saving time change has been performed or not.
pub type BKP_R = crate::BitReader;
///Field `BKP` writer - Backup This bit can be written by the user to memorize whether the daylight saving time change has been performed or not.
pub type BKP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COSEL` reader - Calibration output selection When COE=1, this bit selects which signal is output on RTC_CALIB. 0: Calibration output is 512 Hz 1: Calibration output is 1 Hz These frequencies are valid for RTCCLK at 32.768 kHz and prescalers at their default values (PREDIV_A=127 and PREDIV_S=255).
pub type COSEL_R = crate::BitReader;
///Field `COSEL` writer - Calibration output selection When COE=1, this bit selects which signal is output on RTC_CALIB. 0: Calibration output is 512 Hz 1: Calibration output is 1 Hz These frequencies are valid for RTCCLK at 32.768 kHz and prescalers at their default values (PREDIV_A=127 and PREDIV_S=255).
pub type COSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `POL` reader - Output polarity This bit is used to configure the polarity of RTC_ALARM output 0: The pin is high when ALRAF/WUTF is asserted (depending on OSEL\[1:0\]) 1: The pin is low when ALRAF/WUTF is asserted (depending on OSEL\[1:0\]).
pub type POL_R = crate::BitReader;
///Field `POL` writer - Output polarity This bit is used to configure the polarity of RTC_ALARM output 0: The pin is high when ALRAF/WUTF is asserted (depending on OSEL\[1:0\]) 1: The pin is low when ALRAF/WUTF is asserted (depending on OSEL\[1:0\]).
pub type POL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OSEL` reader - Output selection These bits are used to select the flag to be routed to RTC_ALARM output 00: Output disabled 01: Alarm A output enabled 10: Reserved 11: Wakeup output enabled
pub type OSEL_R = crate::FieldReader;
///Field `OSEL` writer - Output selection These bits are used to select the flag to be routed to RTC_ALARM output 00: Output disabled 01: Alarm A output enabled 10: Reserved 11: Wakeup output enabled
pub type OSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `COE` reader - Calibration output enable This bit enables the RTC_CALIB output 0: Calibration output disabled 1: Calibration output enabled
pub type COE_R = crate::BitReader;
///Field `COE` writer - Calibration output enable This bit enables the RTC_CALIB output 0: Calibration output disabled 1: Calibration output enabled
pub type COE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - Wakeup clock selection 000: RTC/16 clock is selected 001: RTC/8 clock is selected 010: RTC/4 clock is selected 011: RTC/2 clock is selected 10x: ck_spre (usually 1 Hz) clock is selected 11x: ck_spre (usually 1 Hz) clock is selected and 216 is added to the WUT counter value
    #[inline(always)]
    pub fn wucksel(&self) -> WUCKSEL_R {
        WUCKSEL_R::new((self.bits & 7) as u8)
    }
    ///Bit 5 - Bypass the shadow registers 0: Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken from the shadow registers, which are updated once every two RTCCLK cycles. 1: Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken directly from the calendar counters.
    #[inline(always)]
    pub fn bypshad(&self) -> BYPSHAD_R {
        BYPSHAD_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Hour format
    #[inline(always)]
    pub fn fmt(&self) -> FMT_R {
        FMT_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - Alarm A enable 0: Alarm A disabled 1: Alarm A enabled
    #[inline(always)]
    pub fn alrae(&self) -> ALRAE_R {
        ALRAE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 10 - Wakeup timer enable 0: Wakeup timer disabled 1: Wakeup timer enabled
    #[inline(always)]
    pub fn wute(&self) -> WUTE_R {
        WUTE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - Alarm A interrupt enable 0: Alarm A interrupt disabled 1: Alarm A interrupt enabled
    #[inline(always)]
    pub fn alraie(&self) -> ALRAIE_R {
        ALRAIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - Wakeup timer interrupt enable 0: Wakeup timer interrupt disabled 1: Wakeup timer interrupt enabled
    #[inline(always)]
    pub fn wutie(&self) -> WUTIE_R {
        WUTIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 18 - Backup This bit can be written by the user to memorize whether the daylight saving time change has been performed or not.
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Calibration output selection When COE=1, this bit selects which signal is output on RTC_CALIB. 0: Calibration output is 512 Hz 1: Calibration output is 1 Hz These frequencies are valid for RTCCLK at 32.768 kHz and prescalers at their default values (PREDIV_A=127 and PREDIV_S=255).
    #[inline(always)]
    pub fn cosel(&self) -> COSEL_R {
        COSEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Output polarity This bit is used to configure the polarity of RTC_ALARM output 0: The pin is high when ALRAF/WUTF is asserted (depending on OSEL\[1:0\]) 1: The pin is low when ALRAF/WUTF is asserted (depending on OSEL\[1:0\]).
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bits 21:22 - Output selection These bits are used to select the flag to be routed to RTC_ALARM output 00: Output disabled 01: Alarm A output enabled 10: Reserved 11: Wakeup output enabled
    #[inline(always)]
    pub fn osel(&self) -> OSEL_R {
        OSEL_R::new(((self.bits >> 21) & 3) as u8)
    }
    ///Bit 23 - Calibration output enable This bit enables the RTC_CALIB output 0: Calibration output disabled 1: Calibration output enabled
    #[inline(always)]
    pub fn coe(&self) -> COE_R {
        COE_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("wucksel", &self.wucksel())
            .field("bypshad", &self.bypshad())
            .field("fmt", &self.fmt())
            .field("alrae", &self.alrae())
            .field("wute", &self.wute())
            .field("alraie", &self.alraie())
            .field("wutie", &self.wutie())
            .field("bkp", &self.bkp())
            .field("cosel", &self.cosel())
            .field("pol", &self.pol())
            .field("osel", &self.osel())
            .field("coe", &self.coe())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Wakeup clock selection 000: RTC/16 clock is selected 001: RTC/8 clock is selected 010: RTC/4 clock is selected 011: RTC/2 clock is selected 10x: ck_spre (usually 1 Hz) clock is selected 11x: ck_spre (usually 1 Hz) clock is selected and 216 is added to the WUT counter value
    #[inline(always)]
    pub fn wucksel(&mut self) -> WUCKSEL_W<'_, CRrs> {
        WUCKSEL_W::new(self, 0)
    }
    ///Bit 5 - Bypass the shadow registers 0: Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken from the shadow registers, which are updated once every two RTCCLK cycles. 1: Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken directly from the calendar counters.
    #[inline(always)]
    pub fn bypshad(&mut self) -> BYPSHAD_W<'_, CRrs> {
        BYPSHAD_W::new(self, 5)
    }
    ///Bit 6 - Hour format
    #[inline(always)]
    pub fn fmt(&mut self) -> FMT_W<'_, CRrs> {
        FMT_W::new(self, 6)
    }
    ///Bit 8 - Alarm A enable 0: Alarm A disabled 1: Alarm A enabled
    #[inline(always)]
    pub fn alrae(&mut self) -> ALRAE_W<'_, CRrs> {
        ALRAE_W::new(self, 8)
    }
    ///Bit 10 - Wakeup timer enable 0: Wakeup timer disabled 1: Wakeup timer enabled
    #[inline(always)]
    pub fn wute(&mut self) -> WUTE_W<'_, CRrs> {
        WUTE_W::new(self, 10)
    }
    ///Bit 12 - Alarm A interrupt enable 0: Alarm A interrupt disabled 1: Alarm A interrupt enabled
    #[inline(always)]
    pub fn alraie(&mut self) -> ALRAIE_W<'_, CRrs> {
        ALRAIE_W::new(self, 12)
    }
    ///Bit 14 - Wakeup timer interrupt enable 0: Wakeup timer interrupt disabled 1: Wakeup timer interrupt enabled
    #[inline(always)]
    pub fn wutie(&mut self) -> WUTIE_W<'_, CRrs> {
        WUTIE_W::new(self, 14)
    }
    ///Bit 16 - Add 1 hour (summer time change) When this bit is set outside initialization mode, 1 hour is added to the calendar time. This bit is always read as 0. 0: No effect 1: Adds 1 hour to the current time. This can be used for summer time change
    #[inline(always)]
    pub fn add1h(&mut self) -> ADD1H_W<'_, CRrs> {
        ADD1H_W::new(self, 16)
    }
    ///Bit 17 - Subtract 1 hour (winter time change) When this bit is set outside initialization mode, 1 hour is subtracted to the calendar time if the current hour is not 0. This bit is always read as 0. Setting this bit has no effect when current hour is 0. 0: No effect 1: Subtracts 1 hour to the current time. This can be used for winter time change.
    #[inline(always)]
    pub fn sub1h(&mut self) -> SUB1H_W<'_, CRrs> {
        SUB1H_W::new(self, 17)
    }
    ///Bit 18 - Backup This bit can be written by the user to memorize whether the daylight saving time change has been performed or not.
    #[inline(always)]
    pub fn bkp(&mut self) -> BKP_W<'_, CRrs> {
        BKP_W::new(self, 18)
    }
    ///Bit 19 - Calibration output selection When COE=1, this bit selects which signal is output on RTC_CALIB. 0: Calibration output is 512 Hz 1: Calibration output is 1 Hz These frequencies are valid for RTCCLK at 32.768 kHz and prescalers at their default values (PREDIV_A=127 and PREDIV_S=255).
    #[inline(always)]
    pub fn cosel(&mut self) -> COSEL_W<'_, CRrs> {
        COSEL_W::new(self, 19)
    }
    ///Bit 20 - Output polarity This bit is used to configure the polarity of RTC_ALARM output 0: The pin is high when ALRAF/WUTF is asserted (depending on OSEL\[1:0\]) 1: The pin is low when ALRAF/WUTF is asserted (depending on OSEL\[1:0\]).
    #[inline(always)]
    pub fn pol(&mut self) -> POL_W<'_, CRrs> {
        POL_W::new(self, 20)
    }
    ///Bits 21:22 - Output selection These bits are used to select the flag to be routed to RTC_ALARM output 00: Output disabled 01: Alarm A output enabled 10: Reserved 11: Wakeup output enabled
    #[inline(always)]
    pub fn osel(&mut self) -> OSEL_W<'_, CRrs> {
        OSEL_W::new(self, 21)
    }
    ///Bit 23 - Calibration output enable This bit enables the RTC_CALIB output 0: Calibration output disabled 1: Calibration output enabled
    #[inline(always)]
    pub fn coe(&mut self) -> COE_W<'_, CRrs> {
        COE_W::new(self, 23)
    }
}
/**RTC_CR register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#RTC:CR)*/
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

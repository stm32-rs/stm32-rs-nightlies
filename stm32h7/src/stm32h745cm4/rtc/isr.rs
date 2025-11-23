///Register `ISR` reader
pub type R = crate::R<ISRrs>;
///Register `ISR` writer
pub type W = crate::W<ISRrs>;
///Field `ALRWF(A,B)` reader - Alarm %s write flag
pub type ALRWF_R = crate::BitReader;
///Field `WUTWF` reader - Wakeup timer write flag This bit is set by hardware up to 2 RTCCLK cycles after the WUTE bit has been set to 0 in RTC_CR, and is cleared up to 2 RTCCLK cycles after the WUTE bit has been set to 1. The wakeup timer values can be changed when WUTE bit is cleared and WUTWF is set.
pub type WUTWF_R = crate::BitReader;
///Field `SHPF` reader - Shift operation pending This flag is set by hardware as soon as a shift operation is initiated by a write to the RTC_SHIFTR register. It is cleared by hardware when the corresponding shift operation has been executed. Writing to the SHPF bit has no effect.
pub type SHPF_R = crate::BitReader;
///Field `INITS` reader - Initialization status flag This bit is set by hardware when the calendar year field is different from 0 (Backup domain reset state).
pub type INITS_R = crate::BitReader;
///Field `RSF` reader - Registers synchronization flag This bit is set by hardware each time the calendar registers are copied into the shadow registers (RTC_SSRx, RTC_TRx and RTC_DRx). This bit is cleared by hardware in initialization mode, while a shift operation is pending (SHPF=1), or when in bypass shadow register mode (BYPSHAD=1). This bit can also be cleared by software. It is cleared either by software or by hardware in initialization mode.
pub type RSF_R = crate::BitReader;
///Field `RSF` writer - Registers synchronization flag This bit is set by hardware each time the calendar registers are copied into the shadow registers (RTC_SSRx, RTC_TRx and RTC_DRx). This bit is cleared by hardware in initialization mode, while a shift operation is pending (SHPF=1), or when in bypass shadow register mode (BYPSHAD=1). This bit can also be cleared by software. It is cleared either by software or by hardware in initialization mode.
pub type RSF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INITF` reader - Initialization flag When this bit is set to 1, the RTC is in initialization state, and the time, date and prescaler registers can be updated.
pub type INITF_R = crate::BitReader;
///Field `INIT` reader - Initialization mode
pub type INIT_R = crate::BitReader;
///Field `INIT` writer - Initialization mode
pub type INIT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALRF(A,B)` reader - Alarm %s flag
pub type ALRF_R = crate::BitReader;
///Field `ALRF(A,B)` writer - Alarm %s flag
pub type ALRF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUTF` reader - Wakeup timer flag This flag is set by hardware when the wakeup auto-reload counter reaches 0. This flag is cleared by software by writing 0. This flag must be cleared by software at least 1.5 RTCCLK periods before WUTF is set to 1 again.
pub type WUTF_R = crate::BitReader;
///Field `WUTF` writer - Wakeup timer flag This flag is set by hardware when the wakeup auto-reload counter reaches 0. This flag is cleared by software by writing 0. This flag must be cleared by software at least 1.5 RTCCLK periods before WUTF is set to 1 again.
pub type WUTF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSF` reader - Time-stamp flag This flag is set by hardware when a time-stamp event occurs. This flag is cleared by software by writing 0.
pub type TSF_R = crate::BitReader;
///Field `TSF` writer - Time-stamp flag This flag is set by hardware when a time-stamp event occurs. This flag is cleared by software by writing 0.
pub type TSF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSOVF` reader - Time-stamp overflow flag This flag is set by hardware when a time-stamp event occurs while TSF is already set. This flag is cleared by software by writing 0. It is recommended to check and then clear TSOVF only after clearing the TSF bit. Otherwise, an overflow might not be noticed if a time-stamp event occurs immediately before the TSF bit is cleared.
pub type TSOVF_R = crate::BitReader;
///Field `TSOVF` writer - Time-stamp overflow flag This flag is set by hardware when a time-stamp event occurs while TSF is already set. This flag is cleared by software by writing 0. It is recommended to check and then clear TSOVF only after clearing the TSF bit. Otherwise, an overflow might not be noticed if a time-stamp event occurs immediately before the TSF bit is cleared.
pub type TSOVF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP1F` reader - RTC_TAMP1 detection flag This flag is set by hardware when a tamper detection event is detected on the RTC_TAMP1 input. It is cleared by software writing 0
pub type TAMP1F_R = crate::BitReader;
///Field `TAMP1F` writer - RTC_TAMP1 detection flag This flag is set by hardware when a tamper detection event is detected on the RTC_TAMP1 input. It is cleared by software writing 0
pub type TAMP1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP2F` reader - RTC_TAMP2 detection flag This flag is set by hardware when a tamper detection event is detected on the RTC_TAMP2 input. It is cleared by software writing 0
pub type TAMP2F_R = crate::BitReader;
///Field `TAMP2F` writer - RTC_TAMP2 detection flag This flag is set by hardware when a tamper detection event is detected on the RTC_TAMP2 input. It is cleared by software writing 0
pub type TAMP2F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP3F` reader - RTC_TAMP3 detection flag This flag is set by hardware when a tamper detection event is detected on the RTC_TAMP3 input. It is cleared by software writing 0
pub type TAMP3F_R = crate::BitReader;
///Field `TAMP3F` writer - RTC_TAMP3 detection flag This flag is set by hardware when a tamper detection event is detected on the RTC_TAMP3 input. It is cleared by software writing 0
pub type TAMP3F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RECALPF` reader - Recalibration pending Flag The RECALPF status flag is automatically set to 1 when software writes to the RTC_CALR register, indicating that the RTC_CALR register is blocked. When the new calibration settings are taken into account, this bit returns to 0. Refer to Re-calibration on-the-fly.
pub type RECALPF_R = crate::BitReader;
///Field `ITSF` reader - Internal tTime-stamp flag
pub type ITSF_R = crate::BitReader;
///Field `ITSF` writer - Internal tTime-stamp flag
pub type ITSF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Alarm (A,B) write flag
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `ALRAWF` field.</div>
    #[inline(always)]
    pub fn alrwf(&self, n: u8) -> ALRWF_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        ALRWF_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Alarm (A,B) write flag
    #[inline(always)]
    pub fn alrwf_iter(&self) -> impl Iterator<Item = ALRWF_R> + '_ {
        (0..2).map(move |n| ALRWF_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - Alarm A write flag
    #[inline(always)]
    pub fn alrawf(&self) -> ALRWF_R {
        ALRWF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Alarm B write flag
    #[inline(always)]
    pub fn alrbwf(&self) -> ALRWF_R {
        ALRWF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Wakeup timer write flag This bit is set by hardware up to 2 RTCCLK cycles after the WUTE bit has been set to 0 in RTC_CR, and is cleared up to 2 RTCCLK cycles after the WUTE bit has been set to 1. The wakeup timer values can be changed when WUTE bit is cleared and WUTWF is set.
    #[inline(always)]
    pub fn wutwf(&self) -> WUTWF_R {
        WUTWF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Shift operation pending This flag is set by hardware as soon as a shift operation is initiated by a write to the RTC_SHIFTR register. It is cleared by hardware when the corresponding shift operation has been executed. Writing to the SHPF bit has no effect.
    #[inline(always)]
    pub fn shpf(&self) -> SHPF_R {
        SHPF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Initialization status flag This bit is set by hardware when the calendar year field is different from 0 (Backup domain reset state).
    #[inline(always)]
    pub fn inits(&self) -> INITS_R {
        INITS_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Registers synchronization flag This bit is set by hardware each time the calendar registers are copied into the shadow registers (RTC_SSRx, RTC_TRx and RTC_DRx). This bit is cleared by hardware in initialization mode, while a shift operation is pending (SHPF=1), or when in bypass shadow register mode (BYPSHAD=1). This bit can also be cleared by software. It is cleared either by software or by hardware in initialization mode.
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Initialization flag When this bit is set to 1, the RTC is in initialization state, and the time, date and prescaler registers can be updated.
    #[inline(always)]
    pub fn initf(&self) -> INITF_R {
        INITF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Initialization mode
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Alarm (A,B) flag
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `ALRAF` field.</div>
    #[inline(always)]
    pub fn alrf(&self, n: u8) -> ALRF_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        ALRF_R::new(((self.bits >> (n + 8)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Alarm (A,B) flag
    #[inline(always)]
    pub fn alrf_iter(&self) -> impl Iterator<Item = ALRF_R> + '_ {
        (0..2).map(move |n| ALRF_R::new(((self.bits >> (n + 8)) & 1) != 0))
    }
    ///Bit 8 - Alarm A flag
    #[inline(always)]
    pub fn alraf(&self) -> ALRF_R {
        ALRF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Alarm B flag
    #[inline(always)]
    pub fn alrbf(&self) -> ALRF_R {
        ALRF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Wakeup timer flag This flag is set by hardware when the wakeup auto-reload counter reaches 0. This flag is cleared by software by writing 0. This flag must be cleared by software at least 1.5 RTCCLK periods before WUTF is set to 1 again.
    #[inline(always)]
    pub fn wutf(&self) -> WUTF_R {
        WUTF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Time-stamp flag This flag is set by hardware when a time-stamp event occurs. This flag is cleared by software by writing 0.
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Time-stamp overflow flag This flag is set by hardware when a time-stamp event occurs while TSF is already set. This flag is cleared by software by writing 0. It is recommended to check and then clear TSOVF only after clearing the TSF bit. Otherwise, an overflow might not be noticed if a time-stamp event occurs immediately before the TSF bit is cleared.
    #[inline(always)]
    pub fn tsovf(&self) -> TSOVF_R {
        TSOVF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - RTC_TAMP1 detection flag This flag is set by hardware when a tamper detection event is detected on the RTC_TAMP1 input. It is cleared by software writing 0
    #[inline(always)]
    pub fn tamp1f(&self) -> TAMP1F_R {
        TAMP1F_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - RTC_TAMP2 detection flag This flag is set by hardware when a tamper detection event is detected on the RTC_TAMP2 input. It is cleared by software writing 0
    #[inline(always)]
    pub fn tamp2f(&self) -> TAMP2F_R {
        TAMP2F_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - RTC_TAMP3 detection flag This flag is set by hardware when a tamper detection event is detected on the RTC_TAMP3 input. It is cleared by software writing 0
    #[inline(always)]
    pub fn tamp3f(&self) -> TAMP3F_R {
        TAMP3F_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Recalibration pending Flag The RECALPF status flag is automatically set to 1 when software writes to the RTC_CALR register, indicating that the RTC_CALR register is blocked. When the new calibration settings are taken into account, this bit returns to 0. Refer to Re-calibration on-the-fly.
    #[inline(always)]
    pub fn recalpf(&self) -> RECALPF_R {
        RECALPF_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Internal tTime-stamp flag
    #[inline(always)]
    pub fn itsf(&self) -> ITSF_R {
        ITSF_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("alrawf", &self.alrawf())
            .field("alrbwf", &self.alrbwf())
            .field("wutwf", &self.wutwf())
            .field("shpf", &self.shpf())
            .field("inits", &self.inits())
            .field("rsf", &self.rsf())
            .field("initf", &self.initf())
            .field("init", &self.init())
            .field("alraf", &self.alraf())
            .field("alrbf", &self.alrbf())
            .field("wutf", &self.wutf())
            .field("tsf", &self.tsf())
            .field("tsovf", &self.tsovf())
            .field("tamp1f", &self.tamp1f())
            .field("tamp2f", &self.tamp2f())
            .field("tamp3f", &self.tamp3f())
            .field("recalpf", &self.recalpf())
            .field("itsf", &self.itsf())
            .finish()
    }
}
impl W {
    ///Bit 5 - Registers synchronization flag This bit is set by hardware each time the calendar registers are copied into the shadow registers (RTC_SSRx, RTC_TRx and RTC_DRx). This bit is cleared by hardware in initialization mode, while a shift operation is pending (SHPF=1), or when in bypass shadow register mode (BYPSHAD=1). This bit can also be cleared by software. It is cleared either by software or by hardware in initialization mode.
    #[inline(always)]
    pub fn rsf(&mut self) -> RSF_W<'_, ISRrs> {
        RSF_W::new(self, 5)
    }
    ///Bit 7 - Initialization mode
    #[inline(always)]
    pub fn init(&mut self) -> INIT_W<'_, ISRrs> {
        INIT_W::new(self, 7)
    }
    ///Alarm (A,B) flag
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `ALRAF` field.</div>
    #[inline(always)]
    pub fn alrf(&mut self, n: u8) -> ALRF_W<'_, ISRrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        ALRF_W::new(self, n + 8)
    }
    ///Bit 8 - Alarm A flag
    #[inline(always)]
    pub fn alraf(&mut self) -> ALRF_W<'_, ISRrs> {
        ALRF_W::new(self, 8)
    }
    ///Bit 9 - Alarm B flag
    #[inline(always)]
    pub fn alrbf(&mut self) -> ALRF_W<'_, ISRrs> {
        ALRF_W::new(self, 9)
    }
    ///Bit 10 - Wakeup timer flag This flag is set by hardware when the wakeup auto-reload counter reaches 0. This flag is cleared by software by writing 0. This flag must be cleared by software at least 1.5 RTCCLK periods before WUTF is set to 1 again.
    #[inline(always)]
    pub fn wutf(&mut self) -> WUTF_W<'_, ISRrs> {
        WUTF_W::new(self, 10)
    }
    ///Bit 11 - Time-stamp flag This flag is set by hardware when a time-stamp event occurs. This flag is cleared by software by writing 0.
    #[inline(always)]
    pub fn tsf(&mut self) -> TSF_W<'_, ISRrs> {
        TSF_W::new(self, 11)
    }
    ///Bit 12 - Time-stamp overflow flag This flag is set by hardware when a time-stamp event occurs while TSF is already set. This flag is cleared by software by writing 0. It is recommended to check and then clear TSOVF only after clearing the TSF bit. Otherwise, an overflow might not be noticed if a time-stamp event occurs immediately before the TSF bit is cleared.
    #[inline(always)]
    pub fn tsovf(&mut self) -> TSOVF_W<'_, ISRrs> {
        TSOVF_W::new(self, 12)
    }
    ///Bit 13 - RTC_TAMP1 detection flag This flag is set by hardware when a tamper detection event is detected on the RTC_TAMP1 input. It is cleared by software writing 0
    #[inline(always)]
    pub fn tamp1f(&mut self) -> TAMP1F_W<'_, ISRrs> {
        TAMP1F_W::new(self, 13)
    }
    ///Bit 14 - RTC_TAMP2 detection flag This flag is set by hardware when a tamper detection event is detected on the RTC_TAMP2 input. It is cleared by software writing 0
    #[inline(always)]
    pub fn tamp2f(&mut self) -> TAMP2F_W<'_, ISRrs> {
        TAMP2F_W::new(self, 14)
    }
    ///Bit 15 - RTC_TAMP3 detection flag This flag is set by hardware when a tamper detection event is detected on the RTC_TAMP3 input. It is cleared by software writing 0
    #[inline(always)]
    pub fn tamp3f(&mut self) -> TAMP3F_W<'_, ISRrs> {
        TAMP3F_W::new(self, 15)
    }
    ///Bit 17 - Internal tTime-stamp flag
    #[inline(always)]
    pub fn itsf(&mut self) -> ITSF_W<'_, ISRrs> {
        ITSF_W::new(self, 17)
    }
}
/**This register is write protected (except for RTC_ISR\[13:8\] bits). The write access procedure is described in RTC register write protection on page9.

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM4.html#RTC:ISR)*/
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`write(|w| ..)` method takes [`isr::W`](W) writer structure
impl crate::Writable for ISRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ISR to value 0x07
impl crate::Resettable for ISRrs {
    const RESET_VALUE: u32 = 0x07;
}

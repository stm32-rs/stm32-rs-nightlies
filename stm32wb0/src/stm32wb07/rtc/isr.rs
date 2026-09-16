///Register `ISR` reader
pub type R = crate::R<ISRrs>;
///Register `ISR` writer
pub type W = crate::W<ISRrs>;
///Field `ALRAWF` reader - Alarm A write flag This bit is set by hardware when Alarm A values can be changed, after the ALRAE bit has been set to 0 in RTC_CR. It is cleared by hardware in initialization mode. 0: Alarm A update not allowed 1: Alarm A update allowed.
pub type ALRAWF_R = crate::BitReader;
///Field `WUTWF` reader - Wakeup timer write flag This bit is set by hardware when the wakeup timer values can be changed, after the WUTE bit has been set to 0 in RTC_CR. 0: Wakeup timer configuration update not allowed 1: Wakeup timer configuration update allowed.
pub type WUTWF_R = crate::BitReader;
///Field `SHPF` reader - Shift operation pending 0: No shift operation is pending 1: A shift operation is pending This flag is set by hardware as soon as a shift operation is initiated by a write to the RTC_SHIFTR register. It is cleared by hardware when the corresponding shift operation has been executed. Writing to the SHPF bit has no effect.
pub type SHPF_R = crate::BitReader;
///Field `SHPF` writer - Shift operation pending 0: No shift operation is pending 1: A shift operation is pending This flag is set by hardware as soon as a shift operation is initiated by a write to the RTC_SHIFTR register. It is cleared by hardware when the corresponding shift operation has been executed. Writing to the SHPF bit has no effect.
pub type SHPF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INITS` reader - Initialization status flag This bit is set by hardware when the calendar year field is different from 0 (power-on reset state). 0: Calendar has not been initialized 1: Calendar has been initialized
pub type INITS_R = crate::BitReader;
///Field `RSF` reader - Registers synchronization flag This bit is set by hardware each time the calendar registers are copied into the shadow registers (RTC_SSRx, RTC_TRx and RTC_DRx). This bit is cleared by hardware in initialization mode, while a shift operation is pending (SHPF=1), or when in bypass shadow regsiter mode (BYPSHAD=1). This bit can also be cleared by software. It is cleared either by software or by hardware in initialization mode. 0: Calendar shadow registers not yet synchronized 1: Calendar shadow registers synchronized.
pub type RSF_R = crate::BitReader;
///Field `RSF` writer - Registers synchronization flag This bit is set by hardware each time the calendar registers are copied into the shadow registers (RTC_SSRx, RTC_TRx and RTC_DRx). This bit is cleared by hardware in initialization mode, while a shift operation is pending (SHPF=1), or when in bypass shadow regsiter mode (BYPSHAD=1). This bit can also be cleared by software. It is cleared either by software or by hardware in initialization mode. 0: Calendar shadow registers not yet synchronized 1: Calendar shadow registers synchronized.
pub type RSF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INITF` reader - Initialization flag When this bit is set to 1, the RTC is in initialization state, and the time, date and prescaler registers can be updated. 0: Calendar registers update is not allowed 1: Calendar registers update is allowed.
pub type INITF_R = crate::BitReader;
///Field `INIT` reader - Initialization mode 0: Free running mode 1: Initialization mode used to program time and date register (RTC_TR and RTC_DR), and prescaler register (RTC_PRER). Counters are stopped and start counting from the new value when INIT is reset.
pub type INIT_R = crate::BitReader;
///Field `INIT` writer - Initialization mode 0: Free running mode 1: Initialization mode used to program time and date register (RTC_TR and RTC_DR), and prescaler register (RTC_PRER). Counters are stopped and start counting from the new value when INIT is reset.
pub type INIT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALRAF` reader - Alarm A flag This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the Alarm A register (RTC_ALRMAR). This flag is cleared by software by writing 0.
pub type ALRAF_R = crate::BitReader;
///Field `ALRAF` writer - Alarm A flag This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the Alarm A register (RTC_ALRMAR). This flag is cleared by software by writing 0.
pub type ALRAF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUTF` reader - Wakeup timer flag This flag is set by hardware when the wakeup auto-reload counter reaches 0. This flag is cleared by software by writing 0. This flag must be cleared by software at least 1.5 RTCCLK periods before WUTF is set to 1 again.
pub type WUTF_R = crate::BitReader;
///Field `WUTF` writer - Wakeup timer flag This flag is set by hardware when the wakeup auto-reload counter reaches 0. This flag is cleared by software by writing 0. This flag must be cleared by software at least 1.5 RTCCLK periods before WUTF is set to 1 again.
pub type WUTF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RECALPF` reader - Recalibration pending Flag The RECALPF status flag is automatically set to 1 when software writes to the RTC_CALR register, indicating that the RTC_CALR register is blocked. When the new calibration settings are taken into account, this bit returns to 0.
pub type RECALPF_R = crate::BitReader;
///Field `RECALPF` writer - Recalibration pending Flag The RECALPF status flag is automatically set to 1 when software writes to the RTC_CALR register, indicating that the RTC_CALR register is blocked. When the new calibration settings are taken into account, this bit returns to 0.
pub type RECALPF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Alarm A write flag This bit is set by hardware when Alarm A values can be changed, after the ALRAE bit has been set to 0 in RTC_CR. It is cleared by hardware in initialization mode. 0: Alarm A update not allowed 1: Alarm A update allowed.
    #[inline(always)]
    pub fn alrawf(&self) -> ALRAWF_R {
        ALRAWF_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - Wakeup timer write flag This bit is set by hardware when the wakeup timer values can be changed, after the WUTE bit has been set to 0 in RTC_CR. 0: Wakeup timer configuration update not allowed 1: Wakeup timer configuration update allowed.
    #[inline(always)]
    pub fn wutwf(&self) -> WUTWF_R {
        WUTWF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Shift operation pending 0: No shift operation is pending 1: A shift operation is pending This flag is set by hardware as soon as a shift operation is initiated by a write to the RTC_SHIFTR register. It is cleared by hardware when the corresponding shift operation has been executed. Writing to the SHPF bit has no effect.
    #[inline(always)]
    pub fn shpf(&self) -> SHPF_R {
        SHPF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Initialization status flag This bit is set by hardware when the calendar year field is different from 0 (power-on reset state). 0: Calendar has not been initialized 1: Calendar has been initialized
    #[inline(always)]
    pub fn inits(&self) -> INITS_R {
        INITS_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Registers synchronization flag This bit is set by hardware each time the calendar registers are copied into the shadow registers (RTC_SSRx, RTC_TRx and RTC_DRx). This bit is cleared by hardware in initialization mode, while a shift operation is pending (SHPF=1), or when in bypass shadow regsiter mode (BYPSHAD=1). This bit can also be cleared by software. It is cleared either by software or by hardware in initialization mode. 0: Calendar shadow registers not yet synchronized 1: Calendar shadow registers synchronized.
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Initialization flag When this bit is set to 1, the RTC is in initialization state, and the time, date and prescaler registers can be updated. 0: Calendar registers update is not allowed 1: Calendar registers update is allowed.
    #[inline(always)]
    pub fn initf(&self) -> INITF_R {
        INITF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Initialization mode 0: Free running mode 1: Initialization mode used to program time and date register (RTC_TR and RTC_DR), and prescaler register (RTC_PRER). Counters are stopped and start counting from the new value when INIT is reset.
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Alarm A flag This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the Alarm A register (RTC_ALRMAR). This flag is cleared by software by writing 0.
    #[inline(always)]
    pub fn alraf(&self) -> ALRAF_R {
        ALRAF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 10 - Wakeup timer flag This flag is set by hardware when the wakeup auto-reload counter reaches 0. This flag is cleared by software by writing 0. This flag must be cleared by software at least 1.5 RTCCLK periods before WUTF is set to 1 again.
    #[inline(always)]
    pub fn wutf(&self) -> WUTF_R {
        WUTF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 16 - Recalibration pending Flag The RECALPF status flag is automatically set to 1 when software writes to the RTC_CALR register, indicating that the RTC_CALR register is blocked. When the new calibration settings are taken into account, this bit returns to 0.
    #[inline(always)]
    pub fn recalpf(&self) -> RECALPF_R {
        RECALPF_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("alrawf", &self.alrawf())
            .field("wutwf", &self.wutwf())
            .field("shpf", &self.shpf())
            .field("inits", &self.inits())
            .field("rsf", &self.rsf())
            .field("initf", &self.initf())
            .field("init", &self.init())
            .field("alraf", &self.alraf())
            .field("wutf", &self.wutf())
            .field("recalpf", &self.recalpf())
            .finish()
    }
}
impl W {
    ///Bit 3 - Shift operation pending 0: No shift operation is pending 1: A shift operation is pending This flag is set by hardware as soon as a shift operation is initiated by a write to the RTC_SHIFTR register. It is cleared by hardware when the corresponding shift operation has been executed. Writing to the SHPF bit has no effect.
    #[inline(always)]
    pub fn shpf(&mut self) -> SHPF_W<'_, ISRrs> {
        SHPF_W::new(self, 3)
    }
    ///Bit 5 - Registers synchronization flag This bit is set by hardware each time the calendar registers are copied into the shadow registers (RTC_SSRx, RTC_TRx and RTC_DRx). This bit is cleared by hardware in initialization mode, while a shift operation is pending (SHPF=1), or when in bypass shadow regsiter mode (BYPSHAD=1). This bit can also be cleared by software. It is cleared either by software or by hardware in initialization mode. 0: Calendar shadow registers not yet synchronized 1: Calendar shadow registers synchronized.
    #[inline(always)]
    pub fn rsf(&mut self) -> RSF_W<'_, ISRrs> {
        RSF_W::new(self, 5)
    }
    ///Bit 7 - Initialization mode 0: Free running mode 1: Initialization mode used to program time and date register (RTC_TR and RTC_DR), and prescaler register (RTC_PRER). Counters are stopped and start counting from the new value when INIT is reset.
    #[inline(always)]
    pub fn init(&mut self) -> INIT_W<'_, ISRrs> {
        INIT_W::new(self, 7)
    }
    ///Bit 8 - Alarm A flag This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the Alarm A register (RTC_ALRMAR). This flag is cleared by software by writing 0.
    #[inline(always)]
    pub fn alraf(&mut self) -> ALRAF_W<'_, ISRrs> {
        ALRAF_W::new(self, 8)
    }
    ///Bit 10 - Wakeup timer flag This flag is set by hardware when the wakeup auto-reload counter reaches 0. This flag is cleared by software by writing 0. This flag must be cleared by software at least 1.5 RTCCLK periods before WUTF is set to 1 again.
    #[inline(always)]
    pub fn wutf(&mut self) -> WUTF_W<'_, ISRrs> {
        WUTF_W::new(self, 10)
    }
    ///Bit 16 - Recalibration pending Flag The RECALPF status flag is automatically set to 1 when software writes to the RTC_CALR register, indicating that the RTC_CALR register is blocked. When the new calibration settings are taken into account, this bit returns to 0.
    #[inline(always)]
    pub fn recalpf(&mut self) -> RECALPF_W<'_, ISRrs> {
        RECALPF_W::new(self, 16)
    }
}
/**RTC_ISR register

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#RTC:ISR)*/
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

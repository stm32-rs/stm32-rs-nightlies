///Register `RTC_SR` reader
pub type R = crate::R<RTC_SRrs>;
///Field `ALRAF` reader - Alarm A flag This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the alarm A register (RTC_ALRMAR).
pub type ALRAF_R = crate::BitReader;
///Field `ALRBF` reader - Alarm B flag This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the alarm B register (RTC_ALRMBR).
pub type ALRBF_R = crate::BitReader;
/**Field `WUTF` reader - Wake-up timer flag This flag is set by hardware when the wake-up auto-reload counter reaches 0. If WUTOCLR\[15:0\]
is different from 0x0000, WUTF is cleared by hardware when the wake-up auto-reload counter reaches WUTOCLR value. If WUTOCLR\[15:0\]
is 0x0000, WUTF must be cleared by software. This flag must be cleared by software at least 1.5 RTCCLK periods before WUTF is set to 1 again.*/
pub type WUTF_R = crate::BitReader;
///Field `TSF` reader - Timestamp flag This flag is set by hardware when a timestamp event occurs. If ITSF flag is set, TSF must be cleared together with ITSF. Note: TSF is not set if TAMPTS1=11 and the tamper flag is read during the 3 ck_apre cycles following tamper event. Refer to Timestamp on tamper event for more details.
pub type TSF_R = crate::BitReader;
///Field `TSOVF` reader - Timestamp overflow flag This flag is set by hardware when a timestamp event occurs while TSF is already set. It is recommended to check and then clear TSOVF only after clearing the TSF bit. Otherwise, an overflow might not be noticed if a timestamp event occurs immediately before the TSF bit is cleared.
pub type TSOVF_R = crate::BitReader;
///Field `ITSF` reader - Internal timestamp flag This flag is set by hardware when a timestamp on the internal event occurs.
pub type ITSF_R = crate::BitReader;
///Field `SSRUF` reader - SSR underflow flag This flag is set by hardware when the SSR rolls under 0. SSRUF is not set when SSCLR=1.
pub type SSRUF_R = crate::BitReader;
impl R {
    ///Bit 0 - Alarm A flag This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the alarm A register (RTC_ALRMAR).
    #[inline(always)]
    pub fn alraf(&self) -> ALRAF_R {
        ALRAF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Alarm B flag This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the alarm B register (RTC_ALRMBR).
    #[inline(always)]
    pub fn alrbf(&self) -> ALRBF_R {
        ALRBF_R::new(((self.bits >> 1) & 1) != 0)
    }
    /**Bit 2 - Wake-up timer flag This flag is set by hardware when the wake-up auto-reload counter reaches 0. If WUTOCLR\[15:0\]
    is different from 0x0000, WUTF is cleared by hardware when the wake-up auto-reload counter reaches WUTOCLR value. If WUTOCLR\[15:0\]
    is 0x0000, WUTF must be cleared by software. This flag must be cleared by software at least 1.5 RTCCLK periods before WUTF is set to 1 again.*/
    #[inline(always)]
    pub fn wutf(&self) -> WUTF_R {
        WUTF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Timestamp flag This flag is set by hardware when a timestamp event occurs. If ITSF flag is set, TSF must be cleared together with ITSF. Note: TSF is not set if TAMPTS1=11 and the tamper flag is read during the 3 ck_apre cycles following tamper event. Refer to Timestamp on tamper event for more details.
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Timestamp overflow flag This flag is set by hardware when a timestamp event occurs while TSF is already set. It is recommended to check and then clear TSOVF only after clearing the TSF bit. Otherwise, an overflow might not be noticed if a timestamp event occurs immediately before the TSF bit is cleared.
    #[inline(always)]
    pub fn tsovf(&self) -> TSOVF_R {
        TSOVF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Internal timestamp flag This flag is set by hardware when a timestamp on the internal event occurs.
    #[inline(always)]
    pub fn itsf(&self) -> ITSF_R {
        ITSF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - SSR underflow flag This flag is set by hardware when the SSR rolls under 0. SSRUF is not set when SSCLR=1.
    #[inline(always)]
    pub fn ssruf(&self) -> SSRUF_R {
        SSRUF_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_SR")
            .field("alraf", &self.alraf())
            .field("alrbf", &self.alrbf())
            .field("wutf", &self.wutf())
            .field("tsf", &self.tsf())
            .field("tsovf", &self.tsovf())
            .field("itsf", &self.itsf())
            .field("ssruf", &self.ssruf())
            .finish()
    }
}
/**RTC status register

You can [`read`](crate::Reg::read) this register and get [`rtc_sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#RTC:RTC_SR)*/
pub struct RTC_SRrs;
impl crate::RegisterSpec for RTC_SRrs {
    type Ux = u32;
}
///`read()` method returns [`rtc_sr::R`](R) reader structure
impl crate::Readable for RTC_SRrs {}
///`reset()` method sets RTC_SR to value 0
impl crate::Resettable for RTC_SRrs {
    const RESET_VALUE: u32 = 0;
}

///Register `ISR` reader
pub type R = crate::R<ISRrs>;
/**SYNC event OK flag This flag is set by hardware when the measured frequency error is smaller than FELIM * 3. This means that either no adjustment of the TRIM value is needed or that an adjustment by one trimming step is enough to compensate the frequency error. An interrupt is generated if the SYNCOKIE bit is set in the CRS_CR register. It is cleared by software by setting the SYNCOKC bit in the CRS_ICR register.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNCOKF {
    ///0: Signal not set
    NotSignaled = 0,
    ///1: Signal set
    Signaled = 1,
}
impl From<SYNCOKF> for bool {
    #[inline(always)]
    fn from(variant: SYNCOKF) -> Self {
        variant as u8 != 0
    }
}
///Field `SYNCOKF` reader - SYNC event OK flag This flag is set by hardware when the measured frequency error is smaller than FELIM * 3. This means that either no adjustment of the TRIM value is needed or that an adjustment by one trimming step is enough to compensate the frequency error. An interrupt is generated if the SYNCOKIE bit is set in the CRS_CR register. It is cleared by software by setting the SYNCOKC bit in the CRS_ICR register.
pub type SYNCOKF_R = crate::BitReader<SYNCOKF>;
impl SYNCOKF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SYNCOKF {
        match self.bits {
            false => SYNCOKF::NotSignaled,
            true => SYNCOKF::Signaled,
        }
    }
    ///Signal not set
    #[inline(always)]
    pub fn is_not_signaled(&self) -> bool {
        *self == SYNCOKF::NotSignaled
    }
    ///Signal set
    #[inline(always)]
    pub fn is_signaled(&self) -> bool {
        *self == SYNCOKF::Signaled
    }
}
///Field `SYNCWARNF` reader - SYNC warning flag This flag is set by hardware when the measured frequency error is greater than or equal to FELIM * 3, but smaller than FELIM * 128. This means that to compensate the frequency error, the TRIM value must be adjusted by two steps or more. An interrupt is generated if the SYNCWARNIE bit is set in the CRS_CR register. It is cleared by software by setting the SYNCWARNC bit in the CRS_ICR register.
pub use SYNCOKF_R as SYNCWARNF_R;
///Field `ERRF` reader - Error flag This flag is set by hardware in case of any synchronization or trimming error. It is the logical OR of the TRIMOVF, SYNCMISS and SYNCERR bits. An interrupt is generated if the ERRIE bit is set in the CRS_CR register. It is cleared by software in reaction to setting the ERRC bit in the CRS_ICR register, which clears the TRIMOVF, SYNCMISS and SYNCERR bits.
pub use SYNCOKF_R as ERRF_R;
///Field `ESYNCF` reader - Expected SYNC flag This flag is set by hardware when the frequency error counter reached a zero value. An interrupt is generated if the ESYNCIE bit is set in the CRS_CR register. It is cleared by software by setting the ESYNCC bit in the CRS_ICR register.
pub use SYNCOKF_R as ESYNCF_R;
///Field `SYNCERR` reader - SYNC error This flag is set by hardware when the SYNC pulse arrives before the ESYNC event and the measured frequency error is greater than or equal to FELIM * 128. This means that the frequency error is too big (internal frequency too low) to be compensated by adjusting the TRIM value, and that some other action has to be taken. An interrupt is generated if the ERRIE bit is set in the CRS_CR register. It is cleared by software by setting the ERRC bit in the CRS_ICR register.
pub use SYNCOKF_R as SYNCERR_R;
///Field `SYNCMISS` reader - SYNC missed This flag is set by hardware when the frequency error counter reaches value FELIM * 128 and no SYNC is detected, meaning either that a SYNC pulse was missed, or the frequency error is too big (internal frequency too high) to be compensated by adjusting the TRIM value, hence some other action must be taken. At this point, the frequency error counter is stopped (waiting for a next SYNC), and an interrupt is generated if the ERRIE bit is set in the CRS_CR register. It is cleared by software by setting the ERRC bit in the CRS_ICR register.
pub use SYNCOKF_R as SYNCMISS_R;
///Field `TRIMOVF` reader - Trimming overflow or underflow This flag is set by hardware when the automatic trimming tries to over- or under-flow the TRIM value. An interrupt is generated if the ERRIE bit is set in the CRS_CR register. It is cleared by software by setting the ERRC bit in the CRS_ICR register.
pub use SYNCOKF_R as TRIMOVF_R;
/**Frequency error direction FEDIR is the counting direction of the frequency error counter latched in the time of the last SYNC event. It shows whether the actual frequency is below or above the target.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FEDIR {
    ///0: Error in up-counting direction
    UpCounting = 0,
    ///1: Error in down-counting direction
    DownCounting = 1,
}
impl From<FEDIR> for bool {
    #[inline(always)]
    fn from(variant: FEDIR) -> Self {
        variant as u8 != 0
    }
}
///Field `FEDIR` reader - Frequency error direction FEDIR is the counting direction of the frequency error counter latched in the time of the last SYNC event. It shows whether the actual frequency is below or above the target.
pub type FEDIR_R = crate::BitReader<FEDIR>;
impl FEDIR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FEDIR {
        match self.bits {
            false => FEDIR::UpCounting,
            true => FEDIR::DownCounting,
        }
    }
    ///Error in up-counting direction
    #[inline(always)]
    pub fn is_up_counting(&self) -> bool {
        *self == FEDIR::UpCounting
    }
    ///Error in down-counting direction
    #[inline(always)]
    pub fn is_down_counting(&self) -> bool {
        *self == FEDIR::DownCounting
    }
}
///Field `FECAP` reader - Frequency error capture FECAP is the frequency error counter value latched in the time of the last SYNC event. Refer to Section15.4.4 for more details about FECAP usage.
pub type FECAP_R = crate::FieldReader<u16>;
impl R {
    ///Bit 0 - SYNC event OK flag This flag is set by hardware when the measured frequency error is smaller than FELIM * 3. This means that either no adjustment of the TRIM value is needed or that an adjustment by one trimming step is enough to compensate the frequency error. An interrupt is generated if the SYNCOKIE bit is set in the CRS_CR register. It is cleared by software by setting the SYNCOKC bit in the CRS_ICR register.
    #[inline(always)]
    pub fn syncokf(&self) -> SYNCOKF_R {
        SYNCOKF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SYNC warning flag This flag is set by hardware when the measured frequency error is greater than or equal to FELIM * 3, but smaller than FELIM * 128. This means that to compensate the frequency error, the TRIM value must be adjusted by two steps or more. An interrupt is generated if the SYNCWARNIE bit is set in the CRS_CR register. It is cleared by software by setting the SYNCWARNC bit in the CRS_ICR register.
    #[inline(always)]
    pub fn syncwarnf(&self) -> SYNCWARNF_R {
        SYNCWARNF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Error flag This flag is set by hardware in case of any synchronization or trimming error. It is the logical OR of the TRIMOVF, SYNCMISS and SYNCERR bits. An interrupt is generated if the ERRIE bit is set in the CRS_CR register. It is cleared by software in reaction to setting the ERRC bit in the CRS_ICR register, which clears the TRIMOVF, SYNCMISS and SYNCERR bits.
    #[inline(always)]
    pub fn errf(&self) -> ERRF_R {
        ERRF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Expected SYNC flag This flag is set by hardware when the frequency error counter reached a zero value. An interrupt is generated if the ESYNCIE bit is set in the CRS_CR register. It is cleared by software by setting the ESYNCC bit in the CRS_ICR register.
    #[inline(always)]
    pub fn esyncf(&self) -> ESYNCF_R {
        ESYNCF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 8 - SYNC error This flag is set by hardware when the SYNC pulse arrives before the ESYNC event and the measured frequency error is greater than or equal to FELIM * 128. This means that the frequency error is too big (internal frequency too low) to be compensated by adjusting the TRIM value, and that some other action has to be taken. An interrupt is generated if the ERRIE bit is set in the CRS_CR register. It is cleared by software by setting the ERRC bit in the CRS_ICR register.
    #[inline(always)]
    pub fn syncerr(&self) -> SYNCERR_R {
        SYNCERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SYNC missed This flag is set by hardware when the frequency error counter reaches value FELIM * 128 and no SYNC is detected, meaning either that a SYNC pulse was missed, or the frequency error is too big (internal frequency too high) to be compensated by adjusting the TRIM value, hence some other action must be taken. At this point, the frequency error counter is stopped (waiting for a next SYNC), and an interrupt is generated if the ERRIE bit is set in the CRS_CR register. It is cleared by software by setting the ERRC bit in the CRS_ICR register.
    #[inline(always)]
    pub fn syncmiss(&self) -> SYNCMISS_R {
        SYNCMISS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Trimming overflow or underflow This flag is set by hardware when the automatic trimming tries to over- or under-flow the TRIM value. An interrupt is generated if the ERRIE bit is set in the CRS_CR register. It is cleared by software by setting the ERRC bit in the CRS_ICR register.
    #[inline(always)]
    pub fn trimovf(&self) -> TRIMOVF_R {
        TRIMOVF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 15 - Frequency error direction FEDIR is the counting direction of the frequency error counter latched in the time of the last SYNC event. It shows whether the actual frequency is below or above the target.
    #[inline(always)]
    pub fn fedir(&self) -> FEDIR_R {
        FEDIR_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:31 - Frequency error capture FECAP is the frequency error counter value latched in the time of the last SYNC event. Refer to Section15.4.4 for more details about FECAP usage.
    #[inline(always)]
    pub fn fecap(&self) -> FECAP_R {
        FECAP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("syncokf", &self.syncokf())
            .field("syncwarnf", &self.syncwarnf())
            .field("errf", &self.errf())
            .field("esyncf", &self.esyncf())
            .field("syncerr", &self.syncerr())
            .field("syncmiss", &self.syncmiss())
            .field("trimovf", &self.trimovf())
            .field("fedir", &self.fedir())
            .field("fecap", &self.fecap())
            .finish()
    }
}
/**CRS interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#CRS:ISR)*/
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`reset()` method sets ISR to value 0
impl crate::Resettable for ISRrs {}

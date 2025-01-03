///Register `RTC_TSSSR` reader
pub type R = crate::R<RTC_TSSSRrs>;
/**Field `SS` reader - Subsecond value/synchronous binary counter values SS\[31:0\]
is the value of the synchronous prescaler counter when the timestamp event occurred.*/
pub type SS_R = crate::FieldReader<u32>;
impl R {
    /**Bits 0:31 - Subsecond value/synchronous binary counter values SS\[31:0\]
    is the value of the synchronous prescaler counter when the timestamp event occurred.*/
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_TSSSR").field("ss", &self.ss()).finish()
    }
}
/**RTC timestamp subsecond register

You can [`read`](crate::Reg::read) this register and get [`rtc_tsssr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#RTC:RTC_TSSSR)*/
pub struct RTC_TSSSRrs;
impl crate::RegisterSpec for RTC_TSSSRrs {
    type Ux = u32;
}
///`read()` method returns [`rtc_tsssr::R`](R) reader structure
impl crate::Readable for RTC_TSSSRrs {}
///`reset()` method sets RTC_TSSSR to value 0
impl crate::Resettable for RTC_TSSSRrs {
    const RESET_VALUE: u32 = 0;
}

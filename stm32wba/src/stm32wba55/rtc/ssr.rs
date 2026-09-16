///Register `SSR` reader
pub type R = crate::R<SSRrs>;
///Field `SS` reader - Synchronous binary counter SS\[31:16\]: Synchronous binary counter MSB values When Binary or Mixed mode is selected (BIN = 01 or 10 or 11): SS\[31:16\] are the 16 MSB of the SS\[31:0\] free-running down-counter. When BCD mode is selected (BIN=00): SS\[31:16\] are forced by hardware to 0x0000. SS\[15:0\]: Subsecond value/synchronous binary counter LSB values When Binary mode is selected (BIN = 01 or 10 or 11): SS\[15:0\] are the 16 LSB of the SS\[31:0\] free-running down-counter. When BCD mode is selected (BIN=00): SS\[15:0\] is the value in the synchronous prescaler counter. The fraction of a second is given by the formula below: Second fraction = (PREDIV_S - SS) / (PREDIV_S + 1) SS can be larger than PREDIV_S only after a shift operation. In that case, the correct time/date is one second less than as indicated by RTC_TR/RTC_DR.
pub type SS_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Synchronous binary counter SS\[31:16\]: Synchronous binary counter MSB values When Binary or Mixed mode is selected (BIN = 01 or 10 or 11): SS\[31:16\] are the 16 MSB of the SS\[31:0\] free-running down-counter. When BCD mode is selected (BIN=00): SS\[31:16\] are forced by hardware to 0x0000. SS\[15:0\]: Subsecond value/synchronous binary counter LSB values When Binary mode is selected (BIN = 01 or 10 or 11): SS\[15:0\] are the 16 LSB of the SS\[31:0\] free-running down-counter. When BCD mode is selected (BIN=00): SS\[15:0\] is the value in the synchronous prescaler counter. The fraction of a second is given by the formula below: Second fraction = (PREDIV_S - SS) / (PREDIV_S + 1) SS can be larger than PREDIV_S only after a shift operation. In that case, the correct time/date is one second less than as indicated by RTC_TR/RTC_DR.
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SSR").field("ss", &self.ss()).finish()
    }
}
/**RTC subsecond register

You can [`read`](crate::Reg::read) this register and get [`ssr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#RTC:SSR)*/
pub struct SSRrs;
impl crate::RegisterSpec for SSRrs {
    type Ux = u32;
}
///`read()` method returns [`ssr::R`](R) reader structure
impl crate::Readable for SSRrs {}
///`reset()` method sets SSR to value 0
impl crate::Resettable for SSRrs {}

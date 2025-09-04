///Register `SSR` reader
pub type R = crate::R<SSRrs>;
///Field `SS` reader - Sub second value SS\[15:0\] is the value in the synchronous prescalers counter. The fraction of a second is given by the formula below: Second fraction = ( PREDIV_S - SS ) / ( PREDIV_S + 1 )
pub type SS_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Sub second value SS\[15:0\] is the value in the synchronous prescalers counter. The fraction of a second is given by the formula below: Second fraction = ( PREDIV_S - SS ) / ( PREDIV_S + 1 )
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SSR").field("ss", &self.ss()).finish()
    }
}
/**RTC_SSR register

You can [`read`](crate::Reg::read) this register and get [`ssr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#RTC:SSR)*/
pub struct SSRrs;
impl crate::RegisterSpec for SSRrs {
    type Ux = u32;
}
///`read()` method returns [`ssr::R`](R) reader structure
impl crate::Readable for SSRrs {}
///`reset()` method sets SSR to value 0
impl crate::Resettable for SSRrs {}

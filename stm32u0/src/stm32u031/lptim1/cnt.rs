///Register `CNT` reader
pub type R = crate::R<CNTrs>;
///Field `CNT` reader - Counter value When the LPTIM is running with an asynchronous clock, reading the LPTIM1_CNT register may return unreliable values. So in this case it is necessary to perform two consecutive read accesses and verify that the two returned values are identical.
pub type CNT_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Counter value When the LPTIM is running with an asynchronous clock, reading the LPTIM1_CNT register may return unreliable values. So in this case it is necessary to perform two consecutive read accesses and verify that the two returned values are identical.
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNT").field("cnt", &self.cnt()).finish()
    }
}
/**LPTIM counter register

You can [`read`](crate::Reg::read) this register and get [`cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#LPTIM1:CNT)*/
pub struct CNTrs;
impl crate::RegisterSpec for CNTrs {
    type Ux = u32;
}
///`read()` method returns [`cnt::R`](R) reader structure
impl crate::Readable for CNTrs {}
///`reset()` method sets CNT to value 0
impl crate::Resettable for CNTrs {}

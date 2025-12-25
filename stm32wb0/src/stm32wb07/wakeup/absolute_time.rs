///Register `ABSOLUTE_TIME` reader
pub type R = crate::R<ABSOLUTE_TIMErs>;
///Field `ABSOLUTE_TIME` reader - absolute time
pub type ABSOLUTE_TIME_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - absolute time
    #[inline(always)]
    pub fn absolute_time(&self) -> ABSOLUTE_TIME_R {
        ABSOLUTE_TIME_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ABSOLUTE_TIME")
            .field("absolute_time", &self.absolute_time())
            .finish()
    }
}
/**ABSOLUTE_TIME register

You can [`read`](crate::Reg::read) this register and get [`absolute_time::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#WAKEUP:ABSOLUTE_TIME)*/
pub struct ABSOLUTE_TIMErs;
impl crate::RegisterSpec for ABSOLUTE_TIMErs {
    type Ux = u32;
}
///`read()` method returns [`absolute_time::R`](R) reader structure
impl crate::Readable for ABSOLUTE_TIMErs {}
///`reset()` method sets ABSOLUTE_TIME to value 0
impl crate::Resettable for ABSOLUTE_TIMErs {}

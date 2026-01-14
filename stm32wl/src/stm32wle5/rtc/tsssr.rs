///Register `TSSSR` reader
pub type R = crate::R<TSSSRrs>;
///Field `SS` reader - Sub second value
pub type SS_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Sub second value
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSSSR").field("ss", &self.ss()).finish()
    }
}
/**Timestamp sub second register

You can [`read`](crate::Reg::read) this register and get [`tsssr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WLE5.html#RTC:TSSSR)*/
pub struct TSSSRrs;
impl crate::RegisterSpec for TSSSRrs {
    type Ux = u32;
}
///`read()` method returns [`tsssr::R`](R) reader structure
impl crate::Readable for TSSSRrs {}
///`reset()` method sets TSSSR to value 0
impl crate::Resettable for TSSSRrs {}

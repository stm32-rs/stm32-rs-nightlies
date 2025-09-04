///Register `TSSSR` reader
pub type R = crate::R<TSSSRrs>;
///Field `SS` reader - Sub second value
pub type SS_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Sub second value
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSSSR").field("ss", &self.ss()).finish()
    }
}
/**time-stamp sub second register

You can [`read`](crate::Reg::read) this register and get [`tsssr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F0x8.html#RTC:TSSSR)*/
pub struct TSSSRrs;
impl crate::RegisterSpec for TSSSRrs {
    type Ux = u32;
}
///`read()` method returns [`tsssr::R`](R) reader structure
impl crate::Readable for TSSSRrs {}
///`reset()` method sets TSSSR to value 0
impl crate::Resettable for TSSSRrs {}

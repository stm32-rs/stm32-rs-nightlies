///Register `TIME_CAPTURE` reader
pub type R = crate::R<TIME_CAPTURErs>;
///Field `TIME_CAPTURE` reader - Interpolated absolute time value captured on specific programmable event through TIME_CAPTURESEL\[2:0\] bit field.
pub type TIME_CAPTURE_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Interpolated absolute time value captured on specific programmable event through TIME_CAPTURESEL\[2:0\] bit field.
    #[inline(always)]
    pub fn time_capture(&self) -> TIME_CAPTURE_R {
        TIME_CAPTURE_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIME_CAPTURE")
            .field("time_capture", &self.time_capture())
            .finish()
    }
}
/**TIME_CAPTURE register

You can [`read`](crate::Reg::read) this register and get [`time_capture::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATUS:TIME_CAPTURE)*/
pub struct TIME_CAPTURErs;
impl crate::RegisterSpec for TIME_CAPTURErs {
    type Ux = u32;
}
///`read()` method returns [`time_capture::R`](R) reader structure
impl crate::Readable for TIME_CAPTURErs {}
///`reset()` method sets TIME_CAPTURE to value 0
impl crate::Resettable for TIME_CAPTURErs {}

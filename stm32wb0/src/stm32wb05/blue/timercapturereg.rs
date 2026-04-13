///Register `TIMERCAPTUREREG` reader
pub type R = crate::R<TIMERCAPTUREREGrs>;
///Field `TIMERCAPTURE` reader - Interpolated absolute time capture register
pub type TIMERCAPTURE_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Interpolated absolute time capture register
    #[inline(always)]
    pub fn timercapture(&self) -> TIMERCAPTURE_R {
        TIMERCAPTURE_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMERCAPTUREREG")
            .field("timercapture", &self.timercapture())
            .finish()
    }
}
/**TIMERCAPTUREREG register

You can [`read`](crate::Reg::read) this register and get [`timercapturereg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#BLUE:TIMERCAPTUREREG)*/
pub struct TIMERCAPTUREREGrs;
impl crate::RegisterSpec for TIMERCAPTUREREGrs {
    type Ux = u32;
}
///`read()` method returns [`timercapturereg::R`](R) reader structure
impl crate::Readable for TIMERCAPTUREREGrs {}
///`reset()` method sets TIMERCAPTUREREG to value 0
impl crate::Resettable for TIMERCAPTUREREGrs {}

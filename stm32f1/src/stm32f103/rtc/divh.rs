///Register `DIVH` reader
pub type R = crate::R<DIVHrs>;
///Field `DIVH` reader - RTC prescaler divider register high
pub type DIVH_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - RTC prescaler divider register high
    #[inline(always)]
    pub fn divh(&self) -> DIVH_R {
        DIVH_R::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIVH").field("divh", &self.divh()).finish()
    }
}
/**RTC Prescaler Divider Register High

You can [`read`](crate::Reg::read) this register and get [`divh::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F103.html#RTC:DIVH)*/
pub struct DIVHrs;
impl crate::RegisterSpec for DIVHrs {
    type Ux = u32;
}
///`read()` method returns [`divh::R`](R) reader structure
impl crate::Readable for DIVHrs {}
///`reset()` method sets DIVH to value 0
impl crate::Resettable for DIVHrs {}

///Register `RFIP_WAKEUPTIME` reader
pub type R = crate::R<RFIP_WAKEUPTIMErs>;
///Field `RFIP_WAKEUPTIME` reader - (Absolute) Target time to wakeup the RFIP.
pub type RFIP_WAKEUPTIME_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - (Absolute) Target time to wakeup the RFIP.
    #[inline(always)]
    pub fn rfip_wakeuptime(&self) -> RFIP_WAKEUPTIME_R {
        RFIP_WAKEUPTIME_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RFIP_WAKEUPTIME")
            .field("rfip_wakeuptime", &self.rfip_wakeuptime())
            .finish()
    }
}
/**RFIP_WAKEUPTIME register

You can [`read`](crate::Reg::read) this register and get [`rfip_wakeuptime::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#RETAINED:RFIP_WAKEUPTIME)*/
pub struct RFIP_WAKEUPTIMErs;
impl crate::RegisterSpec for RFIP_WAKEUPTIMErs {
    type Ux = u32;
}
///`read()` method returns [`rfip_wakeuptime::R`](R) reader structure
impl crate::Readable for RFIP_WAKEUPTIMErs {}
///`reset()` method sets RFIP_WAKEUPTIME to value 0
impl crate::Resettable for RFIP_WAKEUPTIMErs {}

///Register `PTPTSHR` reader
pub type R = crate::R<PTPTSHRrs>;
///Field `STS` reader - STS
pub type STS_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - STS
    #[inline(always)]
    pub fn sts(&self) -> STS_R {
        STS_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PTPTSHR").field("sts", &self.sts()).finish()
    }
}
/**Ethernet PTP time stamp high register

You can [`read`](crate::Reg::read) this register and get [`ptptshr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F469.html#Ethernet_PTP:PTPTSHR)*/
pub struct PTPTSHRrs;
impl crate::RegisterSpec for PTPTSHRrs {
    type Ux = u32;
}
///`read()` method returns [`ptptshr::R`](R) reader structure
impl crate::Readable for PTPTSHRrs {}
///`reset()` method sets PTPTSHR to value 0
impl crate::Resettable for PTPTSHRrs {}

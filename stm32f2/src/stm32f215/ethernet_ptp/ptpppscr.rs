///Register `PTPPPSCR` reader
pub type R = crate::R<PTPPPSCRrs>;
///Field `PPSFREQ` reader - PPS frequency selection
pub type PPSFREQ_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - PPS frequency selection
    #[inline(always)]
    pub fn ppsfreq(&self) -> PPSFREQ_R {
        PPSFREQ_R::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PTPPPSCR")
            .field("ppsfreq", &self.ppsfreq())
            .finish()
    }
}
/**Ethernet PTP PPS control register

You can [`read`](crate::Reg::read) this register and get [`ptpppscr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F215.html#Ethernet_PTP:PTPPPSCR)*/
pub struct PTPPPSCRrs;
impl crate::RegisterSpec for PTPPPSCRrs {
    type Ux = u32;
}
///`read()` method returns [`ptpppscr::R`](R) reader structure
impl crate::Readable for PTPPPSCRrs {}
///`reset()` method sets PTPPPSCR to value 0
impl crate::Resettable for PTPPPSCRrs {}

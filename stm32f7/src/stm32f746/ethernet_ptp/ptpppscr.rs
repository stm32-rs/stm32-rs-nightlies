///Register `PTPPPSCR` reader
pub type R = crate::R<PTPPPSCRrs>;
///Field `TSSO` reader - TSSO
pub type TSSO_R = crate::BitReader;
///Field `TSTTR` reader - TSTTR
pub type TSTTR_R = crate::BitReader;
impl R {
    ///Bit 0 - TSSO
    #[inline(always)]
    pub fn tsso(&self) -> TSSO_R {
        TSSO_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TSTTR
    #[inline(always)]
    pub fn tsttr(&self) -> TSTTR_R {
        TSTTR_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PTPPPSCR")
            .field("tsso", &self.tsso())
            .field("tsttr", &self.tsttr())
            .finish()
    }
}
/**Ethernet PTP PPS control register

You can [`read`](crate::Reg::read) this register and get [`ptpppscr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F746.html#Ethernet_PTP:PTPPPSCR)*/
pub struct PTPPPSCRrs;
impl crate::RegisterSpec for PTPPPSCRrs {
    type Ux = u32;
}
///`read()` method returns [`ptpppscr::R`](R) reader structure
impl crate::Readable for PTPPPSCRrs {}
///`reset()` method sets PTPPPSCR to value 0
impl crate::Resettable for PTPPPSCRrs {}

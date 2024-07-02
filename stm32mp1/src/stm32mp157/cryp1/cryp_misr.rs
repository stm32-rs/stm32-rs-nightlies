///Register `CRYP_MISR` reader
pub type R = crate::R<CRYP_MISRrs>;
///Field `INMIS` reader - INMIS
pub type INMIS_R = crate::BitReader;
///Field `OUTMIS` reader - OUTMIS
pub type OUTMIS_R = crate::BitReader;
impl R {
    ///Bit 0 - INMIS
    #[inline(always)]
    pub fn inmis(&self) -> INMIS_R {
        INMIS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - OUTMIS
    #[inline(always)]
    pub fn outmis(&self) -> OUTMIS_R {
        OUTMIS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRYP_MISR")
            .field("inmis", &self.inmis())
            .field("outmis", &self.outmis())
            .finish()
    }
}
/**The CRYP_MISR register is the masked interrupt status register. It is a read-only register. When a read operation is performed, this register gives the current masked status of the corresponding interrupt, i.e. the interrupt information taking CRYP_IMSCR mask into account. Write operations have no effect.

You can [`read`](crate::Reg::read) this register and get [`cryp_misr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#CRYP1:CRYP_MISR)*/
pub struct CRYP_MISRrs;
impl crate::RegisterSpec for CRYP_MISRrs {
    type Ux = u32;
}
///`read()` method returns [`cryp_misr::R`](R) reader structure
impl crate::Readable for CRYP_MISRrs {}
///`reset()` method sets CRYP_MISR to value 0
impl crate::Resettable for CRYP_MISRrs {
    const RESET_VALUE: u32 = 0;
}

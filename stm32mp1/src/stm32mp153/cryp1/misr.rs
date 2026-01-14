///Register `MISR` reader
pub type R = crate::R<MISRrs>;
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
        f.debug_struct("MISR")
            .field("inmis", &self.inmis())
            .field("outmis", &self.outmis())
            .finish()
    }
}
/**The CRYP_MISR register is the masked interrupt status register. It is a read-only register. When a read operation is performed, this register gives the current masked status of the corresponding interrupt, i.e. the interrupt information taking CRYP_IMSCR mask into account. Write operations have no effect.

You can [`read`](crate::Reg::read) this register and get [`misr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#CRYP1:MISR)*/
pub struct MISRrs;
impl crate::RegisterSpec for MISRrs {
    type Ux = u32;
}
///`read()` method returns [`misr::R`](R) reader structure
impl crate::Readable for MISRrs {}
///`reset()` method sets MISR to value 0
impl crate::Resettable for MISRrs {}

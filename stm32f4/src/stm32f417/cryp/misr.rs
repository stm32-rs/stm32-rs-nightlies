///Register `MISR` reader
pub type R = crate::R<MISRrs>;
///Field `INMIS` reader - Input FIFO service masked interrupt status
pub type INMIS_R = crate::BitReader;
///Field `OUTMIS` reader - Output FIFO service masked interrupt status
pub type OUTMIS_R = crate::BitReader;
impl R {
    ///Bit 0 - Input FIFO service masked interrupt status
    #[inline(always)]
    pub fn inmis(&self) -> INMIS_R {
        INMIS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Output FIFO service masked interrupt status
    #[inline(always)]
    pub fn outmis(&self) -> OUTMIS_R {
        OUTMIS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISR")
            .field("outmis", &self.outmis())
            .field("inmis", &self.inmis())
            .finish()
    }
}
/**masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`misr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#CRYP:MISR)*/
pub struct MISRrs;
impl crate::RegisterSpec for MISRrs {
    type Ux = u32;
}
///`read()` method returns [`misr::R`](R) reader structure
impl crate::Readable for MISRrs {}
///`reset()` method sets MISR to value 0
impl crate::Resettable for MISRrs {}

///Register `MMCTGFSCCR` reader
pub type R = crate::R<MMCTGFSCCRrs>;
///Field `TGFSCC` reader - TGFSCC
pub type TGFSCC_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - TGFSCC
    #[inline(always)]
    pub fn tgfscc(&self) -> TGFSCC_R {
        TGFSCC_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMCTGFSCCR")
            .field("tgfscc", &self.tgfscc())
            .finish()
    }
}
/**Ethernet MMC transmitted good frames after a single collision counter

You can [`read`](crate::Reg::read) this register and get [`mmctgfsccr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#Ethernet_MMC:MMCTGFSCCR)*/
pub struct MMCTGFSCCRrs;
impl crate::RegisterSpec for MMCTGFSCCRrs {
    type Ux = u32;
}
///`read()` method returns [`mmctgfsccr::R`](R) reader structure
impl crate::Readable for MMCTGFSCCRrs {}
///`reset()` method sets MMCTGFSCCR to value 0
impl crate::Resettable for MMCTGFSCCRrs {}

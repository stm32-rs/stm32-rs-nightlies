///Register `MMCTGFCR` reader
pub type R = crate::R<MMCTGFCRrs>;
///Field `TGFC` reader - Transmitted good frames counter
pub type TGFC_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Transmitted good frames counter
    #[inline(always)]
    pub fn tgfc(&self) -> TGFC_R {
        TGFC_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMCTGFCR")
            .field("tgfc", &self.tgfc())
            .finish()
    }
}
/**Ethernet MMC transmitted good frames counter register

You can [`read`](crate::Reg::read) this register and get [`mmctgfcr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F215.html#Ethernet_MMC:MMCTGFCR)*/
pub struct MMCTGFCRrs;
impl crate::RegisterSpec for MMCTGFCRrs {
    type Ux = u32;
}
///`read()` method returns [`mmctgfcr::R`](R) reader structure
impl crate::Readable for MMCTGFCRrs {}
///`reset()` method sets MMCTGFCR to value 0
impl crate::Resettable for MMCTGFCRrs {}

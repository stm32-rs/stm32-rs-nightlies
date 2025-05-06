///Register `MMCTGFMSCCR` reader
pub type R = crate::R<MMCTGFMSCCRrs>;
///Field `TGFMSCC` reader - TGFMSCC
pub type TGFMSCC_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - TGFMSCC
    #[inline(always)]
    pub fn tgfmscc(&self) -> TGFMSCC_R {
        TGFMSCC_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMCTGFMSCCR")
            .field("tgfmscc", &self.tgfmscc())
            .finish()
    }
}
/**Ethernet MMC transmitted good frames after more than a single collision

You can [`read`](crate::Reg::read) this register and get [`mmctgfmsccr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F777.html#Ethernet_MMC:MMCTGFMSCCR)*/
pub struct MMCTGFMSCCRrs;
impl crate::RegisterSpec for MMCTGFMSCCRrs {
    type Ux = u32;
}
///`read()` method returns [`mmctgfmsccr::R`](R) reader structure
impl crate::Readable for MMCTGFMSCCRrs {}
///`reset()` method sets MMCTGFMSCCR to value 0
impl crate::Resettable for MMCTGFMSCCRrs {}

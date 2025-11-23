///Register `MMCRGUFCR` reader
pub type R = crate::R<MMCRGUFCRrs>;
///Field `RGUFC` reader - RGUFC
pub type RGUFC_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - RGUFC
    #[inline(always)]
    pub fn rgufc(&self) -> RGUFC_R {
        RGUFC_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMCRGUFCR")
            .field("rgufc", &self.rgufc())
            .finish()
    }
}
/**MMC received good unicast frames counter register

You can [`read`](crate::Reg::read) this register and get [`mmcrgufcr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F765.html#Ethernet_MMC:MMCRGUFCR)*/
pub struct MMCRGUFCRrs;
impl crate::RegisterSpec for MMCRGUFCRrs {
    type Ux = u32;
}
///`read()` method returns [`mmcrgufcr::R`](R) reader structure
impl crate::Readable for MMCRGUFCRrs {}
///`reset()` method sets MMCRGUFCR to value 0
impl crate::Resettable for MMCRGUFCRrs {}

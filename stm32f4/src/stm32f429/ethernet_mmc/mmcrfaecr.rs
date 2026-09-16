///Register `MMCRFAECR` reader
pub type R = crate::R<MMCRFAECRrs>;
///Field `RFAEC` reader - RFAEC
pub type RFAEC_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - RFAEC
    #[inline(always)]
    pub fn rfaec(&self) -> RFAEC_R {
        RFAEC_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMCRFAECR")
            .field("rfaec", &self.rfaec())
            .finish()
    }
}
/**Ethernet MMC received frames with alignment error counter register

You can [`read`](crate::Reg::read) this register and get [`mmcrfaecr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F429.html#Ethernet_MMC:MMCRFAECR)*/
pub struct MMCRFAECRrs;
impl crate::RegisterSpec for MMCRFAECRrs {
    type Ux = u32;
}
///`read()` method returns [`mmcrfaecr::R`](R) reader structure
impl crate::Readable for MMCRFAECRrs {}
///`reset()` method sets MMCRFAECR to value 0
impl crate::Resettable for MMCRFAECRrs {}

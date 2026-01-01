///Register `MMCRFCECR` reader
pub type R = crate::R<MMCRFCECRrs>;
///Field `RFCFC` reader - RFCFC
pub type RFCFC_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - RFCFC
    #[inline(always)]
    pub fn rfcfc(&self) -> RFCFC_R {
        RFCFC_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMCRFCECR")
            .field("rfcfc", &self.rfcfc())
            .finish()
    }
}
/**Ethernet MMC received frames with CRC error counter register

You can [`read`](crate::Reg::read) this register and get [`mmcrfcecr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F407.html#Ethernet_MMC:MMCRFCECR)*/
pub struct MMCRFCECRrs;
impl crate::RegisterSpec for MMCRFCECRrs {
    type Ux = u32;
}
///`read()` method returns [`mmcrfcecr::R`](R) reader structure
impl crate::Readable for MMCRFCECRrs {}
///`reset()` method sets MMCRFCECR to value 0
impl crate::Resettable for MMCRFCECRrs {}

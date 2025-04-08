///Register `DMACHTDR` reader
pub type R = crate::R<DMACHTDRrs>;
///Field `HTDAP` reader - Host transmit descriptor address pointer
pub type HTDAP_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Host transmit descriptor address pointer
    #[inline(always)]
    pub fn htdap(&self) -> HTDAP_R {
        HTDAP_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACHTDR")
            .field("htdap", &self.htdap())
            .finish()
    }
}
/**Ethernet DMA current host transmit descriptor register

You can [`read`](crate::Reg::read) this register and get [`dmachtdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F407.html#Ethernet_DMA:DMACHTDR)*/
pub struct DMACHTDRrs;
impl crate::RegisterSpec for DMACHTDRrs {
    type Ux = u32;
}
///`read()` method returns [`dmachtdr::R`](R) reader structure
impl crate::Readable for DMACHTDRrs {}
///`reset()` method sets DMACHTDR to value 0
impl crate::Resettable for DMACHTDRrs {}

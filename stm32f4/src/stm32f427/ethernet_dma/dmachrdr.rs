///Register `DMACHRDR` reader
pub type R = crate::R<DMACHRDRrs>;
///Field `HRDAP` reader - Host receive descriptor address pointer
pub type HRDAP_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Host receive descriptor address pointer
    #[inline(always)]
    pub fn hrdap(&self) -> HRDAP_R {
        HRDAP_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACHRDR")
            .field("hrdap", &self.hrdap())
            .finish()
    }
}
/**Ethernet DMA current host receive descriptor register

You can [`read`](crate::Reg::read) this register and get [`dmachrdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F427.html#Ethernet_DMA:DMACHRDR)*/
pub struct DMACHRDRrs;
impl crate::RegisterSpec for DMACHRDRrs {
    type Ux = u32;
}
///`read()` method returns [`dmachrdr::R`](R) reader structure
impl crate::Readable for DMACHRDRrs {}
///`reset()` method sets DMACHRDR to value 0
impl crate::Resettable for DMACHRDRrs {}

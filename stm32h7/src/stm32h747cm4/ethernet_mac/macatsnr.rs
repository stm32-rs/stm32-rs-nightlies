///Register `MACATSNR` reader
pub type R = crate::R<MACATSNRrs>;
///Field `AUXTSLO` reader - Auxiliary Timestamp
pub type AUXTSLO_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:30 - Auxiliary Timestamp
    #[inline(always)]
    pub fn auxtslo(&self) -> AUXTSLO_R {
        AUXTSLO_R::new(self.bits & 0x7fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACATSNR")
            .field("auxtslo", &self.auxtslo())
            .finish()
    }
}
/**Auxiliary timestamp nanoseconds register

You can [`read`](crate::Reg::read) this register and get [`macatsnr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#Ethernet_MAC:MACATSNR)*/
pub struct MACATSNRrs;
impl crate::RegisterSpec for MACATSNRrs {
    type Ux = u32;
}
///`read()` method returns [`macatsnr::R`](R) reader structure
impl crate::Readable for MACATSNRrs {}
///`reset()` method sets MACATSNR to value 0
impl crate::Resettable for MACATSNRrs {}

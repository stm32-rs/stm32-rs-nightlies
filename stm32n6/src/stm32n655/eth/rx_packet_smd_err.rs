///Register `RX_PACKET_SMD_ERR` reader
pub type R = crate::R<RX_PACKET_SMD_ERRrs>;
///Field `PSEC` reader - Rx Packet SMD Error Counter
pub type PSEC_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Rx Packet SMD Error Counter
    #[inline(always)]
    pub fn psec(&self) -> PSEC_R {
        PSEC_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_PACKET_SMD_ERR")
            .field("psec", &self.psec())
            .finish()
    }
}
/**MMC Rx packet SMD error register

You can [`read`](crate::Reg::read) this register and get [`rx_packet_smd_err::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:RX_PACKET_SMD_ERR)*/
pub struct RX_PACKET_SMD_ERRrs;
impl crate::RegisterSpec for RX_PACKET_SMD_ERRrs {
    type Ux = u32;
}
///`read()` method returns [`rx_packet_smd_err::R`](R) reader structure
impl crate::Readable for RX_PACKET_SMD_ERRrs {}
///`reset()` method sets RX_PACKET_SMD_ERR to value 0
impl crate::Resettable for RX_PACKET_SMD_ERRrs {}

///Register `RX_CRC_ERROR_PACKETS` reader
pub type R = crate::R<RX_CRC_ERROR_PACKETSrs>;
///Field `RXCRCERR` reader - Rx CRC Error Packets
pub type RXCRCERR_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Rx CRC Error Packets
    #[inline(always)]
    pub fn rxcrcerr(&self) -> RXCRCERR_R {
        RXCRCERR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_CRC_ERROR_PACKETS")
            .field("rxcrcerr", &self.rxcrcerr())
            .finish()
    }
}
/**Rx CRC error packets register

You can [`read`](crate::Reg::read) this register and get [`rx_crc_error_packets::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#ETH:RX_CRC_ERROR_PACKETS)*/
pub struct RX_CRC_ERROR_PACKETSrs;
impl crate::RegisterSpec for RX_CRC_ERROR_PACKETSrs {
    type Ux = u32;
}
///`read()` method returns [`rx_crc_error_packets::R`](R) reader structure
impl crate::Readable for RX_CRC_ERROR_PACKETSrs {}
///`reset()` method sets RX_CRC_ERROR_PACKETS to value 0
impl crate::Resettable for RX_CRC_ERROR_PACKETSrs {}

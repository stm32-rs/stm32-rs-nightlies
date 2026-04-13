///Register `RX_ALIGNMENT_ERROR_PACKETS` reader
pub type R = crate::R<RX_ALIGNMENT_ERROR_PACKETSrs>;
///Field `RXALGNERR` reader - Rx Alignment Error Packets
pub type RXALGNERR_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Rx Alignment Error Packets
    #[inline(always)]
    pub fn rxalgnerr(&self) -> RXALGNERR_R {
        RXALGNERR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_ALIGNMENT_ERROR_PACKETS")
            .field("rxalgnerr", &self.rxalgnerr())
            .finish()
    }
}
/**Rx alignment error packets register

You can [`read`](crate::Reg::read) this register and get [`rx_alignment_error_packets::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#Ethernet_MAC:RX_ALIGNMENT_ERROR_PACKETS)*/
pub struct RX_ALIGNMENT_ERROR_PACKETSrs;
impl crate::RegisterSpec for RX_ALIGNMENT_ERROR_PACKETSrs {
    type Ux = u32;
}
///`read()` method returns [`rx_alignment_error_packets::R`](R) reader structure
impl crate::Readable for RX_ALIGNMENT_ERROR_PACKETSrs {}
///`reset()` method sets RX_ALIGNMENT_ERROR_PACKETS to value 0
impl crate::Resettable for RX_ALIGNMENT_ERROR_PACKETSrs {}

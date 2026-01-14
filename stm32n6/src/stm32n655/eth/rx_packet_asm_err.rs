///Register `RX_PACKET_ASM_ERR` reader
pub type R = crate::R<RX_PACKET_ASM_ERRrs>;
///Field `PAEC` reader - Rx Packet Assembly Error Counter
pub type PAEC_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Rx Packet Assembly Error Counter
    #[inline(always)]
    pub fn paec(&self) -> PAEC_R {
        PAEC_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_PACKET_ASM_ERR")
            .field("paec", &self.paec())
            .finish()
    }
}
/**MMC Rx packet assembly error register

You can [`read`](crate::Reg::read) this register and get [`rx_packet_asm_err::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:RX_PACKET_ASM_ERR)*/
pub struct RX_PACKET_ASM_ERRrs;
impl crate::RegisterSpec for RX_PACKET_ASM_ERRrs {
    type Ux = u32;
}
///`read()` method returns [`rx_packet_asm_err::R`](R) reader structure
impl crate::Readable for RX_PACKET_ASM_ERRrs {}
///`reset()` method sets RX_PACKET_ASM_ERR to value 0
impl crate::Resettable for RX_PACKET_ASM_ERRrs {}

///Register `RX_PACKET_ASM_OKR` reader
pub type R = crate::R<RX_PACKET_ASM_OKRrs>;
///Field `PAOC` reader - Rx Packet Assembly OK Counter
pub type PAOC_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Rx Packet Assembly OK Counter
    #[inline(always)]
    pub fn paoc(&self) -> PAOC_R {
        PAOC_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_PACKET_ASM_OKR")
            .field("paoc", &self.paoc())
            .finish()
    }
}
/**MMC Rx packet assembly OK register

You can [`read`](crate::Reg::read) this register and get [`rx_packet_asm_okr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#ETH:RX_PACKET_ASM_OKR)*/
pub struct RX_PACKET_ASM_OKRrs;
impl crate::RegisterSpec for RX_PACKET_ASM_OKRrs {
    type Ux = u32;
}
///`read()` method returns [`rx_packet_asm_okr::R`](R) reader structure
impl crate::Readable for RX_PACKET_ASM_OKRrs {}
///`reset()` method sets RX_PACKET_ASM_OKR to value 0
impl crate::Resettable for RX_PACKET_ASM_OKRrs {}

///Register `TX_PACKET_COUNT_GOOD` reader
pub type R = crate::R<TX_PACKET_COUNT_GOODrs>;
///Field `TXPKTG` reader - Tx Packet Count Good
pub type TXPKTG_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Tx Packet Count Good
    #[inline(always)]
    pub fn txpktg(&self) -> TXPKTG_R {
        TXPKTG_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_PACKET_COUNT_GOOD")
            .field("txpktg", &self.txpktg())
            .finish()
    }
}
/**Tx packet count good register

You can [`read`](crate::Reg::read) this register and get [`tx_packet_count_good::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#Ethernet_MAC:TX_PACKET_COUNT_GOOD)*/
pub struct TX_PACKET_COUNT_GOODrs;
impl crate::RegisterSpec for TX_PACKET_COUNT_GOODrs {
    type Ux = u32;
}
///`read()` method returns [`tx_packet_count_good::R`](R) reader structure
impl crate::Readable for TX_PACKET_COUNT_GOODrs {}
///`reset()` method sets TX_PACKET_COUNT_GOOD to value 0
impl crate::Resettable for TX_PACKET_COUNT_GOODrs {}

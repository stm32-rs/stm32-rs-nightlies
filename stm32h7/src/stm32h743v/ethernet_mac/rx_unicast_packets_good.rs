///Register `RX_UNICAST_PACKETS_GOOD` reader
pub type R = crate::R<RX_UNICAST_PACKETS_GOODrs>;
///Field `RXUCASTG` reader - Rx Unicast Packets Good
pub type RXUCASTG_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Rx Unicast Packets Good
    #[inline(always)]
    pub fn rxucastg(&self) -> RXUCASTG_R {
        RXUCASTG_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_UNICAST_PACKETS_GOOD")
            .field("rxucastg", &self.rxucastg())
            .finish()
    }
}
/**Rx unicast packets good register

You can [`read`](crate::Reg::read) this register and get [`rx_unicast_packets_good::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743V.html#Ethernet_MAC:RX_UNICAST_PACKETS_GOOD)*/
pub struct RX_UNICAST_PACKETS_GOODrs;
impl crate::RegisterSpec for RX_UNICAST_PACKETS_GOODrs {
    type Ux = u32;
}
///`read()` method returns [`rx_unicast_packets_good::R`](R) reader structure
impl crate::Readable for RX_UNICAST_PACKETS_GOODrs {}
///`reset()` method sets RX_UNICAST_PACKETS_GOOD to value 0
impl crate::Resettable for RX_UNICAST_PACKETS_GOODrs {}

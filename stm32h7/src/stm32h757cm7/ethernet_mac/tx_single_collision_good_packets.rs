///Register `TX_SINGLE_COLLISION_GOOD_PACKETS` reader
pub type R = crate::R<TX_SINGLE_COLLISION_GOOD_PACKETSrs>;
///Field `TXSNGLCOLG` reader - Tx Single Collision Good Packets
pub type TXSNGLCOLG_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Tx Single Collision Good Packets
    #[inline(always)]
    pub fn txsnglcolg(&self) -> TXSNGLCOLG_R {
        TXSNGLCOLG_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_SINGLE_COLLISION_GOOD_PACKETS")
            .field("txsnglcolg", &self.txsnglcolg())
            .finish()
    }
}
/**Tx single collision good packets register

You can [`read`](crate::Reg::read) this register and get [`tx_single_collision_good_packets::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM7.html#Ethernet_MAC:TX_SINGLE_COLLISION_GOOD_PACKETS)*/
pub struct TX_SINGLE_COLLISION_GOOD_PACKETSrs;
impl crate::RegisterSpec for TX_SINGLE_COLLISION_GOOD_PACKETSrs {
    type Ux = u32;
}
///`read()` method returns [`tx_single_collision_good_packets::R`](R) reader structure
impl crate::Readable for TX_SINGLE_COLLISION_GOOD_PACKETSrs {}
///`reset()` method sets TX_SINGLE_COLLISION_GOOD_PACKETS to value 0
impl crate::Resettable for TX_SINGLE_COLLISION_GOOD_PACKETSrs {}

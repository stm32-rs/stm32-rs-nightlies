///Register `TX_MULTIPLE_COLLISION_GOOD_PACKETS` reader
pub type R = crate::R<TX_MULTIPLE_COLLISION_GOOD_PACKETSrs>;
///Field `TXMULTCOLG` reader - Tx Multiple Collision Good Packets
pub type TXMULTCOLG_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Tx Multiple Collision Good Packets
    #[inline(always)]
    pub fn txmultcolg(&self) -> TXMULTCOLG_R {
        TXMULTCOLG_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_MULTIPLE_COLLISION_GOOD_PACKETS")
            .field("txmultcolg", &self.txmultcolg())
            .finish()
    }
}
/**Tx multiple collision good packets register

You can [`read`](crate::Reg::read) this register and get [`tx_multiple_collision_good_packets::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM7.html#Ethernet_MAC:TX_MULTIPLE_COLLISION_GOOD_PACKETS)*/
pub struct TX_MULTIPLE_COLLISION_GOOD_PACKETSrs;
impl crate::RegisterSpec for TX_MULTIPLE_COLLISION_GOOD_PACKETSrs {
    type Ux = u32;
}
///`read()` method returns [`tx_multiple_collision_good_packets::R`](R) reader structure
impl crate::Readable for TX_MULTIPLE_COLLISION_GOOD_PACKETSrs {}
///`reset()` method sets TX_MULTIPLE_COLLISION_GOOD_PACKETS to value 0
impl crate::Resettable for TX_MULTIPLE_COLLISION_GOOD_PACKETSrs {}

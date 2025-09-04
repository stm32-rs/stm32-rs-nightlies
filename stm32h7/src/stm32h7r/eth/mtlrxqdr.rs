///Register `MTLRXQDR` reader
pub type R = crate::R<MTLRXQDRrs>;
///Field `RWCSTS` reader - MTL Rx Queue Write Controller Active Status When high, this bit indicates that the MTL Rx queue Write controller is active, and it is transferring a received packet to the Rx queue.
pub type RWCSTS_R = crate::BitReader;
///Field `RRCSTS` reader - MTL Rx Queue Read Controller State This field gives the state of the Rx queue Read controller:
pub type RRCSTS_R = crate::FieldReader;
///Field `RXQSTS` reader - MTL Rx Queue Fill-Level Status This field gives the status of the fill-level of the Rx queue:
pub type RXQSTS_R = crate::FieldReader;
///Field `PRXQ` reader - Number of Packets in Receive Queue This field indicates the current number of packets in the Rx queue. The theoretical maximum value for this field is 256Kbyte/16bytes = 16K Packets, that is, Max_Queue_Size/Min_Packet_Size.
pub type PRXQ_R = crate::FieldReader<u16>;
impl R {
    ///Bit 0 - MTL Rx Queue Write Controller Active Status When high, this bit indicates that the MTL Rx queue Write controller is active, and it is transferring a received packet to the Rx queue.
    #[inline(always)]
    pub fn rwcsts(&self) -> RWCSTS_R {
        RWCSTS_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - MTL Rx Queue Read Controller State This field gives the state of the Rx queue Read controller:
    #[inline(always)]
    pub fn rrcsts(&self) -> RRCSTS_R {
        RRCSTS_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bits 4:5 - MTL Rx Queue Fill-Level Status This field gives the status of the fill-level of the Rx queue:
    #[inline(always)]
    pub fn rxqsts(&self) -> RXQSTS_R {
        RXQSTS_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 16:29 - Number of Packets in Receive Queue This field indicates the current number of packets in the Rx queue. The theoretical maximum value for this field is 256Kbyte/16bytes = 16K Packets, that is, Max_Queue_Size/Min_Packet_Size.
    #[inline(always)]
    pub fn prxq(&self) -> PRXQ_R {
        PRXQ_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTLRXQDR")
            .field("rwcsts", &self.rwcsts())
            .field("rrcsts", &self.rrcsts())
            .field("rxqsts", &self.rxqsts())
            .field("prxq", &self.prxq())
            .finish()
    }
}
/**Rx queue debug register

You can [`read`](crate::Reg::read) this register and get [`mtlrxqdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#ETH:MTLRXQDR)*/
pub struct MTLRXQDRrs;
impl crate::RegisterSpec for MTLRXQDRrs {
    type Ux = u32;
}
///`read()` method returns [`mtlrxqdr::R`](R) reader structure
impl crate::Readable for MTLRXQDRrs {}
///`reset()` method sets MTLRXQDR to value 0
impl crate::Resettable for MTLRXQDRrs {}

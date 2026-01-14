///Register `MACDR` reader
pub type R = crate::R<MACDRrs>;
///Field `RPESTS` reader - MAC MII Receive Protocol Engine Status When this bit is set, it indicates that the MAC MII receive protocol engine is actively receiving data, and it is not in the Idle state.
pub type RPESTS_R = crate::BitReader;
///Field `RFCFCSTS` reader - MAC Receive Packet Controller FIFO Status When this bit is set, this field indicates the active state of the small FIFO Read and Write controllers of the MAC Receive Packet Controller module.
pub type RFCFCSTS_R = crate::FieldReader;
///Field `TPESTS` reader - MAC MII Transmit Protocol Engine Status When this bit is set, it indicates that the MAC MII transmit protocol engine is actively transmitting data, and it is not in the Idle state.
pub type TPESTS_R = crate::BitReader;
///Field `TFCSTS` reader - MAC Transmit Packet Controller Status This field indicates the state of the MAC Transmit Packet Controller module: Status of the previous packet IPG or backoff period to be over
pub type TFCSTS_R = crate::FieldReader;
impl R {
    ///Bit 0 - MAC MII Receive Protocol Engine Status When this bit is set, it indicates that the MAC MII receive protocol engine is actively receiving data, and it is not in the Idle state.
    #[inline(always)]
    pub fn rpests(&self) -> RPESTS_R {
        RPESTS_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - MAC Receive Packet Controller FIFO Status When this bit is set, this field indicates the active state of the small FIFO Read and Write controllers of the MAC Receive Packet Controller module.
    #[inline(always)]
    pub fn rfcfcsts(&self) -> RFCFCSTS_R {
        RFCFCSTS_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bit 16 - MAC MII Transmit Protocol Engine Status When this bit is set, it indicates that the MAC MII transmit protocol engine is actively transmitting data, and it is not in the Idle state.
    #[inline(always)]
    pub fn tpests(&self) -> TPESTS_R {
        TPESTS_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:18 - MAC Transmit Packet Controller Status This field indicates the state of the MAC Transmit Packet Controller module: Status of the previous packet IPG or backoff period to be over
    #[inline(always)]
    pub fn tfcsts(&self) -> TFCSTS_R {
        TFCSTS_R::new(((self.bits >> 17) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACDR")
            .field("rpests", &self.rpests())
            .field("rfcfcsts", &self.rfcfcsts())
            .field("tpests", &self.tpests())
            .field("tfcsts", &self.tfcsts())
            .finish()
    }
}
/**Debug register

You can [`read`](crate::Reg::read) this register and get [`macdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#ETH:MACDR)*/
pub struct MACDRrs;
impl crate::RegisterSpec for MACDRrs {
    type Ux = u32;
}
///`read()` method returns [`macdr::R`](R) reader structure
impl crate::Readable for MACDRrs {}
///`reset()` method sets MACDR to value 0
impl crate::Resettable for MACDRrs {}

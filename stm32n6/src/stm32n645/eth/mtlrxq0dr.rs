///Register `MTLRXQ0DR` reader
pub type R = crate::R<MTLRXQ0DRrs>;
///Field `RWCSTS` reader - MTL Rx Queue Write Controller Active Status
pub type RWCSTS_R = crate::BitReader;
///Field `RRCSTS` reader - MTL Rx Queue Read Controller State
pub type RRCSTS_R = crate::FieldReader;
///Field `RXQSTS` reader - MTL Rx Queue Fill-Level Status
pub type RXQSTS_R = crate::FieldReader;
///Field `PRXQ` reader - Number of Packets in Receive Queue
pub type PRXQ_R = crate::FieldReader<u16>;
impl R {
    ///Bit 0 - MTL Rx Queue Write Controller Active Status
    #[inline(always)]
    pub fn rwcsts(&self) -> RWCSTS_R {
        RWCSTS_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - MTL Rx Queue Read Controller State
    #[inline(always)]
    pub fn rrcsts(&self) -> RRCSTS_R {
        RRCSTS_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bits 4:5 - MTL Rx Queue Fill-Level Status
    #[inline(always)]
    pub fn rxqsts(&self) -> RXQSTS_R {
        RXQSTS_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 16:29 - Number of Packets in Receive Queue
    #[inline(always)]
    pub fn prxq(&self) -> PRXQ_R {
        PRXQ_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTLRXQ0DR")
            .field("rwcsts", &self.rwcsts())
            .field("rrcsts", &self.rrcsts())
            .field("rxqsts", &self.rxqsts())
            .field("prxq", &self.prxq())
            .finish()
    }
}
/**R0 queue 0 debug register

You can [`read`](crate::Reg::read) this register and get [`mtlrxq0dr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#ETH:MTLRXQ0DR)*/
pub struct MTLRXQ0DRrs;
impl crate::RegisterSpec for MTLRXQ0DRrs {
    type Ux = u32;
}
///`read()` method returns [`mtlrxq0dr::R`](R) reader structure
impl crate::Readable for MTLRXQ0DRrs {}
///`reset()` method sets MTLRXQ0DR to value 0
impl crate::Resettable for MTLRXQ0DRrs {}

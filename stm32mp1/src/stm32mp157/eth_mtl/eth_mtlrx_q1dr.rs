///Register `ETH_MTLRxQ1DR` reader
pub type R = crate::R<ETH_MTLRX_Q1DRrs>;
///Field `RWCSTS` reader - RWCSTS
pub type RWCSTS_R = crate::BitReader;
///Field `RRCSTS` reader - RRCSTS
pub type RRCSTS_R = crate::FieldReader;
///Field `RXQSTS` reader - RXQSTS
pub type RXQSTS_R = crate::FieldReader;
///Field `PRXQ` reader - PRXQ
pub type PRXQ_R = crate::FieldReader<u16>;
impl R {
    ///Bit 0 - RWCSTS
    #[inline(always)]
    pub fn rwcsts(&self) -> RWCSTS_R {
        RWCSTS_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - RRCSTS
    #[inline(always)]
    pub fn rrcsts(&self) -> RRCSTS_R {
        RRCSTS_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bits 4:5 - RXQSTS
    #[inline(always)]
    pub fn rxqsts(&self) -> RXQSTS_R {
        RXQSTS_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 16:29 - PRXQ
    #[inline(always)]
    pub fn prxq(&self) -> PRXQ_R {
        PRXQ_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETH_MTLRxQ1DR")
            .field("rwcsts", &self.rwcsts())
            .field("rrcsts", &self.rrcsts())
            .field("rxqsts", &self.rxqsts())
            .field("prxq", &self.prxq())
            .finish()
    }
}
/**Rx queue i debug register

You can [`read`](crate::Reg::read) this register and get [`eth_mtlrx_q1dr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MTL:ETH_MTLRxQ1DR)*/
pub struct ETH_MTLRX_Q1DRrs;
impl crate::RegisterSpec for ETH_MTLRX_Q1DRrs {
    type Ux = u32;
}
///`read()` method returns [`eth_mtlrx_q1dr::R`](R) reader structure
impl crate::Readable for ETH_MTLRX_Q1DRrs {}
///`reset()` method sets ETH_MTLRxQ1DR to value 0
impl crate::Resettable for ETH_MTLRX_Q1DRrs {
    const RESET_VALUE: u32 = 0;
}

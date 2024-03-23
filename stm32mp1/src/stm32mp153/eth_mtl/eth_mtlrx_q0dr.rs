#[doc = "Register `ETH_MTLRxQ0DR` reader"]
pub type R = crate::R<ETH_MTLRX_Q0DRrs>;
#[doc = "Field `RWCSTS` reader - RWCSTS"]
pub type RWCSTS_R = crate::BitReader;
#[doc = "Field `RRCSTS` reader - RRCSTS"]
pub type RRCSTS_R = crate::FieldReader;
#[doc = "Field `RXQSTS` reader - RXQSTS"]
pub type RXQSTS_R = crate::FieldReader;
#[doc = "Field `PRXQ` reader - PRXQ"]
pub type PRXQ_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - RWCSTS"]
    #[inline(always)]
    pub fn rwcsts(&self) -> RWCSTS_R {
        RWCSTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - RRCSTS"]
    #[inline(always)]
    pub fn rrcsts(&self) -> RRCSTS_R {
        RRCSTS_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 4:5 - RXQSTS"]
    #[inline(always)]
    pub fn rxqsts(&self) -> RXQSTS_R {
        RXQSTS_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 16:29 - PRXQ"]
    #[inline(always)]
    pub fn prxq(&self) -> PRXQ_R {
        PRXQ_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
#[doc = "Rx queue i debug register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mtlrx_q0dr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MTLRX_Q0DRrs;
impl crate::RegisterSpec for ETH_MTLRX_Q0DRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_mtlrx_q0dr::R`](R) reader structure"]
impl crate::Readable for ETH_MTLRX_Q0DRrs {}
#[doc = "`reset()` method sets ETH_MTLRxQ0DR to value 0"]
impl crate::Resettable for ETH_MTLRX_Q0DRrs {
    const RESET_VALUE: u32 = 0;
}

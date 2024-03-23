#[doc = "Register `MTLRxQDR` reader"]
pub type R = crate::R<MTLRX_QDRrs>;
#[doc = "Field `RWCSTS` reader - MTL Rx Queue Write Controller Active Status"]
pub type RWCSTS_R = crate::BitReader;
#[doc = "Field `RRCSTS` reader - MTL Rx Queue Read Controller State"]
pub type RRCSTS_R = crate::FieldReader;
#[doc = "Field `RXQSTS` reader - MTL Rx Queue Fill-Level Status"]
pub type RXQSTS_R = crate::FieldReader;
#[doc = "Field `PRXQ` reader - Number of Packets in Receive Queue"]
pub type PRXQ_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - MTL Rx Queue Write Controller Active Status"]
    #[inline(always)]
    pub fn rwcsts(&self) -> RWCSTS_R {
        RWCSTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - MTL Rx Queue Read Controller State"]
    #[inline(always)]
    pub fn rrcsts(&self) -> RRCSTS_R {
        RRCSTS_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 4:5 - MTL Rx Queue Fill-Level Status"]
    #[inline(always)]
    pub fn rxqsts(&self) -> RXQSTS_R {
        RXQSTS_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 16:29 - Number of Packets in Receive Queue"]
    #[inline(always)]
    pub fn prxq(&self) -> PRXQ_R {
        PRXQ_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
#[doc = "Rx queue debug register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtlrx_qdr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MTLRX_QDRrs;
impl crate::RegisterSpec for MTLRX_QDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtlrx_qdr::R`](R) reader structure"]
impl crate::Readable for MTLRX_QDRrs {}
#[doc = "`reset()` method sets MTLRxQDR to value 0"]
impl crate::Resettable for MTLRX_QDRrs {
    const RESET_VALUE: u32 = 0;
}

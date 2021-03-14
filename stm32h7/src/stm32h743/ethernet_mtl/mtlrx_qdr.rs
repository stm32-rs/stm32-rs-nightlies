#[doc = "Reader of register MTLRxQDR"]
pub type R = crate::R<u32, super::MTLRXQDR>;
#[doc = "Reader of field `RWCSTS`"]
pub type RWCSTS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RRCSTS`"]
pub type RRCSTS_R = crate::R<u8, u8>;
#[doc = "Reader of field `RXQSTS`"]
pub type RXQSTS_R = crate::R<u8, u8>;
#[doc = "Reader of field `PRXQ`"]
pub type PRXQ_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bit 0 - MTL Rx Queue Write Controller Active Status"]
    #[inline(always)]
    pub fn rwcsts(&self) -> RWCSTS_R {
        RWCSTS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - MTL Rx Queue Read Controller State"]
    #[inline(always)]
    pub fn rrcsts(&self) -> RRCSTS_R {
        RRCSTS_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - MTL Rx Queue Fill-Level Status"]
    #[inline(always)]
    pub fn rxqsts(&self) -> RXQSTS_R {
        RXQSTS_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 16:29 - Number of Packets in Receive Queue"]
    #[inline(always)]
    pub fn prxq(&self) -> PRXQ_R {
        PRXQ_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}

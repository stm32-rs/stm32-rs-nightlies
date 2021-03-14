#[doc = "Reader of register ETH_MTLRxQ0CR"]
pub type R = crate::R<u32, super::ETH_MTLRXQ0CR>;
#[doc = "Writer for register ETH_MTLRxQ0CR"]
pub type W = crate::W<u32, super::ETH_MTLRXQ0CR>;
#[doc = "Register ETH_MTLRxQ0CR `reset()`'s with value 0"]
impl crate::ResetValue for super::ETH_MTLRXQ0CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RXQ_WEGT`"]
pub type RXQ_WEGT_R = crate::R<u8, u8>;
#[doc = "Reader of field `RXQ_FRM_ARBIT`"]
pub type RXQ_FRM_ARBIT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:2 - RXQ_WEGT"]
    #[inline(always)]
    pub fn rxq_wegt(&self) -> RXQ_WEGT_R {
        RXQ_WEGT_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - RXQ_FRM_ARBIT"]
    #[inline(always)]
    pub fn rxq_frm_arbit(&self) -> RXQ_FRM_ARBIT_R {
        RXQ_FRM_ARBIT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {}

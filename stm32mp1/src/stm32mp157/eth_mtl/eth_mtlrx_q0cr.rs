#[doc = "Register `ETH_MTLRxQ0CR` reader"]
pub type R = crate::R<ETH_MTLRX_Q0CRrs>;
#[doc = "Register `ETH_MTLRxQ0CR` writer"]
pub type W = crate::W<ETH_MTLRX_Q0CRrs>;
#[doc = "Field `RXQ_WEGT` reader - RXQ_WEGT"]
pub type RXQ_WEGT_R = crate::FieldReader;
#[doc = "Field `RXQ_FRM_ARBIT` reader - RXQ_FRM_ARBIT"]
pub type RXQ_FRM_ARBIT_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - RXQ_WEGT"]
    #[inline(always)]
    pub fn rxq_wegt(&self) -> RXQ_WEGT_R {
        RXQ_WEGT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - RXQ_FRM_ARBIT"]
    #[inline(always)]
    pub fn rxq_frm_arbit(&self) -> RXQ_FRM_ARBIT_R {
        RXQ_FRM_ARBIT_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {}
#[doc = "Rx queue 0 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mtlrx_q0cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_mtlrx_q0cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MTLRX_Q0CRrs;
impl crate::RegisterSpec for ETH_MTLRX_Q0CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_mtlrx_q0cr::R`](R) reader structure"]
impl crate::Readable for ETH_MTLRX_Q0CRrs {}
#[doc = "`write(|w| ..)` method takes [`eth_mtlrx_q0cr::W`](W) writer structure"]
impl crate::Writable for ETH_MTLRX_Q0CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_MTLRxQ0CR to value 0"]
impl crate::Resettable for ETH_MTLRX_Q0CRrs {
    const RESET_VALUE: u32 = 0;
}

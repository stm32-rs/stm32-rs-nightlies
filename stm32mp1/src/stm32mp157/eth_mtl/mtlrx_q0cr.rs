///Register `MTLRxQ0CR` reader
pub type R = crate::R<MTLRX_Q0CRrs>;
///Register `MTLRxQ0CR` writer
pub type W = crate::W<MTLRX_Q0CRrs>;
///Field `RXQ_WEGT` reader - RXQ_WEGT
pub type RXQ_WEGT_R = crate::FieldReader;
///Field `RXQ_FRM_ARBIT` reader - RXQ_FRM_ARBIT
pub type RXQ_FRM_ARBIT_R = crate::BitReader;
impl R {
    ///Bits 0:2 - RXQ_WEGT
    #[inline(always)]
    pub fn rxq_wegt(&self) -> RXQ_WEGT_R {
        RXQ_WEGT_R::new((self.bits & 7) as u8)
    }
    ///Bit 3 - RXQ_FRM_ARBIT
    #[inline(always)]
    pub fn rxq_frm_arbit(&self) -> RXQ_FRM_ARBIT_R {
        RXQ_FRM_ARBIT_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTLRxQ0CR")
            .field("rxq_wegt", &self.rxq_wegt())
            .field("rxq_frm_arbit", &self.rxq_frm_arbit())
            .finish()
    }
}
impl W {}
/**Rx queue 0 control register

You can [`read`](crate::Reg::read) this register and get [`mtlrx_q0cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlrx_q0cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MTL:MTLRxQ0CR)*/
pub struct MTLRX_Q0CRrs;
impl crate::RegisterSpec for MTLRX_Q0CRrs {
    type Ux = u32;
}
///`read()` method returns [`mtlrx_q0cr::R`](R) reader structure
impl crate::Readable for MTLRX_Q0CRrs {}
///`write(|w| ..)` method takes [`mtlrx_q0cr::W`](W) writer structure
impl crate::Writable for MTLRX_Q0CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MTLRxQ0CR to value 0
impl crate::Resettable for MTLRX_Q0CRrs {}

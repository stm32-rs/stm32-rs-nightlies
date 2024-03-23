#[doc = "Register `CEC_TXDR` writer"]
pub type W = crate::W<CEC_TXDRrs>;
#[doc = "Field `TXD` writer - Tx Data register"]
pub type TXD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Tx Data register"]
    #[inline(always)]
    #[must_use]
    pub fn txd(&mut self) -> TXD_W<CEC_TXDRrs> {
        TXD_W::new(self, 0)
    }
}
#[doc = "CEC Tx data register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cec_txdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CEC_TXDRrs;
impl crate::RegisterSpec for CEC_TXDRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cec_txdr::W`](W) writer structure"]
impl crate::Writable for CEC_TXDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CEC_TXDR to value 0"]
impl crate::Resettable for CEC_TXDRrs {
    const RESET_VALUE: u32 = 0;
}

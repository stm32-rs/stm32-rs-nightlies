#[doc = "Register `ETH_MTLTxQ1LCR` reader"]
pub type R = crate::R<ETH_MTLTX_Q1LCRrs>;
#[doc = "Register `ETH_MTLTxQ1LCR` writer"]
pub type W = crate::W<ETH_MTLTX_Q1LCRrs>;
#[doc = "Field `LC` reader - LC"]
pub type LC_R = crate::FieldReader<u32>;
#[doc = "Field `LC` writer - LC"]
pub type LC_W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:28 - LC"]
    #[inline(always)]
    pub fn lc(&self) -> LC_R {
        LC_R::new(self.bits & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:28 - LC"]
    #[inline(always)]
    #[must_use]
    pub fn lc(&mut self) -> LC_W<ETH_MTLTX_Q1LCRrs> {
        LC_W::new(self, 0)
    }
}
#[doc = "The loCredit register contains the loCredit value required for the credit-based shaper algorithm for the Queue.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mtltx_q1lcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_mtltx_q1lcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MTLTX_Q1LCRrs;
impl crate::RegisterSpec for ETH_MTLTX_Q1LCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_mtltx_q1lcr::R`](R) reader structure"]
impl crate::Readable for ETH_MTLTX_Q1LCRrs {}
#[doc = "`write(|w| ..)` method takes [`eth_mtltx_q1lcr::W`](W) writer structure"]
impl crate::Writable for ETH_MTLTX_Q1LCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_MTLTxQ1LCR to value 0"]
impl crate::Resettable for ETH_MTLTX_Q1LCRrs {
    const RESET_VALUE: u32 = 0;
}

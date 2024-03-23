#[doc = "Register `ETH_MTLTxQ1HCR` reader"]
pub type R = crate::R<ETH_MTLTX_Q1HCRrs>;
#[doc = "Register `ETH_MTLTxQ1HCR` writer"]
pub type W = crate::W<ETH_MTLTX_Q1HCRrs>;
#[doc = "Field `HC` reader - HC"]
pub type HC_R = crate::FieldReader<u32>;
#[doc = "Field `HC` writer - HC"]
pub type HC_W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:28 - HC"]
    #[inline(always)]
    pub fn hc(&self) -> HC_R {
        HC_R::new(self.bits & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:28 - HC"]
    #[inline(always)]
    #[must_use]
    pub fn hc(&mut self) -> HC_W<ETH_MTLTX_Q1HCRrs> {
        HC_W::new(self, 0)
    }
}
#[doc = "The hiCredit register contains the hiCredit value required for the credit-based shaper algorithm for the Queue.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mtltx_q1hcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_mtltx_q1hcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MTLTX_Q1HCRrs;
impl crate::RegisterSpec for ETH_MTLTX_Q1HCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_mtltx_q1hcr::R`](R) reader structure"]
impl crate::Readable for ETH_MTLTX_Q1HCRrs {}
#[doc = "`write(|w| ..)` method takes [`eth_mtltx_q1hcr::W`](W) writer structure"]
impl crate::Writable for ETH_MTLTX_Q1HCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_MTLTxQ1HCR to value 0"]
impl crate::Resettable for ETH_MTLTX_Q1HCRrs {
    const RESET_VALUE: u32 = 0;
}

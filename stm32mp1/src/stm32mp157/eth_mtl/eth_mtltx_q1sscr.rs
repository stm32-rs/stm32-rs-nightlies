#[doc = "Register `ETH_MTLTxQ1SSCR` reader"]
pub type R = crate::R<ETH_MTLTX_Q1SSCRrs>;
#[doc = "Register `ETH_MTLTxQ1SSCR` writer"]
pub type W = crate::W<ETH_MTLTX_Q1SSCRrs>;
#[doc = "Field `SSC` reader - SSC"]
pub type SSC_R = crate::FieldReader<u16>;
#[doc = "Field `SSC` writer - SSC"]
pub type SSC_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - SSC"]
    #[inline(always)]
    pub fn ssc(&self) -> SSC_R {
        SSC_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - SSC"]
    #[inline(always)]
    #[must_use]
    pub fn ssc(&mut self) -> SSC_W<ETH_MTLTX_Q1SSCRrs> {
        SSC_W::new(self, 0)
    }
}
#[doc = "The sendSlopeCredit register contains the sendSlope credit value required for the credit-based shaper algorithm for the Queue.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mtltx_q1sscr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_mtltx_q1sscr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MTLTX_Q1SSCRrs;
impl crate::RegisterSpec for ETH_MTLTX_Q1SSCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_mtltx_q1sscr::R`](R) reader structure"]
impl crate::Readable for ETH_MTLTX_Q1SSCRrs {}
#[doc = "`write(|w| ..)` method takes [`eth_mtltx_q1sscr::W`](W) writer structure"]
impl crate::Writable for ETH_MTLTX_Q1SSCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_MTLTxQ1SSCR to value 0"]
impl crate::Resettable for ETH_MTLTX_Q1SSCRrs {
    const RESET_VALUE: u32 = 0;
}

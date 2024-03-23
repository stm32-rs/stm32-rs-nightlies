#[doc = "Register `ETH_MACTSECNR` reader"]
pub type R = crate::R<ETH_MACTSECNRrs>;
#[doc = "Register `ETH_MACTSECNR` writer"]
pub type W = crate::W<ETH_MACTSECNRrs>;
#[doc = "Field `TSEC` reader - TSEC"]
pub type TSEC_R = crate::FieldReader<u32>;
#[doc = "Field `TSEC` writer - TSEC"]
pub type TSEC_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TSEC"]
    #[inline(always)]
    pub fn tsec(&self) -> TSEC_R {
        TSEC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TSEC"]
    #[inline(always)]
    #[must_use]
    pub fn tsec(&mut self) -> TSEC_W<ETH_MACTSECNRrs> {
        TSEC_W::new(self, 0)
    }
}
#[doc = "This register contains the correction value in nanoseconds to be used with the captured timestamp value in the egress path.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mactsecnr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_mactsecnr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MACTSECNRrs;
impl crate::RegisterSpec for ETH_MACTSECNRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_mactsecnr::R`](R) reader structure"]
impl crate::Readable for ETH_MACTSECNRrs {}
#[doc = "`write(|w| ..)` method takes [`eth_mactsecnr::W`](W) writer structure"]
impl crate::Writable for ETH_MACTSECNRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_MACTSECNR to value 0"]
impl crate::Resettable for ETH_MACTSECNRrs {
    const RESET_VALUE: u32 = 0;
}

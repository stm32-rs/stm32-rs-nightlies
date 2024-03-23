#[doc = "Register `ETH_MACTSIACR` reader"]
pub type R = crate::R<ETH_MACTSIACRrs>;
#[doc = "Register `ETH_MACTSIACR` writer"]
pub type W = crate::W<ETH_MACTSIACRrs>;
#[doc = "Field `OSTIAC` reader - OSTIAC"]
pub type OSTIAC_R = crate::FieldReader<u32>;
#[doc = "Field `OSTIAC` writer - OSTIAC"]
pub type OSTIAC_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - OSTIAC"]
    #[inline(always)]
    pub fn ostiac(&self) -> OSTIAC_R {
        OSTIAC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - OSTIAC"]
    #[inline(always)]
    #[must_use]
    pub fn ostiac(&mut self) -> OSTIAC_W<ETH_MACTSIACRrs> {
        OSTIAC_W::new(self, 0)
    }
}
#[doc = "The MAC Timestamp Ingress Asymmetry Correction register contains the Ingress Asymmetry Correction value to be used while updating correction field in PDelay_Resp PTP messages.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mactsiacr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_mactsiacr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MACTSIACRrs;
impl crate::RegisterSpec for ETH_MACTSIACRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_mactsiacr::R`](R) reader structure"]
impl crate::Readable for ETH_MACTSIACRrs {}
#[doc = "`write(|w| ..)` method takes [`eth_mactsiacr::W`](W) writer structure"]
impl crate::Writable for ETH_MACTSIACRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_MACTSIACR to value 0"]
impl crate::Resettable for ETH_MACTSIACRrs {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `DMACTxRLR` reader"]
pub type R = crate::R<DMACTX_RLRrs>;
#[doc = "Register `DMACTxRLR` writer"]
pub type W = crate::W<DMACTX_RLRrs>;
#[doc = "Field `TDRL` reader - Transmit Descriptor Ring Length"]
pub type TDRL_R = crate::FieldReader<u16>;
#[doc = "Field `TDRL` writer - Transmit Descriptor Ring Length"]
pub type TDRL_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Transmit Descriptor Ring Length"]
    #[inline(always)]
    pub fn tdrl(&self) -> TDRL_R {
        TDRL_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Transmit Descriptor Ring Length"]
    #[inline(always)]
    #[must_use]
    pub fn tdrl(&mut self) -> TDRL_W<DMACTX_RLRrs> {
        TDRL_W::new(self, 0)
    }
}
#[doc = "Channel Tx descriptor ring length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmactx_rlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmactx_rlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACTX_RLRrs;
impl crate::RegisterSpec for DMACTX_RLRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmactx_rlr::R`](R) reader structure"]
impl crate::Readable for DMACTX_RLRrs {}
#[doc = "`write(|w| ..)` method takes [`dmactx_rlr::W`](W) writer structure"]
impl crate::Writable for DMACTX_RLRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACTxRLR to value 0"]
impl crate::Resettable for DMACTX_RLRrs {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `ETH_DMAC1TxRLR` reader"]
pub type R = crate::R<ETH_DMAC1TX_RLRrs>;
#[doc = "Register `ETH_DMAC1TxRLR` writer"]
pub type W = crate::W<ETH_DMAC1TX_RLRrs>;
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
    pub fn tdrl(&mut self) -> TDRL_W<ETH_DMAC1TX_RLRrs> {
        TDRL_W::new(self, 0)
    }
}
#[doc = "Channel Tx descriptor ring length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_dmac1tx_rlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_dmac1tx_rlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_DMAC1TX_RLRrs;
impl crate::RegisterSpec for ETH_DMAC1TX_RLRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_dmac1tx_rlr::R`](R) reader structure"]
impl crate::Readable for ETH_DMAC1TX_RLRrs {}
#[doc = "`write(|w| ..)` method takes [`eth_dmac1tx_rlr::W`](W) writer structure"]
impl crate::Writable for ETH_DMAC1TX_RLRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_DMAC1TxRLR to value 0"]
impl crate::Resettable for ETH_DMAC1TX_RLRrs {
    const RESET_VALUE: u32 = 0;
}

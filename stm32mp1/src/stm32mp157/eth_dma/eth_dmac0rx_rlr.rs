#[doc = "Register `ETH_DMAC0RxRLR` reader"]
pub type R = crate::R<ETH_DMAC0RX_RLRrs>;
#[doc = "Register `ETH_DMAC0RxRLR` writer"]
pub type W = crate::W<ETH_DMAC0RX_RLRrs>;
#[doc = "Field `RDRL` reader - Receive Descriptor Ring Length"]
pub type RDRL_R = crate::FieldReader<u16>;
#[doc = "Field `RDRL` writer - Receive Descriptor Ring Length"]
pub type RDRL_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Receive Descriptor Ring Length"]
    #[inline(always)]
    pub fn rdrl(&self) -> RDRL_R {
        RDRL_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Receive Descriptor Ring Length"]
    #[inline(always)]
    #[must_use]
    pub fn rdrl(&mut self) -> RDRL_W<ETH_DMAC0RX_RLRrs> {
        RDRL_W::new(self, 0)
    }
}
#[doc = "Channel Rx descriptor ring length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_dmac0rx_rlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_dmac0rx_rlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_DMAC0RX_RLRrs;
impl crate::RegisterSpec for ETH_DMAC0RX_RLRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_dmac0rx_rlr::R`](R) reader structure"]
impl crate::Readable for ETH_DMAC0RX_RLRrs {}
#[doc = "`write(|w| ..)` method takes [`eth_dmac0rx_rlr::W`](W) writer structure"]
impl crate::Writable for ETH_DMAC0RX_RLRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_DMAC0RxRLR to value 0x8000"]
impl crate::Resettable for ETH_DMAC0RX_RLRrs {
    const RESET_VALUE: u32 = 0x8000;
}

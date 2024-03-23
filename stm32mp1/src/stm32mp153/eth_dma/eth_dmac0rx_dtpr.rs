#[doc = "Register `ETH_DMAC0RxDTPR` reader"]
pub type R = crate::R<ETH_DMAC0RX_DTPRrs>;
#[doc = "Register `ETH_DMAC0RxDTPR` writer"]
pub type W = crate::W<ETH_DMAC0RX_DTPRrs>;
#[doc = "Field `RDT` reader - Receive Descriptor Tail Pointer"]
pub type RDT_R = crate::FieldReader<u32>;
#[doc = "Field `RDT` writer - Receive Descriptor Tail Pointer"]
pub type RDT_W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 3:31 - Receive Descriptor Tail Pointer"]
    #[inline(always)]
    pub fn rdt(&self) -> RDT_R {
        RDT_R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 3:31 - Receive Descriptor Tail Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn rdt(&mut self) -> RDT_W<ETH_DMAC0RX_DTPRrs> {
        RDT_W::new(self, 3)
    }
}
#[doc = "Channel Rx descriptor tail pointer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_dmac0rx_dtpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_dmac0rx_dtpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_DMAC0RX_DTPRrs;
impl crate::RegisterSpec for ETH_DMAC0RX_DTPRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_dmac0rx_dtpr::R`](R) reader structure"]
impl crate::Readable for ETH_DMAC0RX_DTPRrs {}
#[doc = "`write(|w| ..)` method takes [`eth_dmac0rx_dtpr::W`](W) writer structure"]
impl crate::Writable for ETH_DMAC0RX_DTPRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_DMAC0RxDTPR to value 0"]
impl crate::Resettable for ETH_DMAC0RX_DTPRrs {
    const RESET_VALUE: u32 = 0;
}

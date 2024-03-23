#[doc = "Register `ETH_DMAC1TxDTPR` reader"]
pub type R = crate::R<ETH_DMAC1TX_DTPRrs>;
#[doc = "Register `ETH_DMAC1TxDTPR` writer"]
pub type W = crate::W<ETH_DMAC1TX_DTPRrs>;
#[doc = "Field `TDT` reader - Transmit Descriptor Tail Pointer"]
pub type TDT_R = crate::FieldReader<u32>;
#[doc = "Field `TDT` writer - Transmit Descriptor Tail Pointer"]
pub type TDT_W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 3:31 - Transmit Descriptor Tail Pointer"]
    #[inline(always)]
    pub fn tdt(&self) -> TDT_R {
        TDT_R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 3:31 - Transmit Descriptor Tail Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn tdt(&mut self) -> TDT_W<ETH_DMAC1TX_DTPRrs> {
        TDT_W::new(self, 3)
    }
}
#[doc = "Channel Tx descriptor tail pointer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_dmac1tx_dtpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_dmac1tx_dtpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_DMAC1TX_DTPRrs;
impl crate::RegisterSpec for ETH_DMAC1TX_DTPRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_dmac1tx_dtpr::R`](R) reader structure"]
impl crate::Readable for ETH_DMAC1TX_DTPRrs {}
#[doc = "`write(|w| ..)` method takes [`eth_dmac1tx_dtpr::W`](W) writer structure"]
impl crate::Writable for ETH_DMAC1TX_DTPRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_DMAC1TxDTPR to value 0"]
impl crate::Resettable for ETH_DMAC1TX_DTPRrs {
    const RESET_VALUE: u32 = 0;
}

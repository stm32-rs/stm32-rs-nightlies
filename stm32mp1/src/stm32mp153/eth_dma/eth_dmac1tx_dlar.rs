#[doc = "Register `ETH_DMAC1TxDLAR` reader"]
pub type R = crate::R<ETH_DMAC1TX_DLARrs>;
#[doc = "Register `ETH_DMAC1TxDLAR` writer"]
pub type W = crate::W<ETH_DMAC1TX_DLARrs>;
#[doc = "Field `TDESLA` reader - Start of Transmit List"]
pub type TDESLA_R = crate::FieldReader<u32>;
#[doc = "Field `TDESLA` writer - Start of Transmit List"]
pub type TDESLA_W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 3:31 - Start of Transmit List"]
    #[inline(always)]
    pub fn tdesla(&self) -> TDESLA_R {
        TDESLA_R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 3:31 - Start of Transmit List"]
    #[inline(always)]
    #[must_use]
    pub fn tdesla(&mut self) -> TDESLA_W<ETH_DMAC1TX_DLARrs> {
        TDESLA_W::new(self, 3)
    }
}
#[doc = "Channel i Tx descriptor list address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_dmac1tx_dlar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_dmac1tx_dlar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_DMAC1TX_DLARrs;
impl crate::RegisterSpec for ETH_DMAC1TX_DLARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_dmac1tx_dlar::R`](R) reader structure"]
impl crate::Readable for ETH_DMAC1TX_DLARrs {}
#[doc = "`write(|w| ..)` method takes [`eth_dmac1tx_dlar::W`](W) writer structure"]
impl crate::Writable for ETH_DMAC1TX_DLARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_DMAC1TxDLAR to value 0"]
impl crate::Resettable for ETH_DMAC1TX_DLARrs {
    const RESET_VALUE: u32 = 0;
}

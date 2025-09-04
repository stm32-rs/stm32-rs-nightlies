///Register `DMAC0TxRLR` reader
pub type R = crate::R<DMAC0TX_RLRrs>;
///Register `DMAC0TxRLR` writer
pub type W = crate::W<DMAC0TX_RLRrs>;
///Field `TDRL` reader - Transmit Descriptor Ring Length
pub type TDRL_R = crate::FieldReader<u16>;
///Field `TDRL` writer - Transmit Descriptor Ring Length
pub type TDRL_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:9 - Transmit Descriptor Ring Length
    #[inline(always)]
    pub fn tdrl(&self) -> TDRL_R {
        TDRL_R::new((self.bits & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAC0TxRLR")
            .field("tdrl", &self.tdrl())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - Transmit Descriptor Ring Length
    #[inline(always)]
    pub fn tdrl(&mut self) -> TDRL_W<DMAC0TX_RLRrs> {
        TDRL_W::new(self, 0)
    }
}
/**Channel Tx descriptor ring length register

You can [`read`](crate::Reg::read) this register and get [`dmac0tx_rlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0tx_rlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_DMA:DMAC0TxRLR)*/
pub struct DMAC0TX_RLRrs;
impl crate::RegisterSpec for DMAC0TX_RLRrs {
    type Ux = u32;
}
///`read()` method returns [`dmac0tx_rlr::R`](R) reader structure
impl crate::Readable for DMAC0TX_RLRrs {}
///`write(|w| ..)` method takes [`dmac0tx_rlr::W`](W) writer structure
impl crate::Writable for DMAC0TX_RLRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMAC0TxRLR to value 0
impl crate::Resettable for DMAC0TX_RLRrs {}

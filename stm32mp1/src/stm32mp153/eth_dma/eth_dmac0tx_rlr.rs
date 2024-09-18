///Register `ETH_DMAC0TxRLR` reader
pub type R = crate::R<ETH_DMAC0TX_RLRrs>;
///Register `ETH_DMAC0TxRLR` writer
pub type W = crate::W<ETH_DMAC0TX_RLRrs>;
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
        f.debug_struct("ETH_DMAC0TxRLR")
            .field("tdrl", &self.tdrl())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - Transmit Descriptor Ring Length
    #[inline(always)]
    #[must_use]
    pub fn tdrl(&mut self) -> TDRL_W<ETH_DMAC0TX_RLRrs> {
        TDRL_W::new(self, 0)
    }
}
/**Channel Tx descriptor ring length register

You can [`read`](crate::Reg::read) this register and get [`eth_dmac0tx_rlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_dmac0tx_rlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:ETH_DMAC0TxRLR)*/
pub struct ETH_DMAC0TX_RLRrs;
impl crate::RegisterSpec for ETH_DMAC0TX_RLRrs {
    type Ux = u32;
}
///`read()` method returns [`eth_dmac0tx_rlr::R`](R) reader structure
impl crate::Readable for ETH_DMAC0TX_RLRrs {}
///`write(|w| ..)` method takes [`eth_dmac0tx_rlr::W`](W) writer structure
impl crate::Writable for ETH_DMAC0TX_RLRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ETH_DMAC0TxRLR to value 0
impl crate::Resettable for ETH_DMAC0TX_RLRrs {
    const RESET_VALUE: u32 = 0;
}

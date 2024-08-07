///Register `ETH_DMAC0TxDTPR` reader
pub type R = crate::R<ETH_DMAC0TX_DTPRrs>;
///Register `ETH_DMAC0TxDTPR` writer
pub type W = crate::W<ETH_DMAC0TX_DTPRrs>;
///Field `TDT` reader - Transmit Descriptor Tail Pointer
pub type TDT_R = crate::FieldReader<u32>;
///Field `TDT` writer - Transmit Descriptor Tail Pointer
pub type TDT_W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    ///Bits 3:31 - Transmit Descriptor Tail Pointer
    #[inline(always)]
    pub fn tdt(&self) -> TDT_R {
        TDT_R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETH_DMAC0TxDTPR")
            .field("tdt", &self.tdt())
            .finish()
    }
}
impl W {
    ///Bits 3:31 - Transmit Descriptor Tail Pointer
    #[inline(always)]
    #[must_use]
    pub fn tdt(&mut self) -> TDT_W<ETH_DMAC0TX_DTPRrs> {
        TDT_W::new(self, 3)
    }
}
/**Channel Tx descriptor tail pointer register

You can [`read`](crate::Reg::read) this register and get [`eth_dmac0tx_dtpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_dmac0tx_dtpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:ETH_DMAC0TxDTPR)*/
pub struct ETH_DMAC0TX_DTPRrs;
impl crate::RegisterSpec for ETH_DMAC0TX_DTPRrs {
    type Ux = u32;
}
///`read()` method returns [`eth_dmac0tx_dtpr::R`](R) reader structure
impl crate::Readable for ETH_DMAC0TX_DTPRrs {}
///`write(|w| ..)` method takes [`eth_dmac0tx_dtpr::W`](W) writer structure
impl crate::Writable for ETH_DMAC0TX_DTPRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ETH_DMAC0TxDTPR to value 0
impl crate::Resettable for ETH_DMAC0TX_DTPRrs {
    const RESET_VALUE: u32 = 0;
}

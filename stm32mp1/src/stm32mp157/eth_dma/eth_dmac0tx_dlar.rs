///Register `ETH_DMAC0TxDLAR` reader
pub type R = crate::R<ETH_DMAC0TX_DLARrs>;
///Register `ETH_DMAC0TxDLAR` writer
pub type W = crate::W<ETH_DMAC0TX_DLARrs>;
///Field `TDESLA` reader - Start of Transmit List
pub type TDESLA_R = crate::FieldReader<u32>;
///Field `TDESLA` writer - Start of Transmit List
pub type TDESLA_W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    ///Bits 3:31 - Start of Transmit List
    #[inline(always)]
    pub fn tdesla(&self) -> TDESLA_R {
        TDESLA_R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETH_DMAC0TxDLAR")
            .field("tdesla", &self.tdesla())
            .finish()
    }
}
impl W {
    ///Bits 3:31 - Start of Transmit List
    #[inline(always)]
    #[must_use]
    pub fn tdesla(&mut self) -> TDESLA_W<ETH_DMAC0TX_DLARrs> {
        TDESLA_W::new(self, 3)
    }
}
/**Channel i Tx descriptor list address register

You can [`read`](crate::Reg::read) this register and get [`eth_dmac0tx_dlar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_dmac0tx_dlar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_DMA:ETH_DMAC0TxDLAR)*/
pub struct ETH_DMAC0TX_DLARrs;
impl crate::RegisterSpec for ETH_DMAC0TX_DLARrs {
    type Ux = u32;
}
///`read()` method returns [`eth_dmac0tx_dlar::R`](R) reader structure
impl crate::Readable for ETH_DMAC0TX_DLARrs {}
///`write(|w| ..)` method takes [`eth_dmac0tx_dlar::W`](W) writer structure
impl crate::Writable for ETH_DMAC0TX_DLARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ETH_DMAC0TxDLAR to value 0
impl crate::Resettable for ETH_DMAC0TX_DLARrs {
    const RESET_VALUE: u32 = 0;
}

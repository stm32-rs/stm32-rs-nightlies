///Register `ETH_DMAC0RxDLAR` reader
pub type R = crate::R<ETH_DMAC0RX_DLARrs>;
///Register `ETH_DMAC0RxDLAR` writer
pub type W = crate::W<ETH_DMAC0RX_DLARrs>;
///Field `RDESLA` reader - Start of Receive List
pub type RDESLA_R = crate::FieldReader<u32>;
///Field `RDESLA` writer - Start of Receive List
pub type RDESLA_W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    ///Bits 3:31 - Start of Receive List
    #[inline(always)]
    pub fn rdesla(&self) -> RDESLA_R {
        RDESLA_R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETH_DMAC0RxDLAR")
            .field("rdesla", &self.rdesla())
            .finish()
    }
}
impl W {
    ///Bits 3:31 - Start of Receive List
    #[inline(always)]
    #[must_use]
    pub fn rdesla(&mut self) -> RDESLA_W<ETH_DMAC0RX_DLARrs> {
        RDESLA_W::new(self, 3)
    }
}
/**Channel Rx descriptor list address register

You can [`read`](crate::Reg::read) this register and get [`eth_dmac0rx_dlar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_dmac0rx_dlar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:ETH_DMAC0RxDLAR)*/
pub struct ETH_DMAC0RX_DLARrs;
impl crate::RegisterSpec for ETH_DMAC0RX_DLARrs {
    type Ux = u32;
}
///`read()` method returns [`eth_dmac0rx_dlar::R`](R) reader structure
impl crate::Readable for ETH_DMAC0RX_DLARrs {}
///`write(|w| ..)` method takes [`eth_dmac0rx_dlar::W`](W) writer structure
impl crate::Writable for ETH_DMAC0RX_DLARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ETH_DMAC0RxDLAR to value 0x8000
impl crate::Resettable for ETH_DMAC0RX_DLARrs {
    const RESET_VALUE: u32 = 0x8000;
}

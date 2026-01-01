///Register `DMAC0RxRLR` reader
pub type R = crate::R<DMAC0RX_RLRrs>;
///Register `DMAC0RxRLR` writer
pub type W = crate::W<DMAC0RX_RLRrs>;
///Field `RDRL` reader - Receive Descriptor Ring Length
pub type RDRL_R = crate::FieldReader<u16>;
///Field `RDRL` writer - Receive Descriptor Ring Length
pub type RDRL_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:9 - Receive Descriptor Ring Length
    #[inline(always)]
    pub fn rdrl(&self) -> RDRL_R {
        RDRL_R::new((self.bits & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAC0RxRLR")
            .field("rdrl", &self.rdrl())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - Receive Descriptor Ring Length
    #[inline(always)]
    pub fn rdrl(&mut self) -> RDRL_W<'_, DMAC0RX_RLRrs> {
        RDRL_W::new(self, 0)
    }
}
/**Channel Rx descriptor ring length register

You can [`read`](crate::Reg::read) this register and get [`dmac0rx_rlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0rx_rlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:DMAC0RxRLR)*/
pub struct DMAC0RX_RLRrs;
impl crate::RegisterSpec for DMAC0RX_RLRrs {
    type Ux = u32;
}
///`read()` method returns [`dmac0rx_rlr::R`](R) reader structure
impl crate::Readable for DMAC0RX_RLRrs {}
///`write(|w| ..)` method takes [`dmac0rx_rlr::W`](W) writer structure
impl crate::Writable for DMAC0RX_RLRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMAC0RxRLR to value 0x8000
impl crate::Resettable for DMAC0RX_RLRrs {
    const RESET_VALUE: u32 = 0x8000;
}

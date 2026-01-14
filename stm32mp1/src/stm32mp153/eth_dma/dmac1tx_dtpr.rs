///Register `DMAC1TxDTPR` reader
pub type R = crate::R<DMAC1TX_DTPRrs>;
///Register `DMAC1TxDTPR` writer
pub type W = crate::W<DMAC1TX_DTPRrs>;
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
        f.debug_struct("DMAC1TxDTPR")
            .field("tdt", &self.tdt())
            .finish()
    }
}
impl W {
    ///Bits 3:31 - Transmit Descriptor Tail Pointer
    #[inline(always)]
    pub fn tdt(&mut self) -> TDT_W<'_, DMAC1TX_DTPRrs> {
        TDT_W::new(self, 3)
    }
}
/**Channel Tx descriptor tail pointer register

You can [`read`](crate::Reg::read) this register and get [`dmac1tx_dtpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac1tx_dtpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:DMAC1TxDTPR)*/
pub struct DMAC1TX_DTPRrs;
impl crate::RegisterSpec for DMAC1TX_DTPRrs {
    type Ux = u32;
}
///`read()` method returns [`dmac1tx_dtpr::R`](R) reader structure
impl crate::Readable for DMAC1TX_DTPRrs {}
///`write(|w| ..)` method takes [`dmac1tx_dtpr::W`](W) writer structure
impl crate::Writable for DMAC1TX_DTPRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMAC1TxDTPR to value 0
impl crate::Resettable for DMAC1TX_DTPRrs {}

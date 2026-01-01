///Register `DMACTxDTPR` reader
pub type R = crate::R<DMACTX_DTPRrs>;
///Register `DMACTxDTPR` writer
pub type W = crate::W<DMACTX_DTPRrs>;
///Field `TDT` reader - Transmit Descriptor Tail Pointer
pub type TDT_R = crate::FieldReader<u32>;
///Field `TDT` writer - Transmit Descriptor Tail Pointer
pub type TDT_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    ///Bits 2:31 - Transmit Descriptor Tail Pointer
    #[inline(always)]
    pub fn tdt(&self) -> TDT_R {
        TDT_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACTxDTPR")
            .field("tdt", &self.tdt())
            .finish()
    }
}
impl W {
    ///Bits 2:31 - Transmit Descriptor Tail Pointer
    #[inline(always)]
    pub fn tdt(&mut self) -> TDT_W<'_, DMACTX_DTPRrs> {
        TDT_W::new(self, 2)
    }
}
/**Channel Tx descriptor tail pointer register

You can [`read`](crate::Reg::read) this register and get [`dmactx_dtpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmactx_dtpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H723.html#Ethernet_DMA:DMACTxDTPR)*/
pub struct DMACTX_DTPRrs;
impl crate::RegisterSpec for DMACTX_DTPRrs {
    type Ux = u32;
}
///`read()` method returns [`dmactx_dtpr::R`](R) reader structure
impl crate::Readable for DMACTX_DTPRrs {}
///`write(|w| ..)` method takes [`dmactx_dtpr::W`](W) writer structure
impl crate::Writable for DMACTX_DTPRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMACTxDTPR to value 0
impl crate::Resettable for DMACTX_DTPRrs {}

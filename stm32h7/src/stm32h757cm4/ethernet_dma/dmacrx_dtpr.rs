///Register `DMACRxDTPR` reader
pub type R = crate::R<DMACRX_DTPRrs>;
///Register `DMACRxDTPR` writer
pub type W = crate::W<DMACRX_DTPRrs>;
///Field `RDT` reader - Receive Descriptor Tail Pointer
pub type RDT_R = crate::FieldReader<u32>;
///Field `RDT` writer - Receive Descriptor Tail Pointer
pub type RDT_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    ///Bits 2:31 - Receive Descriptor Tail Pointer
    #[inline(always)]
    pub fn rdt(&self) -> RDT_R {
        RDT_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACRxDTPR")
            .field("rdt", &self.rdt())
            .finish()
    }
}
impl W {
    ///Bits 2:31 - Receive Descriptor Tail Pointer
    #[inline(always)]
    pub fn rdt(&mut self) -> RDT_W<'_, DMACRX_DTPRrs> {
        RDT_W::new(self, 2)
    }
}
/**Channel Rx descriptor tail pointer register

You can [`read`](crate::Reg::read) this register and get [`dmacrx_dtpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacrx_dtpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#Ethernet_DMA:DMACRxDTPR)*/
pub struct DMACRX_DTPRrs;
impl crate::RegisterSpec for DMACRX_DTPRrs {
    type Ux = u32;
}
///`read()` method returns [`dmacrx_dtpr::R`](R) reader structure
impl crate::Readable for DMACRX_DTPRrs {}
///`write(|w| ..)` method takes [`dmacrx_dtpr::W`](W) writer structure
impl crate::Writable for DMACRX_DTPRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMACRxDTPR to value 0
impl crate::Resettable for DMACRX_DTPRrs {}

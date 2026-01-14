///Register `DMACRxRLR` reader
pub type R = crate::R<DMACRX_RLRrs>;
///Register `DMACRxRLR` writer
pub type W = crate::W<DMACRX_RLRrs>;
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
        f.debug_struct("DMACRxRLR")
            .field("rdrl", &self.rdrl())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - Receive Descriptor Ring Length
    #[inline(always)]
    pub fn rdrl(&mut self) -> RDRL_W<'_, DMACRX_RLRrs> {
        RDRL_W::new(self, 0)
    }
}
/**Channel Rx descriptor ring length register

You can [`read`](crate::Reg::read) this register and get [`dmacrx_rlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacrx_rlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#Ethernet_DMA:DMACRxRLR)*/
pub struct DMACRX_RLRrs;
impl crate::RegisterSpec for DMACRX_RLRrs {
    type Ux = u32;
}
///`read()` method returns [`dmacrx_rlr::R`](R) reader structure
impl crate::Readable for DMACRX_RLRrs {}
///`write(|w| ..)` method takes [`dmacrx_rlr::W`](W) writer structure
impl crate::Writable for DMACRX_RLRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMACRxRLR to value 0
impl crate::Resettable for DMACRX_RLRrs {}

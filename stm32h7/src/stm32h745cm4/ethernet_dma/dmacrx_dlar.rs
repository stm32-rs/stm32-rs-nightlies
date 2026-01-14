///Register `DMACRxDLAR` reader
pub type R = crate::R<DMACRX_DLARrs>;
///Register `DMACRxDLAR` writer
pub type W = crate::W<DMACRX_DLARrs>;
///Field `RDESLA` reader - Start of Receive List
pub type RDESLA_R = crate::FieldReader<u32>;
///Field `RDESLA` writer - Start of Receive List
pub type RDESLA_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    ///Bits 2:31 - Start of Receive List
    #[inline(always)]
    pub fn rdesla(&self) -> RDESLA_R {
        RDESLA_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACRxDLAR")
            .field("rdesla", &self.rdesla())
            .finish()
    }
}
impl W {
    ///Bits 2:31 - Start of Receive List
    #[inline(always)]
    pub fn rdesla(&mut self) -> RDESLA_W<'_, DMACRX_DLARrs> {
        RDESLA_W::new(self, 2)
    }
}
/**Channel Rx descriptor list address register

You can [`read`](crate::Reg::read) this register and get [`dmacrx_dlar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacrx_dlar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM4.html#Ethernet_DMA:DMACRxDLAR)*/
pub struct DMACRX_DLARrs;
impl crate::RegisterSpec for DMACRX_DLARrs {
    type Ux = u32;
}
///`read()` method returns [`dmacrx_dlar::R`](R) reader structure
impl crate::Readable for DMACRX_DLARrs {}
///`write(|w| ..)` method takes [`dmacrx_dlar::W`](W) writer structure
impl crate::Writable for DMACRX_DLARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMACRxDLAR to value 0
impl crate::Resettable for DMACRX_DLARrs {}

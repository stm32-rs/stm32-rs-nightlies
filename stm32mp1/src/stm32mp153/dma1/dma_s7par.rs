///Register `DMA_S7PAR` reader
pub type R = crate::R<DMA_S7PARrs>;
///Register `DMA_S7PAR` writer
pub type W = crate::W<DMA_S7PARrs>;
///Field `PAR` reader - PAR
pub type PAR_R = crate::FieldReader<u32>;
///Field `PAR` writer - PAR
pub type PAR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - PAR
    #[inline(always)]
    pub fn par(&self) -> PAR_R {
        PAR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_S7PAR")
            .field("par", &self.par())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - PAR
    #[inline(always)]
    #[must_use]
    pub fn par(&mut self) -> PAR_W<DMA_S7PARrs> {
        PAR_W::new(self, 0)
    }
}
/**DMA stream 7 peripheral address register

You can [`read`](crate::Reg::read) this register and get [`dma_s7par::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_s7par::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DMA1:DMA_S7PAR)*/
pub struct DMA_S7PARrs;
impl crate::RegisterSpec for DMA_S7PARrs {
    type Ux = u32;
}
///`read()` method returns [`dma_s7par::R`](R) reader structure
impl crate::Readable for DMA_S7PARrs {}
///`write(|w| ..)` method takes [`dma_s7par::W`](W) writer structure
impl crate::Writable for DMA_S7PARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DMA_S7PAR to value 0
impl crate::Resettable for DMA_S7PARrs {
    const RESET_VALUE: u32 = 0;
}

///Register `DMA_CNDTR3` reader
pub type R = crate::R<DMA_CNDTR3rs>;
///Register `DMA_CNDTR3` writer
pub type W = crate::W<DMA_CNDTR3rs>;
///Field `NDT` reader - Number of data to transfer
pub type NDT_R = crate::FieldReader<u16>;
///Field `NDT` writer - Number of data to transfer
pub type NDT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Number of data to transfer
    #[inline(always)]
    pub fn ndt(&self) -> NDT_R {
        NDT_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_CNDTR3")
            .field("ndt", &self.ndt())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Number of data to transfer
    #[inline(always)]
    pub fn ndt(&mut self) -> NDT_W<DMA_CNDTR3rs> {
        NDT_W::new(self, 0)
    }
}
/**DMA channel 3 number of data to transfer register

You can [`read`](crate::Reg::read) this register and get [`dma_cndtr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cndtr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#DMA1:DMA_CNDTR3)*/
pub struct DMA_CNDTR3rs;
impl crate::RegisterSpec for DMA_CNDTR3rs {
    type Ux = u32;
}
///`read()` method returns [`dma_cndtr3::R`](R) reader structure
impl crate::Readable for DMA_CNDTR3rs {}
///`write(|w| ..)` method takes [`dma_cndtr3::W`](W) writer structure
impl crate::Writable for DMA_CNDTR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DMA_CNDTR3 to value 0
impl crate::Resettable for DMA_CNDTR3rs {
    const RESET_VALUE: u32 = 0;
}

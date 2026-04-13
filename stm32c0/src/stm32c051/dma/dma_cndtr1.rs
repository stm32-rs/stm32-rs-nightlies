///Register `DMA_CNDTR1` reader
pub type R = crate::R<DMA_CNDTR1rs>;
///Register `DMA_CNDTR1` writer
pub type W = crate::W<DMA_CNDTR1rs>;
///Field `NDT` reader - Number of data to transfer (0 to 2<sup>16</sup> - 1) This bitfield is updated by hardware when the channel is enabled: It is decremented after each single DMA read followed by write transfer, indicating the remaining amount of data items to transfer. It is kept at zero when the programmed amount of data to transfer is reached, if the channel is not in circular mode (CIRC = 0 in the DMA_CCRx register). It is reloaded automatically by the previously programmed value, when the transfer is complete, if the channel is in circular mode (CIRC = 1). If this bitfield is zero, no transfer can be served whatever the channel status (enabled or not). Note: This bitfield is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1).
pub type NDT_R = crate::FieldReader<u16>;
///Field `NDT` writer - Number of data to transfer (0 to 2<sup>16</sup> - 1) This bitfield is updated by hardware when the channel is enabled: It is decremented after each single DMA read followed by write transfer, indicating the remaining amount of data items to transfer. It is kept at zero when the programmed amount of data to transfer is reached, if the channel is not in circular mode (CIRC = 0 in the DMA_CCRx register). It is reloaded automatically by the previously programmed value, when the transfer is complete, if the channel is in circular mode (CIRC = 1). If this bitfield is zero, no transfer can be served whatever the channel status (enabled or not). Note: This bitfield is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1).
pub type NDT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Number of data to transfer (0 to 2<sup>16</sup> - 1) This bitfield is updated by hardware when the channel is enabled: It is decremented after each single DMA read followed by write transfer, indicating the remaining amount of data items to transfer. It is kept at zero when the programmed amount of data to transfer is reached, if the channel is not in circular mode (CIRC = 0 in the DMA_CCRx register). It is reloaded automatically by the previously programmed value, when the transfer is complete, if the channel is in circular mode (CIRC = 1). If this bitfield is zero, no transfer can be served whatever the channel status (enabled or not). Note: This bitfield is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1).
    #[inline(always)]
    pub fn ndt(&self) -> NDT_R {
        NDT_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_CNDTR1")
            .field("ndt", &self.ndt())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Number of data to transfer (0 to 2<sup>16</sup> - 1) This bitfield is updated by hardware when the channel is enabled: It is decremented after each single DMA read followed by write transfer, indicating the remaining amount of data items to transfer. It is kept at zero when the programmed amount of data to transfer is reached, if the channel is not in circular mode (CIRC = 0 in the DMA_CCRx register). It is reloaded automatically by the previously programmed value, when the transfer is complete, if the channel is in circular mode (CIRC = 1). If this bitfield is zero, no transfer can be served whatever the channel status (enabled or not). Note: This bitfield is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1).
    #[inline(always)]
    pub fn ndt(&mut self) -> NDT_W<'_, DMA_CNDTR1rs> {
        NDT_W::new(self, 0)
    }
}
/**DMA channel 1 number of data to transfer register

You can [`read`](crate::Reg::read) this register and get [`dma_cndtr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cndtr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#DMA:DMA_CNDTR1)*/
pub struct DMA_CNDTR1rs;
impl crate::RegisterSpec for DMA_CNDTR1rs {
    type Ux = u32;
}
///`read()` method returns [`dma_cndtr1::R`](R) reader structure
impl crate::Readable for DMA_CNDTR1rs {}
///`write(|w| ..)` method takes [`dma_cndtr1::W`](W) writer structure
impl crate::Writable for DMA_CNDTR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMA_CNDTR1 to value 0
impl crate::Resettable for DMA_CNDTR1rs {}

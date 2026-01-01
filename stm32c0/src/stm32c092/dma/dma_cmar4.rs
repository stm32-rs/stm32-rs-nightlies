///Register `DMA_CMAR4` reader
pub type R = crate::R<DMA_CMAR4rs>;
///Register `DMA_CMAR4` writer
pub type W = crate::W<DMA_CMAR4rs>;
///Field `MA` reader - Peripheral address It contains the base address of the memory from/to which the data is read/written. When MSIZE\[1:0\] = 01 (16 bits), bit 0 of MA\[31:0\] is ignored. Access is automatically aligned to a half-word address. When MSIZE\[1:0\] = 10 (32 bits), bits 1 and 0 of MA\[31:0\] are ignored. Access is automatically aligned to a word address. In memory-to-memory mode, this bitfield identifies the memory source address if DIR = 1 and the memory destination address if DIR = 0. In peripheral-to-peripheral mode, this bitfield identifies the peripheral source address if DIR = 1 and the peripheral destination address if DIR = 0. Note: This bitfield is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1).
pub type MA_R = crate::FieldReader<u32>;
///Field `MA` writer - Peripheral address It contains the base address of the memory from/to which the data is read/written. When MSIZE\[1:0\] = 01 (16 bits), bit 0 of MA\[31:0\] is ignored. Access is automatically aligned to a half-word address. When MSIZE\[1:0\] = 10 (32 bits), bits 1 and 0 of MA\[31:0\] are ignored. Access is automatically aligned to a word address. In memory-to-memory mode, this bitfield identifies the memory source address if DIR = 1 and the memory destination address if DIR = 0. In peripheral-to-peripheral mode, this bitfield identifies the peripheral source address if DIR = 1 and the peripheral destination address if DIR = 0. Note: This bitfield is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1).
pub type MA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Peripheral address It contains the base address of the memory from/to which the data is read/written. When MSIZE\[1:0\] = 01 (16 bits), bit 0 of MA\[31:0\] is ignored. Access is automatically aligned to a half-word address. When MSIZE\[1:0\] = 10 (32 bits), bits 1 and 0 of MA\[31:0\] are ignored. Access is automatically aligned to a word address. In memory-to-memory mode, this bitfield identifies the memory source address if DIR = 1 and the memory destination address if DIR = 0. In peripheral-to-peripheral mode, this bitfield identifies the peripheral source address if DIR = 1 and the peripheral destination address if DIR = 0. Note: This bitfield is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1).
    #[inline(always)]
    pub fn ma(&self) -> MA_R {
        MA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_CMAR4").field("ma", &self.ma()).finish()
    }
}
impl W {
    ///Bits 0:31 - Peripheral address It contains the base address of the memory from/to which the data is read/written. When MSIZE\[1:0\] = 01 (16 bits), bit 0 of MA\[31:0\] is ignored. Access is automatically aligned to a half-word address. When MSIZE\[1:0\] = 10 (32 bits), bits 1 and 0 of MA\[31:0\] are ignored. Access is automatically aligned to a word address. In memory-to-memory mode, this bitfield identifies the memory source address if DIR = 1 and the memory destination address if DIR = 0. In peripheral-to-peripheral mode, this bitfield identifies the peripheral source address if DIR = 1 and the peripheral destination address if DIR = 0. Note: This bitfield is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1).
    #[inline(always)]
    pub fn ma(&mut self) -> MA_W<'_, DMA_CMAR4rs> {
        MA_W::new(self, 0)
    }
}
/**DMA channel 4 memory address register

You can [`read`](crate::Reg::read) this register and get [`dma_cmar4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cmar4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#DMA:DMA_CMAR4)*/
pub struct DMA_CMAR4rs;
impl crate::RegisterSpec for DMA_CMAR4rs {
    type Ux = u32;
}
///`read()` method returns [`dma_cmar4::R`](R) reader structure
impl crate::Readable for DMA_CMAR4rs {}
///`write(|w| ..)` method takes [`dma_cmar4::W`](W) writer structure
impl crate::Writable for DMA_CMAR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMA_CMAR4 to value 0
impl crate::Resettable for DMA_CMAR4rs {}

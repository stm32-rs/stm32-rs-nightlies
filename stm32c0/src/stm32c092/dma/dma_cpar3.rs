///Register `DMA_CPAR3` reader
pub type R = crate::R<DMA_CPAR3rs>;
///Register `DMA_CPAR3` writer
pub type W = crate::W<DMA_CPAR3rs>;
///Field `PA` reader - Peripheral address It contains the base address of the peripheral data register from/to which the data is read/written. When PSIZE\[1:0\] = 01 (16 bits), bit 0 of PA\[31:0\] is ignored. Access is automatically aligned to a half-word address. When PSIZE\[1:0\] = 10 (32 bits), bits 1 and 0 of PA\[31:0\] are ignored. Access is automatically aligned to a word address. In memory-to-memory mode, this bitfield identifies the memory destination address if DIR = 1 and the memory source address if DIR = 0. In peripheral-to-peripheral mode, this bitfield identifies the peripheral destination address if DIR = 1 and the peripheral source address if DIR = 0. Note: This bitfield is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1).
pub type PA_R = crate::FieldReader<u32>;
///Field `PA` writer - Peripheral address It contains the base address of the peripheral data register from/to which the data is read/written. When PSIZE\[1:0\] = 01 (16 bits), bit 0 of PA\[31:0\] is ignored. Access is automatically aligned to a half-word address. When PSIZE\[1:0\] = 10 (32 bits), bits 1 and 0 of PA\[31:0\] are ignored. Access is automatically aligned to a word address. In memory-to-memory mode, this bitfield identifies the memory destination address if DIR = 1 and the memory source address if DIR = 0. In peripheral-to-peripheral mode, this bitfield identifies the peripheral destination address if DIR = 1 and the peripheral source address if DIR = 0. Note: This bitfield is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1).
pub type PA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Peripheral address It contains the base address of the peripheral data register from/to which the data is read/written. When PSIZE\[1:0\] = 01 (16 bits), bit 0 of PA\[31:0\] is ignored. Access is automatically aligned to a half-word address. When PSIZE\[1:0\] = 10 (32 bits), bits 1 and 0 of PA\[31:0\] are ignored. Access is automatically aligned to a word address. In memory-to-memory mode, this bitfield identifies the memory destination address if DIR = 1 and the memory source address if DIR = 0. In peripheral-to-peripheral mode, this bitfield identifies the peripheral destination address if DIR = 1 and the peripheral source address if DIR = 0. Note: This bitfield is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1).
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_CPAR3").field("pa", &self.pa()).finish()
    }
}
impl W {
    ///Bits 0:31 - Peripheral address It contains the base address of the peripheral data register from/to which the data is read/written. When PSIZE\[1:0\] = 01 (16 bits), bit 0 of PA\[31:0\] is ignored. Access is automatically aligned to a half-word address. When PSIZE\[1:0\] = 10 (32 bits), bits 1 and 0 of PA\[31:0\] are ignored. Access is automatically aligned to a word address. In memory-to-memory mode, this bitfield identifies the memory destination address if DIR = 1 and the memory source address if DIR = 0. In peripheral-to-peripheral mode, this bitfield identifies the peripheral destination address if DIR = 1 and the peripheral source address if DIR = 0. Note: This bitfield is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1).
    #[inline(always)]
    pub fn pa(&mut self) -> PA_W<'_, DMA_CPAR3rs> {
        PA_W::new(self, 0)
    }
}
/**DMA channel 3 peripheral address register

You can [`read`](crate::Reg::read) this register and get [`dma_cpar3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cpar3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#DMA:DMA_CPAR3)*/
pub struct DMA_CPAR3rs;
impl crate::RegisterSpec for DMA_CPAR3rs {
    type Ux = u32;
}
///`read()` method returns [`dma_cpar3::R`](R) reader structure
impl crate::Readable for DMA_CPAR3rs {}
///`write(|w| ..)` method takes [`dma_cpar3::W`](W) writer structure
impl crate::Writable for DMA_CPAR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMA_CPAR3 to value 0
impl crate::Resettable for DMA_CPAR3rs {}

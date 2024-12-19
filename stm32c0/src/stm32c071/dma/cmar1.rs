///Register `CMAR1` reader
pub type R = crate::R<CMAR1rs>;
///Register `CMAR1` writer
pub type W = crate::W<CMAR1rs>;
/**Field `MA` reader - Peripheral address It contains the base address of the memory from/to which the data is read/written. When MSIZE\[1:0\]
= 01 (16 bits), bit 0 of MA\[31:0\]
is ignored. Access is automatically aligned to a half-word address. When MSIZE\[1:0\]
= 10 (32 bits), bits 1 and 0 of MA\[31:0\]
are ignored. Access is automatically aligned to a word address. In memory-to-memory mode, this bitfield identifies the memory source address if DIR = 1 and the memory destination address if DIR = 0. In peripheral-to-peripheral mode, this bitfield identifies the peripheral source address if DIR = 1 and the peripheral destination address if DIR = 0. Note: This bitfield is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1).*/
pub type MA_R = crate::FieldReader<u32>;
/**Field `MA` writer - Peripheral address It contains the base address of the memory from/to which the data is read/written. When MSIZE\[1:0\]
= 01 (16 bits), bit 0 of MA\[31:0\]
is ignored. Access is automatically aligned to a half-word address. When MSIZE\[1:0\]
= 10 (32 bits), bits 1 and 0 of MA\[31:0\]
are ignored. Access is automatically aligned to a word address. In memory-to-memory mode, this bitfield identifies the memory source address if DIR = 1 and the memory destination address if DIR = 0. In peripheral-to-peripheral mode, this bitfield identifies the peripheral source address if DIR = 1 and the peripheral destination address if DIR = 0. Note: This bitfield is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1).*/
pub type MA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    /**Bits 0:31 - Peripheral address It contains the base address of the memory from/to which the data is read/written. When MSIZE\[1:0\]
    = 01 (16 bits), bit 0 of MA\[31:0\]
    is ignored. Access is automatically aligned to a half-word address. When MSIZE\[1:0\]
    = 10 (32 bits), bits 1 and 0 of MA\[31:0\]
    are ignored. Access is automatically aligned to a word address. In memory-to-memory mode, this bitfield identifies the memory source address if DIR = 1 and the memory destination address if DIR = 0. In peripheral-to-peripheral mode, this bitfield identifies the peripheral source address if DIR = 1 and the peripheral destination address if DIR = 0. Note: This bitfield is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1).*/
    #[inline(always)]
    pub fn ma(&self) -> MA_R {
        MA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMAR1").field("ma", &self.ma()).finish()
    }
}
impl W {
    /**Bits 0:31 - Peripheral address It contains the base address of the memory from/to which the data is read/written. When MSIZE\[1:0\]
    = 01 (16 bits), bit 0 of MA\[31:0\]
    is ignored. Access is automatically aligned to a half-word address. When MSIZE\[1:0\]
    = 10 (32 bits), bits 1 and 0 of MA\[31:0\]
    are ignored. Access is automatically aligned to a word address. In memory-to-memory mode, this bitfield identifies the memory source address if DIR = 1 and the memory destination address if DIR = 0. In peripheral-to-peripheral mode, this bitfield identifies the peripheral source address if DIR = 1 and the peripheral destination address if DIR = 0. Note: This bitfield is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1).*/
    #[inline(always)]
    pub fn ma(&mut self) -> MA_W<CMAR1rs> {
        MA_W::new(self, 0)
    }
}
/**DMA channel 1 memory address register

You can [`read`](crate::Reg::read) this register and get [`cmar1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmar1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#DMA:CMAR1)*/
pub struct CMAR1rs;
impl crate::RegisterSpec for CMAR1rs {
    type Ux = u32;
}
///`read()` method returns [`cmar1::R`](R) reader structure
impl crate::Readable for CMAR1rs {}
///`write(|w| ..)` method takes [`cmar1::W`](W) writer structure
impl crate::Writable for CMAR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CMAR1 to value 0
impl crate::Resettable for CMAR1rs {
    const RESET_VALUE: u32 = 0;
}

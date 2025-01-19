///Register `BDMA_CPAR6` reader
pub type R = crate::R<BDMA_CPAR6rs>;
///Register `BDMA_CPAR6` writer
pub type W = crate::W<BDMA_CPAR6rs>;
/**Field `PA` reader - peripheral address It contains the base address of the peripheral data register from/to which the data is read/written. When PSIZE\[1:0\]
= 01 (16 bits), bit 0 of PA\[31:0\]
is ignored. Access is automatically aligned to a half-word address. When PSIZE = 10 (32 bits), bits 1 and 0 of PA\[31:0\]
are ignored. Access is automatically aligned to a word address. In memory-to-memory mode, this register identifies the memory destination address if DIR = 1 and the memory source address if DIR = 0. In peripheral-to-peripheral mode, this register identifies the peripheral destination address DIR = 1 and the peripheral source address if DIR = 0. Note: this register is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1).*/
pub type PA_R = crate::FieldReader<u32>;
/**Field `PA` writer - peripheral address It contains the base address of the peripheral data register from/to which the data is read/written. When PSIZE\[1:0\]
= 01 (16 bits), bit 0 of PA\[31:0\]
is ignored. Access is automatically aligned to a half-word address. When PSIZE = 10 (32 bits), bits 1 and 0 of PA\[31:0\]
are ignored. Access is automatically aligned to a word address. In memory-to-memory mode, this register identifies the memory destination address if DIR = 1 and the memory source address if DIR = 0. In peripheral-to-peripheral mode, this register identifies the peripheral destination address DIR = 1 and the peripheral source address if DIR = 0. Note: this register is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1).*/
pub type PA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    /**Bits 0:31 - peripheral address It contains the base address of the peripheral data register from/to which the data is read/written. When PSIZE\[1:0\]
    = 01 (16 bits), bit 0 of PA\[31:0\]
    is ignored. Access is automatically aligned to a half-word address. When PSIZE = 10 (32 bits), bits 1 and 0 of PA\[31:0\]
    are ignored. Access is automatically aligned to a word address. In memory-to-memory mode, this register identifies the memory destination address if DIR = 1 and the memory source address if DIR = 0. In peripheral-to-peripheral mode, this register identifies the peripheral destination address DIR = 1 and the peripheral source address if DIR = 0. Note: this register is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1).*/
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BDMA_CPAR6")
            .field("pa", &self.pa())
            .finish()
    }
}
impl W {
    /**Bits 0:31 - peripheral address It contains the base address of the peripheral data register from/to which the data is read/written. When PSIZE\[1:0\]
    = 01 (16 bits), bit 0 of PA\[31:0\]
    is ignored. Access is automatically aligned to a half-word address. When PSIZE = 10 (32 bits), bits 1 and 0 of PA\[31:0\]
    are ignored. Access is automatically aligned to a word address. In memory-to-memory mode, this register identifies the memory destination address if DIR = 1 and the memory source address if DIR = 0. In peripheral-to-peripheral mode, this register identifies the peripheral destination address DIR = 1 and the peripheral source address if DIR = 0. Note: this register is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1).*/
    #[inline(always)]
    pub fn pa(&mut self) -> PA_W<BDMA_CPAR6rs> {
        PA_W::new(self, 0)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`bdma_cpar6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdma_cpar6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#BDMA:BDMA_CPAR6)*/
pub struct BDMA_CPAR6rs;
impl crate::RegisterSpec for BDMA_CPAR6rs {
    type Ux = u32;
}
///`read()` method returns [`bdma_cpar6::R`](R) reader structure
impl crate::Readable for BDMA_CPAR6rs {}
///`write(|w| ..)` method takes [`bdma_cpar6::W`](W) writer structure
impl crate::Writable for BDMA_CPAR6rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BDMA_CPAR6 to value 0
impl crate::Resettable for BDMA_CPAR6rs {
    const RESET_VALUE: u32 = 0;
}

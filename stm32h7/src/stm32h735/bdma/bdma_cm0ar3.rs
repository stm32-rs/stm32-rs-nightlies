///Register `BDMA_CM0AR3` reader
pub type R = crate::R<BDMA_CM0AR3rs>;
///Register `BDMA_CM0AR3` writer
pub type W = crate::W<BDMA_CM0AR3rs>;
/**Field `MA` reader - peripheral address It contains the base address of the memory from/to which the data is read/written. When MSIZE\[1:0\]
= 01 (16 bits), bit 0 of MA\[31:0\]
is ignored. Access is automatically aligned to a half-word address. When MSIZE = 10 (32 bits), bits 1 and 0 of MA\[31:0\]
are ignored. Access is automatically aligned to a word address. In memory-to-memory mode, this register identifies the memory source address if DIR = 1 and the memory destination address if DIR = 0. In peripheral-to-peripheral mode, this register identifies the peripheral source address DIR = 1 and the peripheral destination address if DIR = 0. Note: this register is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1).*/
pub type MA_R = crate::FieldReader<u32>;
/**Field `MA` writer - peripheral address It contains the base address of the memory from/to which the data is read/written. When MSIZE\[1:0\]
= 01 (16 bits), bit 0 of MA\[31:0\]
is ignored. Access is automatically aligned to a half-word address. When MSIZE = 10 (32 bits), bits 1 and 0 of MA\[31:0\]
are ignored. Access is automatically aligned to a word address. In memory-to-memory mode, this register identifies the memory source address if DIR = 1 and the memory destination address if DIR = 0. In peripheral-to-peripheral mode, this register identifies the peripheral source address DIR = 1 and the peripheral destination address if DIR = 0. Note: this register is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1).*/
pub type MA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    /**Bits 0:31 - peripheral address It contains the base address of the memory from/to which the data is read/written. When MSIZE\[1:0\]
    = 01 (16 bits), bit 0 of MA\[31:0\]
    is ignored. Access is automatically aligned to a half-word address. When MSIZE = 10 (32 bits), bits 1 and 0 of MA\[31:0\]
    are ignored. Access is automatically aligned to a word address. In memory-to-memory mode, this register identifies the memory source address if DIR = 1 and the memory destination address if DIR = 0. In peripheral-to-peripheral mode, this register identifies the peripheral source address DIR = 1 and the peripheral destination address if DIR = 0. Note: this register is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1).*/
    #[inline(always)]
    pub fn ma(&self) -> MA_R {
        MA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BDMA_CM0AR3")
            .field("ma", &self.ma())
            .finish()
    }
}
impl W {
    /**Bits 0:31 - peripheral address It contains the base address of the memory from/to which the data is read/written. When MSIZE\[1:0\]
    = 01 (16 bits), bit 0 of MA\[31:0\]
    is ignored. Access is automatically aligned to a half-word address. When MSIZE = 10 (32 bits), bits 1 and 0 of MA\[31:0\]
    are ignored. Access is automatically aligned to a word address. In memory-to-memory mode, this register identifies the memory source address if DIR = 1 and the memory destination address if DIR = 0. In peripheral-to-peripheral mode, this register identifies the peripheral source address DIR = 1 and the peripheral destination address if DIR = 0. Note: this register is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1).*/
    #[inline(always)]
    pub fn ma(&mut self) -> MA_W<BDMA_CM0AR3rs> {
        MA_W::new(self, 0)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`bdma_cm0ar3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdma_cm0ar3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#BDMA:BDMA_CM0AR3)*/
pub struct BDMA_CM0AR3rs;
impl crate::RegisterSpec for BDMA_CM0AR3rs {
    type Ux = u32;
}
///`read()` method returns [`bdma_cm0ar3::R`](R) reader structure
impl crate::Readable for BDMA_CM0AR3rs {}
///`write(|w| ..)` method takes [`bdma_cm0ar3::W`](W) writer structure
impl crate::Writable for BDMA_CM0AR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BDMA_CM0AR3 to value 0
impl crate::Resettable for BDMA_CM0AR3rs {
    const RESET_VALUE: u32 = 0;
}
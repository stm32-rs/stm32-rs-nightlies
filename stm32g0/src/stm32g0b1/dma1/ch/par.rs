///Register `PAR` reader
pub type R = crate::R<PARrs>;
///Register `PAR` writer
pub type W = crate::W<PARrs>;
///Field `PA` reader - peripheral address It contains the base address of the peripheral data register from/to which the data will be read/written. When PSIZE\[1:0\]=01 (16 bits), bit 0 of PA\[31:0\] is ignored. Access is automatically aligned to a half-word address. When PSIZE=10 (32 bits), bits 1 and 0 of PA\[31:0\] are ignored. Access is automatically aligned to a word address. In memory-to-memory mode, this register identifies the memory destination address if DIR=1 and the memory source address if DIR=0. In peripheral-to-peripheral mode, this register identifies the peripheral destination address DIR=1 and the peripheral source address if DIR=0. Note: this register is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN=1).
pub type PA_R = crate::FieldReader<u32>;
///Field `PA` writer - peripheral address It contains the base address of the peripheral data register from/to which the data will be read/written. When PSIZE\[1:0\]=01 (16 bits), bit 0 of PA\[31:0\] is ignored. Access is automatically aligned to a half-word address. When PSIZE=10 (32 bits), bits 1 and 0 of PA\[31:0\] are ignored. Access is automatically aligned to a word address. In memory-to-memory mode, this register identifies the memory destination address if DIR=1 and the memory source address if DIR=0. In peripheral-to-peripheral mode, this register identifies the peripheral destination address DIR=1 and the peripheral source address if DIR=0. Note: this register is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN=1).
pub type PA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - peripheral address It contains the base address of the peripheral data register from/to which the data will be read/written. When PSIZE\[1:0\]=01 (16 bits), bit 0 of PA\[31:0\] is ignored. Access is automatically aligned to a half-word address. When PSIZE=10 (32 bits), bits 1 and 0 of PA\[31:0\] are ignored. Access is automatically aligned to a word address. In memory-to-memory mode, this register identifies the memory destination address if DIR=1 and the memory source address if DIR=0. In peripheral-to-peripheral mode, this register identifies the peripheral destination address DIR=1 and the peripheral source address if DIR=0. Note: this register is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN=1).
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAR").field("pa", &self.pa()).finish()
    }
}
impl W {
    ///Bits 0:31 - peripheral address It contains the base address of the peripheral data register from/to which the data will be read/written. When PSIZE\[1:0\]=01 (16 bits), bit 0 of PA\[31:0\] is ignored. Access is automatically aligned to a half-word address. When PSIZE=10 (32 bits), bits 1 and 0 of PA\[31:0\] are ignored. Access is automatically aligned to a word address. In memory-to-memory mode, this register identifies the memory destination address if DIR=1 and the memory source address if DIR=0. In peripheral-to-peripheral mode, this register identifies the peripheral destination address DIR=1 and the peripheral source address if DIR=0. Note: this register is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN=1).
    #[inline(always)]
    pub fn pa(&mut self) -> PA_W<'_, PARrs> {
        PA_W::new(self, 0)
    }
}
/**DMA channel 1 peripheral address register

You can [`read`](crate::Reg::read) this register and get [`par::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`par::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PARrs;
impl crate::RegisterSpec for PARrs {
    type Ux = u32;
}
///`read()` method returns [`par::R`](R) reader structure
impl crate::Readable for PARrs {}
///`write(|w| ..)` method takes [`par::W`](W) writer structure
impl crate::Writable for PARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PAR to value 0
impl crate::Resettable for PARrs {}

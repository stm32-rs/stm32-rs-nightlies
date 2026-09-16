///Register `CPAR7` reader
pub type R = crate::R<CPAR7rs>;
///Register `CPAR7` writer
pub type W = crate::W<CPAR7rs>;
///Field `PA` reader - PA\[31:0\]: Peripheral address Base address of the peripheral data register from/to which the data will be read/written. When PSIZE is 01 (16-bit), the PA\[0\] bit is ignored. Access is automatically aligned to a halfword address. When PSIZE is 10 (32-bit), PA\[1:0\] are ignored. Access is automatically aligned to a word address.
pub type PA_R = crate::FieldReader<u32>;
///Field `PA` writer - PA\[31:0\]: Peripheral address Base address of the peripheral data register from/to which the data will be read/written. When PSIZE is 01 (16-bit), the PA\[0\] bit is ignored. Access is automatically aligned to a halfword address. When PSIZE is 10 (32-bit), PA\[1:0\] are ignored. Access is automatically aligned to a word address.
pub type PA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - PA\[31:0\]: Peripheral address Base address of the peripheral data register from/to which the data will be read/written. When PSIZE is 01 (16-bit), the PA\[0\] bit is ignored. Access is automatically aligned to a halfword address. When PSIZE is 10 (32-bit), PA\[1:0\] are ignored. Access is automatically aligned to a word address.
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPAR7").field("pa", &self.pa()).finish()
    }
}
impl W {
    ///Bits 0:31 - PA\[31:0\]: Peripheral address Base address of the peripheral data register from/to which the data will be read/written. When PSIZE is 01 (16-bit), the PA\[0\] bit is ignored. Access is automatically aligned to a halfword address. When PSIZE is 10 (32-bit), PA\[1:0\] are ignored. Access is automatically aligned to a word address.
    #[inline(always)]
    pub fn pa(&mut self) -> PA_W<'_, CPAR7rs> {
        PA_W::new(self, 0)
    }
}
/**DMA_CPARx register

You can [`read`](crate::Reg::read) this register and get [`cpar7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpar7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DMA:CPAR7)*/
pub struct CPAR7rs;
impl crate::RegisterSpec for CPAR7rs {
    type Ux = u32;
}
///`read()` method returns [`cpar7::R`](R) reader structure
impl crate::Readable for CPAR7rs {}
///`write(|w| ..)` method takes [`cpar7::W`](W) writer structure
impl crate::Writable for CPAR7rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CPAR7 to value 0
impl crate::Resettable for CPAR7rs {}

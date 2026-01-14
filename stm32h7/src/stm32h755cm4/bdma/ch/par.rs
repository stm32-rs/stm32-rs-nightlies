///Register `PAR` reader
pub type R = crate::R<PARrs>;
///Register `PAR` writer
pub type W = crate::W<PARrs>;
///Field `PA` reader - Peripheral address Base address of the peripheral data register from/to which the data will be read/written. When PSIZE is 01 (16-bit), the PA\[0\] bit is ignored. Access is automatically aligned to a half-word address. When PSIZE is 10 (32-bit), PA\[1:0\] are ignored. Access is automatically aligned to a word address.
pub type PA_R = crate::FieldReader<u32>;
///Field `PA` writer - Peripheral address Base address of the peripheral data register from/to which the data will be read/written. When PSIZE is 01 (16-bit), the PA\[0\] bit is ignored. Access is automatically aligned to a half-word address. When PSIZE is 10 (32-bit), PA\[1:0\] are ignored. Access is automatically aligned to a word address.
pub type PA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Peripheral address Base address of the peripheral data register from/to which the data will be read/written. When PSIZE is 01 (16-bit), the PA\[0\] bit is ignored. Access is automatically aligned to a half-word address. When PSIZE is 10 (32-bit), PA\[1:0\] are ignored. Access is automatically aligned to a word address.
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
    ///Bits 0:31 - Peripheral address Base address of the peripheral data register from/to which the data will be read/written. When PSIZE is 01 (16-bit), the PA\[0\] bit is ignored. Access is automatically aligned to a half-word address. When PSIZE is 10 (32-bit), PA\[1:0\] are ignored. Access is automatically aligned to a word address.
    #[inline(always)]
    pub fn pa(&mut self) -> PA_W<'_, PARrs> {
        PA_W::new(self, 0)
    }
}
/**This register must not be written when the channel is enabled.

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

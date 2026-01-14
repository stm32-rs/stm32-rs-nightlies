///Register `S6PAR` reader
pub type R = crate::R<S6PARrs>;
///Register `S6PAR` writer
pub type W = crate::W<S6PARrs>;
///Field `PAR` reader - PAR
pub type PAR_R = crate::FieldReader<u32>;
///Field `PAR` writer - PAR
pub type PAR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - PAR
    #[inline(always)]
    pub fn par(&self) -> PAR_R {
        PAR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S6PAR").field("par", &self.par()).finish()
    }
}
impl W {
    ///Bits 0:31 - PAR
    #[inline(always)]
    pub fn par(&mut self) -> PAR_W<'_, S6PARrs> {
        PAR_W::new(self, 0)
    }
}
/**DMA stream 6 peripheral address register

You can [`read`](crate::Reg::read) this register and get [`s6par::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s6par::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DMA1:S6PAR)*/
pub struct S6PARrs;
impl crate::RegisterSpec for S6PARrs {
    type Ux = u32;
}
///`read()` method returns [`s6par::R`](R) reader structure
impl crate::Readable for S6PARrs {}
///`write(|w| ..)` method takes [`s6par::W`](W) writer structure
impl crate::Writable for S6PARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets S6PAR to value 0
impl crate::Resettable for S6PARrs {}

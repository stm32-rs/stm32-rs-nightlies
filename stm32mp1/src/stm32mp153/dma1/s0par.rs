///Register `S0PAR` reader
pub type R = crate::R<S0PARrs>;
///Register `S0PAR` writer
pub type W = crate::W<S0PARrs>;
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
        f.debug_struct("S0PAR").field("par", &self.par()).finish()
    }
}
impl W {
    ///Bits 0:31 - PAR
    #[inline(always)]
    pub fn par(&mut self) -> PAR_W<'_, S0PARrs> {
        PAR_W::new(self, 0)
    }
}
/**DMA stream 0 peripheral address register

You can [`read`](crate::Reg::read) this register and get [`s0par::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s0par::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DMA1:S0PAR)*/
pub struct S0PARrs;
impl crate::RegisterSpec for S0PARrs {
    type Ux = u32;
}
///`read()` method returns [`s0par::R`](R) reader structure
impl crate::Readable for S0PARrs {}
///`write(|w| ..)` method takes [`s0par::W`](W) writer structure
impl crate::Writable for S0PARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets S0PAR to value 0
impl crate::Resettable for S0PARrs {}

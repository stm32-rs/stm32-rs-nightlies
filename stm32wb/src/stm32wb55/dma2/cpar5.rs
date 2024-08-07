///Register `CPAR5` reader
pub type R = crate::R<CPAR5rs>;
///Register `CPAR5` writer
pub type W = crate::W<CPAR5rs>;
///Field `PA` reader - Peripheral address
pub type PA_R = crate::FieldReader<u32>;
///Field `PA` writer - Peripheral address
pub type PA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Peripheral address
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPAR5").field("pa", &self.pa()).finish()
    }
}
impl W {
    ///Bits 0:31 - Peripheral address
    #[inline(always)]
    #[must_use]
    pub fn pa(&mut self) -> PA_W<CPAR5rs> {
        PA_W::new(self, 0)
    }
}
/**channel x peripheral address register

You can [`read`](crate::Reg::read) this register and get [`cpar5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpar5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#DMA2:CPAR5)*/
pub struct CPAR5rs;
impl crate::RegisterSpec for CPAR5rs {
    type Ux = u32;
}
///`read()` method returns [`cpar5::R`](R) reader structure
impl crate::Readable for CPAR5rs {}
///`write(|w| ..)` method takes [`cpar5::W`](W) writer structure
impl crate::Writable for CPAR5rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CPAR5 to value 0
impl crate::Resettable for CPAR5rs {
    const RESET_VALUE: u32 = 0;
}

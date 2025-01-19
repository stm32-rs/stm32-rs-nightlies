///Register `CPAR4` reader
pub type R = crate::R<CPAR4rs>;
///Register `CPAR4` writer
pub type W = crate::W<CPAR4rs>;
///Field `PA` reader - peripheral address
pub type PA_R = crate::FieldReader<u32>;
///Field `PA` writer - peripheral address
pub type PA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl R {
    ///Bits 0:31 - peripheral address
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPAR4").field("pa", &self.pa()).finish()
    }
}
impl W {
    ///Bits 0:31 - peripheral address
    #[inline(always)]
    pub fn pa(&mut self) -> PA_W<CPAR4rs> {
        PA_W::new(self, 0)
    }
}
/**channel x peripheral address register

You can [`read`](crate::Reg::read) this register and get [`cpar4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpar4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#DMA1:CPAR4)*/
pub struct CPAR4rs;
impl crate::RegisterSpec for CPAR4rs {
    type Ux = u32;
}
///`read()` method returns [`cpar4::R`](R) reader structure
impl crate::Readable for CPAR4rs {}
///`write(|w| ..)` method takes [`cpar4::W`](W) writer structure
impl crate::Writable for CPAR4rs {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CPAR4 to value 0
impl crate::Resettable for CPAR4rs {
    const RESET_VALUE: u32 = 0;
}

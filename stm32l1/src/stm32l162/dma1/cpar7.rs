///Register `CPAR7` reader
pub type R = crate::R<CPAR7rs>;
///Register `CPAR7` writer
pub type W = crate::W<CPAR7rs>;
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
        f.debug_struct("CPAR7").field("pa", &self.pa()).finish()
    }
}
impl W {
    ///Bits 0:31 - Peripheral address
    #[inline(always)]
    #[must_use]
    pub fn pa(&mut self) -> PA_W<CPAR7rs> {
        PA_W::new(self, 0)
    }
}
/**channel x peripheral address register

You can [`read`](crate::Reg::read) this register and get [`cpar7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpar7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L162.html#DMA1:CPAR7)*/
pub struct CPAR7rs;
impl crate::RegisterSpec for CPAR7rs {
    type Ux = u32;
}
///`read()` method returns [`cpar7::R`](R) reader structure
impl crate::Readable for CPAR7rs {}
///`write(|w| ..)` method takes [`cpar7::W`](W) writer structure
impl crate::Writable for CPAR7rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CPAR7 to value 0
impl crate::Resettable for CPAR7rs {
    const RESET_VALUE: u32 = 0;
}

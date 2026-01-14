///Register `CICR1` reader
pub type R = crate::R<CICR1rs>;
///Register `CICR1` writer
pub type W = crate::W<CICR1rs>;
///Field `PA` reader - Port A channel identification for capture
pub type PA_R = crate::FieldReader<u16>;
///Field `PA` writer - Port A channel identification for capture
pub type PA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Port A channel identification for capture
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CICR1").field("pa", &self.pa()).finish()
    }
}
impl W {
    ///Bits 0:15 - Port A channel identification for capture
    #[inline(always)]
    pub fn pa(&mut self) -> PA_W<'_, CICR1rs> {
        PA_W::new(self, 0)
    }
}
/**Channel identification for capture register

You can [`read`](crate::Reg::read) this register and get [`cicr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cicr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#RI:CICR1)*/
pub struct CICR1rs;
impl crate::RegisterSpec for CICR1rs {
    type Ux = u32;
}
///`read()` method returns [`cicr1::R`](R) reader structure
impl crate::Readable for CICR1rs {}
///`write(|w| ..)` method takes [`cicr1::W`](W) writer structure
impl crate::Writable for CICR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CICR1 to value 0
impl crate::Resettable for CICR1rs {}

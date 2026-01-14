///Register `CMR1` reader
pub type R = crate::R<CMR1rs>;
///Register `CMR1` writer
pub type W = crate::W<CMR1rs>;
///Field `PA` reader - Port A channel masking
pub type PA_R = crate::FieldReader<u16>;
///Field `PA` writer - Port A channel masking
pub type PA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Port A channel masking
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMR1").field("pa", &self.pa()).finish()
    }
}
impl W {
    ///Bits 0:15 - Port A channel masking
    #[inline(always)]
    pub fn pa(&mut self) -> PA_W<'_, CMR1rs> {
        PA_W::new(self, 0)
    }
}
/**Channel mask register

You can [`read`](crate::Reg::read) this register and get [`cmr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L162.html#RI:CMR1)*/
pub struct CMR1rs;
impl crate::RegisterSpec for CMR1rs {
    type Ux = u32;
}
///`read()` method returns [`cmr1::R`](R) reader structure
impl crate::Readable for CMR1rs {}
///`write(|w| ..)` method takes [`cmr1::W`](W) writer structure
impl crate::Writable for CMR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CMR1 to value 0
impl crate::Resettable for CMR1rs {}

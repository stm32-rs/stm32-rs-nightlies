///Register `JOFR1` reader
pub type R = crate::R<JOFR1rs>;
///Register `JOFR1` writer
pub type W = crate::W<JOFR1rs>;
///Field `JOFFSET1` reader - Data offset for injected channel x
pub type JOFFSET1_R = crate::FieldReader<u16>;
///Field `JOFFSET1` writer - Data offset for injected channel x
pub type JOFFSET1_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:11 - Data offset for injected channel x
    #[inline(always)]
    pub fn joffset1(&self) -> JOFFSET1_R {
        JOFFSET1_R::new((self.bits & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JOFR1")
            .field("joffset1", &self.joffset1())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - Data offset for injected channel x
    #[inline(always)]
    pub fn joffset1(&mut self) -> JOFFSET1_W<'_, JOFR1rs> {
        JOFFSET1_W::new(self, 0)
    }
}
/**injected channel data offset register x

You can [`read`](crate::Reg::read) this register and get [`jofr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jofr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#ADC1:JOFR1)*/
pub struct JOFR1rs;
impl crate::RegisterSpec for JOFR1rs {
    type Ux = u32;
}
///`read()` method returns [`jofr1::R`](R) reader structure
impl crate::Readable for JOFR1rs {}
///`write(|w| ..)` method takes [`jofr1::W`](W) writer structure
impl crate::Writable for JOFR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets JOFR1 to value 0
impl crate::Resettable for JOFR1rs {}

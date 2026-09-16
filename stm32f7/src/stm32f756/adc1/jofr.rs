///Register `JOFR%s` reader
pub type R = crate::R<JOFRrs>;
///Register `JOFR%s` writer
pub type W = crate::W<JOFRrs>;
///Field `JOFFSET` reader - Data offset for injected channel
pub type JOFFSET_R = crate::FieldReader<u16>;
///Field `JOFFSET` writer - Data offset for injected channel
pub type JOFFSET_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16, crate::Safe>;
impl R {
    ///Bits 0:11 - Data offset for injected channel
    #[inline(always)]
    pub fn joffset(&self) -> JOFFSET_R {
        JOFFSET_R::new((self.bits & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JOFR")
            .field("joffset", &self.joffset())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - Data offset for injected channel
    #[inline(always)]
    pub fn joffset(&mut self) -> JOFFSET_W<'_, JOFRrs> {
        JOFFSET_W::new(self, 0)
    }
}
/**injected channel data offset register %s

You can [`read`](crate::Reg::read) this register and get [`jofr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jofr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F756.html#ADC1:JOFR[1])*/
pub struct JOFRrs;
impl crate::RegisterSpec for JOFRrs {
    type Ux = u32;
}
///`read()` method returns [`jofr::R`](R) reader structure
impl crate::Readable for JOFRrs {}
///`write(|w| ..)` method takes [`jofr::W`](W) writer structure
impl crate::Writable for JOFRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets JOFR%s to value 0
impl crate::Resettable for JOFRrs {}

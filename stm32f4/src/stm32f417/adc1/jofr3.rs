///Register `JOFR3` reader
pub type R = crate::R<JOFR3rs>;
///Register `JOFR3` writer
pub type W = crate::W<JOFR3rs>;
///Field `JOFFSET3` reader - Data offset for injected channel x
pub type JOFFSET3_R = crate::FieldReader<u16>;
///Field `JOFFSET3` writer - Data offset for injected channel x
pub type JOFFSET3_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:11 - Data offset for injected channel x
    #[inline(always)]
    pub fn joffset3(&self) -> JOFFSET3_R {
        JOFFSET3_R::new((self.bits & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JOFR3")
            .field("joffset3", &self.joffset3())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - Data offset for injected channel x
    #[inline(always)]
    pub fn joffset3(&mut self) -> JOFFSET3_W<'_, JOFR3rs> {
        JOFFSET3_W::new(self, 0)
    }
}
/**injected channel data offset register x

You can [`read`](crate::Reg::read) this register and get [`jofr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jofr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#ADC1:JOFR3)*/
pub struct JOFR3rs;
impl crate::RegisterSpec for JOFR3rs {
    type Ux = u32;
}
///`read()` method returns [`jofr3::R`](R) reader structure
impl crate::Readable for JOFR3rs {}
///`write(|w| ..)` method takes [`jofr3::W`](W) writer structure
impl crate::Writable for JOFR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets JOFR3 to value 0
impl crate::Resettable for JOFR3rs {}

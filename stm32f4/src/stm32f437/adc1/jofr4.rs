///Register `JOFR4` reader
pub type R = crate::R<JOFR4rs>;
///Register `JOFR4` writer
pub type W = crate::W<JOFR4rs>;
///Field `JOFFSET4` reader - Data offset for injected channel x
pub type JOFFSET4_R = crate::FieldReader<u16>;
///Field `JOFFSET4` writer - Data offset for injected channel x
pub type JOFFSET4_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:11 - Data offset for injected channel x
    #[inline(always)]
    pub fn joffset4(&self) -> JOFFSET4_R {
        JOFFSET4_R::new((self.bits & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JOFR4")
            .field("joffset4", &self.joffset4())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - Data offset for injected channel x
    #[inline(always)]
    pub fn joffset4(&mut self) -> JOFFSET4_W<'_, JOFR4rs> {
        JOFFSET4_W::new(self, 0)
    }
}
/**injected channel data offset register x

You can [`read`](crate::Reg::read) this register and get [`jofr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jofr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#ADC1:JOFR4)*/
pub struct JOFR4rs;
impl crate::RegisterSpec for JOFR4rs {
    type Ux = u32;
}
///`read()` method returns [`jofr4::R`](R) reader structure
impl crate::Readable for JOFR4rs {}
///`write(|w| ..)` method takes [`jofr4::W`](W) writer structure
impl crate::Writable for JOFR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets JOFR4 to value 0
impl crate::Resettable for JOFR4rs {}

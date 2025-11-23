///Register `JOFR2` reader
pub type R = crate::R<JOFR2rs>;
///Register `JOFR2` writer
pub type W = crate::W<JOFR2rs>;
///Field `JOFFSET2` reader - Data offset for injected channel x
pub type JOFFSET2_R = crate::FieldReader<u16>;
///Field `JOFFSET2` writer - Data offset for injected channel x
pub type JOFFSET2_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:11 - Data offset for injected channel x
    #[inline(always)]
    pub fn joffset2(&self) -> JOFFSET2_R {
        JOFFSET2_R::new((self.bits & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JOFR2")
            .field("joffset2", &self.joffset2())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - Data offset for injected channel x
    #[inline(always)]
    pub fn joffset2(&mut self) -> JOFFSET2_W<'_, JOFR2rs> {
        JOFFSET2_W::new(self, 0)
    }
}
/**injected channel data offset register x

You can [`read`](crate::Reg::read) this register and get [`jofr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jofr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#ADC1:JOFR2)*/
pub struct JOFR2rs;
impl crate::RegisterSpec for JOFR2rs {
    type Ux = u32;
}
///`read()` method returns [`jofr2::R`](R) reader structure
impl crate::Readable for JOFR2rs {}
///`write(|w| ..)` method takes [`jofr2::W`](W) writer structure
impl crate::Writable for JOFR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets JOFR2 to value 0
impl crate::Resettable for JOFR2rs {}

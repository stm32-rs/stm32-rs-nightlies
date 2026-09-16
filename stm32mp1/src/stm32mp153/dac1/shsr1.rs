///Register `SHSR1` reader
pub type R = crate::R<SHSR1rs>;
///Register `SHSR1` writer
pub type W = crate::W<SHSR1rs>;
///Field `TSAMPLE1` reader - TSAMPLE1
pub type TSAMPLE1_R = crate::FieldReader<u16>;
///Field `TSAMPLE1` writer - TSAMPLE1
pub type TSAMPLE1_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:9 - TSAMPLE1
    #[inline(always)]
    pub fn tsample1(&self) -> TSAMPLE1_R {
        TSAMPLE1_R::new((self.bits & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHSR1")
            .field("tsample1", &self.tsample1())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - TSAMPLE1
    #[inline(always)]
    pub fn tsample1(&mut self) -> TSAMPLE1_W<'_, SHSR1rs> {
        TSAMPLE1_W::new(self, 0)
    }
}
/**DAC channel 1 sample and hold sample time register

You can [`read`](crate::Reg::read) this register and get [`shsr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shsr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DAC1:SHSR1)*/
pub struct SHSR1rs;
impl crate::RegisterSpec for SHSR1rs {
    type Ux = u32;
}
///`read()` method returns [`shsr1::R`](R) reader structure
impl crate::Readable for SHSR1rs {}
///`write(|w| ..)` method takes [`shsr1::W`](W) writer structure
impl crate::Writable for SHSR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SHSR1 to value 0
impl crate::Resettable for SHSR1rs {}

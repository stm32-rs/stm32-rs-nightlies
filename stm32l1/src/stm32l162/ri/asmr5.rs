///Register `ASMR5` reader
pub type R = crate::R<ASMR5rs>;
///Register `ASMR5` writer
pub type W = crate::W<ASMR5rs>;
///Field `PG` reader - Port G analog switch mode selection
pub type PG_R = crate::FieldReader<u16>;
///Field `PG` writer - Port G analog switch mode selection
pub type PG_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Port G analog switch mode selection
    #[inline(always)]
    pub fn pg(&self) -> PG_R {
        PG_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ASMR5").field("pg", &self.pg()).finish()
    }
}
impl W {
    ///Bits 0:15 - Port G analog switch mode selection
    #[inline(always)]
    pub fn pg(&mut self) -> PG_W<'_, ASMR5rs> {
        PG_W::new(self, 0)
    }
}
/**Analog switch mode register

You can [`read`](crate::Reg::read) this register and get [`asmr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`asmr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L162.html#RI:ASMR5)*/
pub struct ASMR5rs;
impl crate::RegisterSpec for ASMR5rs {
    type Ux = u32;
}
///`read()` method returns [`asmr5::R`](R) reader structure
impl crate::Readable for ASMR5rs {}
///`write(|w| ..)` method takes [`asmr5::W`](W) writer structure
impl crate::Writable for ASMR5rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ASMR5 to value 0
impl crate::Resettable for ASMR5rs {}

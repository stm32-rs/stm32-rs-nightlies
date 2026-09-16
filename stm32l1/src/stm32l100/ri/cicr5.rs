///Register `CICR5` reader
pub type R = crate::R<CICR5rs>;
///Register `CICR5` writer
pub type W = crate::W<CICR5rs>;
///Field `PG` reader - Port G channel identification for capture
pub type PG_R = crate::FieldReader<u16>;
///Field `PG` writer - Port G channel identification for capture
pub type PG_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Port G channel identification for capture
    #[inline(always)]
    pub fn pg(&self) -> PG_R {
        PG_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CICR5").field("pg", &self.pg()).finish()
    }
}
impl W {
    ///Bits 0:15 - Port G channel identification for capture
    #[inline(always)]
    pub fn pg(&mut self) -> PG_W<'_, CICR5rs> {
        PG_W::new(self, 0)
    }
}
/**Channel identification for capture register

You can [`read`](crate::Reg::read) this register and get [`cicr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cicr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#RI:CICR5)*/
pub struct CICR5rs;
impl crate::RegisterSpec for CICR5rs {
    type Ux = u32;
}
///`read()` method returns [`cicr5::R`](R) reader structure
impl crate::Readable for CICR5rs {}
///`write(|w| ..)` method takes [`cicr5::W`](W) writer structure
impl crate::Writable for CICR5rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CICR5 to value 0
impl crate::Resettable for CICR5rs {}

///Register `CMR5` reader
pub type R = crate::R<CMR5rs>;
///Register `CMR5` writer
pub type W = crate::W<CMR5rs>;
///Field `PG` reader - Port G channel masking
pub type PG_R = crate::FieldReader<u16>;
///Field `PG` writer - Port G channel masking
pub type PG_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Port G channel masking
    #[inline(always)]
    pub fn pg(&self) -> PG_R {
        PG_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMR5").field("pg", &self.pg()).finish()
    }
}
impl W {
    ///Bits 0:15 - Port G channel masking
    #[inline(always)]
    pub fn pg(&mut self) -> PG_W<'_, CMR5rs> {
        PG_W::new(self, 0)
    }
}
/**Channel mask register

You can [`read`](crate::Reg::read) this register and get [`cmr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#RI:CMR5)*/
pub struct CMR5rs;
impl crate::RegisterSpec for CMR5rs {
    type Ux = u32;
}
///`read()` method returns [`cmr5::R`](R) reader structure
impl crate::Readable for CMR5rs {}
///`write(|w| ..)` method takes [`cmr5::W`](W) writer structure
impl crate::Writable for CMR5rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CMR5 to value 0
impl crate::Resettable for CMR5rs {}

///Register `ALCC2R` reader
pub type R = crate::R<ALCC2Rrs>;
///Register `ALCC2R` writer
pub type W = crate::W<ALCC2Rrs>;
///Field `LINE` reader - line number Compare value 2 for the absolute line counter.
pub type LINE_R = crate::FieldReader<u16>;
///Field `LINE` writer - line number Compare value 2 for the absolute line counter.
pub type LINE_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:11 - line number Compare value 2 for the absolute line counter.
    #[inline(always)]
    pub fn line(&self) -> LINE_R {
        LINE_R::new((self.bits & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ALCC2R")
            .field("line", &self.line())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - line number Compare value 2 for the absolute line counter.
    #[inline(always)]
    pub fn line(&mut self) -> LINE_W<'_, ALCC2Rrs> {
        LINE_W::new(self, 0)
    }
}
/**GFXTIM absolute line counter compare 2 register

You can [`read`](crate::Reg::read) this register and get [`alcc2r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alcc2r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#GFXTIM:ALCC2R)*/
pub struct ALCC2Rrs;
impl crate::RegisterSpec for ALCC2Rrs {
    type Ux = u32;
}
///`read()` method returns [`alcc2r::R`](R) reader structure
impl crate::Readable for ALCC2Rrs {}
///`write(|w| ..)` method takes [`alcc2r::W`](W) writer structure
impl crate::Writable for ALCC2Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ALCC2R to value 0
impl crate::Resettable for ALCC2Rrs {}

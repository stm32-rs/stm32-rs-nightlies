///Register `ALCC1R` reader
pub type R = crate::R<ALCC1Rrs>;
///Register `ALCC1R` writer
pub type W = crate::W<ALCC1Rrs>;
///Field `LINE` reader - line number
pub type LINE_R = crate::FieldReader<u16>;
///Field `LINE` writer - line number
pub type LINE_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:11 - line number
    #[inline(always)]
    pub fn line(&self) -> LINE_R {
        LINE_R::new((self.bits & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ALCC1R")
            .field("line", &self.line())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - line number
    #[inline(always)]
    pub fn line(&mut self) -> LINE_W<'_, ALCC1Rrs> {
        LINE_W::new(self, 0)
    }
}
/**GFXTIM absolute line counter compare 1 register

You can [`read`](crate::Reg::read) this register and get [`alcc1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alcc1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#GFXTIM:ALCC1R)*/
pub struct ALCC1Rrs;
impl crate::RegisterSpec for ALCC1Rrs {
    type Ux = u32;
}
///`read()` method returns [`alcc1r::R`](R) reader structure
impl crate::Readable for ALCC1Rrs {}
///`write(|w| ..)` method takes [`alcc1r::W`](W) writer structure
impl crate::Writable for ALCC1Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ALCC1R to value 0
impl crate::Resettable for ALCC1Rrs {}

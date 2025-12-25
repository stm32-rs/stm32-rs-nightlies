///Register `ALCR` reader
pub type R = crate::R<ALCRrs>;
///Register `ALCR` writer
pub type W = crate::W<ALCRrs>;
///Field `LINE` reader - line number Current value of the absolute line counter. Note: This field can only be written when the absolute frame counter is disabled.
pub type LINE_R = crate::FieldReader<u16>;
///Field `LINE` writer - line number Current value of the absolute line counter. Note: This field can only be written when the absolute frame counter is disabled.
pub type LINE_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:11 - line number Current value of the absolute line counter. Note: This field can only be written when the absolute frame counter is disabled.
    #[inline(always)]
    pub fn line(&self) -> LINE_R {
        LINE_R::new((self.bits & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ALCR").field("line", &self.line()).finish()
    }
}
impl W {
    ///Bits 0:11 - line number Current value of the absolute line counter. Note: This field can only be written when the absolute frame counter is disabled.
    #[inline(always)]
    pub fn line(&mut self) -> LINE_W<'_, ALCRrs> {
        LINE_W::new(self, 0)
    }
}
/**GFXTIM absolute line counter register

You can [`read`](crate::Reg::read) this register and get [`alcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#GFXTIM:ALCR)*/
pub struct ALCRrs;
impl crate::RegisterSpec for ALCRrs {
    type Ux = u32;
}
///`read()` method returns [`alcr::R`](R) reader structure
impl crate::Readable for ALCRrs {}
///`write(|w| ..)` method takes [`alcr::W`](W) writer structure
impl crate::Writable for ALCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ALCR to value 0
impl crate::Resettable for ALCRrs {}

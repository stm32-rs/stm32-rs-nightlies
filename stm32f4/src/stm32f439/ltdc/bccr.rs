///Register `BCCR` reader
pub type R = crate::R<BCCRrs>;
///Register `BCCR` writer
pub type W = crate::W<BCCRrs>;
///Field `BC` reader - Background Color Red value
pub type BC_R = crate::FieldReader<u32>;
///Field `BC` writer - Background Color Red value
pub type BC_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    ///Bits 0:23 - Background Color Red value
    #[inline(always)]
    pub fn bc(&self) -> BC_R {
        BC_R::new(self.bits & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BCCR").field("bc", &self.bc()).finish()
    }
}
impl W {
    ///Bits 0:23 - Background Color Red value
    #[inline(always)]
    pub fn bc(&mut self) -> BC_W<'_, BCCRrs> {
        BC_W::new(self, 0)
    }
}
/**Background Color Configuration Register

You can [`read`](crate::Reg::read) this register and get [`bccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#LTDC:BCCR)*/
pub struct BCCRrs;
impl crate::RegisterSpec for BCCRrs {
    type Ux = u32;
}
///`read()` method returns [`bccr::R`](R) reader structure
impl crate::Readable for BCCRrs {}
///`write(|w| ..)` method takes [`bccr::W`](W) writer structure
impl crate::Writable for BCCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BCCR to value 0
impl crate::Resettable for BCCRrs {}

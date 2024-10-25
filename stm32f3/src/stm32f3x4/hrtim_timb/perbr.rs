///Register `PERBR` reader
pub type R = crate::R<PERBRrs>;
///Register `PERBR` writer
pub type W = crate::W<PERBRrs>;
///Field `PERx` reader - Timerx Period value
pub type PERX_R = crate::FieldReader<u16>;
///Field `PERx` writer - Timerx Period value
pub type PERX_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    ///Bits 0:15 - Timerx Period value
    #[inline(always)]
    pub fn perx(&self) -> PERX_R {
        PERX_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERBR").field("perx", &self.perx()).finish()
    }
}
impl W {
    ///Bits 0:15 - Timerx Period value
    #[inline(always)]
    #[must_use]
    pub fn perx(&mut self) -> PERX_W<PERBRrs> {
        PERX_W::new(self, 0)
    }
}
/**Timerx Period Register

You can [`read`](crate::Reg::read) this register and get [`perbr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perbr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMB:PERBR)*/
pub struct PERBRrs;
impl crate::RegisterSpec for PERBRrs {
    type Ux = u32;
}
///`read()` method returns [`perbr::R`](R) reader structure
impl crate::Readable for PERBRrs {}
///`write(|w| ..)` method takes [`perbr::W`](W) writer structure
impl crate::Writable for PERBRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PERBR to value 0xffff
impl crate::Resettable for PERBRrs {
    const RESET_VALUE: u32 = 0xffff;
}

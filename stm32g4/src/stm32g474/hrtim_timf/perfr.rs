///Register `PERFR` reader
pub type R = crate::R<PERFRrs>;
///Register `PERFR` writer
pub type W = crate::W<PERFRrs>;
///Field `PERx` reader - Timerx Period value
pub type PERX_R = crate::FieldReader<u16>;
///Field `PERx` writer - Timerx Period value
pub type PERX_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Timerx Period value
    #[inline(always)]
    pub fn perx(&self) -> PERX_R {
        PERX_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERFR").field("perx", &self.perx()).finish()
    }
}
impl W {
    ///Bits 0:15 - Timerx Period value
    #[inline(always)]
    #[must_use]
    pub fn perx(&mut self) -> PERX_W<PERFRrs> {
        PERX_W::new(self, 0)
    }
}
/**Timerx Period Register

You can [`read`](crate::Reg::read) this register and get [`perfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474xx.html#HRTIM_TIMF:PERFR)*/
pub struct PERFRrs;
impl crate::RegisterSpec for PERFRrs {
    type Ux = u32;
}
///`read()` method returns [`perfr::R`](R) reader structure
impl crate::Readable for PERFRrs {}
///`write(|w| ..)` method takes [`perfr::W`](W) writer structure
impl crate::Writable for PERFRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PERFR to value 0xffff
impl crate::Resettable for PERFRrs {
    const RESET_VALUE: u32 = 0xffff;
}

///Register `TIM15_RCR` reader
pub type R = crate::R<TIM15_RCRrs>;
///Register `TIM15_RCR` writer
pub type W = crate::W<TIM15_RCRrs>;
///Field `REP` reader - Repetition counter value
pub type REP_R = crate::FieldReader;
///Field `REP` writer - Repetition counter value
pub type REP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Repetition counter value
    #[inline(always)]
    pub fn rep(&self) -> REP_R {
        REP_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM15_RCR")
            .field("rep", &self.rep())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Repetition counter value
    #[inline(always)]
    pub fn rep(&mut self) -> REP_W<'_, TIM15_RCRrs> {
        REP_W::new(self, 0)
    }
}
/**TIM15 repetition counter register

You can [`read`](crate::Reg::read) this register and get [`tim15_rcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim15_rcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM15:TIM15_RCR)*/
pub struct TIM15_RCRrs;
impl crate::RegisterSpec for TIM15_RCRrs {
    type Ux = u16;
}
///`read()` method returns [`tim15_rcr::R`](R) reader structure
impl crate::Readable for TIM15_RCRrs {}
///`write(|w| ..)` method takes [`tim15_rcr::W`](W) writer structure
impl crate::Writable for TIM15_RCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM15_RCR to value 0
impl crate::Resettable for TIM15_RCRrs {}

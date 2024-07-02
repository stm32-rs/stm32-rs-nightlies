///Register `TIM1_RCR` reader
pub type R = crate::R<TIM1_RCRrs>;
///Register `TIM1_RCR` writer
pub type W = crate::W<TIM1_RCRrs>;
///Field `REP` reader - REP
pub type REP_R = crate::FieldReader<u16>;
///Field `REP` writer - REP
pub type REP_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - REP
    #[inline(always)]
    pub fn rep(&self) -> REP_R {
        REP_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM1_RCR")
            .field("rep", &self.rep())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - REP
    #[inline(always)]
    #[must_use]
    pub fn rep(&mut self) -> REP_W<TIM1_RCRrs> {
        REP_W::new(self, 0)
    }
}
/**TIM1 repetition counter register

You can [`read`](crate::Reg::read) this register and get [`tim1_rcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_rcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM1:TIM1_RCR)*/
pub struct TIM1_RCRrs;
impl crate::RegisterSpec for TIM1_RCRrs {
    type Ux = u16;
}
///`read()` method returns [`tim1_rcr::R`](R) reader structure
impl crate::Readable for TIM1_RCRrs {}
///`write(|w| ..)` method takes [`tim1_rcr::W`](W) writer structure
impl crate::Writable for TIM1_RCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
///`reset()` method sets TIM1_RCR to value 0
impl crate::Resettable for TIM1_RCRrs {
    const RESET_VALUE: u16 = 0;
}

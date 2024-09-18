///Register `TIM2_RCR` reader
pub type R = crate::R<TIM2_RCRrs>;
///Register `TIM2_RCR` writer
pub type W = crate::W<TIM2_RCRrs>;
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
        f.debug_struct("TIM2_RCR")
            .field("rep", &self.rep())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - REP
    #[inline(always)]
    #[must_use]
    pub fn rep(&mut self) -> REP_W<TIM2_RCRrs> {
        REP_W::new(self, 0)
    }
}
/**TIM2 repetition counter register

You can [`read`](crate::Reg::read) this register and get [`tim2_rcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_rcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM2:TIM2_RCR)*/
pub struct TIM2_RCRrs;
impl crate::RegisterSpec for TIM2_RCRrs {
    type Ux = u16;
}
///`read()` method returns [`tim2_rcr::R`](R) reader structure
impl crate::Readable for TIM2_RCRrs {}
///`write(|w| ..)` method takes [`tim2_rcr::W`](W) writer structure
impl crate::Writable for TIM2_RCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
///`reset()` method sets TIM2_RCR to value 0
impl crate::Resettable for TIM2_RCRrs {
    const RESET_VALUE: u16 = 0;
}

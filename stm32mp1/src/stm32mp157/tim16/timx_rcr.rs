///Register `TIMx_RCR` reader
pub type R = crate::R<TIMX_RCRrs>;
///Register `TIMx_RCR` writer
pub type W = crate::W<TIMX_RCRrs>;
///Field `REP` reader - REP
pub type REP_R = crate::FieldReader;
///Field `REP` writer - REP
pub type REP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - REP
    #[inline(always)]
    pub fn rep(&self) -> REP_R {
        REP_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMx_RCR")
            .field("rep", &self.rep())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - REP
    #[inline(always)]
    #[must_use]
    pub fn rep(&mut self) -> REP_W<TIMX_RCRrs> {
        REP_W::new(self, 0)
    }
}
/**TIM16/TIM17 repetition counter register

You can [`read`](crate::Reg::read) this register and get [`timx_rcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timx_rcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM16:TIMx_RCR)*/
pub struct TIMX_RCRrs;
impl crate::RegisterSpec for TIMX_RCRrs {
    type Ux = u16;
}
///`read()` method returns [`timx_rcr::R`](R) reader structure
impl crate::Readable for TIMX_RCRrs {}
///`write(|w| ..)` method takes [`timx_rcr::W`](W) writer structure
impl crate::Writable for TIMX_RCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
///`reset()` method sets TIMx_RCR to value 0
impl crate::Resettable for TIMX_RCRrs {
    const RESET_VALUE: u16 = 0;
}

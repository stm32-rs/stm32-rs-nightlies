///Register `TIM16_RCR` reader
pub type R = crate::R<TIM16_RCRrs>;
///Register `TIM16_RCR` writer
pub type W = crate::W<TIM16_RCRrs>;
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
        f.debug_struct("TIM16_RCR")
            .field("rep", &self.rep())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Repetition counter value
    #[inline(always)]
    pub fn rep(&mut self) -> REP_W<TIM16_RCRrs> {
        REP_W::new(self, 0)
    }
}
/**TIM16 repetition counter register

You can [`read`](crate::Reg::read) this register and get [`tim16_rcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim16_rcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#TIM16:TIM16_RCR)*/
pub struct TIM16_RCRrs;
impl crate::RegisterSpec for TIM16_RCRrs {
    type Ux = u16;
}
///`read()` method returns [`tim16_rcr::R`](R) reader structure
impl crate::Readable for TIM16_RCRrs {}
///`write(|w| ..)` method takes [`tim16_rcr::W`](W) writer structure
impl crate::Writable for TIM16_RCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
///`reset()` method sets TIM16_RCR to value 0
impl crate::Resettable for TIM16_RCRrs {
    const RESET_VALUE: u16 = 0;
}

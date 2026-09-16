///Register `_RCR` reader
pub type R = crate::R<_RCRrs>;
///Register `_RCR` writer
pub type W = crate::W<_RCRrs>;
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
        f.debug_struct("_RCR").field("rep", &self.rep()).finish()
    }
}
impl W {
    ///Bits 0:7 - REP
    #[inline(always)]
    pub fn rep(&mut self) -> REP_W<'_, _RCRrs> {
        REP_W::new(self, 0)
    }
}
/**TIM15 repetition counter register

You can [`read`](crate::Reg::read) this register and get [`_rcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_rcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM15:_RCR)*/
pub struct _RCRrs;
impl crate::RegisterSpec for _RCRrs {
    type Ux = u16;
}
///`read()` method returns [`_rcr::R`](R) reader structure
impl crate::Readable for _RCRrs {}
///`write(|w| ..)` method takes [`_rcr::W`](W) writer structure
impl crate::Writable for _RCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets _RCR to value 0
impl crate::Resettable for _RCRrs {}

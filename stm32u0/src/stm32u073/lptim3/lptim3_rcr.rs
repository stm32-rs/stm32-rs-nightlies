///Register `LPTIM3_RCR` reader
pub type R = crate::R<LPTIM3_RCRrs>;
///Register `LPTIM3_RCR` writer
pub type W = crate::W<LPTIM3_RCRrs>;
///Field `REP` reader - Repetition register value REP is the repetition value for the LPTIM.
pub type REP_R = crate::FieldReader;
///Field `REP` writer - Repetition register value REP is the repetition value for the LPTIM.
pub type REP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Repetition register value REP is the repetition value for the LPTIM.
    #[inline(always)]
    pub fn rep(&self) -> REP_R {
        REP_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPTIM3_RCR")
            .field("rep", &self.rep())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Repetition register value REP is the repetition value for the LPTIM.
    #[inline(always)]
    pub fn rep(&mut self) -> REP_W<LPTIM3_RCRrs> {
        REP_W::new(self, 0)
    }
}
/**LPTIM repetition register

You can [`read`](crate::Reg::read) this register and get [`lptim3_rcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim3_rcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LPTIM3:LPTIM3_RCR)*/
pub struct LPTIM3_RCRrs;
impl crate::RegisterSpec for LPTIM3_RCRrs {
    type Ux = u32;
}
///`read()` method returns [`lptim3_rcr::R`](R) reader structure
impl crate::Readable for LPTIM3_RCRrs {}
///`write(|w| ..)` method takes [`lptim3_rcr::W`](W) writer structure
impl crate::Writable for LPTIM3_RCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LPTIM3_RCR to value 0
impl crate::Resettable for LPTIM3_RCRrs {
    const RESET_VALUE: u32 = 0;
}

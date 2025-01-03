///Register `LPTIM2_RCR` reader
pub type R = crate::R<LPTIM2_RCRrs>;
///Register `LPTIM2_RCR` writer
pub type W = crate::W<LPTIM2_RCRrs>;
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
        f.debug_struct("LPTIM2_RCR")
            .field("rep", &self.rep())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Repetition register value REP is the repetition value for the LPTIM.
    #[inline(always)]
    pub fn rep(&mut self) -> REP_W<LPTIM2_RCRrs> {
        REP_W::new(self, 0)
    }
}
/**LPTIM repetition register

You can [`read`](crate::Reg::read) this register and get [`lptim2_rcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim2_rcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#LPTIM2:LPTIM2_RCR)*/
pub struct LPTIM2_RCRrs;
impl crate::RegisterSpec for LPTIM2_RCRrs {
    type Ux = u32;
}
///`read()` method returns [`lptim2_rcr::R`](R) reader structure
impl crate::Readable for LPTIM2_RCRrs {}
///`write(|w| ..)` method takes [`lptim2_rcr::W`](W) writer structure
impl crate::Writable for LPTIM2_RCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LPTIM2_RCR to value 0
impl crate::Resettable for LPTIM2_RCRrs {
    const RESET_VALUE: u32 = 0;
}

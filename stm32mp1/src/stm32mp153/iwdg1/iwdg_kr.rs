///Register `IWDG_KR` writer
pub type W = crate::W<IWDG_KRrs>;
///Field `KEY` writer - KEY
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl core::fmt::Debug for crate::generic::Reg<IWDG_KRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:15 - KEY
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<IWDG_KRrs> {
        KEY_W::new(self, 0)
    }
}
/**Key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iwdg_kr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#IWDG1:IWDG_KR)*/
pub struct IWDG_KRrs;
impl crate::RegisterSpec for IWDG_KRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`iwdg_kr::W`](W) writer structure
impl crate::Writable for IWDG_KRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IWDG_KR to value 0
impl crate::Resettable for IWDG_KRrs {
    const RESET_VALUE: u32 = 0;
}

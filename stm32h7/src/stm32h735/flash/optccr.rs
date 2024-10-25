///Register `OPTCCR` writer
pub type W = crate::W<OPTCCRrs>;
///Field `CLR_OPTCHANGEERR` writer - OPTCHANGEERR reset bit
pub type CLR_OPTCHANGEERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<OPTCCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 30 - OPTCHANGEERR reset bit
    #[inline(always)]
    #[must_use]
    pub fn clr_optchangeerr(&mut self) -> CLR_OPTCHANGEERR_W<OPTCCRrs> {
        CLR_OPTCHANGEERR_W::new(self, 30)
    }
}
/**FLASH option clear control register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optccr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H73x.html#FLASH:OPTCCR)*/
pub struct OPTCCRrs;
impl crate::RegisterSpec for OPTCCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`optccr::W`](W) writer structure
impl crate::Writable for OPTCCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OPTCCR to value 0
impl crate::Resettable for OPTCCRrs {
    const RESET_VALUE: u32 = 0;
}

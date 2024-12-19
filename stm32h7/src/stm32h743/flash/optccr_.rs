///Register `OPTCCR_` writer
pub type W = crate::W<OPTCCR_rs>;
///Field `CLR_OPTCHANGEERR` writer - OPTCHANGEERR reset bit
pub type CLR_OPTCHANGEERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<OPTCCR_rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 30 - OPTCHANGEERR reset bit
    #[inline(always)]
    pub fn clr_optchangeerr(&mut self) -> CLR_OPTCHANGEERR_W<OPTCCR_rs> {
        CLR_OPTCHANGEERR_W::new(self, 30)
    }
}
/**FLASH option clear control register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optccr_::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#FLASH:OPTCCR_)*/
pub struct OPTCCR_rs;
impl crate::RegisterSpec for OPTCCR_rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`optccr_::W`](W) writer structure
impl crate::Writable for OPTCCR_rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OPTCCR_ to value 0
impl crate::Resettable for OPTCCR_rs {
    const RESET_VALUE: u32 = 0;
}

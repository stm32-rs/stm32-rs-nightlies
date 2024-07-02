///Register `DDRPERFM_CCR` writer
pub type W = crate::W<DDRPERFM_CCRrs>;
///Field `CCLR` writer - CCLR
pub type CCLR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TCLR` writer - TCLR
pub type TCLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<DDRPERFM_CCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:3 - CCLR
    #[inline(always)]
    #[must_use]
    pub fn cclr(&mut self) -> CCLR_W<DDRPERFM_CCRrs> {
        CCLR_W::new(self, 0)
    }
    ///Bit 31 - TCLR
    #[inline(always)]
    #[must_use]
    pub fn tclr(&mut self) -> TCLR_W<DDRPERFM_CCRrs> {
        TCLR_W::new(self, 31)
    }
}
/**Write-only register. A read request returns all zeros

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrperfm_ccr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPERFM:DDRPERFM_CCR)*/
pub struct DDRPERFM_CCRrs;
impl crate::RegisterSpec for DDRPERFM_CCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ddrperfm_ccr::W`](W) writer structure
impl crate::Writable for DDRPERFM_CCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DDRPERFM_CCR to value 0
impl crate::Resettable for DDRPERFM_CCRrs {
    const RESET_VALUE: u32 = 0;
}

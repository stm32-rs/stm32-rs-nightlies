///Register `DDRPERFM_ICR` writer
pub type W = crate::W<DDRPERFM_ICRrs>;
///Field `OVF` writer - OVF
pub type OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<DDRPERFM_ICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - OVF
    #[inline(always)]
    #[must_use]
    pub fn ovf(&mut self) -> OVF_W<DDRPERFM_ICRrs> {
        OVF_W::new(self, 0)
    }
}
/**Write-only register. A read request returns all zeros

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrperfm_icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPERFM:DDRPERFM_ICR)*/
pub struct DDRPERFM_ICRrs;
impl crate::RegisterSpec for DDRPERFM_ICRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ddrperfm_icr::W`](W) writer structure
impl crate::Writable for DDRPERFM_ICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DDRPERFM_ICR to value 0
impl crate::Resettable for DDRPERFM_ICRrs {
    const RESET_VALUE: u32 = 0;
}

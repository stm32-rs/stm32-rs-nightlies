///Register `CRYP_K1RR` writer
pub type W = crate::W<CRYP_K1RRrs>;
///Field `K` writer - K
pub type K_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<CRYP_K1RRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - K
    #[inline(always)]
    #[must_use]
    pub fn k(&mut self) -> K_W<CRYP_K1RRrs> {
        K_W::new(self, 0)
    }
}
/**Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details.

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cryp_k1rr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#CRYP1:CRYP_K1RR)*/
pub struct CRYP_K1RRrs;
impl crate::RegisterSpec for CRYP_K1RRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`cryp_k1rr::W`](W) writer structure
impl crate::Writable for CRYP_K1RRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CRYP_K1RR to value 0
impl crate::Resettable for CRYP_K1RRrs {
    const RESET_VALUE: u32 = 0;
}

///Register `K1RR` writer
pub type W = crate::W<K1RRrs>;
///Field `K` writer - K
pub type K_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<K1RRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - K
    #[inline(always)]
    pub fn k(&mut self) -> K_W<'_, K1RRrs> {
        K_W::new(self, 0)
    }
}
/**Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details.

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`k1rr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#CRYP1:K1RR)*/
pub struct K1RRrs;
impl crate::RegisterSpec for K1RRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`k1rr::W`](W) writer structure
impl crate::Writable for K1RRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets K1RR to value 0
impl crate::Resettable for K1RRrs {}

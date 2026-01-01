///Register `K0LR` writer
pub type W = crate::W<K0LRrs>;
///Field `b2` writer - b224
pub type B2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<K0LRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - b224
    #[inline(always)]
    pub fn b2(&mut self) -> B2_W<'_, K0LRrs> {
        B2_W::new(self, 0)
    }
}
/**key registers

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`k0lr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B0.html#CRYP:K0LR)*/
pub struct K0LRrs;
impl crate::RegisterSpec for K0LRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`k0lr::W`](W) writer structure
impl crate::Writable for K0LRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets K0LR to value 0
impl crate::Resettable for K0LRrs {}

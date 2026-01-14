///Register `K1LR` writer
pub type W = crate::W<K1LRrs>;
///Field `b1` writer - b160
pub type B1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<K1LRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - b160
    #[inline(always)]
    pub fn b1(&mut self) -> B1_W<'_, K1LRrs> {
        B1_W::new(self, 0)
    }
}
/**key registers

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`k1lr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM7.html#CRYP:K1LR)*/
pub struct K1LRrs;
impl crate::RegisterSpec for K1LRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`k1lr::W`](W) writer structure
impl crate::Writable for K1LRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets K1LR to value 0
impl crate::Resettable for K1LRrs {}

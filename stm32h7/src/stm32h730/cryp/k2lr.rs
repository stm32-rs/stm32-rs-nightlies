///Register `K2LR` writer
pub type W = crate::W<K2LRrs>;
///Field `k` writer - k96
pub type K_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
///Field `b121` writer - b121
pub type B121_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<K2LRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:30 - k96
    #[inline(always)]
    pub fn k(&mut self) -> K_W<'_, K2LRrs> {
        K_W::new(self, 0)
    }
    ///Bit 25 - b121
    #[inline(always)]
    pub fn b121(&mut self) -> B121_W<'_, K2LRrs> {
        B121_W::new(self, 25)
    }
}
/**key registers

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`k2lr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#CRYP:K2LR)*/
pub struct K2LRrs;
impl crate::RegisterSpec for K2LRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`k2lr::W`](W) writer structure
impl crate::Writable for K2LRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets K2LR to value 0
impl crate::Resettable for K2LRrs {}

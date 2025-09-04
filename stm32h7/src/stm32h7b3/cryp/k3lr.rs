///Register `K3LR` writer
pub type W = crate::W<K3LRrs>;
///Field `b` writer - b32
pub type B_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<K3LRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - b32
    #[inline(always)]
    pub fn b(&mut self) -> B_W<K3LRrs> {
        B_W::new(self, 0)
    }
}
/**key registers

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`k3lr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#CRYP:K3LR)*/
pub struct K3LRrs;
impl crate::RegisterSpec for K3LRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`k3lr::W`](W) writer structure
impl crate::Writable for K3LRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets K3LR to value 0
impl crate::Resettable for K3LRrs {}

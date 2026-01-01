///Register `K0RR` writer
pub type W = crate::W<K0RRrs>;
///Field `b` writer - b192
pub type B_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<K0RRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - b192
    #[inline(always)]
    pub fn b(&mut self) -> B_W<'_, K0RRrs> {
        B_W::new(self, 0)
    }
}
/**key registers

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`k0rr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#CRYP:K0RR)*/
pub struct K0RRrs;
impl crate::RegisterSpec for K0RRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`k0rr::W`](W) writer structure
impl crate::Writable for K0RRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets K0RR to value 0
impl crate::Resettable for K0RRrs {}

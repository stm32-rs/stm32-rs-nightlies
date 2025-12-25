///Register `K2RR` writer
pub type W = crate::W<K2RRrs>;
///Field `b` writer - b64
pub type B_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<K2RRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - b64
    #[inline(always)]
    pub fn b(&mut self) -> B_W<'_, K2RRrs> {
        B_W::new(self, 0)
    }
}
/**key registers

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`k2rr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#CRYP:K2RR)*/
pub struct K2RRrs;
impl crate::RegisterSpec for K2RRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`k2rr::W`](W) writer structure
impl crate::Writable for K2RRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets K2RR to value 0
impl crate::Resettable for K2RRrs {}

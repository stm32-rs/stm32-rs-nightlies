///Register `R3KEYR0` writer
pub type W = crate::W<R3KEYR0rs>;
///Field `REG_KEY` writer - Region key, bits \[31:0\]
pub type REG_KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<R3KEYR0rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - Region key, bits \[31:0\]
    #[inline(always)]
    pub fn reg_key(&mut self) -> REG_KEY_W<'_, R3KEYR0rs> {
        REG_KEY_W::new(self, 0)
    }
}
/**OTFDEC region 3 key register 0

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r3keyr0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#OTFDEC1:R3KEYR0)*/
pub struct R3KEYR0rs;
impl crate::RegisterSpec for R3KEYR0rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`r3keyr0::W`](W) writer structure
impl crate::Writable for R3KEYR0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets R3KEYR0 to value 0
impl crate::Resettable for R3KEYR0rs {}

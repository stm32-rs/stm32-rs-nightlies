///Register `CC2KEYR0` writer
pub type W = crate::W<CC2KEYR0rs>;
///Field `KEY` writer - cipher key, bits \[31:0\]
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<CC2KEYR0rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - cipher key, bits \[31:0\]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W<'_, CC2KEYR0rs> {
        KEY_W::new(self, 0)
    }
}
/**MCE cipher context 2 key register 0

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc2keyr0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MCE1:CC2KEYR0)*/
pub struct CC2KEYR0rs;
impl crate::RegisterSpec for CC2KEYR0rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`cc2keyr0::W`](W) writer structure
impl crate::Writable for CC2KEYR0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CC2KEYR0 to value 0
impl crate::Resettable for CC2KEYR0rs {}

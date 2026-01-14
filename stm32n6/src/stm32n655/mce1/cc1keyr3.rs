///Register `CC1KEYR3` writer
pub type W = crate::W<CC1KEYR3rs>;
///Field `KEY` writer - cipher key, bits \[127:96\]
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<CC1KEYR3rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - cipher key, bits \[127:96\]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W<'_, CC1KEYR3rs> {
        KEY_W::new(self, 0)
    }
}
/**MCE cipher context 1 key register 3

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc1keyr3::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:CC1KEYR3)*/
pub struct CC1KEYR3rs;
impl crate::RegisterSpec for CC1KEYR3rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`cc1keyr3::W`](W) writer structure
impl crate::Writable for CC1KEYR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CC1KEYR3 to value 0
impl crate::Resettable for CC1KEYR3rs {}

///Register `R2KEYR3` writer
pub type W = crate::W<R2KEYR3rs>;
///Field `REGx_KEY` writer - Region key, bits \[127:96\] Refer to the OTFDEC_RxKEYR0 register for description of the KEY\[127:0\] bitfield.
pub type REGX_KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<R2KEYR3rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - Region key, bits \[127:96\] Refer to the OTFDEC_RxKEYR0 register for description of the KEY\[127:0\] bitfield.
    #[inline(always)]
    pub fn regx_key(&mut self) -> REGX_KEY_W<'_, R2KEYR3rs> {
        REGX_KEY_W::new(self, 0)
    }
}
/**OTFDEC region 2 key register 3

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r2keyr3::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#OTFDEC1:R2KEYR3)*/
pub struct R2KEYR3rs;
impl crate::RegisterSpec for R2KEYR3rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`r2keyr3::W`](W) writer structure
impl crate::Writable for R2KEYR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets R2KEYR3 to value 0
impl crate::Resettable for R2KEYR3rs {}

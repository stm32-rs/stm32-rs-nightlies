///Register `R4KEYR2` writer
pub type W = crate::W<R4KEYR2rs>;
///Field `REGx_KEY` writer - Region key, bits \[95:64\] Refer to the OTFDEC_RxKEYR0 register for description of the KEY\[127:0\] bitfield.
pub type REGX_KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<R4KEYR2rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - Region key, bits \[95:64\] Refer to the OTFDEC_RxKEYR0 register for description of the KEY\[127:0\] bitfield.
    #[inline(always)]
    pub fn regx_key(&mut self) -> REGX_KEY_W<'_, R4KEYR2rs> {
        REGX_KEY_W::new(self, 0)
    }
}
/**OTFDEC region 4 key register 2

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r4keyr2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#OTFDEC1:R4KEYR2)*/
pub struct R4KEYR2rs;
impl crate::RegisterSpec for R4KEYR2rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`r4keyr2::W`](W) writer structure
impl crate::Writable for R4KEYR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets R4KEYR2 to value 0
impl crate::Resettable for R4KEYR2rs {}

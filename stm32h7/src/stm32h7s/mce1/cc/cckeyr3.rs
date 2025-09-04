///Register `CCKEYR3` writer
pub type W = crate::W<CCKEYR3rs>;
///Field `KEY` writer - cipher key, bits \[127:96\] Refer to the MCE_CCzKEYR0 register for description of the KEY\[127:0\] bitfield.
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<CCKEYR3rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - cipher key, bits \[127:96\] Refer to the MCE_CCzKEYR0 register for description of the KEY\[127:0\] bitfield.
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W<CCKEYR3rs> {
        KEY_W::new(self, 0)
    }
}
/**MCE cipher context 1 key register 3

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cckeyr3::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CCKEYR3rs;
impl crate::RegisterSpec for CCKEYR3rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`cckeyr3::W`](W) writer structure
impl crate::Writable for CCKEYR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCKEYR3 to value 0
impl crate::Resettable for CCKEYR3rs {}

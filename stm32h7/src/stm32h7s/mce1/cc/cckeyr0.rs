///Register `CCKEYR0` writer
pub type W = crate::W<CCKEYR0rs>;
///Field `KEY` writer - cipher key, bits \[31:0\] This register is used by the block or stream cipher of MCE when CTXID = z in encrypted region configuration register. KEY\[127:0\] must be correctly initialize before CCEN bit is set in MCE_CCzCFGR register.
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<CCKEYR0rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - cipher key, bits \[31:0\] This register is used by the block or stream cipher of MCE when CTXID = z in encrypted region configuration register. KEY\[127:0\] must be correctly initialize before CCEN bit is set in MCE_CCzCFGR register.
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W<'_, CCKEYR0rs> {
        KEY_W::new(self, 0)
    }
}
/**MCE cipher context 1 key register 0

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cckeyr0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CCKEYR0rs;
impl crate::RegisterSpec for CCKEYR0rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`cckeyr0::W`](W) writer structure
impl crate::Writable for CCKEYR0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCKEYR0 to value 0
impl crate::Resettable for CCKEYR0rs {}

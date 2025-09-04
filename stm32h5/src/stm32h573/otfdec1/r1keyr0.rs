///Register `R1KEYR0` writer
pub type W = crate::W<R1KEYR0rs>;
///Field `REGx_KEY` writer - Region key, bits \[31:0\] This register must be written before the region corresponding REG_EN bit in OTFDEC_RxCFGR is set. Reading this register returns a zero value. Writing to this register is discarded if performed while the region CONFIGLOCK or KEYLOCK bit is set in the OTFDEC_RxCFGR. Note: When application successfully changes MODE bits in OTFDEC_RxCFGR and OTFDEC_RxKEYR, and associated KEYCRC are erased.
pub type REGX_KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<R1KEYR0rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - Region key, bits \[31:0\] This register must be written before the region corresponding REG_EN bit in OTFDEC_RxCFGR is set. Reading this register returns a zero value. Writing to this register is discarded if performed while the region CONFIGLOCK or KEYLOCK bit is set in the OTFDEC_RxCFGR. Note: When application successfully changes MODE bits in OTFDEC_RxCFGR and OTFDEC_RxKEYR, and associated KEYCRC are erased.
    #[inline(always)]
    pub fn regx_key(&mut self) -> REGX_KEY_W<R1KEYR0rs> {
        REGX_KEY_W::new(self, 0)
    }
}
/**OTFDEC region 1 key register 0

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r1keyr0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#OTFDEC1:R1KEYR0)*/
pub struct R1KEYR0rs;
impl crate::RegisterSpec for R1KEYR0rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`r1keyr0::W`](W) writer structure
impl crate::Writable for R1KEYR0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets R1KEYR0 to value 0
impl crate::Resettable for R1KEYR0rs {}

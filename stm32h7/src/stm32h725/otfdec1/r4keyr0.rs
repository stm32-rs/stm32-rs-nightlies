///Register `R4KEYR0` writer
pub type W = crate::W<R4KEYR0rs>;
///Field `REGx_KEY` writer - Region key, bits \[31:0\] This register must be written before the region corresponding REG_EN bit in the RxCFGR register is set. Reading this register returns a zero value. Writing this register while the region CONFIGLOCK or KEYLOCK bit is set in the RxCFGR register will be discarded. Note: When application successfully changes MODE bits in RxCFGR register RxKEYR registers and associated KEYCRC are erased.
pub type REGX_KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<R4KEYR0rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - Region key, bits \[31:0\] This register must be written before the region corresponding REG_EN bit in the RxCFGR register is set. Reading this register returns a zero value. Writing this register while the region CONFIGLOCK or KEYLOCK bit is set in the RxCFGR register will be discarded. Note: When application successfully changes MODE bits in RxCFGR register RxKEYR registers and associated KEYCRC are erased.
    #[inline(always)]
    pub fn regx_key(&mut self) -> REGX_KEY_W<R4KEYR0rs> {
        REGX_KEY_W::new(self, 0)
    }
}
/**OTFDEC region 4 key register 0

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r4keyr0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H725.html#OTFDEC1:R4KEYR0)*/
pub struct R4KEYR0rs;
impl crate::RegisterSpec for R4KEYR0rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`r4keyr0::W`](W) writer structure
impl crate::Writable for R4KEYR0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets R4KEYR0 to value 0
impl crate::Resettable for R4KEYR0rs {}

///Register `OTFDEC_R1KEYR0` writer
pub type W = crate::W<OTFDEC_R1KEYR0rs>;
/**Field `REGx_KEY` writer - Region key, bits \[31:0\]
This register must be written before the region corresponding REG_EN bit in the RxCFGR register is set. Reading this register returns a zero value. Writing this register while the region CONFIGLOCK or KEYLOCK bit is set in the RxCFGR register will be discarded. Note: When application successfully changes MODE bits in RxCFGR register RxKEYR registers and associated KEYCRC are erased.*/
pub type REGX_KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<OTFDEC_R1KEYR0rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    /**Bits 0:31 - Region key, bits \[31:0\]
    This register must be written before the region corresponding REG_EN bit in the RxCFGR register is set. Reading this register returns a zero value. Writing this register while the region CONFIGLOCK or KEYLOCK bit is set in the RxCFGR register will be discarded. Note: When application successfully changes MODE bits in RxCFGR register RxKEYR registers and associated KEYCRC are erased.*/
    #[inline(always)]
    pub fn regx_key(&mut self) -> REGX_KEY_W<OTFDEC_R1KEYR0rs> {
        REGX_KEY_W::new(self, 0)
    }
}
/**OTFDEC region 1 key register 0

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otfdec_r1keyr0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#OTFDEC1:OTFDEC_R1KEYR0)*/
pub struct OTFDEC_R1KEYR0rs;
impl crate::RegisterSpec for OTFDEC_R1KEYR0rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`otfdec_r1keyr0::W`](W) writer structure
impl crate::Writable for OTFDEC_R1KEYR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OTFDEC_R1KEYR0 to value 0
impl crate::Resettable for OTFDEC_R1KEYR0rs {
    const RESET_VALUE: u32 = 0;
}
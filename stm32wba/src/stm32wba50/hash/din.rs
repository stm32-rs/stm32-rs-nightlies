///Register `DIN` writer
pub type W = crate::W<DINrs>;
///Field `DATAIN` writer - Data input Writing this register pushes the current register content into the IN FIFO, and the register takes the new value presented on the AHB databus. Reading this register returns zeros.
pub type DATAIN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<DINrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - Data input Writing this register pushes the current register content into the IN FIFO, and the register takes the new value presented on the AHB databus. Reading this register returns zeros.
    #[inline(always)]
    pub fn datain(&mut self) -> DATAIN_W<'_, DINrs> {
        DATAIN_W::new(self, 0)
    }
}
/**HASH data input register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`din::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#HASH:DIN)*/
pub struct DINrs;
impl crate::RegisterSpec for DINrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`din::W`](W) writer structure
impl crate::Writable for DINrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DIN to value 0
impl crate::Resettable for DINrs {}

///Register `GPOCLR` writer
pub type W = crate::W<GPOCLRrs>;
///Field `HDPGPOCLR` writer - When a bit is written to 1, the corresponding HDP GPO is cleared.
pub type HDPGPOCLR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for crate::generic::Reg<GPOCLRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:7 - When a bit is written to 1, the corresponding HDP GPO is cleared.
    #[inline(always)]
    pub fn hdpgpoclr(&mut self) -> HDPGPOCLR_W<'_, GPOCLRrs> {
        HDPGPOCLR_W::new(self, 0)
    }
}
/**HDP general purpose output clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpoclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#HDP:GPOCLR)*/
pub struct GPOCLRrs;
impl crate::RegisterSpec for GPOCLRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`gpoclr::W`](W) writer structure
impl crate::Writable for GPOCLRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GPOCLR to value 0
impl crate::Resettable for GPOCLRrs {}

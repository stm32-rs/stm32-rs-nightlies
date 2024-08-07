///Register `HDP_GPOCLR` writer
pub type W = crate::W<HDP_GPOCLRrs>;
///Field `HDPGPOCLR` writer - HDPGPOCLR
pub type HDPGPOCLR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for crate::generic::Reg<HDP_GPOCLRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:7 - HDPGPOCLR
    #[inline(always)]
    #[must_use]
    pub fn hdpgpoclr(&mut self) -> HDPGPOCLR_W<HDP_GPOCLRrs> {
        HDPGPOCLR_W::new(self, 0)
    }
}
/**HDP GPO clear

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hdp_gpoclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HDP:HDP_GPOCLR)*/
pub struct HDP_GPOCLRrs;
impl crate::RegisterSpec for HDP_GPOCLRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`hdp_gpoclr::W`](W) writer structure
impl crate::Writable for HDP_GPOCLRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HDP_GPOCLR to value 0
impl crate::Resettable for HDP_GPOCLRrs {
    const RESET_VALUE: u32 = 0;
}

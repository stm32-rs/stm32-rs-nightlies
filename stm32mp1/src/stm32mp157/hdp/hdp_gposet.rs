///Register `HDP_GPOSET` writer
pub type W = crate::W<HDP_GPOSETrs>;
///Field `HDPGPOSET` writer - HDPGPOSET
pub type HDPGPOSET_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for crate::generic::Reg<HDP_GPOSETrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:7 - HDPGPOSET
    #[inline(always)]
    #[must_use]
    pub fn hdpgposet(&mut self) -> HDPGPOSET_W<HDP_GPOSETrs> {
        HDPGPOSET_W::new(self, 0)
    }
}
/**HDP GPO set

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hdp_gposet::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HDP:HDP_GPOSET)*/
pub struct HDP_GPOSETrs;
impl crate::RegisterSpec for HDP_GPOSETrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`hdp_gposet::W`](W) writer structure
impl crate::Writable for HDP_GPOSETrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HDP_GPOSET to value 0
impl crate::Resettable for HDP_GPOSETrs {
    const RESET_VALUE: u32 = 0;
}

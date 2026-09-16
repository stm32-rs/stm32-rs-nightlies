///Register `GPOSET` writer
pub type W = crate::W<GPOSETrs>;
///Field `HDPGPOSET` writer - HDPGPOSET
pub type HDPGPOSET_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for crate::generic::Reg<GPOSETrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:7 - HDPGPOSET
    #[inline(always)]
    pub fn hdpgposet(&mut self) -> HDPGPOSET_W<'_, GPOSETrs> {
        HDPGPOSET_W::new(self, 0)
    }
}
/**HDP GPO set

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gposet::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HDP:GPOSET)*/
pub struct GPOSETrs;
impl crate::RegisterSpec for GPOSETrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`gposet::W`](W) writer structure
impl crate::Writable for GPOSETrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GPOSET to value 0
impl crate::Resettable for GPOSETrs {}

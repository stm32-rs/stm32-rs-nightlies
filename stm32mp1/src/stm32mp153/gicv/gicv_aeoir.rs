///Register `GICV_AEOIR` writer
pub type W = crate::W<GICV_AEOIRrs>;
///Field `EOIINTID` writer - EOIINTID
pub type EOIINTID_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `CPUID` writer - CPUID
pub type CPUID_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<GICV_AEOIRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:9 - EOIINTID
    #[inline(always)]
    #[must_use]
    pub fn eoiintid(&mut self) -> EOIINTID_W<GICV_AEOIRrs> {
        EOIINTID_W::new(self, 0)
    }
    ///Bit 10 - CPUID
    #[inline(always)]
    #[must_use]
    pub fn cpuid(&mut self) -> CPUID_W<GICV_AEOIRrs> {
        CPUID_W::new(self, 10)
    }
}
/**GICV VM aliased end of interrupt register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicv_aeoir::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICV:GICV_AEOIR)*/
pub struct GICV_AEOIRrs;
impl crate::RegisterSpec for GICV_AEOIRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`gicv_aeoir::W`](W) writer structure
impl crate::Writable for GICV_AEOIRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GICV_AEOIR to value 0
impl crate::Resettable for GICV_AEOIRrs {
    const RESET_VALUE: u32 = 0;
}

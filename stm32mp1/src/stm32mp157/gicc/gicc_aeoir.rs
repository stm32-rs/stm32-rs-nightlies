///Register `GICC_AEOIR` writer
pub type W = crate::W<GICC_AEOIRrs>;
///Field `EOIINTID` writer - EOIINTID
pub type EOIINTID_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `CPUID` writer - CPUID
pub type CPUID_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<GICC_AEOIRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:9 - EOIINTID
    #[inline(always)]
    #[must_use]
    pub fn eoiintid(&mut self) -> EOIINTID_W<GICC_AEOIRrs> {
        EOIINTID_W::new(self, 0)
    }
    ///Bit 10 - CPUID
    #[inline(always)]
    #[must_use]
    pub fn cpuid(&mut self) -> CPUID_W<GICC_AEOIRrs> {
        CPUID_W::new(self, 10)
    }
}
/**GICC_AEOIR is an alias of the Non-secure GICC_EOIR. A secure access to this register is similar to a non-secure access to GICC_EOIR, except that the GICC_CTLR.EOImodeS bit is used.

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicc_aeoir::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICC:GICC_AEOIR)*/
pub struct GICC_AEOIRrs;
impl crate::RegisterSpec for GICC_AEOIRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`gicc_aeoir::W`](W) writer structure
impl crate::Writable for GICC_AEOIRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GICC_AEOIR to value 0
impl crate::Resettable for GICC_AEOIRrs {
    const RESET_VALUE: u32 = 0;
}

///Register `GICC_EOIR` writer
pub type W = crate::W<GICC_EOIRrs>;
///Field `EOIINTID` writer - EOIINTID
pub type EOIINTID_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `CPUID` writer - CPUID
pub type CPUID_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<GICC_EOIRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:9 - EOIINTID
    #[inline(always)]
    #[must_use]
    pub fn eoiintid(&mut self) -> EOIINTID_W<GICC_EOIRrs> {
        EOIINTID_W::new(self, 0)
    }
    ///Bit 10 - CPUID
    #[inline(always)]
    #[must_use]
    pub fn cpuid(&mut self) -> CPUID_W<GICC_EOIRrs> {
        CPUID_W::new(self, 10)
    }
}
/**GICC end of interrupt register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicc_eoir::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICC:GICC_EOIR)*/
pub struct GICC_EOIRrs;
impl crate::RegisterSpec for GICC_EOIRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`gicc_eoir::W`](W) writer structure
impl crate::Writable for GICC_EOIRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GICC_EOIR to value 0
impl crate::Resettable for GICC_EOIRrs {
    const RESET_VALUE: u32 = 0;
}

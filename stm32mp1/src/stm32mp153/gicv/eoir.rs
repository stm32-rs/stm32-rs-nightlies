///Register `EOIR` writer
pub type W = crate::W<EOIRrs>;
///Field `EOIINTID` writer - EOIINTID
pub type EOIINTID_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `CPUID` writer - CPUID
pub type CPUID_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<EOIRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:9 - EOIINTID
    #[inline(always)]
    pub fn eoiintid(&mut self) -> EOIINTID_W<'_, EOIRrs> {
        EOIINTID_W::new(self, 0)
    }
    ///Bit 10 - CPUID
    #[inline(always)]
    pub fn cpuid(&mut self) -> CPUID_W<'_, EOIRrs> {
        CPUID_W::new(self, 10)
    }
}
/**GICV VM end of interrupt register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eoir::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICV:EOIR)*/
pub struct EOIRrs;
impl crate::RegisterSpec for EOIRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`eoir::W`](W) writer structure
impl crate::Writable for EOIRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EOIR to value 0
impl crate::Resettable for EOIRrs {}

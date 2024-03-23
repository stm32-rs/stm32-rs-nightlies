#[doc = "Register `GICC_AEOIR` writer"]
pub type W = crate::W<GICC_AEOIRrs>;
#[doc = "Field `EOIINTID` writer - EOIINTID"]
pub type EOIINTID_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `CPUID` writer - CPUID"]
pub type CPUID_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 0:9 - EOIINTID"]
    #[inline(always)]
    #[must_use]
    pub fn eoiintid(&mut self) -> EOIINTID_W<GICC_AEOIRrs> {
        EOIINTID_W::new(self, 0)
    }
    #[doc = "Bit 10 - CPUID"]
    #[inline(always)]
    #[must_use]
    pub fn cpuid(&mut self) -> CPUID_W<GICC_AEOIRrs> {
        CPUID_W::new(self, 10)
    }
}
#[doc = "GICC_AEOIR is an alias of the Non-secure GICC_EOIR. A secure access to this register is similar to a non-secure access to GICC_EOIR, except that the GICC_CTLR.EOImodeS bit is used.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicc_aeoir::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICC_AEOIRrs;
impl crate::RegisterSpec for GICC_AEOIRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gicc_aeoir::W`](W) writer structure"]
impl crate::Writable for GICC_AEOIRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICC_AEOIR to value 0"]
impl crate::Resettable for GICC_AEOIRrs {
    const RESET_VALUE: u32 = 0;
}

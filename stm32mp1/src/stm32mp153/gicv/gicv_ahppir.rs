#[doc = "Register `GICV_AHPPIR` reader"]
pub type R = crate::R<GICV_AHPPIRrs>;
#[doc = "Field `PENDINTID` reader - PENDINTID"]
pub type PENDINTID_R = crate::FieldReader<u16>;
#[doc = "Field `CPUID` reader - CPUID"]
pub type CPUID_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:9 - PENDINTID"]
    #[inline(always)]
    pub fn pendintid(&self) -> PENDINTID_R {
        PENDINTID_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - CPUID"]
    #[inline(always)]
    pub fn cpuid(&self) -> CPUID_R {
        CPUID_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "GICV VM aliased highest priority pending interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicv_ahppir::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICV_AHPPIRrs;
impl crate::RegisterSpec for GICV_AHPPIRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicv_ahppir::R`](R) reader structure"]
impl crate::Readable for GICV_AHPPIRrs {}
#[doc = "`reset()` method sets GICV_AHPPIR to value 0x03ff"]
impl crate::Resettable for GICV_AHPPIRrs {
    const RESET_VALUE: u32 = 0x03ff;
}

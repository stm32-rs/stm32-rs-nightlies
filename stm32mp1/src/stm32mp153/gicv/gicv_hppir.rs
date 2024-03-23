#[doc = "Register `GICV_HPPIR` reader"]
pub type R = crate::R<GICV_HPPIRrs>;
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
#[doc = "GICV VM highest priority pending interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicv_hppir::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICV_HPPIRrs;
impl crate::RegisterSpec for GICV_HPPIRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicv_hppir::R`](R) reader structure"]
impl crate::Readable for GICV_HPPIRrs {}
#[doc = "`reset()` method sets GICV_HPPIR to value 0x03ff"]
impl crate::Resettable for GICV_HPPIRrs {
    const RESET_VALUE: u32 = 0x03ff;
}

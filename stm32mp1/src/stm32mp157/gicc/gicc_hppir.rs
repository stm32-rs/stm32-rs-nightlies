#[doc = "Register `GICC_HPPIR` reader"]
pub type R = crate::R<GICC_HPPIRrs>;
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
#[doc = "GICC highest priority pending interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicc_hppir::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICC_HPPIRrs;
impl crate::RegisterSpec for GICC_HPPIRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicc_hppir::R`](R) reader structure"]
impl crate::Readable for GICC_HPPIRrs {}
#[doc = "`reset()` method sets GICC_HPPIR to value 0x03ff"]
impl crate::Resettable for GICC_HPPIRrs {
    const RESET_VALUE: u32 = 0x03ff;
}

#[doc = "Register `GICC_AIAR` reader"]
pub type R = crate::R<GICC_AIARrs>;
#[doc = "Field `INTERRUPT_ID` reader - INTERRUPT_ID"]
pub type INTERRUPT_ID_R = crate::FieldReader<u16>;
#[doc = "Field `CPUID` reader - CPUID"]
pub type CPUID_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:9 - INTERRUPT_ID"]
    #[inline(always)]
    pub fn interrupt_id(&self) -> INTERRUPT_ID_R {
        INTERRUPT_ID_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - CPUID"]
    #[inline(always)]
    pub fn cpuid(&self) -> CPUID_R {
        CPUID_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "GICC_AIAR is an alias of the non-secure view of GICC_IAR. A secure access to this register is identical to a non-secure access to GICC_IAR.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicc_aiar::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICC_AIARrs;
impl crate::RegisterSpec for GICC_AIARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicc_aiar::R`](R) reader structure"]
impl crate::Readable for GICC_AIARrs {}
#[doc = "`reset()` method sets GICC_AIAR to value 0x03ff"]
impl crate::Resettable for GICC_AIARrs {
    const RESET_VALUE: u32 = 0x03ff;
}

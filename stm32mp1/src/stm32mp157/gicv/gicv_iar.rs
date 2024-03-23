#[doc = "Register `GICV_IAR` reader"]
pub type R = crate::R<GICV_IARrs>;
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
#[doc = "GICV VM interrupt acknowledge register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicv_iar::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICV_IARrs;
impl crate::RegisterSpec for GICV_IARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicv_iar::R`](R) reader structure"]
impl crate::Readable for GICV_IARrs {}
#[doc = "`reset()` method sets GICV_IAR to value 0x03ff"]
impl crate::Resettable for GICV_IARrs {
    const RESET_VALUE: u32 = 0x03ff;
}

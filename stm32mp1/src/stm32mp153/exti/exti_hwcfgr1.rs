#[doc = "Register `EXTI_HWCFGR1` reader"]
pub type R = crate::R<EXTI_HWCFGR1rs>;
#[doc = "Field `NBEVENTS` reader - NBEVENTS"]
pub type NBEVENTS_R = crate::FieldReader;
#[doc = "Field `NBCPUS` reader - NBCPUS"]
pub type NBCPUS_R = crate::FieldReader;
#[doc = "Field `CPUEVTEN` reader - CPUEVTEN"]
pub type CPUEVTEN_R = crate::FieldReader;
#[doc = "Field `NBIOPORT` reader - NBIOPORT"]
pub type NBIOPORT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - NBEVENTS"]
    #[inline(always)]
    pub fn nbevents(&self) -> NBEVENTS_R {
        NBEVENTS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - NBCPUS"]
    #[inline(always)]
    pub fn nbcpus(&self) -> NBCPUS_R {
        NBCPUS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - CPUEVTEN"]
    #[inline(always)]
    pub fn cpuevten(&self) -> CPUEVTEN_R {
        CPUEVTEN_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - NBIOPORT"]
    #[inline(always)]
    pub fn nbioport(&self) -> NBIOPORT_R {
        NBIOPORT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "EXTI hardware configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_hwcfgr1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTI_HWCFGR1rs;
impl crate::RegisterSpec for EXTI_HWCFGR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exti_hwcfgr1::R`](R) reader structure"]
impl crate::Readable for EXTI_HWCFGR1rs {}
#[doc = "`reset()` method sets EXTI_HWCFGR1 to value 0x000b_214b"]
impl crate::Resettable for EXTI_HWCFGR1rs {
    const RESET_VALUE: u32 = 0x000b_214b;
}

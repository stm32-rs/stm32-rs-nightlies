#[doc = "Register `HWCFGR1` reader"]
pub type R = crate::R<HWCFGR1rs>;
#[doc = "Field `NBEVENTS` reader - configuration number of event"]
pub type NBEVENTS_R = crate::FieldReader;
#[doc = "Field `NBCPUS` reader - configuration number of CPUs"]
pub type NBCPUS_R = crate::FieldReader;
#[doc = "Field `CPUEVTEN` reader - HW configuration of CPU event output enable"]
pub type CPUEVTEN_R = crate::FieldReader;
#[doc = "Field `NBIOPORT` reader - HW configuration of number of IO ports"]
pub type NBIOPORT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - configuration number of event"]
    #[inline(always)]
    pub fn nbevents(&self) -> NBEVENTS_R {
        NBEVENTS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - configuration number of CPUs"]
    #[inline(always)]
    pub fn nbcpus(&self) -> NBCPUS_R {
        NBCPUS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - HW configuration of CPU event output enable"]
    #[inline(always)]
    pub fn cpuevten(&self) -> CPUEVTEN_R {
        CPUEVTEN_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - HW configuration of number of IO ports"]
    #[inline(always)]
    pub fn nbioport(&self) -> NBIOPORT_R {
        NBIOPORT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "Hardware configuration registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HWCFGR1rs;
impl crate::RegisterSpec for HWCFGR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwcfgr1::R`](R) reader structure"]
impl crate::Readable for HWCFGR1rs {}
#[doc = "`reset()` method sets HWCFGR1 to value 0x0005_1021"]
impl crate::Resettable for HWCFGR1rs {
    const RESET_VALUE: u32 = 0x0005_1021;
}

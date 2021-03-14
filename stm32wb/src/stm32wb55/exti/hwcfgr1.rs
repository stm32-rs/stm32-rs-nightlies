#[doc = "Reader of register HWCFGR1"]
pub type R = crate::R<u32, super::HWCFGR1>;
#[doc = "Reader of field `NBEVENTS`"]
pub type NBEVENTS_R = crate::R<u8, u8>;
#[doc = "Reader of field `NBCPUS`"]
pub type NBCPUS_R = crate::R<u8, u8>;
#[doc = "Reader of field `CPUEVTEN`"]
pub type CPUEVTEN_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - HW configuration number of event"]
    #[inline(always)]
    pub fn nbevents(&self) -> NBEVENTS_R {
        NBEVENTS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - HW configuration number of CPUs"]
    #[inline(always)]
    pub fn nbcpus(&self) -> NBCPUS_R {
        NBCPUS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - HW configuration of CPU(m) event output enable"]
    #[inline(always)]
    pub fn cpuevten(&self) -> CPUEVTEN_R {
        CPUEVTEN_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}

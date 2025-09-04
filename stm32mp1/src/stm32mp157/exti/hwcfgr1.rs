///Register `HWCFGR1` reader
pub type R = crate::R<HWCFGR1rs>;
///Field `NBEVENTS` reader - NBEVENTS
pub type NBEVENTS_R = crate::FieldReader;
///Field `NBCPUS` reader - NBCPUS
pub type NBCPUS_R = crate::FieldReader;
///Field `CPUEVTEN` reader - CPUEVTEN
pub type CPUEVTEN_R = crate::FieldReader;
///Field `NBIOPORT` reader - NBIOPORT
pub type NBIOPORT_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - NBEVENTS
    #[inline(always)]
    pub fn nbevents(&self) -> NBEVENTS_R {
        NBEVENTS_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - NBCPUS
    #[inline(always)]
    pub fn nbcpus(&self) -> NBCPUS_R {
        NBCPUS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - CPUEVTEN
    #[inline(always)]
    pub fn cpuevten(&self) -> CPUEVTEN_R {
        CPUEVTEN_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:23 - NBIOPORT
    #[inline(always)]
    pub fn nbioport(&self) -> NBIOPORT_R {
        NBIOPORT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWCFGR1")
            .field("nbevents", &self.nbevents())
            .field("nbcpus", &self.nbcpus())
            .field("cpuevten", &self.cpuevten())
            .field("nbioport", &self.nbioport())
            .finish()
    }
}
/**EXTI hardware configuration register 1

You can [`read`](crate::Reg::read) this register and get [`hwcfgr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#EXTI:HWCFGR1)*/
pub struct HWCFGR1rs;
impl crate::RegisterSpec for HWCFGR1rs {
    type Ux = u32;
}
///`read()` method returns [`hwcfgr1::R`](R) reader structure
impl crate::Readable for HWCFGR1rs {}
///`reset()` method sets HWCFGR1 to value 0x000b_214b
impl crate::Resettable for HWCFGR1rs {
    const RESET_VALUE: u32 = 0x000b_214b;
}

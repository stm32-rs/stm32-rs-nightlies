///Register `TAMP_MISR` reader
pub type R = crate::R<TAMP_MISRrs>;
///Field `TAMP1MF` reader - TAMP1 interrupt masked flag This flag is set by hardware when the tamper 1 interrupt is raised.
pub type TAMP1MF_R = crate::BitReader;
///Field `TAMP2MF` reader - TAMP2 interrupt masked flag This flag is set by hardware when the tamper 2 interrupt is raised.
pub type TAMP2MF_R = crate::BitReader;
///Field `TAMP3MF` reader - TAMP3 interrupt masked flag This flag is set by hardware when the tamper 3 interrupt is raised.
pub type TAMP3MF_R = crate::BitReader;
///Field `TAMP4MF` reader - TAMP4 interrupt masked flag This flag is set by hardware when the tamper 4 interrupt is raised.
pub type TAMP4MF_R = crate::BitReader;
///Field `TAMP5MF` reader - TAMP5 interrupt masked flag This flag is set by hardware when the tamper 5 interrupt is raised.
pub type TAMP5MF_R = crate::BitReader;
///Field `ITAMP3MF` reader - Internal tamper 3 interrupt masked flag This flag is set by hardware when the internal tamper 3 interrupt is raised.
pub type ITAMP3MF_R = crate::BitReader;
///Field `ITAMP4MF` reader - Internal tamper 4 interrupt masked flag This flag is set by hardware when the internal tamper 4 interrupt is raised.
pub type ITAMP4MF_R = crate::BitReader;
///Field `ITAMP5MF` reader - Internal tamper 5 interrupt masked flag This flag is set by hardware when the internal tamper 5 interrupt is raised.
pub type ITAMP5MF_R = crate::BitReader;
///Field `ITAMP6MF` reader - Internal tamper 6 interrupt masked flag This flag is set by hardware when the internal tamper 6 interrupt is raised.
pub type ITAMP6MF_R = crate::BitReader;
impl R {
    ///Bit 0 - TAMP1 interrupt masked flag This flag is set by hardware when the tamper 1 interrupt is raised.
    #[inline(always)]
    pub fn tamp1mf(&self) -> TAMP1MF_R {
        TAMP1MF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TAMP2 interrupt masked flag This flag is set by hardware when the tamper 2 interrupt is raised.
    #[inline(always)]
    pub fn tamp2mf(&self) -> TAMP2MF_R {
        TAMP2MF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TAMP3 interrupt masked flag This flag is set by hardware when the tamper 3 interrupt is raised.
    #[inline(always)]
    pub fn tamp3mf(&self) -> TAMP3MF_R {
        TAMP3MF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TAMP4 interrupt masked flag This flag is set by hardware when the tamper 4 interrupt is raised.
    #[inline(always)]
    pub fn tamp4mf(&self) -> TAMP4MF_R {
        TAMP4MF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TAMP5 interrupt masked flag This flag is set by hardware when the tamper 5 interrupt is raised.
    #[inline(always)]
    pub fn tamp5mf(&self) -> TAMP5MF_R {
        TAMP5MF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 18 - Internal tamper 3 interrupt masked flag This flag is set by hardware when the internal tamper 3 interrupt is raised.
    #[inline(always)]
    pub fn itamp3mf(&self) -> ITAMP3MF_R {
        ITAMP3MF_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Internal tamper 4 interrupt masked flag This flag is set by hardware when the internal tamper 4 interrupt is raised.
    #[inline(always)]
    pub fn itamp4mf(&self) -> ITAMP4MF_R {
        ITAMP4MF_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Internal tamper 5 interrupt masked flag This flag is set by hardware when the internal tamper 5 interrupt is raised.
    #[inline(always)]
    pub fn itamp5mf(&self) -> ITAMP5MF_R {
        ITAMP5MF_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Internal tamper 6 interrupt masked flag This flag is set by hardware when the internal tamper 6 interrupt is raised.
    #[inline(always)]
    pub fn itamp6mf(&self) -> ITAMP6MF_R {
        ITAMP6MF_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TAMP_MISR")
            .field("tamp1mf", &self.tamp1mf())
            .field("tamp2mf", &self.tamp2mf())
            .field("tamp3mf", &self.tamp3mf())
            .field("tamp4mf", &self.tamp4mf())
            .field("tamp5mf", &self.tamp5mf())
            .field("itamp3mf", &self.itamp3mf())
            .field("itamp4mf", &self.itamp4mf())
            .field("itamp5mf", &self.itamp5mf())
            .field("itamp6mf", &self.itamp6mf())
            .finish()
    }
}
/**TAMP masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`tamp_misr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#TAMP:TAMP_MISR)*/
pub struct TAMP_MISRrs;
impl crate::RegisterSpec for TAMP_MISRrs {
    type Ux = u32;
}
///`read()` method returns [`tamp_misr::R`](R) reader structure
impl crate::Readable for TAMP_MISRrs {}
///`reset()` method sets TAMP_MISR to value 0
impl crate::Resettable for TAMP_MISRrs {
    const RESET_VALUE: u32 = 0;
}
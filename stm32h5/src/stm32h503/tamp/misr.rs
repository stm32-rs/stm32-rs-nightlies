#[doc = "Register `MISR` reader"]
pub type R = crate::R<MISRrs>;
#[doc = "Field `TAMP1MF` reader - TAMP1 interrupt masked flag This flag is set by hardware when the tamper 1 interrupt is raised."]
pub type TAMP1MF_R = crate::BitReader;
#[doc = "Field `TAMP2MF` reader - TAMP2 interrupt masked flag This flag is set by hardware when the tamper 2 interrupt is raised."]
pub type TAMP2MF_R = crate::BitReader;
#[doc = "Field `ITAMP1MF` reader - Internal tamper 1 interrupt masked flag This flag is set by hardware when the internal tamper 1 interrupt is raised."]
pub type ITAMP1MF_R = crate::BitReader;
#[doc = "Field `ITAMP2MF` reader - Internal tamper 2 interrupt masked flag This flag is set by hardware when the internal tamper 2 interrupt is raised."]
pub type ITAMP2MF_R = crate::BitReader;
#[doc = "Field `ITAMP3MF` reader - Internal tamper 3 interrupt masked flag This flag is set by hardware when the internal tamper 3 interrupt is raised."]
pub type ITAMP3MF_R = crate::BitReader;
#[doc = "Field `ITAMP4MF` reader - Internal tamper 4 interrupt masked flag This flag is set by hardware when the internal tamper 4 interrupt is raised."]
pub type ITAMP4MF_R = crate::BitReader;
#[doc = "Field `ITAMP5MF` reader - Internal tamper 5 interrupt masked flag This flag is set by hardware when the internal tamper 5 interrupt is raised."]
pub type ITAMP5MF_R = crate::BitReader;
#[doc = "Field `ITAMP6MF` reader - Internal tamper 6 interrupt masked flag This flag is set by hardware when the internal tamper 6 interrupt is raised."]
pub type ITAMP6MF_R = crate::BitReader;
#[doc = "Field `ITAMP7MF` reader - Internal tamper 7 tamper interrupt masked flag This flag is set by hardware when the internal tamper 7 interrupt is raised."]
pub type ITAMP7MF_R = crate::BitReader;
#[doc = "Field `ITAMP8MF` reader - Internal tamper 8 interrupt masked flag This flag is set by hardware when the internal tamper 8 interrupt is raised."]
pub type ITAMP8MF_R = crate::BitReader;
#[doc = "Field `ITAMP9MF` reader - internal tamper 9 interrupt masked flag This flag is set by hardware when the internal tamper 9 interrupt is raised."]
pub type ITAMP9MF_R = crate::BitReader;
#[doc = "Field `ITAMP11MF` reader - internal tamper 11 interrupt masked flag This flag is set by hardware when the internal tamper 11 interrupt is raised."]
pub type ITAMP11MF_R = crate::BitReader;
#[doc = "Field `ITAMP12MF` reader - internal tamper 12 interrupt masked flag This flag is set by hardware when the internal tamper 12 interrupt is raised."]
pub type ITAMP12MF_R = crate::BitReader;
#[doc = "Field `ITAMP13MF` reader - internal tamper 13 interrupt masked flag This flag is set by hardware when the internal tamper 13 interrupt is raised."]
pub type ITAMP13MF_R = crate::BitReader;
#[doc = "Field `ITAMP15MF` reader - internal tamper 15 interrupt masked flag This flag is set by hardware when the internal tamper 15 interrupt is raised."]
pub type ITAMP15MF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TAMP1 interrupt masked flag This flag is set by hardware when the tamper 1 interrupt is raised."]
    #[inline(always)]
    pub fn tamp1mf(&self) -> TAMP1MF_R {
        TAMP1MF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TAMP2 interrupt masked flag This flag is set by hardware when the tamper 2 interrupt is raised."]
    #[inline(always)]
    pub fn tamp2mf(&self) -> TAMP2MF_R {
        TAMP2MF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - Internal tamper 1 interrupt masked flag This flag is set by hardware when the internal tamper 1 interrupt is raised."]
    #[inline(always)]
    pub fn itamp1mf(&self) -> ITAMP1MF_R {
        ITAMP1MF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Internal tamper 2 interrupt masked flag This flag is set by hardware when the internal tamper 2 interrupt is raised."]
    #[inline(always)]
    pub fn itamp2mf(&self) -> ITAMP2MF_R {
        ITAMP2MF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Internal tamper 3 interrupt masked flag This flag is set by hardware when the internal tamper 3 interrupt is raised."]
    #[inline(always)]
    pub fn itamp3mf(&self) -> ITAMP3MF_R {
        ITAMP3MF_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Internal tamper 4 interrupt masked flag This flag is set by hardware when the internal tamper 4 interrupt is raised."]
    #[inline(always)]
    pub fn itamp4mf(&self) -> ITAMP4MF_R {
        ITAMP4MF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Internal tamper 5 interrupt masked flag This flag is set by hardware when the internal tamper 5 interrupt is raised."]
    #[inline(always)]
    pub fn itamp5mf(&self) -> ITAMP5MF_R {
        ITAMP5MF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Internal tamper 6 interrupt masked flag This flag is set by hardware when the internal tamper 6 interrupt is raised."]
    #[inline(always)]
    pub fn itamp6mf(&self) -> ITAMP6MF_R {
        ITAMP6MF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Internal tamper 7 tamper interrupt masked flag This flag is set by hardware when the internal tamper 7 interrupt is raised."]
    #[inline(always)]
    pub fn itamp7mf(&self) -> ITAMP7MF_R {
        ITAMP7MF_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Internal tamper 8 interrupt masked flag This flag is set by hardware when the internal tamper 8 interrupt is raised."]
    #[inline(always)]
    pub fn itamp8mf(&self) -> ITAMP8MF_R {
        ITAMP8MF_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - internal tamper 9 interrupt masked flag This flag is set by hardware when the internal tamper 9 interrupt is raised."]
    #[inline(always)]
    pub fn itamp9mf(&self) -> ITAMP9MF_R {
        ITAMP9MF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - internal tamper 11 interrupt masked flag This flag is set by hardware when the internal tamper 11 interrupt is raised."]
    #[inline(always)]
    pub fn itamp11mf(&self) -> ITAMP11MF_R {
        ITAMP11MF_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - internal tamper 12 interrupt masked flag This flag is set by hardware when the internal tamper 12 interrupt is raised."]
    #[inline(always)]
    pub fn itamp12mf(&self) -> ITAMP12MF_R {
        ITAMP12MF_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - internal tamper 13 interrupt masked flag This flag is set by hardware when the internal tamper 13 interrupt is raised."]
    #[inline(always)]
    pub fn itamp13mf(&self) -> ITAMP13MF_R {
        ITAMP13MF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - internal tamper 15 interrupt masked flag This flag is set by hardware when the internal tamper 15 interrupt is raised."]
    #[inline(always)]
    pub fn itamp15mf(&self) -> ITAMP15MF_R {
        ITAMP15MF_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[doc = "TAMP masked interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MISRrs;
impl crate::RegisterSpec for MISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misr::R`](R) reader structure"]
impl crate::Readable for MISRrs {}
#[doc = "`reset()` method sets MISR to value 0"]
impl crate::Resettable for MISRrs {
    const RESET_VALUE: u32 = 0;
}

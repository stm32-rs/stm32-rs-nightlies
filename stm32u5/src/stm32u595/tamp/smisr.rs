#[doc = "Register `SMISR` reader"]
pub type R = crate::R<SMISRrs>;
#[doc = "Field `TAMP1MF` reader - TAMP1MF"]
pub type TAMP1MF_R = crate::BitReader;
#[doc = "Field `TAMP2MF` reader - TAMP2MF"]
pub type TAMP2MF_R = crate::BitReader;
#[doc = "Field `TAMP3MF` reader - TAMP3MF"]
pub type TAMP3MF_R = crate::BitReader;
#[doc = "Field `TAMP4MF` reader - TAMP4MF"]
pub type TAMP4MF_R = crate::BitReader;
#[doc = "Field `TAMP5MF` reader - TAMP5MF"]
pub type TAMP5MF_R = crate::BitReader;
#[doc = "Field `TAMP6MF` reader - TAMP6MF"]
pub type TAMP6MF_R = crate::BitReader;
#[doc = "Field `TAMP7MF` reader - TAMP7MF"]
pub type TAMP7MF_R = crate::BitReader;
#[doc = "Field `TAMP8MF` reader - TAMP8MF"]
pub type TAMP8MF_R = crate::BitReader;
#[doc = "Field `ITAMP1MF` reader - ITAMP1MF"]
pub type ITAMP1MF_R = crate::BitReader;
#[doc = "Field `ITAMP2MF` reader - ITAMP2MF"]
pub type ITAMP2MF_R = crate::BitReader;
#[doc = "Field `ITAMP3MF` reader - ITAMP3MF"]
pub type ITAMP3MF_R = crate::BitReader;
#[doc = "Field `ITAMP5MF` reader - ITAMP5MF"]
pub type ITAMP5MF_R = crate::BitReader;
#[doc = "Field `ITAMP6MF` reader - ITAMP6MF"]
pub type ITAMP6MF_R = crate::BitReader;
#[doc = "Field `ITAMP7MF` reader - ITAMP7MF"]
pub type ITAMP7MF_R = crate::BitReader;
#[doc = "Field `ITAMP8MF` reader - ITAMP8MF"]
pub type ITAMP8MF_R = crate::BitReader;
#[doc = "Field `ITAMP9MF` reader - ITAMP9MF"]
pub type ITAMP9MF_R = crate::BitReader;
#[doc = "Field `ITAMP11MF` reader - ITAMP11MF"]
pub type ITAMP11MF_R = crate::BitReader;
#[doc = "Field `ITAMP12MF` reader - ITAMP12MF"]
pub type ITAMP12MF_R = crate::BitReader;
#[doc = "Field `ITAMP13MF` reader - ITAMP13MF"]
pub type ITAMP13MF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TAMP1MF"]
    #[inline(always)]
    pub fn tamp1mf(&self) -> TAMP1MF_R {
        TAMP1MF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TAMP2MF"]
    #[inline(always)]
    pub fn tamp2mf(&self) -> TAMP2MF_R {
        TAMP2MF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TAMP3MF"]
    #[inline(always)]
    pub fn tamp3mf(&self) -> TAMP3MF_R {
        TAMP3MF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TAMP4MF"]
    #[inline(always)]
    pub fn tamp4mf(&self) -> TAMP4MF_R {
        TAMP4MF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TAMP5MF"]
    #[inline(always)]
    pub fn tamp5mf(&self) -> TAMP5MF_R {
        TAMP5MF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TAMP6MF"]
    #[inline(always)]
    pub fn tamp6mf(&self) -> TAMP6MF_R {
        TAMP6MF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TAMP7MF"]
    #[inline(always)]
    pub fn tamp7mf(&self) -> TAMP7MF_R {
        TAMP7MF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TAMP8MF"]
    #[inline(always)]
    pub fn tamp8mf(&self) -> TAMP8MF_R {
        TAMP8MF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - ITAMP1MF"]
    #[inline(always)]
    pub fn itamp1mf(&self) -> ITAMP1MF_R {
        ITAMP1MF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ITAMP2MF"]
    #[inline(always)]
    pub fn itamp2mf(&self) -> ITAMP2MF_R {
        ITAMP2MF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ITAMP3MF"]
    #[inline(always)]
    pub fn itamp3mf(&self) -> ITAMP3MF_R {
        ITAMP3MF_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - ITAMP5MF"]
    #[inline(always)]
    pub fn itamp5mf(&self) -> ITAMP5MF_R {
        ITAMP5MF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ITAMP6MF"]
    #[inline(always)]
    pub fn itamp6mf(&self) -> ITAMP6MF_R {
        ITAMP6MF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ITAMP7MF"]
    #[inline(always)]
    pub fn itamp7mf(&self) -> ITAMP7MF_R {
        ITAMP7MF_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - ITAMP8MF"]
    #[inline(always)]
    pub fn itamp8mf(&self) -> ITAMP8MF_R {
        ITAMP8MF_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - ITAMP9MF"]
    #[inline(always)]
    pub fn itamp9mf(&self) -> ITAMP9MF_R {
        ITAMP9MF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - ITAMP11MF"]
    #[inline(always)]
    pub fn itamp11mf(&self) -> ITAMP11MF_R {
        ITAMP11MF_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - ITAMP12MF"]
    #[inline(always)]
    pub fn itamp12mf(&self) -> ITAMP12MF_R {
        ITAMP12MF_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - ITAMP13MF"]
    #[inline(always)]
    pub fn itamp13mf(&self) -> ITAMP13MF_R {
        ITAMP13MF_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[doc = "TAMP secure masked interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smisr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMISRrs;
impl crate::RegisterSpec for SMISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smisr::R`](R) reader structure"]
impl crate::Readable for SMISRrs {}
#[doc = "`reset()` method sets SMISR to value 0"]
impl crate::Resettable for SMISRrs {
    const RESET_VALUE: u32 = 0;
}

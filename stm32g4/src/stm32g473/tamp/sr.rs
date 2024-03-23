#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "Field `TAMP1F` reader - TAMP1F"]
pub type TAMP1F_R = crate::BitReader;
#[doc = "Field `TAMP2F` reader - TAMP2F"]
pub type TAMP2F_R = crate::BitReader;
#[doc = "Field `TAMP3F` reader - TAMP3F"]
pub type TAMP3F_R = crate::BitReader;
#[doc = "Field `ITAMP3F` reader - ITAMP3F"]
pub type ITAMP3F_R = crate::BitReader;
#[doc = "Field `ITAMP4F` reader - ITAMP4F"]
pub type ITAMP4F_R = crate::BitReader;
#[doc = "Field `ITAMP5F` reader - ITAMP5F"]
pub type ITAMP5F_R = crate::BitReader;
#[doc = "Field `ITAMP6F` reader - ITAMP6F"]
pub type ITAMP6F_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TAMP1F"]
    #[inline(always)]
    pub fn tamp1f(&self) -> TAMP1F_R {
        TAMP1F_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TAMP2F"]
    #[inline(always)]
    pub fn tamp2f(&self) -> TAMP2F_R {
        TAMP2F_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TAMP3F"]
    #[inline(always)]
    pub fn tamp3f(&self) -> TAMP3F_R {
        TAMP3F_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 18 - ITAMP3F"]
    #[inline(always)]
    pub fn itamp3f(&self) -> ITAMP3F_R {
        ITAMP3F_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ITAMP4F"]
    #[inline(always)]
    pub fn itamp4f(&self) -> ITAMP4F_R {
        ITAMP4F_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ITAMP5F"]
    #[inline(always)]
    pub fn itamp5f(&self) -> ITAMP5F_R {
        ITAMP5F_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ITAMP6F"]
    #[inline(always)]
    pub fn itamp6f(&self) -> ITAMP6F_R {
        ITAMP6F_R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[doc = "TAMP status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SRrs {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `CSLOCKR` reader"]
pub type R = crate::R<CSLOCKRrs>;
#[doc = "Register `CSLOCKR` writer"]
pub type W = crate::W<CSLOCKRrs>;
#[doc = "Field `LOCKSVTAIRCR` reader - LOCKSVTAIRCR"]
pub type LOCKSVTAIRCR_R = crate::BitReader;
#[doc = "Field `LOCKSVTAIRCR` writer - LOCKSVTAIRCR"]
pub type LOCKSVTAIRCR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCKSMPU` reader - LOCKSMPU"]
pub type LOCKSMPU_R = crate::BitReader;
#[doc = "Field `LOCKSMPU` writer - LOCKSMPU"]
pub type LOCKSMPU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCKSAU` reader - LOCKSAU"]
pub type LOCKSAU_R = crate::BitReader;
#[doc = "Field `LOCKSAU` writer - LOCKSAU"]
pub type LOCKSAU_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LOCKSVTAIRCR"]
    #[inline(always)]
    pub fn locksvtaircr(&self) -> LOCKSVTAIRCR_R {
        LOCKSVTAIRCR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LOCKSMPU"]
    #[inline(always)]
    pub fn locksmpu(&self) -> LOCKSMPU_R {
        LOCKSMPU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LOCKSAU"]
    #[inline(always)]
    pub fn locksau(&self) -> LOCKSAU_R {
        LOCKSAU_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LOCKSVTAIRCR"]
    #[inline(always)]
    #[must_use]
    pub fn locksvtaircr(&mut self) -> LOCKSVTAIRCR_W<CSLOCKRrs> {
        LOCKSVTAIRCR_W::new(self, 0)
    }
    #[doc = "Bit 1 - LOCKSMPU"]
    #[inline(always)]
    #[must_use]
    pub fn locksmpu(&mut self) -> LOCKSMPU_W<CSLOCKRrs> {
        LOCKSMPU_W::new(self, 1)
    }
    #[doc = "Bit 2 - LOCKSAU"]
    #[inline(always)]
    #[must_use]
    pub fn locksau(&mut self) -> LOCKSAU_W<CSLOCKRrs> {
        LOCKSAU_W::new(self, 2)
    }
}
#[doc = "SYSCFG CPU secure lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cslockr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cslockr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSLOCKRrs;
impl crate::RegisterSpec for CSLOCKRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cslockr::R`](R) reader structure"]
impl crate::Readable for CSLOCKRrs {}
#[doc = "`write(|w| ..)` method takes [`cslockr::W`](W) writer structure"]
impl crate::Writable for CSLOCKRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSLOCKR to value 0"]
impl crate::Resettable for CSLOCKRrs {
    const RESET_VALUE: u32 = 0;
}

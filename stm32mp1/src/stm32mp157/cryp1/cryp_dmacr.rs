#[doc = "Register `CRYP_DMACR` reader"]
pub type R = crate::R<CRYP_DMACRrs>;
#[doc = "Register `CRYP_DMACR` writer"]
pub type W = crate::W<CRYP_DMACRrs>;
#[doc = "Field `DIEN` reader - DIEN"]
pub type DIEN_R = crate::BitReader;
#[doc = "Field `DIEN` writer - DIEN"]
pub type DIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOEN` reader - DOEN"]
pub type DOEN_R = crate::BitReader;
#[doc = "Field `DOEN` writer - DOEN"]
pub type DOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DIEN"]
    #[inline(always)]
    pub fn dien(&self) -> DIEN_R {
        DIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DOEN"]
    #[inline(always)]
    pub fn doen(&self) -> DOEN_R {
        DOEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DIEN"]
    #[inline(always)]
    #[must_use]
    pub fn dien(&mut self) -> DIEN_W<CRYP_DMACRrs> {
        DIEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - DOEN"]
    #[inline(always)]
    #[must_use]
    pub fn doen(&mut self) -> DOEN_W<CRYP_DMACRrs> {
        DOEN_W::new(self, 1)
    }
}
#[doc = "CRYP DMA control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cryp_dmacr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cryp_dmacr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRYP_DMACRrs;
impl crate::RegisterSpec for CRYP_DMACRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cryp_dmacr::R`](R) reader structure"]
impl crate::Readable for CRYP_DMACRrs {}
#[doc = "`write(|w| ..)` method takes [`cryp_dmacr::W`](W) writer structure"]
impl crate::Writable for CRYP_DMACRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRYP_DMACR to value 0"]
impl crate::Resettable for CRYP_DMACRrs {
    const RESET_VALUE: u32 = 0;
}

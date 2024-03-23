#[doc = "Register `LPDMA_RCFGLOCKR` reader"]
pub type R = crate::R<LPDMA_RCFGLOCKRrs>;
#[doc = "Register `LPDMA_RCFGLOCKR` writer"]
pub type W = crate::W<LPDMA_RCFGLOCKRrs>;
#[doc = "Field `LOCK0` reader - LOCK0"]
pub type LOCK0_R = crate::BitReader;
#[doc = "Field `LOCK0` writer - LOCK0"]
pub type LOCK0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK1` reader - LOCK1"]
pub type LOCK1_R = crate::BitReader;
#[doc = "Field `LOCK1` writer - LOCK1"]
pub type LOCK1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK2` reader - LOCK2"]
pub type LOCK2_R = crate::BitReader;
#[doc = "Field `LOCK2` writer - LOCK2"]
pub type LOCK2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK3` reader - LOCK3"]
pub type LOCK3_R = crate::BitReader;
#[doc = "Field `LOCK3` writer - LOCK3"]
pub type LOCK3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LOCK0"]
    #[inline(always)]
    pub fn lock0(&self) -> LOCK0_R {
        LOCK0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LOCK1"]
    #[inline(always)]
    pub fn lock1(&self) -> LOCK1_R {
        LOCK1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LOCK2"]
    #[inline(always)]
    pub fn lock2(&self) -> LOCK2_R {
        LOCK2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LOCK3"]
    #[inline(always)]
    pub fn lock3(&self) -> LOCK3_R {
        LOCK3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LOCK0"]
    #[inline(always)]
    #[must_use]
    pub fn lock0(&mut self) -> LOCK0_W<LPDMA_RCFGLOCKRrs> {
        LOCK0_W::new(self, 0)
    }
    #[doc = "Bit 1 - LOCK1"]
    #[inline(always)]
    #[must_use]
    pub fn lock1(&mut self) -> LOCK1_W<LPDMA_RCFGLOCKRrs> {
        LOCK1_W::new(self, 1)
    }
    #[doc = "Bit 2 - LOCK2"]
    #[inline(always)]
    #[must_use]
    pub fn lock2(&mut self) -> LOCK2_W<LPDMA_RCFGLOCKRrs> {
        LOCK2_W::new(self, 2)
    }
    #[doc = "Bit 3 - LOCK3"]
    #[inline(always)]
    #[must_use]
    pub fn lock3(&mut self) -> LOCK3_W<LPDMA_RCFGLOCKRrs> {
        LOCK3_W::new(self, 3)
    }
}
#[doc = "LPDMA configuration lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpdma_rcfglockr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpdma_rcfglockr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPDMA_RCFGLOCKRrs;
impl crate::RegisterSpec for LPDMA_RCFGLOCKRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpdma_rcfglockr::R`](R) reader structure"]
impl crate::Readable for LPDMA_RCFGLOCKRrs {}
#[doc = "`write(|w| ..)` method takes [`lpdma_rcfglockr::W`](W) writer structure"]
impl crate::Writable for LPDMA_RCFGLOCKRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPDMA_RCFGLOCKR to value 0"]
impl crate::Resettable for LPDMA_RCFGLOCKRrs {
    const RESET_VALUE: u32 = 0;
}

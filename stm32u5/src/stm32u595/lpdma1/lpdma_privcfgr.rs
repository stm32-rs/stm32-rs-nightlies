#[doc = "Register `LPDMA_PRIVCFGR` reader"]
pub type R = crate::R<LPDMA_PRIVCFGRrs>;
#[doc = "Register `LPDMA_PRIVCFGR` writer"]
pub type W = crate::W<LPDMA_PRIVCFGRrs>;
#[doc = "Field `PRIV0` reader - PRIV0"]
pub type PRIV0_R = crate::BitReader;
#[doc = "Field `PRIV0` writer - PRIV0"]
pub type PRIV0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV1` reader - PRIV1"]
pub type PRIV1_R = crate::BitReader;
#[doc = "Field `PRIV1` writer - PRIV1"]
pub type PRIV1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV2` reader - PRIV2"]
pub type PRIV2_R = crate::BitReader;
#[doc = "Field `PRIV2` writer - PRIV2"]
pub type PRIV2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV3` reader - PRIV3"]
pub type PRIV3_R = crate::BitReader;
#[doc = "Field `PRIV3` writer - PRIV3"]
pub type PRIV3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PRIV0"]
    #[inline(always)]
    pub fn priv0(&self) -> PRIV0_R {
        PRIV0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PRIV1"]
    #[inline(always)]
    pub fn priv1(&self) -> PRIV1_R {
        PRIV1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PRIV2"]
    #[inline(always)]
    pub fn priv2(&self) -> PRIV2_R {
        PRIV2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PRIV3"]
    #[inline(always)]
    pub fn priv3(&self) -> PRIV3_R {
        PRIV3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PRIV0"]
    #[inline(always)]
    #[must_use]
    pub fn priv0(&mut self) -> PRIV0_W<LPDMA_PRIVCFGRrs> {
        PRIV0_W::new(self, 0)
    }
    #[doc = "Bit 1 - PRIV1"]
    #[inline(always)]
    #[must_use]
    pub fn priv1(&mut self) -> PRIV1_W<LPDMA_PRIVCFGRrs> {
        PRIV1_W::new(self, 1)
    }
    #[doc = "Bit 2 - PRIV2"]
    #[inline(always)]
    #[must_use]
    pub fn priv2(&mut self) -> PRIV2_W<LPDMA_PRIVCFGRrs> {
        PRIV2_W::new(self, 2)
    }
    #[doc = "Bit 3 - PRIV3"]
    #[inline(always)]
    #[must_use]
    pub fn priv3(&mut self) -> PRIV3_W<LPDMA_PRIVCFGRrs> {
        PRIV3_W::new(self, 3)
    }
}
#[doc = "LPDMA privileged configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpdma_privcfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpdma_privcfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPDMA_PRIVCFGRrs;
impl crate::RegisterSpec for LPDMA_PRIVCFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpdma_privcfgr::R`](R) reader structure"]
impl crate::Readable for LPDMA_PRIVCFGRrs {}
#[doc = "`write(|w| ..)` method takes [`lpdma_privcfgr::W`](W) writer structure"]
impl crate::Writable for LPDMA_PRIVCFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPDMA_PRIVCFGR to value 0"]
impl crate::Resettable for LPDMA_PRIVCFGRrs {
    const RESET_VALUE: u32 = 0;
}

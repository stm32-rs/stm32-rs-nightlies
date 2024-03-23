#[doc = "Register `GPDMA_PRIVCFGR` reader"]
pub type R = crate::R<GPDMA_PRIVCFGRrs>;
#[doc = "Register `GPDMA_PRIVCFGR` writer"]
pub type W = crate::W<GPDMA_PRIVCFGRrs>;
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
#[doc = "Field `PRIV4` reader - PRIV4"]
pub type PRIV4_R = crate::BitReader;
#[doc = "Field `PRIV4` writer - PRIV4"]
pub type PRIV4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV5` reader - PRIV5"]
pub type PRIV5_R = crate::BitReader;
#[doc = "Field `PRIV5` writer - PRIV5"]
pub type PRIV5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV6` reader - PRIV6"]
pub type PRIV6_R = crate::BitReader;
#[doc = "Field `PRIV6` writer - PRIV6"]
pub type PRIV6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV7` reader - PRIV7"]
pub type PRIV7_R = crate::BitReader;
#[doc = "Field `PRIV7` writer - PRIV7"]
pub type PRIV7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV8` reader - PRIV8"]
pub type PRIV8_R = crate::BitReader;
#[doc = "Field `PRIV8` writer - PRIV8"]
pub type PRIV8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV9` reader - PRIV9"]
pub type PRIV9_R = crate::BitReader;
#[doc = "Field `PRIV9` writer - PRIV9"]
pub type PRIV9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV10` reader - PRIV10"]
pub type PRIV10_R = crate::BitReader;
#[doc = "Field `PRIV10` writer - PRIV10"]
pub type PRIV10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV11` reader - PRIV11"]
pub type PRIV11_R = crate::BitReader;
#[doc = "Field `PRIV11` writer - PRIV11"]
pub type PRIV11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV12` reader - PRIV12"]
pub type PRIV12_R = crate::BitReader;
#[doc = "Field `PRIV12` writer - PRIV12"]
pub type PRIV12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV13` reader - PRIV13"]
pub type PRIV13_R = crate::BitReader;
#[doc = "Field `PRIV13` writer - PRIV13"]
pub type PRIV13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV14` reader - PRIV14"]
pub type PRIV14_R = crate::BitReader;
#[doc = "Field `PRIV14` writer - PRIV14"]
pub type PRIV14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV15` reader - PRIV15"]
pub type PRIV15_R = crate::BitReader;
#[doc = "Field `PRIV15` writer - PRIV15"]
pub type PRIV15_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 4 - PRIV4"]
    #[inline(always)]
    pub fn priv4(&self) -> PRIV4_R {
        PRIV4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PRIV5"]
    #[inline(always)]
    pub fn priv5(&self) -> PRIV5_R {
        PRIV5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PRIV6"]
    #[inline(always)]
    pub fn priv6(&self) -> PRIV6_R {
        PRIV6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PRIV7"]
    #[inline(always)]
    pub fn priv7(&self) -> PRIV7_R {
        PRIV7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PRIV8"]
    #[inline(always)]
    pub fn priv8(&self) -> PRIV8_R {
        PRIV8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PRIV9"]
    #[inline(always)]
    pub fn priv9(&self) -> PRIV9_R {
        PRIV9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PRIV10"]
    #[inline(always)]
    pub fn priv10(&self) -> PRIV10_R {
        PRIV10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PRIV11"]
    #[inline(always)]
    pub fn priv11(&self) -> PRIV11_R {
        PRIV11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PRIV12"]
    #[inline(always)]
    pub fn priv12(&self) -> PRIV12_R {
        PRIV12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PRIV13"]
    #[inline(always)]
    pub fn priv13(&self) -> PRIV13_R {
        PRIV13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PRIV14"]
    #[inline(always)]
    pub fn priv14(&self) -> PRIV14_R {
        PRIV14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PRIV15"]
    #[inline(always)]
    pub fn priv15(&self) -> PRIV15_R {
        PRIV15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PRIV0"]
    #[inline(always)]
    #[must_use]
    pub fn priv0(&mut self) -> PRIV0_W<GPDMA_PRIVCFGRrs> {
        PRIV0_W::new(self, 0)
    }
    #[doc = "Bit 1 - PRIV1"]
    #[inline(always)]
    #[must_use]
    pub fn priv1(&mut self) -> PRIV1_W<GPDMA_PRIVCFGRrs> {
        PRIV1_W::new(self, 1)
    }
    #[doc = "Bit 2 - PRIV2"]
    #[inline(always)]
    #[must_use]
    pub fn priv2(&mut self) -> PRIV2_W<GPDMA_PRIVCFGRrs> {
        PRIV2_W::new(self, 2)
    }
    #[doc = "Bit 3 - PRIV3"]
    #[inline(always)]
    #[must_use]
    pub fn priv3(&mut self) -> PRIV3_W<GPDMA_PRIVCFGRrs> {
        PRIV3_W::new(self, 3)
    }
    #[doc = "Bit 4 - PRIV4"]
    #[inline(always)]
    #[must_use]
    pub fn priv4(&mut self) -> PRIV4_W<GPDMA_PRIVCFGRrs> {
        PRIV4_W::new(self, 4)
    }
    #[doc = "Bit 5 - PRIV5"]
    #[inline(always)]
    #[must_use]
    pub fn priv5(&mut self) -> PRIV5_W<GPDMA_PRIVCFGRrs> {
        PRIV5_W::new(self, 5)
    }
    #[doc = "Bit 6 - PRIV6"]
    #[inline(always)]
    #[must_use]
    pub fn priv6(&mut self) -> PRIV6_W<GPDMA_PRIVCFGRrs> {
        PRIV6_W::new(self, 6)
    }
    #[doc = "Bit 7 - PRIV7"]
    #[inline(always)]
    #[must_use]
    pub fn priv7(&mut self) -> PRIV7_W<GPDMA_PRIVCFGRrs> {
        PRIV7_W::new(self, 7)
    }
    #[doc = "Bit 8 - PRIV8"]
    #[inline(always)]
    #[must_use]
    pub fn priv8(&mut self) -> PRIV8_W<GPDMA_PRIVCFGRrs> {
        PRIV8_W::new(self, 8)
    }
    #[doc = "Bit 9 - PRIV9"]
    #[inline(always)]
    #[must_use]
    pub fn priv9(&mut self) -> PRIV9_W<GPDMA_PRIVCFGRrs> {
        PRIV9_W::new(self, 9)
    }
    #[doc = "Bit 10 - PRIV10"]
    #[inline(always)]
    #[must_use]
    pub fn priv10(&mut self) -> PRIV10_W<GPDMA_PRIVCFGRrs> {
        PRIV10_W::new(self, 10)
    }
    #[doc = "Bit 11 - PRIV11"]
    #[inline(always)]
    #[must_use]
    pub fn priv11(&mut self) -> PRIV11_W<GPDMA_PRIVCFGRrs> {
        PRIV11_W::new(self, 11)
    }
    #[doc = "Bit 12 - PRIV12"]
    #[inline(always)]
    #[must_use]
    pub fn priv12(&mut self) -> PRIV12_W<GPDMA_PRIVCFGRrs> {
        PRIV12_W::new(self, 12)
    }
    #[doc = "Bit 13 - PRIV13"]
    #[inline(always)]
    #[must_use]
    pub fn priv13(&mut self) -> PRIV13_W<GPDMA_PRIVCFGRrs> {
        PRIV13_W::new(self, 13)
    }
    #[doc = "Bit 14 - PRIV14"]
    #[inline(always)]
    #[must_use]
    pub fn priv14(&mut self) -> PRIV14_W<GPDMA_PRIVCFGRrs> {
        PRIV14_W::new(self, 14)
    }
    #[doc = "Bit 15 - PRIV15"]
    #[inline(always)]
    #[must_use]
    pub fn priv15(&mut self) -> PRIV15_W<GPDMA_PRIVCFGRrs> {
        PRIV15_W::new(self, 15)
    }
}
#[doc = "GPDMA privileged configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpdma_privcfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpdma_privcfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPDMA_PRIVCFGRrs;
impl crate::RegisterSpec for GPDMA_PRIVCFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpdma_privcfgr::R`](R) reader structure"]
impl crate::Readable for GPDMA_PRIVCFGRrs {}
#[doc = "`write(|w| ..)` method takes [`gpdma_privcfgr::W`](W) writer structure"]
impl crate::Writable for GPDMA_PRIVCFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPDMA_PRIVCFGR to value 0"]
impl crate::Resettable for GPDMA_PRIVCFGRrs {
    const RESET_VALUE: u32 = 0;
}

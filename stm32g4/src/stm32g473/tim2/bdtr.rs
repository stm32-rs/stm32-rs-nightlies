#[doc = "Register `BDTR` reader"]
pub type R = crate::R<BDTRrs>;
#[doc = "Register `BDTR` writer"]
pub type W = crate::W<BDTRrs>;
#[doc = "Field `DTG` reader - Dead-time generator setup"]
pub type DTG_R = crate::FieldReader;
#[doc = "Field `DTG` writer - Dead-time generator setup"]
pub type DTG_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LOCK` reader - Lock configuration"]
pub type LOCK_R = crate::FieldReader;
#[doc = "Field `LOCK` writer - Lock configuration"]
pub type LOCK_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSSI` reader - Off-state selection for Idle mode"]
pub type OSSI_R = crate::BitReader;
#[doc = "Field `OSSI` writer - Off-state selection for Idle mode"]
pub type OSSI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSSR` reader - Off-state selection for Run mode"]
pub type OSSR_R = crate::BitReader;
#[doc = "Field `OSSR` writer - Off-state selection for Run mode"]
pub type OSSR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKE` reader - Break enable"]
pub type BKE_R = crate::BitReader;
#[doc = "Field `BKE` writer - Break enable"]
pub type BKE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKP` reader - Break polarity"]
pub type BKP_R = crate::BitReader;
#[doc = "Field `BKP` writer - Break polarity"]
pub type BKP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AOE` reader - Automatic output enable"]
pub type AOE_R = crate::BitReader;
#[doc = "Field `AOE` writer - Automatic output enable"]
pub type AOE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MOE` reader - Main output enable"]
pub type MOE_R = crate::BitReader;
#[doc = "Field `MOE` writer - Main output enable"]
pub type MOE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKF` reader - Break filter"]
pub type BKF_R = crate::FieldReader;
#[doc = "Field `BKF` writer - Break filter"]
pub type BKF_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BK2F` reader - Break 2 filter"]
pub type BK2F_R = crate::FieldReader;
#[doc = "Field `BK2F` writer - Break 2 filter"]
pub type BK2F_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BK2E` reader - Break 2 Enable"]
pub type BK2E_R = crate::BitReader;
#[doc = "Field `BK2E` writer - Break 2 Enable"]
pub type BK2E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2P` reader - Break 2 polarity"]
pub type BK2P_R = crate::BitReader;
#[doc = "Field `BK2P` writer - Break 2 polarity"]
pub type BK2P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKDSRM` reader - BKDSRM"]
pub type BKDSRM_R = crate::BitReader;
#[doc = "Field `BKDSRM` writer - BKDSRM"]
pub type BKDSRM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2DSRM` reader - BK2DSRM"]
pub type BK2DSRM_R = crate::BitReader;
#[doc = "Field `BK2DSRM` writer - BK2DSRM"]
pub type BK2DSRM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKBID` reader - BKBID"]
pub type BKBID_R = crate::BitReader;
#[doc = "Field `BKBID` writer - BKBID"]
pub type BKBID_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2ID` reader - BK2ID"]
pub type BK2ID_R = crate::BitReader;
#[doc = "Field `BK2ID` writer - BK2ID"]
pub type BK2ID_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Dead-time generator setup"]
    #[inline(always)]
    pub fn dtg(&self) -> DTG_R {
        DTG_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - Lock configuration"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Off-state selection for Idle mode"]
    #[inline(always)]
    pub fn ossi(&self) -> OSSI_R {
        OSSI_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Off-state selection for Run mode"]
    #[inline(always)]
    pub fn ossr(&self) -> OSSR_R {
        OSSR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Break enable"]
    #[inline(always)]
    pub fn bke(&self) -> BKE_R {
        BKE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Break polarity"]
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Automatic output enable"]
    #[inline(always)]
    pub fn aoe(&self) -> AOE_R {
        AOE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Main output enable"]
    #[inline(always)]
    pub fn moe(&self) -> MOE_R {
        MOE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Break filter"]
    #[inline(always)]
    pub fn bkf(&self) -> BKF_R {
        BKF_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Break 2 filter"]
    #[inline(always)]
    pub fn bk2f(&self) -> BK2F_R {
        BK2F_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Break 2 Enable"]
    #[inline(always)]
    pub fn bk2e(&self) -> BK2E_R {
        BK2E_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Break 2 polarity"]
    #[inline(always)]
    pub fn bk2p(&self) -> BK2P_R {
        BK2P_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - BKDSRM"]
    #[inline(always)]
    pub fn bkdsrm(&self) -> BKDSRM_R {
        BKDSRM_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - BK2DSRM"]
    #[inline(always)]
    pub fn bk2dsrm(&self) -> BK2DSRM_R {
        BK2DSRM_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - BKBID"]
    #[inline(always)]
    pub fn bkbid(&self) -> BKBID_R {
        BKBID_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - BK2ID"]
    #[inline(always)]
    pub fn bk2id(&self) -> BK2ID_R {
        BK2ID_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Dead-time generator setup"]
    #[inline(always)]
    #[must_use]
    pub fn dtg(&mut self) -> DTG_W<BDTRrs> {
        DTG_W::new(self, 0)
    }
    #[doc = "Bits 8:9 - Lock configuration"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<BDTRrs> {
        LOCK_W::new(self, 8)
    }
    #[doc = "Bit 10 - Off-state selection for Idle mode"]
    #[inline(always)]
    #[must_use]
    pub fn ossi(&mut self) -> OSSI_W<BDTRrs> {
        OSSI_W::new(self, 10)
    }
    #[doc = "Bit 11 - Off-state selection for Run mode"]
    #[inline(always)]
    #[must_use]
    pub fn ossr(&mut self) -> OSSR_W<BDTRrs> {
        OSSR_W::new(self, 11)
    }
    #[doc = "Bit 12 - Break enable"]
    #[inline(always)]
    #[must_use]
    pub fn bke(&mut self) -> BKE_W<BDTRrs> {
        BKE_W::new(self, 12)
    }
    #[doc = "Bit 13 - Break polarity"]
    #[inline(always)]
    #[must_use]
    pub fn bkp(&mut self) -> BKP_W<BDTRrs> {
        BKP_W::new(self, 13)
    }
    #[doc = "Bit 14 - Automatic output enable"]
    #[inline(always)]
    #[must_use]
    pub fn aoe(&mut self) -> AOE_W<BDTRrs> {
        AOE_W::new(self, 14)
    }
    #[doc = "Bit 15 - Main output enable"]
    #[inline(always)]
    #[must_use]
    pub fn moe(&mut self) -> MOE_W<BDTRrs> {
        MOE_W::new(self, 15)
    }
    #[doc = "Bits 16:19 - Break filter"]
    #[inline(always)]
    #[must_use]
    pub fn bkf(&mut self) -> BKF_W<BDTRrs> {
        BKF_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Break 2 filter"]
    #[inline(always)]
    #[must_use]
    pub fn bk2f(&mut self) -> BK2F_W<BDTRrs> {
        BK2F_W::new(self, 20)
    }
    #[doc = "Bit 24 - Break 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bk2e(&mut self) -> BK2E_W<BDTRrs> {
        BK2E_W::new(self, 24)
    }
    #[doc = "Bit 25 - Break 2 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn bk2p(&mut self) -> BK2P_W<BDTRrs> {
        BK2P_W::new(self, 25)
    }
    #[doc = "Bit 26 - BKDSRM"]
    #[inline(always)]
    #[must_use]
    pub fn bkdsrm(&mut self) -> BKDSRM_W<BDTRrs> {
        BKDSRM_W::new(self, 26)
    }
    #[doc = "Bit 27 - BK2DSRM"]
    #[inline(always)]
    #[must_use]
    pub fn bk2dsrm(&mut self) -> BK2DSRM_W<BDTRrs> {
        BK2DSRM_W::new(self, 27)
    }
    #[doc = "Bit 28 - BKBID"]
    #[inline(always)]
    #[must_use]
    pub fn bkbid(&mut self) -> BKBID_W<BDTRrs> {
        BKBID_W::new(self, 28)
    }
    #[doc = "Bit 29 - BK2ID"]
    #[inline(always)]
    #[must_use]
    pub fn bk2id(&mut self) -> BK2ID_W<BDTRrs> {
        BK2ID_W::new(self, 29)
    }
}
#[doc = "break and dead-time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdtr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdtr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BDTRrs;
impl crate::RegisterSpec for BDTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bdtr::R`](R) reader structure"]
impl crate::Readable for BDTRrs {}
#[doc = "`write(|w| ..)` method takes [`bdtr::W`](W) writer structure"]
impl crate::Writable for BDTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BDTR to value 0"]
impl crate::Resettable for BDTRrs {
    const RESET_VALUE: u32 = 0;
}

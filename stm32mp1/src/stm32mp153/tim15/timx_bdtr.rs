#[doc = "Register `TIMx_BDTR` reader"]
pub type R = crate::R<TIMX_BDTRrs>;
#[doc = "Register `TIMx_BDTR` writer"]
pub type W = crate::W<TIMX_BDTRrs>;
#[doc = "Field `DTG` reader - DTG"]
pub type DTG_R = crate::FieldReader;
#[doc = "Field `DTG` writer - DTG"]
pub type DTG_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LOCK` reader - LOCK"]
pub type LOCK_R = crate::FieldReader;
#[doc = "Field `LOCK` writer - LOCK"]
pub type LOCK_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSSI` reader - OSSI"]
pub type OSSI_R = crate::BitReader;
#[doc = "Field `OSSI` writer - OSSI"]
pub type OSSI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSSR` reader - OSSR"]
pub type OSSR_R = crate::BitReader;
#[doc = "Field `OSSR` writer - OSSR"]
pub type OSSR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKE` reader - BKE"]
pub type BKE_R = crate::BitReader;
#[doc = "Field `BKE` writer - BKE"]
pub type BKE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKP` reader - BKP"]
pub type BKP_R = crate::BitReader;
#[doc = "Field `BKP` writer - BKP"]
pub type BKP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AOE` reader - AOE"]
pub type AOE_R = crate::BitReader;
#[doc = "Field `AOE` writer - AOE"]
pub type AOE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MOE` reader - MOE"]
pub type MOE_R = crate::BitReader;
#[doc = "Field `MOE` writer - MOE"]
pub type MOE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKF` reader - BKF"]
pub type BKF_R = crate::FieldReader;
#[doc = "Field `BKF` writer - BKF"]
pub type BKF_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BKDSRM` reader - BKDSRM"]
pub type BKDSRM_R = crate::BitReader;
#[doc = "Field `BKDSRM` writer - BKDSRM"]
pub type BKDSRM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKBID` reader - BKBID"]
pub type BKBID_R = crate::BitReader;
#[doc = "Field `BKBID` writer - BKBID"]
pub type BKBID_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - DTG"]
    #[inline(always)]
    pub fn dtg(&self) -> DTG_R {
        DTG_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - LOCK"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - OSSI"]
    #[inline(always)]
    pub fn ossi(&self) -> OSSI_R {
        OSSI_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - OSSR"]
    #[inline(always)]
    pub fn ossr(&self) -> OSSR_R {
        OSSR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - BKE"]
    #[inline(always)]
    pub fn bke(&self) -> BKE_R {
        BKE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - BKP"]
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - AOE"]
    #[inline(always)]
    pub fn aoe(&self) -> AOE_R {
        AOE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - MOE"]
    #[inline(always)]
    pub fn moe(&self) -> MOE_R {
        MOE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - BKF"]
    #[inline(always)]
    pub fn bkf(&self) -> BKF_R {
        BKF_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 26 - BKDSRM"]
    #[inline(always)]
    pub fn bkdsrm(&self) -> BKDSRM_R {
        BKDSRM_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - BKBID"]
    #[inline(always)]
    pub fn bkbid(&self) -> BKBID_R {
        BKBID_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - DTG"]
    #[inline(always)]
    #[must_use]
    pub fn dtg(&mut self) -> DTG_W<TIMX_BDTRrs> {
        DTG_W::new(self, 0)
    }
    #[doc = "Bits 8:9 - LOCK"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<TIMX_BDTRrs> {
        LOCK_W::new(self, 8)
    }
    #[doc = "Bit 10 - OSSI"]
    #[inline(always)]
    #[must_use]
    pub fn ossi(&mut self) -> OSSI_W<TIMX_BDTRrs> {
        OSSI_W::new(self, 10)
    }
    #[doc = "Bit 11 - OSSR"]
    #[inline(always)]
    #[must_use]
    pub fn ossr(&mut self) -> OSSR_W<TIMX_BDTRrs> {
        OSSR_W::new(self, 11)
    }
    #[doc = "Bit 12 - BKE"]
    #[inline(always)]
    #[must_use]
    pub fn bke(&mut self) -> BKE_W<TIMX_BDTRrs> {
        BKE_W::new(self, 12)
    }
    #[doc = "Bit 13 - BKP"]
    #[inline(always)]
    #[must_use]
    pub fn bkp(&mut self) -> BKP_W<TIMX_BDTRrs> {
        BKP_W::new(self, 13)
    }
    #[doc = "Bit 14 - AOE"]
    #[inline(always)]
    #[must_use]
    pub fn aoe(&mut self) -> AOE_W<TIMX_BDTRrs> {
        AOE_W::new(self, 14)
    }
    #[doc = "Bit 15 - MOE"]
    #[inline(always)]
    #[must_use]
    pub fn moe(&mut self) -> MOE_W<TIMX_BDTRrs> {
        MOE_W::new(self, 15)
    }
    #[doc = "Bits 16:19 - BKF"]
    #[inline(always)]
    #[must_use]
    pub fn bkf(&mut self) -> BKF_W<TIMX_BDTRrs> {
        BKF_W::new(self, 16)
    }
    #[doc = "Bit 26 - BKDSRM"]
    #[inline(always)]
    #[must_use]
    pub fn bkdsrm(&mut self) -> BKDSRM_W<TIMX_BDTRrs> {
        BKDSRM_W::new(self, 26)
    }
    #[doc = "Bit 28 - BKBID"]
    #[inline(always)]
    #[must_use]
    pub fn bkbid(&mut self) -> BKBID_W<TIMX_BDTRrs> {
        BKBID_W::new(self, 28)
    }
}
#[doc = "As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timx_bdtr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timx_bdtr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMX_BDTRrs;
impl crate::RegisterSpec for TIMX_BDTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timx_bdtr::R`](R) reader structure"]
impl crate::Readable for TIMX_BDTRrs {}
#[doc = "`write(|w| ..)` method takes [`timx_bdtr::W`](W) writer structure"]
impl crate::Writable for TIMX_BDTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMx_BDTR to value 0"]
impl crate::Resettable for TIMX_BDTRrs {
    const RESET_VALUE: u32 = 0;
}

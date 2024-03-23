#[doc = "Register `TIM2_BDTR` reader"]
pub type R = crate::R<TIM2_BDTRrs>;
#[doc = "Register `TIM2_BDTR` writer"]
pub type W = crate::W<TIM2_BDTRrs>;
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
#[doc = "Field `BK2F` reader - BK2F"]
pub type BK2F_R = crate::FieldReader;
#[doc = "Field `BK2F` writer - BK2F"]
pub type BK2F_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BK2E` reader - BK2E"]
pub type BK2E_R = crate::BitReader;
#[doc = "Field `BK2E` writer - BK2E"]
pub type BK2E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2P` reader - BK2P"]
pub type BK2P_R = crate::BitReader;
#[doc = "Field `BK2P` writer - BK2P"]
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
#[doc = "Field `BK2BID` reader - BK2BID"]
pub type BK2BID_R = crate::BitReader;
#[doc = "Field `BK2BID` writer - BK2BID"]
pub type BK2BID_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bits 20:23 - BK2F"]
    #[inline(always)]
    pub fn bk2f(&self) -> BK2F_R {
        BK2F_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - BK2E"]
    #[inline(always)]
    pub fn bk2e(&self) -> BK2E_R {
        BK2E_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - BK2P"]
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
    #[doc = "Bit 29 - BK2BID"]
    #[inline(always)]
    pub fn bk2bid(&self) -> BK2BID_R {
        BK2BID_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - DTG"]
    #[inline(always)]
    #[must_use]
    pub fn dtg(&mut self) -> DTG_W<TIM2_BDTRrs> {
        DTG_W::new(self, 0)
    }
    #[doc = "Bits 8:9 - LOCK"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<TIM2_BDTRrs> {
        LOCK_W::new(self, 8)
    }
    #[doc = "Bit 10 - OSSI"]
    #[inline(always)]
    #[must_use]
    pub fn ossi(&mut self) -> OSSI_W<TIM2_BDTRrs> {
        OSSI_W::new(self, 10)
    }
    #[doc = "Bit 11 - OSSR"]
    #[inline(always)]
    #[must_use]
    pub fn ossr(&mut self) -> OSSR_W<TIM2_BDTRrs> {
        OSSR_W::new(self, 11)
    }
    #[doc = "Bit 12 - BKE"]
    #[inline(always)]
    #[must_use]
    pub fn bke(&mut self) -> BKE_W<TIM2_BDTRrs> {
        BKE_W::new(self, 12)
    }
    #[doc = "Bit 13 - BKP"]
    #[inline(always)]
    #[must_use]
    pub fn bkp(&mut self) -> BKP_W<TIM2_BDTRrs> {
        BKP_W::new(self, 13)
    }
    #[doc = "Bit 14 - AOE"]
    #[inline(always)]
    #[must_use]
    pub fn aoe(&mut self) -> AOE_W<TIM2_BDTRrs> {
        AOE_W::new(self, 14)
    }
    #[doc = "Bit 15 - MOE"]
    #[inline(always)]
    #[must_use]
    pub fn moe(&mut self) -> MOE_W<TIM2_BDTRrs> {
        MOE_W::new(self, 15)
    }
    #[doc = "Bits 16:19 - BKF"]
    #[inline(always)]
    #[must_use]
    pub fn bkf(&mut self) -> BKF_W<TIM2_BDTRrs> {
        BKF_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - BK2F"]
    #[inline(always)]
    #[must_use]
    pub fn bk2f(&mut self) -> BK2F_W<TIM2_BDTRrs> {
        BK2F_W::new(self, 20)
    }
    #[doc = "Bit 24 - BK2E"]
    #[inline(always)]
    #[must_use]
    pub fn bk2e(&mut self) -> BK2E_W<TIM2_BDTRrs> {
        BK2E_W::new(self, 24)
    }
    #[doc = "Bit 25 - BK2P"]
    #[inline(always)]
    #[must_use]
    pub fn bk2p(&mut self) -> BK2P_W<TIM2_BDTRrs> {
        BK2P_W::new(self, 25)
    }
    #[doc = "Bit 26 - BKDSRM"]
    #[inline(always)]
    #[must_use]
    pub fn bkdsrm(&mut self) -> BKDSRM_W<TIM2_BDTRrs> {
        BKDSRM_W::new(self, 26)
    }
    #[doc = "Bit 27 - BK2DSRM"]
    #[inline(always)]
    #[must_use]
    pub fn bk2dsrm(&mut self) -> BK2DSRM_W<TIM2_BDTRrs> {
        BK2DSRM_W::new(self, 27)
    }
    #[doc = "Bit 28 - BKBID"]
    #[inline(always)]
    #[must_use]
    pub fn bkbid(&mut self) -> BKBID_W<TIM2_BDTRrs> {
        BKBID_W::new(self, 28)
    }
    #[doc = "Bit 29 - BK2BID"]
    #[inline(always)]
    #[must_use]
    pub fn bk2bid(&mut self) -> BK2BID_W<TIM2_BDTRrs> {
        BK2BID_W::new(self, 29)
    }
}
#[doc = "As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim2_bdtr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim2_bdtr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM2_BDTRrs;
impl crate::RegisterSpec for TIM2_BDTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim2_bdtr::R`](R) reader structure"]
impl crate::Readable for TIM2_BDTRrs {}
#[doc = "`write(|w| ..)` method takes [`tim2_bdtr::W`](W) writer structure"]
impl crate::Writable for TIM2_BDTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM2_BDTR to value 0"]
impl crate::Resettable for TIM2_BDTRrs {
    const RESET_VALUE: u32 = 0;
}

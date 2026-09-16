///Register `BDTR` reader
pub type R = crate::R<BDTRrs>;
///Register `BDTR` writer
pub type W = crate::W<BDTRrs>;
///Field `DTG` reader - DTG
pub type DTG_R = crate::FieldReader;
///Field `DTG` writer - DTG
pub type DTG_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `LOCK` reader - LOCK
pub type LOCK_R = crate::FieldReader;
///Field `LOCK` writer - LOCK
pub type LOCK_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OSSI` reader - OSSI
pub type OSSI_R = crate::BitReader;
///Field `OSSI` writer - OSSI
pub type OSSI_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OSSR` reader - OSSR
pub type OSSR_R = crate::BitReader;
///Field `OSSR` writer - OSSR
pub type OSSR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKE` reader - BKE
pub type BKE_R = crate::BitReader;
///Field `BKE` writer - BKE
pub type BKE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKP` reader - BKP
pub type BKP_R = crate::BitReader;
///Field `BKP` writer - BKP
pub type BKP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AOE` reader - AOE
pub type AOE_R = crate::BitReader;
///Field `AOE` writer - AOE
pub type AOE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MOE` reader - MOE
pub type MOE_R = crate::BitReader;
///Field `MOE` writer - MOE
pub type MOE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKF` reader - BKF
pub type BKF_R = crate::FieldReader;
///Field `BKF` writer - BKF
pub type BKF_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `BK2F` reader - BK2F
pub type BK2F_R = crate::FieldReader;
///Field `BK2F` writer - BK2F
pub type BK2F_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `BK2E` reader - BK2E
pub type BK2E_R = crate::BitReader;
///Field `BK2E` writer - BK2E
pub type BK2E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BK2P` reader - BK2P
pub type BK2P_R = crate::BitReader;
///Field `BK2P` writer - BK2P
pub type BK2P_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKDSRM` reader - BKDSRM
pub type BKDSRM_R = crate::BitReader;
///Field `BKDSRM` writer - BKDSRM
pub type BKDSRM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BK2DSRM` reader - BK2DSRM
pub type BK2DSRM_R = crate::BitReader;
///Field `BK2DSRM` writer - BK2DSRM
pub type BK2DSRM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKBID` reader - BKBID
pub type BKBID_R = crate::BitReader;
///Field `BKBID` writer - BKBID
pub type BKBID_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BK2BID` reader - BK2BID
pub type BK2BID_R = crate::BitReader;
///Field `BK2BID` writer - BK2BID
pub type BK2BID_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - DTG
    #[inline(always)]
    pub fn dtg(&self) -> DTG_R {
        DTG_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:9 - LOCK
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 10 - OSSI
    #[inline(always)]
    pub fn ossi(&self) -> OSSI_R {
        OSSI_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - OSSR
    #[inline(always)]
    pub fn ossr(&self) -> OSSR_R {
        OSSR_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - BKE
    #[inline(always)]
    pub fn bke(&self) -> BKE_R {
        BKE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - BKP
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - AOE
    #[inline(always)]
    pub fn aoe(&self) -> AOE_R {
        AOE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - MOE
    #[inline(always)]
    pub fn moe(&self) -> MOE_R {
        MOE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:19 - BKF
    #[inline(always)]
    pub fn bkf(&self) -> BKF_R {
        BKF_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - BK2F
    #[inline(always)]
    pub fn bk2f(&self) -> BK2F_R {
        BK2F_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bit 24 - BK2E
    #[inline(always)]
    pub fn bk2e(&self) -> BK2E_R {
        BK2E_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - BK2P
    #[inline(always)]
    pub fn bk2p(&self) -> BK2P_R {
        BK2P_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - BKDSRM
    #[inline(always)]
    pub fn bkdsrm(&self) -> BKDSRM_R {
        BKDSRM_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - BK2DSRM
    #[inline(always)]
    pub fn bk2dsrm(&self) -> BK2DSRM_R {
        BK2DSRM_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - BKBID
    #[inline(always)]
    pub fn bkbid(&self) -> BKBID_R {
        BKBID_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - BK2BID
    #[inline(always)]
    pub fn bk2bid(&self) -> BK2BID_R {
        BK2BID_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BDTR")
            .field("dtg", &self.dtg())
            .field("lock", &self.lock())
            .field("ossi", &self.ossi())
            .field("ossr", &self.ossr())
            .field("bke", &self.bke())
            .field("bkp", &self.bkp())
            .field("aoe", &self.aoe())
            .field("moe", &self.moe())
            .field("bkf", &self.bkf())
            .field("bk2f", &self.bk2f())
            .field("bk2e", &self.bk2e())
            .field("bk2p", &self.bk2p())
            .field("bkdsrm", &self.bkdsrm())
            .field("bk2dsrm", &self.bk2dsrm())
            .field("bkbid", &self.bkbid())
            .field("bk2bid", &self.bk2bid())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - DTG
    #[inline(always)]
    pub fn dtg(&mut self) -> DTG_W<'_, BDTRrs> {
        DTG_W::new(self, 0)
    }
    ///Bits 8:9 - LOCK
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<'_, BDTRrs> {
        LOCK_W::new(self, 8)
    }
    ///Bit 10 - OSSI
    #[inline(always)]
    pub fn ossi(&mut self) -> OSSI_W<'_, BDTRrs> {
        OSSI_W::new(self, 10)
    }
    ///Bit 11 - OSSR
    #[inline(always)]
    pub fn ossr(&mut self) -> OSSR_W<'_, BDTRrs> {
        OSSR_W::new(self, 11)
    }
    ///Bit 12 - BKE
    #[inline(always)]
    pub fn bke(&mut self) -> BKE_W<'_, BDTRrs> {
        BKE_W::new(self, 12)
    }
    ///Bit 13 - BKP
    #[inline(always)]
    pub fn bkp(&mut self) -> BKP_W<'_, BDTRrs> {
        BKP_W::new(self, 13)
    }
    ///Bit 14 - AOE
    #[inline(always)]
    pub fn aoe(&mut self) -> AOE_W<'_, BDTRrs> {
        AOE_W::new(self, 14)
    }
    ///Bit 15 - MOE
    #[inline(always)]
    pub fn moe(&mut self) -> MOE_W<'_, BDTRrs> {
        MOE_W::new(self, 15)
    }
    ///Bits 16:19 - BKF
    #[inline(always)]
    pub fn bkf(&mut self) -> BKF_W<'_, BDTRrs> {
        BKF_W::new(self, 16)
    }
    ///Bits 20:23 - BK2F
    #[inline(always)]
    pub fn bk2f(&mut self) -> BK2F_W<'_, BDTRrs> {
        BK2F_W::new(self, 20)
    }
    ///Bit 24 - BK2E
    #[inline(always)]
    pub fn bk2e(&mut self) -> BK2E_W<'_, BDTRrs> {
        BK2E_W::new(self, 24)
    }
    ///Bit 25 - BK2P
    #[inline(always)]
    pub fn bk2p(&mut self) -> BK2P_W<'_, BDTRrs> {
        BK2P_W::new(self, 25)
    }
    ///Bit 26 - BKDSRM
    #[inline(always)]
    pub fn bkdsrm(&mut self) -> BKDSRM_W<'_, BDTRrs> {
        BKDSRM_W::new(self, 26)
    }
    ///Bit 27 - BK2DSRM
    #[inline(always)]
    pub fn bk2dsrm(&mut self) -> BK2DSRM_W<'_, BDTRrs> {
        BK2DSRM_W::new(self, 27)
    }
    ///Bit 28 - BKBID
    #[inline(always)]
    pub fn bkbid(&mut self) -> BKBID_W<'_, BDTRrs> {
        BKBID_W::new(self, 28)
    }
    ///Bit 29 - BK2BID
    #[inline(always)]
    pub fn bk2bid(&mut self) -> BK2BID_W<'_, BDTRrs> {
        BK2BID_W::new(self, 29)
    }
}
/**As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\[3:0\], BKF\[3:0\], AOE, BKP, BKE, OSSI, OSSR and DTG\[7:0\] can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register.

You can [`read`](crate::Reg::read) this register and get [`bdtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM5:BDTR)*/
pub struct BDTRrs;
impl crate::RegisterSpec for BDTRrs {
    type Ux = u32;
}
///`read()` method returns [`bdtr::R`](R) reader structure
impl crate::Readable for BDTRrs {}
///`write(|w| ..)` method takes [`bdtr::W`](W) writer structure
impl crate::Writable for BDTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BDTR to value 0
impl crate::Resettable for BDTRrs {}

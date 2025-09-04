///Register `C5CIDCFGR` reader
pub type R = crate::R<C5CIDCFGRrs>;
///Register `C5CIDCFGR` writer
pub type W = crate::W<C5CIDCFGRrs>;
///Field `CFEN` reader - CID filtering enable of the channel x
pub type CFEN_R = crate::BitReader;
///Field `CFEN` writer - CID filtering enable of the channel x
pub type CFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEM_EN` reader - semaphore mode enable (for the CID allocation policy to the channel x)
pub type SEM_EN_R = crate::BitReader;
///Field `SEM_EN` writer - semaphore mode enable (for the CID allocation policy to the channel x)
pub type SEM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCID` reader - allocate a static/single CID to the channel x (for when the channel x CID configuration is not in semaphore mode)
pub type SCID_R = crate::FieldReader;
///Field `SCID` writer - allocate a static/single CID to the channel x (for when the channel x CID configuration is not in semaphore mode)
pub type SCID_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SEM_WLIST_CID0` reader - white-listed CID0 in the CID allocation pool (for when the channel x in semaphore mode)
pub type SEM_WLIST_CID0_R = crate::BitReader;
///Field `SEM_WLIST_CID0` writer - white-listed CID0 in the CID allocation pool (for when the channel x in semaphore mode)
pub type SEM_WLIST_CID0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEM_WLIST_CID1` reader - white-listed CID1 in the CID allocation pool (for when the channel x in semaphore mode)
pub type SEM_WLIST_CID1_R = crate::BitReader;
///Field `SEM_WLIST_CID1` writer - white-listed CID1 in the CID allocation pool (for when the channel x in semaphore mode)
pub type SEM_WLIST_CID1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEM_WLIST_CID2` reader - white-listed CID2 in the CID allocation pool (for when the channel x in semaphore mode)
pub type SEM_WLIST_CID2_R = crate::BitReader;
///Field `SEM_WLIST_CID2` writer - white-listed CID2 in the CID allocation pool (for when the channel x in semaphore mode)
pub type SEM_WLIST_CID2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEM_WLIST_CID3` reader - white-listed CID3 in the CID allocation pool (for when the channel x in semaphore mode)
pub type SEM_WLIST_CID3_R = crate::BitReader;
///Field `SEM_WLIST_CID3` writer - white-listed CID3 in the CID allocation pool (for when the channel x in semaphore mode)
pub type SEM_WLIST_CID3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEM_WLIST_CID4` reader - white-listed CID4 in the CID allocation pool (for when the channel x in semaphore mode)
pub type SEM_WLIST_CID4_R = crate::BitReader;
///Field `SEM_WLIST_CID4` writer - white-listed CID4 in the CID allocation pool (for when the channel x in semaphore mode)
pub type SEM_WLIST_CID4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEM_WLIST_CID5` reader - white-listed CID5 in the CID allocation pool (for when the channel x in semaphore mode)
pub type SEM_WLIST_CID5_R = crate::BitReader;
///Field `SEM_WLIST_CID5` writer - white-listed CID5 in the CID allocation pool (for when the channel x in semaphore mode)
pub type SEM_WLIST_CID5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEM_WLIST_CID6` reader - white-listed CID6 in the CID allocation pool (for when the channel x in semaphore mode)
pub type SEM_WLIST_CID6_R = crate::BitReader;
///Field `SEM_WLIST_CID6` writer - white-listed CID6 in the CID allocation pool (for when the channel x in semaphore mode)
pub type SEM_WLIST_CID6_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - CID filtering enable of the channel x
    #[inline(always)]
    pub fn cfen(&self) -> CFEN_R {
        CFEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - semaphore mode enable (for the CID allocation policy to the channel x)
    #[inline(always)]
    pub fn sem_en(&self) -> SEM_EN_R {
        SEM_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 4:6 - allocate a static/single CID to the channel x (for when the channel x CID configuration is not in semaphore mode)
    #[inline(always)]
    pub fn scid(&self) -> SCID_R {
        SCID_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 16 - white-listed CID0 in the CID allocation pool (for when the channel x in semaphore mode)
    #[inline(always)]
    pub fn sem_wlist_cid0(&self) -> SEM_WLIST_CID0_R {
        SEM_WLIST_CID0_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - white-listed CID1 in the CID allocation pool (for when the channel x in semaphore mode)
    #[inline(always)]
    pub fn sem_wlist_cid1(&self) -> SEM_WLIST_CID1_R {
        SEM_WLIST_CID1_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - white-listed CID2 in the CID allocation pool (for when the channel x in semaphore mode)
    #[inline(always)]
    pub fn sem_wlist_cid2(&self) -> SEM_WLIST_CID2_R {
        SEM_WLIST_CID2_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - white-listed CID3 in the CID allocation pool (for when the channel x in semaphore mode)
    #[inline(always)]
    pub fn sem_wlist_cid3(&self) -> SEM_WLIST_CID3_R {
        SEM_WLIST_CID3_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - white-listed CID4 in the CID allocation pool (for when the channel x in semaphore mode)
    #[inline(always)]
    pub fn sem_wlist_cid4(&self) -> SEM_WLIST_CID4_R {
        SEM_WLIST_CID4_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - white-listed CID5 in the CID allocation pool (for when the channel x in semaphore mode)
    #[inline(always)]
    pub fn sem_wlist_cid5(&self) -> SEM_WLIST_CID5_R {
        SEM_WLIST_CID5_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - white-listed CID6 in the CID allocation pool (for when the channel x in semaphore mode)
    #[inline(always)]
    pub fn sem_wlist_cid6(&self) -> SEM_WLIST_CID6_R {
        SEM_WLIST_CID6_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C5CIDCFGR")
            .field("cfen", &self.cfen())
            .field("sem_en", &self.sem_en())
            .field("scid", &self.scid())
            .field("sem_wlist_cid0", &self.sem_wlist_cid0())
            .field("sem_wlist_cid1", &self.sem_wlist_cid1())
            .field("sem_wlist_cid2", &self.sem_wlist_cid2())
            .field("sem_wlist_cid3", &self.sem_wlist_cid3())
            .field("sem_wlist_cid4", &self.sem_wlist_cid4())
            .field("sem_wlist_cid5", &self.sem_wlist_cid5())
            .field("sem_wlist_cid6", &self.sem_wlist_cid6())
            .finish()
    }
}
impl W {
    ///Bit 0 - CID filtering enable of the channel x
    #[inline(always)]
    pub fn cfen(&mut self) -> CFEN_W<C5CIDCFGRrs> {
        CFEN_W::new(self, 0)
    }
    ///Bit 1 - semaphore mode enable (for the CID allocation policy to the channel x)
    #[inline(always)]
    pub fn sem_en(&mut self) -> SEM_EN_W<C5CIDCFGRrs> {
        SEM_EN_W::new(self, 1)
    }
    ///Bits 4:6 - allocate a static/single CID to the channel x (for when the channel x CID configuration is not in semaphore mode)
    #[inline(always)]
    pub fn scid(&mut self) -> SCID_W<C5CIDCFGRrs> {
        SCID_W::new(self, 4)
    }
    ///Bit 16 - white-listed CID0 in the CID allocation pool (for when the channel x in semaphore mode)
    #[inline(always)]
    pub fn sem_wlist_cid0(&mut self) -> SEM_WLIST_CID0_W<C5CIDCFGRrs> {
        SEM_WLIST_CID0_W::new(self, 16)
    }
    ///Bit 17 - white-listed CID1 in the CID allocation pool (for when the channel x in semaphore mode)
    #[inline(always)]
    pub fn sem_wlist_cid1(&mut self) -> SEM_WLIST_CID1_W<C5CIDCFGRrs> {
        SEM_WLIST_CID1_W::new(self, 17)
    }
    ///Bit 18 - white-listed CID2 in the CID allocation pool (for when the channel x in semaphore mode)
    #[inline(always)]
    pub fn sem_wlist_cid2(&mut self) -> SEM_WLIST_CID2_W<C5CIDCFGRrs> {
        SEM_WLIST_CID2_W::new(self, 18)
    }
    ///Bit 19 - white-listed CID3 in the CID allocation pool (for when the channel x in semaphore mode)
    #[inline(always)]
    pub fn sem_wlist_cid3(&mut self) -> SEM_WLIST_CID3_W<C5CIDCFGRrs> {
        SEM_WLIST_CID3_W::new(self, 19)
    }
    ///Bit 20 - white-listed CID4 in the CID allocation pool (for when the channel x in semaphore mode)
    #[inline(always)]
    pub fn sem_wlist_cid4(&mut self) -> SEM_WLIST_CID4_W<C5CIDCFGRrs> {
        SEM_WLIST_CID4_W::new(self, 20)
    }
    ///Bit 21 - white-listed CID5 in the CID allocation pool (for when the channel x in semaphore mode)
    #[inline(always)]
    pub fn sem_wlist_cid5(&mut self) -> SEM_WLIST_CID5_W<C5CIDCFGRrs> {
        SEM_WLIST_CID5_W::new(self, 21)
    }
    ///Bit 22 - white-listed CID6 in the CID allocation pool (for when the channel x in semaphore mode)
    #[inline(always)]
    pub fn sem_wlist_cid6(&mut self) -> SEM_WLIST_CID6_W<C5CIDCFGRrs> {
        SEM_WLIST_CID6_W::new(self, 22)
    }
}
/**HPDMA channel 5 CID register

You can [`read`](crate::Reg::read) this register and get [`c5cidcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c5cidcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#HPDMA:C5CIDCFGR)*/
pub struct C5CIDCFGRrs;
impl crate::RegisterSpec for C5CIDCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`c5cidcfgr::R`](R) reader structure
impl crate::Readable for C5CIDCFGRrs {}
///`write(|w| ..)` method takes [`c5cidcfgr::W`](W) writer structure
impl crate::Writable for C5CIDCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C5CIDCFGR to value 0
impl crate::Resettable for C5CIDCFGRrs {}

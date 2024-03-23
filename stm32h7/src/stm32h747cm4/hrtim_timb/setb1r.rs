#[doc = "Register `SETB1R` reader"]
pub type R = crate::R<SETB1Rrs>;
#[doc = "Register `SETB1R` writer"]
pub type W = crate::W<SETB1Rrs>;
#[doc = "Field `SST` reader - Software Set trigger"]
pub type SST_R = crate::BitReader;
#[doc = "Field `SST` writer - Software Set trigger"]
pub type SST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESYNC` reader - Timer A resynchronizaton"]
pub type RESYNC_R = crate::BitReader;
#[doc = "Field `RESYNC` writer - Timer A resynchronizaton"]
pub type RESYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PER` reader - Timer A Period"]
pub type PER_R = crate::BitReader;
#[doc = "Field `PER` writer - Timer A Period"]
pub type PER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP1` reader - Timer A compare 1"]
pub type CMP1_R = crate::BitReader;
#[doc = "Field `CMP1` writer - Timer A compare 1"]
pub type CMP1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP2` reader - Timer A compare 2"]
pub type CMP2_R = crate::BitReader;
#[doc = "Field `CMP2` writer - Timer A compare 2"]
pub type CMP2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP3` reader - Timer A compare 3"]
pub type CMP3_R = crate::BitReader;
#[doc = "Field `CMP3` writer - Timer A compare 3"]
pub type CMP3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP4` reader - Timer A compare 4"]
pub type CMP4_R = crate::BitReader;
#[doc = "Field `CMP4` writer - Timer A compare 4"]
pub type CMP4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSTPER` reader - Master Period"]
pub type MSTPER_R = crate::BitReader;
#[doc = "Field `MSTPER` writer - Master Period"]
pub type MSTPER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSTCMP1` reader - Master Compare 1"]
pub type MSTCMP1_R = crate::BitReader;
#[doc = "Field `MSTCMP1` writer - Master Compare 1"]
pub type MSTCMP1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSTCMP2` reader - Master Compare 2"]
pub type MSTCMP2_R = crate::BitReader;
#[doc = "Field `MSTCMP2` writer - Master Compare 2"]
pub type MSTCMP2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSTCMP3` reader - Master Compare 3"]
pub type MSTCMP3_R = crate::BitReader;
#[doc = "Field `MSTCMP3` writer - Master Compare 3"]
pub type MSTCMP3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSTCMP4` reader - Master Compare 4"]
pub type MSTCMP4_R = crate::BitReader;
#[doc = "Field `MSTCMP4` writer - Master Compare 4"]
pub type MSTCMP4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEVNT1` reader - Timer Event 1"]
pub type TIMEVNT1_R = crate::BitReader;
#[doc = "Field `TIMEVNT1` writer - Timer Event 1"]
pub type TIMEVNT1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEVNT2` reader - Timer Event 2"]
pub type TIMEVNT2_R = crate::BitReader;
#[doc = "Field `TIMEVNT2` writer - Timer Event 2"]
pub type TIMEVNT2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEVNT3` reader - Timer Event 3"]
pub type TIMEVNT3_R = crate::BitReader;
#[doc = "Field `TIMEVNT3` writer - Timer Event 3"]
pub type TIMEVNT3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEVNT4` reader - Timer Event 4"]
pub type TIMEVNT4_R = crate::BitReader;
#[doc = "Field `TIMEVNT4` writer - Timer Event 4"]
pub type TIMEVNT4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEVNT5` reader - Timer Event 5"]
pub type TIMEVNT5_R = crate::BitReader;
#[doc = "Field `TIMEVNT5` writer - Timer Event 5"]
pub type TIMEVNT5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEVNT6` reader - Timer Event 6"]
pub type TIMEVNT6_R = crate::BitReader;
#[doc = "Field `TIMEVNT6` writer - Timer Event 6"]
pub type TIMEVNT6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEVNT7` reader - Timer Event 7"]
pub type TIMEVNT7_R = crate::BitReader;
#[doc = "Field `TIMEVNT7` writer - Timer Event 7"]
pub type TIMEVNT7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEVNT8` reader - Timer Event 8"]
pub type TIMEVNT8_R = crate::BitReader;
#[doc = "Field `TIMEVNT8` writer - Timer Event 8"]
pub type TIMEVNT8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEVNT9` reader - Timer Event 9"]
pub type TIMEVNT9_R = crate::BitReader;
#[doc = "Field `TIMEVNT9` writer - Timer Event 9"]
pub type TIMEVNT9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTEVNT1` reader - External Event 1"]
pub type EXTEVNT1_R = crate::BitReader;
#[doc = "Field `EXTEVNT1` writer - External Event 1"]
pub type EXTEVNT1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTEVNT2` reader - External Event 2"]
pub type EXTEVNT2_R = crate::BitReader;
#[doc = "Field `EXTEVNT2` writer - External Event 2"]
pub type EXTEVNT2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTEVNT3` reader - External Event 3"]
pub type EXTEVNT3_R = crate::BitReader;
#[doc = "Field `EXTEVNT3` writer - External Event 3"]
pub type EXTEVNT3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTEVNT4` reader - External Event 4"]
pub type EXTEVNT4_R = crate::BitReader;
#[doc = "Field `EXTEVNT4` writer - External Event 4"]
pub type EXTEVNT4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTEVNT5` reader - External Event 5"]
pub type EXTEVNT5_R = crate::BitReader;
#[doc = "Field `EXTEVNT5` writer - External Event 5"]
pub type EXTEVNT5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTEVNT6` reader - External Event 6"]
pub type EXTEVNT6_R = crate::BitReader;
#[doc = "Field `EXTEVNT6` writer - External Event 6"]
pub type EXTEVNT6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTEVNT7` reader - External Event 7"]
pub type EXTEVNT7_R = crate::BitReader;
#[doc = "Field `EXTEVNT7` writer - External Event 7"]
pub type EXTEVNT7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTEVNT8` reader - External Event 8"]
pub type EXTEVNT8_R = crate::BitReader;
#[doc = "Field `EXTEVNT8` writer - External Event 8"]
pub type EXTEVNT8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTEVNT9` reader - External Event 9"]
pub type EXTEVNT9_R = crate::BitReader;
#[doc = "Field `EXTEVNT9` writer - External Event 9"]
pub type EXTEVNT9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTEVNT10` reader - External Event 10"]
pub type EXTEVNT10_R = crate::BitReader;
#[doc = "Field `EXTEVNT10` writer - External Event 10"]
pub type EXTEVNT10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPDATE` reader - Registers update (transfer preload to active)"]
pub type UPDATE_R = crate::BitReader;
#[doc = "Field `UPDATE` writer - Registers update (transfer preload to active)"]
pub type UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Software Set trigger"]
    #[inline(always)]
    pub fn sst(&self) -> SST_R {
        SST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer A resynchronizaton"]
    #[inline(always)]
    pub fn resync(&self) -> RESYNC_R {
        RESYNC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer A Period"]
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer A compare 1"]
    #[inline(always)]
    pub fn cmp1(&self) -> CMP1_R {
        CMP1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer A compare 2"]
    #[inline(always)]
    pub fn cmp2(&self) -> CMP2_R {
        CMP2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer A compare 3"]
    #[inline(always)]
    pub fn cmp3(&self) -> CMP3_R {
        CMP3_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Timer A compare 4"]
    #[inline(always)]
    pub fn cmp4(&self) -> CMP4_R {
        CMP4_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Master Period"]
    #[inline(always)]
    pub fn mstper(&self) -> MSTPER_R {
        MSTPER_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Master Compare 1"]
    #[inline(always)]
    pub fn mstcmp1(&self) -> MSTCMP1_R {
        MSTCMP1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Master Compare 2"]
    #[inline(always)]
    pub fn mstcmp2(&self) -> MSTCMP2_R {
        MSTCMP2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Master Compare 3"]
    #[inline(always)]
    pub fn mstcmp3(&self) -> MSTCMP3_R {
        MSTCMP3_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Master Compare 4"]
    #[inline(always)]
    pub fn mstcmp4(&self) -> MSTCMP4_R {
        MSTCMP4_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Timer Event 1"]
    #[inline(always)]
    pub fn timevnt1(&self) -> TIMEVNT1_R {
        TIMEVNT1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Timer Event 2"]
    #[inline(always)]
    pub fn timevnt2(&self) -> TIMEVNT2_R {
        TIMEVNT2_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Timer Event 3"]
    #[inline(always)]
    pub fn timevnt3(&self) -> TIMEVNT3_R {
        TIMEVNT3_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Timer Event 4"]
    #[inline(always)]
    pub fn timevnt4(&self) -> TIMEVNT4_R {
        TIMEVNT4_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Timer Event 5"]
    #[inline(always)]
    pub fn timevnt5(&self) -> TIMEVNT5_R {
        TIMEVNT5_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Timer Event 6"]
    #[inline(always)]
    pub fn timevnt6(&self) -> TIMEVNT6_R {
        TIMEVNT6_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timer Event 7"]
    #[inline(always)]
    pub fn timevnt7(&self) -> TIMEVNT7_R {
        TIMEVNT7_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Timer Event 8"]
    #[inline(always)]
    pub fn timevnt8(&self) -> TIMEVNT8_R {
        TIMEVNT8_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Timer Event 9"]
    #[inline(always)]
    pub fn timevnt9(&self) -> TIMEVNT9_R {
        TIMEVNT9_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - External Event 1"]
    #[inline(always)]
    pub fn extevnt1(&self) -> EXTEVNT1_R {
        EXTEVNT1_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - External Event 2"]
    #[inline(always)]
    pub fn extevnt2(&self) -> EXTEVNT2_R {
        EXTEVNT2_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - External Event 3"]
    #[inline(always)]
    pub fn extevnt3(&self) -> EXTEVNT3_R {
        EXTEVNT3_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - External Event 4"]
    #[inline(always)]
    pub fn extevnt4(&self) -> EXTEVNT4_R {
        EXTEVNT4_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - External Event 5"]
    #[inline(always)]
    pub fn extevnt5(&self) -> EXTEVNT5_R {
        EXTEVNT5_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - External Event 6"]
    #[inline(always)]
    pub fn extevnt6(&self) -> EXTEVNT6_R {
        EXTEVNT6_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - External Event 7"]
    #[inline(always)]
    pub fn extevnt7(&self) -> EXTEVNT7_R {
        EXTEVNT7_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - External Event 8"]
    #[inline(always)]
    pub fn extevnt8(&self) -> EXTEVNT8_R {
        EXTEVNT8_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - External Event 9"]
    #[inline(always)]
    pub fn extevnt9(&self) -> EXTEVNT9_R {
        EXTEVNT9_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - External Event 10"]
    #[inline(always)]
    pub fn extevnt10(&self) -> EXTEVNT10_R {
        EXTEVNT10_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Registers update (transfer preload to active)"]
    #[inline(always)]
    pub fn update(&self) -> UPDATE_R {
        UPDATE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Set trigger"]
    #[inline(always)]
    #[must_use]
    pub fn sst(&mut self) -> SST_W<SETB1Rrs> {
        SST_W::new(self, 0)
    }
    #[doc = "Bit 1 - Timer A resynchronizaton"]
    #[inline(always)]
    #[must_use]
    pub fn resync(&mut self) -> RESYNC_W<SETB1Rrs> {
        RESYNC_W::new(self, 1)
    }
    #[doc = "Bit 2 - Timer A Period"]
    #[inline(always)]
    #[must_use]
    pub fn per(&mut self) -> PER_W<SETB1Rrs> {
        PER_W::new(self, 2)
    }
    #[doc = "Bit 3 - Timer A compare 1"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1(&mut self) -> CMP1_W<SETB1Rrs> {
        CMP1_W::new(self, 3)
    }
    #[doc = "Bit 4 - Timer A compare 2"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2(&mut self) -> CMP2_W<SETB1Rrs> {
        CMP2_W::new(self, 4)
    }
    #[doc = "Bit 5 - Timer A compare 3"]
    #[inline(always)]
    #[must_use]
    pub fn cmp3(&mut self) -> CMP3_W<SETB1Rrs> {
        CMP3_W::new(self, 5)
    }
    #[doc = "Bit 6 - Timer A compare 4"]
    #[inline(always)]
    #[must_use]
    pub fn cmp4(&mut self) -> CMP4_W<SETB1Rrs> {
        CMP4_W::new(self, 6)
    }
    #[doc = "Bit 7 - Master Period"]
    #[inline(always)]
    #[must_use]
    pub fn mstper(&mut self) -> MSTPER_W<SETB1Rrs> {
        MSTPER_W::new(self, 7)
    }
    #[doc = "Bit 8 - Master Compare 1"]
    #[inline(always)]
    #[must_use]
    pub fn mstcmp1(&mut self) -> MSTCMP1_W<SETB1Rrs> {
        MSTCMP1_W::new(self, 8)
    }
    #[doc = "Bit 9 - Master Compare 2"]
    #[inline(always)]
    #[must_use]
    pub fn mstcmp2(&mut self) -> MSTCMP2_W<SETB1Rrs> {
        MSTCMP2_W::new(self, 9)
    }
    #[doc = "Bit 10 - Master Compare 3"]
    #[inline(always)]
    #[must_use]
    pub fn mstcmp3(&mut self) -> MSTCMP3_W<SETB1Rrs> {
        MSTCMP3_W::new(self, 10)
    }
    #[doc = "Bit 11 - Master Compare 4"]
    #[inline(always)]
    #[must_use]
    pub fn mstcmp4(&mut self) -> MSTCMP4_W<SETB1Rrs> {
        MSTCMP4_W::new(self, 11)
    }
    #[doc = "Bit 12 - Timer Event 1"]
    #[inline(always)]
    #[must_use]
    pub fn timevnt1(&mut self) -> TIMEVNT1_W<SETB1Rrs> {
        TIMEVNT1_W::new(self, 12)
    }
    #[doc = "Bit 13 - Timer Event 2"]
    #[inline(always)]
    #[must_use]
    pub fn timevnt2(&mut self) -> TIMEVNT2_W<SETB1Rrs> {
        TIMEVNT2_W::new(self, 13)
    }
    #[doc = "Bit 14 - Timer Event 3"]
    #[inline(always)]
    #[must_use]
    pub fn timevnt3(&mut self) -> TIMEVNT3_W<SETB1Rrs> {
        TIMEVNT3_W::new(self, 14)
    }
    #[doc = "Bit 15 - Timer Event 4"]
    #[inline(always)]
    #[must_use]
    pub fn timevnt4(&mut self) -> TIMEVNT4_W<SETB1Rrs> {
        TIMEVNT4_W::new(self, 15)
    }
    #[doc = "Bit 16 - Timer Event 5"]
    #[inline(always)]
    #[must_use]
    pub fn timevnt5(&mut self) -> TIMEVNT5_W<SETB1Rrs> {
        TIMEVNT5_W::new(self, 16)
    }
    #[doc = "Bit 17 - Timer Event 6"]
    #[inline(always)]
    #[must_use]
    pub fn timevnt6(&mut self) -> TIMEVNT6_W<SETB1Rrs> {
        TIMEVNT6_W::new(self, 17)
    }
    #[doc = "Bit 18 - Timer Event 7"]
    #[inline(always)]
    #[must_use]
    pub fn timevnt7(&mut self) -> TIMEVNT7_W<SETB1Rrs> {
        TIMEVNT7_W::new(self, 18)
    }
    #[doc = "Bit 19 - Timer Event 8"]
    #[inline(always)]
    #[must_use]
    pub fn timevnt8(&mut self) -> TIMEVNT8_W<SETB1Rrs> {
        TIMEVNT8_W::new(self, 19)
    }
    #[doc = "Bit 20 - Timer Event 9"]
    #[inline(always)]
    #[must_use]
    pub fn timevnt9(&mut self) -> TIMEVNT9_W<SETB1Rrs> {
        TIMEVNT9_W::new(self, 20)
    }
    #[doc = "Bit 21 - External Event 1"]
    #[inline(always)]
    #[must_use]
    pub fn extevnt1(&mut self) -> EXTEVNT1_W<SETB1Rrs> {
        EXTEVNT1_W::new(self, 21)
    }
    #[doc = "Bit 22 - External Event 2"]
    #[inline(always)]
    #[must_use]
    pub fn extevnt2(&mut self) -> EXTEVNT2_W<SETB1Rrs> {
        EXTEVNT2_W::new(self, 22)
    }
    #[doc = "Bit 23 - External Event 3"]
    #[inline(always)]
    #[must_use]
    pub fn extevnt3(&mut self) -> EXTEVNT3_W<SETB1Rrs> {
        EXTEVNT3_W::new(self, 23)
    }
    #[doc = "Bit 24 - External Event 4"]
    #[inline(always)]
    #[must_use]
    pub fn extevnt4(&mut self) -> EXTEVNT4_W<SETB1Rrs> {
        EXTEVNT4_W::new(self, 24)
    }
    #[doc = "Bit 25 - External Event 5"]
    #[inline(always)]
    #[must_use]
    pub fn extevnt5(&mut self) -> EXTEVNT5_W<SETB1Rrs> {
        EXTEVNT5_W::new(self, 25)
    }
    #[doc = "Bit 26 - External Event 6"]
    #[inline(always)]
    #[must_use]
    pub fn extevnt6(&mut self) -> EXTEVNT6_W<SETB1Rrs> {
        EXTEVNT6_W::new(self, 26)
    }
    #[doc = "Bit 27 - External Event 7"]
    #[inline(always)]
    #[must_use]
    pub fn extevnt7(&mut self) -> EXTEVNT7_W<SETB1Rrs> {
        EXTEVNT7_W::new(self, 27)
    }
    #[doc = "Bit 28 - External Event 8"]
    #[inline(always)]
    #[must_use]
    pub fn extevnt8(&mut self) -> EXTEVNT8_W<SETB1Rrs> {
        EXTEVNT8_W::new(self, 28)
    }
    #[doc = "Bit 29 - External Event 9"]
    #[inline(always)]
    #[must_use]
    pub fn extevnt9(&mut self) -> EXTEVNT9_W<SETB1Rrs> {
        EXTEVNT9_W::new(self, 29)
    }
    #[doc = "Bit 30 - External Event 10"]
    #[inline(always)]
    #[must_use]
    pub fn extevnt10(&mut self) -> EXTEVNT10_W<SETB1Rrs> {
        EXTEVNT10_W::new(self, 30)
    }
    #[doc = "Bit 31 - Registers update (transfer preload to active)"]
    #[inline(always)]
    #[must_use]
    pub fn update(&mut self) -> UPDATE_W<SETB1Rrs> {
        UPDATE_W::new(self, 31)
    }
}
#[doc = "Timerx Output1 Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`setb1r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`setb1r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SETB1Rrs;
impl crate::RegisterSpec for SETB1Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`setb1r::R`](R) reader structure"]
impl crate::Readable for SETB1Rrs {}
#[doc = "`write(|w| ..)` method takes [`setb1r::W`](W) writer structure"]
impl crate::Writable for SETB1Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SETB1R to value 0"]
impl crate::Resettable for SETB1Rrs {
    const RESET_VALUE: u32 = 0;
}

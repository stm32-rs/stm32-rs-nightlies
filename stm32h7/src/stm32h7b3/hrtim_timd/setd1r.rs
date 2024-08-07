///Register `SETD1R` reader
pub type R = crate::R<SETD1Rrs>;
///Register `SETD1R` writer
pub type W = crate::W<SETD1Rrs>;
///Field `SST` reader - Software Set trigger
pub type SST_R = crate::BitReader;
///Field `SST` writer - Software Set trigger
pub type SST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RESYNC` reader - Timer A resynchronizaton
pub type RESYNC_R = crate::BitReader;
///Field `RESYNC` writer - Timer A resynchronizaton
pub type RESYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PER` reader - Timer A Period
pub type PER_R = crate::BitReader;
///Field `PER` writer - Timer A Period
pub type PER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMP1` reader - Timer A compare 1
pub type CMP1_R = crate::BitReader;
///Field `CMP1` writer - Timer A compare 1
pub type CMP1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMP2` reader - Timer A compare 2
pub type CMP2_R = crate::BitReader;
///Field `CMP2` writer - Timer A compare 2
pub type CMP2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMP3` reader - Timer A compare 3
pub type CMP3_R = crate::BitReader;
///Field `CMP3` writer - Timer A compare 3
pub type CMP3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMP4` reader - Timer A compare 4
pub type CMP4_R = crate::BitReader;
///Field `CMP4` writer - Timer A compare 4
pub type CMP4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSTPER` reader - Master Period
pub type MSTPER_R = crate::BitReader;
///Field `MSTPER` writer - Master Period
pub type MSTPER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSTCMP1` reader - Master Compare 1
pub type MSTCMP1_R = crate::BitReader;
///Field `MSTCMP1` writer - Master Compare 1
pub type MSTCMP1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSTCMP2` reader - Master Compare 2
pub type MSTCMP2_R = crate::BitReader;
///Field `MSTCMP2` writer - Master Compare 2
pub type MSTCMP2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSTCMP3` reader - Master Compare 3
pub type MSTCMP3_R = crate::BitReader;
///Field `MSTCMP3` writer - Master Compare 3
pub type MSTCMP3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSTCMP4` reader - Master Compare 4
pub type MSTCMP4_R = crate::BitReader;
///Field `MSTCMP4` writer - Master Compare 4
pub type MSTCMP4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMEVNT1` reader - Timer Event 1
pub type TIMEVNT1_R = crate::BitReader;
///Field `TIMEVNT1` writer - Timer Event 1
pub type TIMEVNT1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMEVNT2` reader - Timer Event 2
pub type TIMEVNT2_R = crate::BitReader;
///Field `TIMEVNT2` writer - Timer Event 2
pub type TIMEVNT2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMEVNT3` reader - Timer Event 3
pub type TIMEVNT3_R = crate::BitReader;
///Field `TIMEVNT3` writer - Timer Event 3
pub type TIMEVNT3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMEVNT4` reader - Timer Event 4
pub type TIMEVNT4_R = crate::BitReader;
///Field `TIMEVNT4` writer - Timer Event 4
pub type TIMEVNT4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMEVNT5` reader - Timer Event 5
pub type TIMEVNT5_R = crate::BitReader;
///Field `TIMEVNT5` writer - Timer Event 5
pub type TIMEVNT5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMEVNT6` reader - Timer Event 6
pub type TIMEVNT6_R = crate::BitReader;
///Field `TIMEVNT6` writer - Timer Event 6
pub type TIMEVNT6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMEVNT7` reader - Timer Event 7
pub type TIMEVNT7_R = crate::BitReader;
///Field `TIMEVNT7` writer - Timer Event 7
pub type TIMEVNT7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMEVNT8` reader - Timer Event 8
pub type TIMEVNT8_R = crate::BitReader;
///Field `TIMEVNT8` writer - Timer Event 8
pub type TIMEVNT8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMEVNT9` reader - Timer Event 9
pub type TIMEVNT9_R = crate::BitReader;
///Field `TIMEVNT9` writer - Timer Event 9
pub type TIMEVNT9_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXTEVNT1` reader - External Event 1
pub type EXTEVNT1_R = crate::BitReader;
///Field `EXTEVNT1` writer - External Event 1
pub type EXTEVNT1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXTEVNT2` reader - External Event 2
pub type EXTEVNT2_R = crate::BitReader;
///Field `EXTEVNT2` writer - External Event 2
pub type EXTEVNT2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXTEVNT3` reader - External Event 3
pub type EXTEVNT3_R = crate::BitReader;
///Field `EXTEVNT3` writer - External Event 3
pub type EXTEVNT3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXTEVNT4` reader - External Event 4
pub type EXTEVNT4_R = crate::BitReader;
///Field `EXTEVNT4` writer - External Event 4
pub type EXTEVNT4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXTEVNT5` reader - External Event 5
pub type EXTEVNT5_R = crate::BitReader;
///Field `EXTEVNT5` writer - External Event 5
pub type EXTEVNT5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXTEVNT6` reader - External Event 6
pub type EXTEVNT6_R = crate::BitReader;
///Field `EXTEVNT6` writer - External Event 6
pub type EXTEVNT6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXTEVNT7` reader - External Event 7
pub type EXTEVNT7_R = crate::BitReader;
///Field `EXTEVNT7` writer - External Event 7
pub type EXTEVNT7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXTEVNT8` reader - External Event 8
pub type EXTEVNT8_R = crate::BitReader;
///Field `EXTEVNT8` writer - External Event 8
pub type EXTEVNT8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXTEVNT9` reader - External Event 9
pub type EXTEVNT9_R = crate::BitReader;
///Field `EXTEVNT9` writer - External Event 9
pub type EXTEVNT9_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXTEVNT10` reader - External Event 10
pub type EXTEVNT10_R = crate::BitReader;
///Field `EXTEVNT10` writer - External Event 10
pub type EXTEVNT10_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UPDATE` reader - Registers update (transfer preload to active)
pub type UPDATE_R = crate::BitReader;
///Field `UPDATE` writer - Registers update (transfer preload to active)
pub type UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Software Set trigger
    #[inline(always)]
    pub fn sst(&self) -> SST_R {
        SST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Timer A resynchronizaton
    #[inline(always)]
    pub fn resync(&self) -> RESYNC_R {
        RESYNC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Timer A Period
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Timer A compare 1
    #[inline(always)]
    pub fn cmp1(&self) -> CMP1_R {
        CMP1_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Timer A compare 2
    #[inline(always)]
    pub fn cmp2(&self) -> CMP2_R {
        CMP2_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Timer A compare 3
    #[inline(always)]
    pub fn cmp3(&self) -> CMP3_R {
        CMP3_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Timer A compare 4
    #[inline(always)]
    pub fn cmp4(&self) -> CMP4_R {
        CMP4_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Master Period
    #[inline(always)]
    pub fn mstper(&self) -> MSTPER_R {
        MSTPER_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Master Compare 1
    #[inline(always)]
    pub fn mstcmp1(&self) -> MSTCMP1_R {
        MSTCMP1_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Master Compare 2
    #[inline(always)]
    pub fn mstcmp2(&self) -> MSTCMP2_R {
        MSTCMP2_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Master Compare 3
    #[inline(always)]
    pub fn mstcmp3(&self) -> MSTCMP3_R {
        MSTCMP3_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Master Compare 4
    #[inline(always)]
    pub fn mstcmp4(&self) -> MSTCMP4_R {
        MSTCMP4_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Timer Event 1
    #[inline(always)]
    pub fn timevnt1(&self) -> TIMEVNT1_R {
        TIMEVNT1_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Timer Event 2
    #[inline(always)]
    pub fn timevnt2(&self) -> TIMEVNT2_R {
        TIMEVNT2_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Timer Event 3
    #[inline(always)]
    pub fn timevnt3(&self) -> TIMEVNT3_R {
        TIMEVNT3_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Timer Event 4
    #[inline(always)]
    pub fn timevnt4(&self) -> TIMEVNT4_R {
        TIMEVNT4_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Timer Event 5
    #[inline(always)]
    pub fn timevnt5(&self) -> TIMEVNT5_R {
        TIMEVNT5_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Timer Event 6
    #[inline(always)]
    pub fn timevnt6(&self) -> TIMEVNT6_R {
        TIMEVNT6_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Timer Event 7
    #[inline(always)]
    pub fn timevnt7(&self) -> TIMEVNT7_R {
        TIMEVNT7_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Timer Event 8
    #[inline(always)]
    pub fn timevnt8(&self) -> TIMEVNT8_R {
        TIMEVNT8_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Timer Event 9
    #[inline(always)]
    pub fn timevnt9(&self) -> TIMEVNT9_R {
        TIMEVNT9_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - External Event 1
    #[inline(always)]
    pub fn extevnt1(&self) -> EXTEVNT1_R {
        EXTEVNT1_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - External Event 2
    #[inline(always)]
    pub fn extevnt2(&self) -> EXTEVNT2_R {
        EXTEVNT2_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - External Event 3
    #[inline(always)]
    pub fn extevnt3(&self) -> EXTEVNT3_R {
        EXTEVNT3_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - External Event 4
    #[inline(always)]
    pub fn extevnt4(&self) -> EXTEVNT4_R {
        EXTEVNT4_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - External Event 5
    #[inline(always)]
    pub fn extevnt5(&self) -> EXTEVNT5_R {
        EXTEVNT5_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - External Event 6
    #[inline(always)]
    pub fn extevnt6(&self) -> EXTEVNT6_R {
        EXTEVNT6_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - External Event 7
    #[inline(always)]
    pub fn extevnt7(&self) -> EXTEVNT7_R {
        EXTEVNT7_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - External Event 8
    #[inline(always)]
    pub fn extevnt8(&self) -> EXTEVNT8_R {
        EXTEVNT8_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - External Event 9
    #[inline(always)]
    pub fn extevnt9(&self) -> EXTEVNT9_R {
        EXTEVNT9_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - External Event 10
    #[inline(always)]
    pub fn extevnt10(&self) -> EXTEVNT10_R {
        EXTEVNT10_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Registers update (transfer preload to active)
    #[inline(always)]
    pub fn update(&self) -> UPDATE_R {
        UPDATE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SETD1R")
            .field("update", &self.update())
            .field("extevnt10", &self.extevnt10())
            .field("extevnt9", &self.extevnt9())
            .field("extevnt8", &self.extevnt8())
            .field("extevnt7", &self.extevnt7())
            .field("extevnt6", &self.extevnt6())
            .field("extevnt5", &self.extevnt5())
            .field("extevnt4", &self.extevnt4())
            .field("extevnt3", &self.extevnt3())
            .field("extevnt2", &self.extevnt2())
            .field("extevnt1", &self.extevnt1())
            .field("timevnt9", &self.timevnt9())
            .field("timevnt8", &self.timevnt8())
            .field("timevnt7", &self.timevnt7())
            .field("timevnt6", &self.timevnt6())
            .field("timevnt5", &self.timevnt5())
            .field("timevnt4", &self.timevnt4())
            .field("timevnt3", &self.timevnt3())
            .field("timevnt2", &self.timevnt2())
            .field("timevnt1", &self.timevnt1())
            .field("mstcmp4", &self.mstcmp4())
            .field("mstcmp3", &self.mstcmp3())
            .field("mstcmp2", &self.mstcmp2())
            .field("mstcmp1", &self.mstcmp1())
            .field("mstper", &self.mstper())
            .field("cmp4", &self.cmp4())
            .field("cmp3", &self.cmp3())
            .field("cmp2", &self.cmp2())
            .field("cmp1", &self.cmp1())
            .field("per", &self.per())
            .field("resync", &self.resync())
            .field("sst", &self.sst())
            .finish()
    }
}
impl W {
    ///Bit 0 - Software Set trigger
    #[inline(always)]
    #[must_use]
    pub fn sst(&mut self) -> SST_W<SETD1Rrs> {
        SST_W::new(self, 0)
    }
    ///Bit 1 - Timer A resynchronizaton
    #[inline(always)]
    #[must_use]
    pub fn resync(&mut self) -> RESYNC_W<SETD1Rrs> {
        RESYNC_W::new(self, 1)
    }
    ///Bit 2 - Timer A Period
    #[inline(always)]
    #[must_use]
    pub fn per(&mut self) -> PER_W<SETD1Rrs> {
        PER_W::new(self, 2)
    }
    ///Bit 3 - Timer A compare 1
    #[inline(always)]
    #[must_use]
    pub fn cmp1(&mut self) -> CMP1_W<SETD1Rrs> {
        CMP1_W::new(self, 3)
    }
    ///Bit 4 - Timer A compare 2
    #[inline(always)]
    #[must_use]
    pub fn cmp2(&mut self) -> CMP2_W<SETD1Rrs> {
        CMP2_W::new(self, 4)
    }
    ///Bit 5 - Timer A compare 3
    #[inline(always)]
    #[must_use]
    pub fn cmp3(&mut self) -> CMP3_W<SETD1Rrs> {
        CMP3_W::new(self, 5)
    }
    ///Bit 6 - Timer A compare 4
    #[inline(always)]
    #[must_use]
    pub fn cmp4(&mut self) -> CMP4_W<SETD1Rrs> {
        CMP4_W::new(self, 6)
    }
    ///Bit 7 - Master Period
    #[inline(always)]
    #[must_use]
    pub fn mstper(&mut self) -> MSTPER_W<SETD1Rrs> {
        MSTPER_W::new(self, 7)
    }
    ///Bit 8 - Master Compare 1
    #[inline(always)]
    #[must_use]
    pub fn mstcmp1(&mut self) -> MSTCMP1_W<SETD1Rrs> {
        MSTCMP1_W::new(self, 8)
    }
    ///Bit 9 - Master Compare 2
    #[inline(always)]
    #[must_use]
    pub fn mstcmp2(&mut self) -> MSTCMP2_W<SETD1Rrs> {
        MSTCMP2_W::new(self, 9)
    }
    ///Bit 10 - Master Compare 3
    #[inline(always)]
    #[must_use]
    pub fn mstcmp3(&mut self) -> MSTCMP3_W<SETD1Rrs> {
        MSTCMP3_W::new(self, 10)
    }
    ///Bit 11 - Master Compare 4
    #[inline(always)]
    #[must_use]
    pub fn mstcmp4(&mut self) -> MSTCMP4_W<SETD1Rrs> {
        MSTCMP4_W::new(self, 11)
    }
    ///Bit 12 - Timer Event 1
    #[inline(always)]
    #[must_use]
    pub fn timevnt1(&mut self) -> TIMEVNT1_W<SETD1Rrs> {
        TIMEVNT1_W::new(self, 12)
    }
    ///Bit 13 - Timer Event 2
    #[inline(always)]
    #[must_use]
    pub fn timevnt2(&mut self) -> TIMEVNT2_W<SETD1Rrs> {
        TIMEVNT2_W::new(self, 13)
    }
    ///Bit 14 - Timer Event 3
    #[inline(always)]
    #[must_use]
    pub fn timevnt3(&mut self) -> TIMEVNT3_W<SETD1Rrs> {
        TIMEVNT3_W::new(self, 14)
    }
    ///Bit 15 - Timer Event 4
    #[inline(always)]
    #[must_use]
    pub fn timevnt4(&mut self) -> TIMEVNT4_W<SETD1Rrs> {
        TIMEVNT4_W::new(self, 15)
    }
    ///Bit 16 - Timer Event 5
    #[inline(always)]
    #[must_use]
    pub fn timevnt5(&mut self) -> TIMEVNT5_W<SETD1Rrs> {
        TIMEVNT5_W::new(self, 16)
    }
    ///Bit 17 - Timer Event 6
    #[inline(always)]
    #[must_use]
    pub fn timevnt6(&mut self) -> TIMEVNT6_W<SETD1Rrs> {
        TIMEVNT6_W::new(self, 17)
    }
    ///Bit 18 - Timer Event 7
    #[inline(always)]
    #[must_use]
    pub fn timevnt7(&mut self) -> TIMEVNT7_W<SETD1Rrs> {
        TIMEVNT7_W::new(self, 18)
    }
    ///Bit 19 - Timer Event 8
    #[inline(always)]
    #[must_use]
    pub fn timevnt8(&mut self) -> TIMEVNT8_W<SETD1Rrs> {
        TIMEVNT8_W::new(self, 19)
    }
    ///Bit 20 - Timer Event 9
    #[inline(always)]
    #[must_use]
    pub fn timevnt9(&mut self) -> TIMEVNT9_W<SETD1Rrs> {
        TIMEVNT9_W::new(self, 20)
    }
    ///Bit 21 - External Event 1
    #[inline(always)]
    #[must_use]
    pub fn extevnt1(&mut self) -> EXTEVNT1_W<SETD1Rrs> {
        EXTEVNT1_W::new(self, 21)
    }
    ///Bit 22 - External Event 2
    #[inline(always)]
    #[must_use]
    pub fn extevnt2(&mut self) -> EXTEVNT2_W<SETD1Rrs> {
        EXTEVNT2_W::new(self, 22)
    }
    ///Bit 23 - External Event 3
    #[inline(always)]
    #[must_use]
    pub fn extevnt3(&mut self) -> EXTEVNT3_W<SETD1Rrs> {
        EXTEVNT3_W::new(self, 23)
    }
    ///Bit 24 - External Event 4
    #[inline(always)]
    #[must_use]
    pub fn extevnt4(&mut self) -> EXTEVNT4_W<SETD1Rrs> {
        EXTEVNT4_W::new(self, 24)
    }
    ///Bit 25 - External Event 5
    #[inline(always)]
    #[must_use]
    pub fn extevnt5(&mut self) -> EXTEVNT5_W<SETD1Rrs> {
        EXTEVNT5_W::new(self, 25)
    }
    ///Bit 26 - External Event 6
    #[inline(always)]
    #[must_use]
    pub fn extevnt6(&mut self) -> EXTEVNT6_W<SETD1Rrs> {
        EXTEVNT6_W::new(self, 26)
    }
    ///Bit 27 - External Event 7
    #[inline(always)]
    #[must_use]
    pub fn extevnt7(&mut self) -> EXTEVNT7_W<SETD1Rrs> {
        EXTEVNT7_W::new(self, 27)
    }
    ///Bit 28 - External Event 8
    #[inline(always)]
    #[must_use]
    pub fn extevnt8(&mut self) -> EXTEVNT8_W<SETD1Rrs> {
        EXTEVNT8_W::new(self, 28)
    }
    ///Bit 29 - External Event 9
    #[inline(always)]
    #[must_use]
    pub fn extevnt9(&mut self) -> EXTEVNT9_W<SETD1Rrs> {
        EXTEVNT9_W::new(self, 29)
    }
    ///Bit 30 - External Event 10
    #[inline(always)]
    #[must_use]
    pub fn extevnt10(&mut self) -> EXTEVNT10_W<SETD1Rrs> {
        EXTEVNT10_W::new(self, 30)
    }
    ///Bit 31 - Registers update (transfer preload to active)
    #[inline(always)]
    #[must_use]
    pub fn update(&mut self) -> UPDATE_W<SETD1Rrs> {
        UPDATE_W::new(self, 31)
    }
}
/**Timerx Output1 Set Register

You can [`read`](crate::Reg::read) this register and get [`setd1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`setd1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#HRTIM_TIMD:SETD1R)*/
pub struct SETD1Rrs;
impl crate::RegisterSpec for SETD1Rrs {
    type Ux = u32;
}
///`read()` method returns [`setd1r::R`](R) reader structure
impl crate::Readable for SETD1Rrs {}
///`write(|w| ..)` method takes [`setd1r::W`](W) writer structure
impl crate::Writable for SETD1Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SETD1R to value 0
impl crate::Resettable for SETD1Rrs {
    const RESET_VALUE: u32 = 0;
}

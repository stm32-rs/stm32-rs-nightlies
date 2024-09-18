///Register `RSTCR` reader
pub type R = crate::R<RSTCRrs>;
///Register `RSTCR` writer
pub type W = crate::W<RSTCRrs>;
///Field `UPDT` reader - Timer A Update reset
pub type UPDT_R = crate::BitReader;
///Field `UPDT` writer - Timer A Update reset
pub type UPDT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMP2` reader - Timer A compare 2 reset
pub type CMP2_R = crate::BitReader;
///Field `CMP2` writer - Timer A compare 2 reset
pub type CMP2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMP4` reader - Timer A compare 4 reset
pub type CMP4_R = crate::BitReader;
///Field `CMP4` writer - Timer A compare 4 reset
pub type CMP4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSTPER` reader - Master timer Period
pub type MSTPER_R = crate::BitReader;
///Field `MSTPER` writer - Master timer Period
pub type MSTPER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSTCMP1` reader - Master compare 1
pub type MSTCMP1_R = crate::BitReader;
///Field `MSTCMP1` writer - Master compare 1
pub type MSTCMP1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSTCMP2` reader - Master compare 2
pub type MSTCMP2_R = crate::BitReader;
///Field `MSTCMP2` writer - Master compare 2
pub type MSTCMP2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSTCMP3` reader - Master compare 3
pub type MSTCMP3_R = crate::BitReader;
///Field `MSTCMP3` writer - Master compare 3
pub type MSTCMP3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSTCMP4` reader - Master compare 4
pub type MSTCMP4_R = crate::BitReader;
///Field `MSTCMP4` writer - Master compare 4
pub type MSTCMP4_W<'a, REG> = crate::BitWriter<'a, REG>;
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
///Field `TIMACMP1` reader - Timer A Compare 1
pub type TIMACMP1_R = crate::BitReader;
///Field `TIMACMP1` writer - Timer A Compare 1
pub type TIMACMP1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMACMP2` reader - Timer A Compare 2
pub type TIMACMP2_R = crate::BitReader;
///Field `TIMACMP2` writer - Timer A Compare 2
pub type TIMACMP2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMACMP4` reader - Timer A Compare 4
pub type TIMACMP4_R = crate::BitReader;
///Field `TIMACMP4` writer - Timer A Compare 4
pub type TIMACMP4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMBCMP1` reader - Timer B Compare 1
pub type TIMBCMP1_R = crate::BitReader;
///Field `TIMBCMP1` writer - Timer B Compare 1
pub type TIMBCMP1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMBCMP2` reader - Timer B Compare 2
pub type TIMBCMP2_R = crate::BitReader;
///Field `TIMBCMP2` writer - Timer B Compare 2
pub type TIMBCMP2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMBCMP4` reader - Timer B Compare 4
pub type TIMBCMP4_R = crate::BitReader;
///Field `TIMBCMP4` writer - Timer B Compare 4
pub type TIMBCMP4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMDCMP1` reader - Timer D Compare 1
pub type TIMDCMP1_R = crate::BitReader;
///Field `TIMDCMP1` writer - Timer D Compare 1
pub type TIMDCMP1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMDCMP2` reader - Timer D Compare 2
pub type TIMDCMP2_R = crate::BitReader;
///Field `TIMDCMP2` writer - Timer D Compare 2
pub type TIMDCMP2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMDCMP4` reader - Timer D Compare 4
pub type TIMDCMP4_R = crate::BitReader;
///Field `TIMDCMP4` writer - Timer D Compare 4
pub type TIMDCMP4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMECMP1` reader - Timer E Compare 1
pub type TIMECMP1_R = crate::BitReader;
///Field `TIMECMP1` writer - Timer E Compare 1
pub type TIMECMP1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMECMP2` reader - Timer E Compare 2
pub type TIMECMP2_R = crate::BitReader;
///Field `TIMECMP2` writer - Timer E Compare 2
pub type TIMECMP2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMECMP4` reader - Timer E Compare 4
pub type TIMECMP4_R = crate::BitReader;
///Field `TIMECMP4` writer - Timer E Compare 4
pub type TIMECMP4_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - Timer A Update reset
    #[inline(always)]
    pub fn updt(&self) -> UPDT_R {
        UPDT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Timer A compare 2 reset
    #[inline(always)]
    pub fn cmp2(&self) -> CMP2_R {
        CMP2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Timer A compare 4 reset
    #[inline(always)]
    pub fn cmp4(&self) -> CMP4_R {
        CMP4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Master timer Period
    #[inline(always)]
    pub fn mstper(&self) -> MSTPER_R {
        MSTPER_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Master compare 1
    #[inline(always)]
    pub fn mstcmp1(&self) -> MSTCMP1_R {
        MSTCMP1_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Master compare 2
    #[inline(always)]
    pub fn mstcmp2(&self) -> MSTCMP2_R {
        MSTCMP2_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Master compare 3
    #[inline(always)]
    pub fn mstcmp3(&self) -> MSTCMP3_R {
        MSTCMP3_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Master compare 4
    #[inline(always)]
    pub fn mstcmp4(&self) -> MSTCMP4_R {
        MSTCMP4_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - External Event 1
    #[inline(always)]
    pub fn extevnt1(&self) -> EXTEVNT1_R {
        EXTEVNT1_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - External Event 2
    #[inline(always)]
    pub fn extevnt2(&self) -> EXTEVNT2_R {
        EXTEVNT2_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - External Event 3
    #[inline(always)]
    pub fn extevnt3(&self) -> EXTEVNT3_R {
        EXTEVNT3_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - External Event 4
    #[inline(always)]
    pub fn extevnt4(&self) -> EXTEVNT4_R {
        EXTEVNT4_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - External Event 5
    #[inline(always)]
    pub fn extevnt5(&self) -> EXTEVNT5_R {
        EXTEVNT5_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - External Event 6
    #[inline(always)]
    pub fn extevnt6(&self) -> EXTEVNT6_R {
        EXTEVNT6_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - External Event 7
    #[inline(always)]
    pub fn extevnt7(&self) -> EXTEVNT7_R {
        EXTEVNT7_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - External Event 8
    #[inline(always)]
    pub fn extevnt8(&self) -> EXTEVNT8_R {
        EXTEVNT8_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - External Event 9
    #[inline(always)]
    pub fn extevnt9(&self) -> EXTEVNT9_R {
        EXTEVNT9_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - External Event 10
    #[inline(always)]
    pub fn extevnt10(&self) -> EXTEVNT10_R {
        EXTEVNT10_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Timer A Compare 1
    #[inline(always)]
    pub fn timacmp1(&self) -> TIMACMP1_R {
        TIMACMP1_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Timer A Compare 2
    #[inline(always)]
    pub fn timacmp2(&self) -> TIMACMP2_R {
        TIMACMP2_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Timer A Compare 4
    #[inline(always)]
    pub fn timacmp4(&self) -> TIMACMP4_R {
        TIMACMP4_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Timer B Compare 1
    #[inline(always)]
    pub fn timbcmp1(&self) -> TIMBCMP1_R {
        TIMBCMP1_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Timer B Compare 2
    #[inline(always)]
    pub fn timbcmp2(&self) -> TIMBCMP2_R {
        TIMBCMP2_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Timer B Compare 4
    #[inline(always)]
    pub fn timbcmp4(&self) -> TIMBCMP4_R {
        TIMBCMP4_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Timer D Compare 1
    #[inline(always)]
    pub fn timdcmp1(&self) -> TIMDCMP1_R {
        TIMDCMP1_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Timer D Compare 2
    #[inline(always)]
    pub fn timdcmp2(&self) -> TIMDCMP2_R {
        TIMDCMP2_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Timer D Compare 4
    #[inline(always)]
    pub fn timdcmp4(&self) -> TIMDCMP4_R {
        TIMDCMP4_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Timer E Compare 1
    #[inline(always)]
    pub fn timecmp1(&self) -> TIMECMP1_R {
        TIMECMP1_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Timer E Compare 2
    #[inline(always)]
    pub fn timecmp2(&self) -> TIMECMP2_R {
        TIMECMP2_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Timer E Compare 4
    #[inline(always)]
    pub fn timecmp4(&self) -> TIMECMP4_R {
        TIMECMP4_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSTCR")
            .field("timecmp4", &self.timecmp4())
            .field("timecmp2", &self.timecmp2())
            .field("timecmp1", &self.timecmp1())
            .field("timdcmp4", &self.timdcmp4())
            .field("timdcmp2", &self.timdcmp2())
            .field("timdcmp1", &self.timdcmp1())
            .field("timbcmp4", &self.timbcmp4())
            .field("timbcmp2", &self.timbcmp2())
            .field("timbcmp1", &self.timbcmp1())
            .field("timacmp4", &self.timacmp4())
            .field("timacmp2", &self.timacmp2())
            .field("timacmp1", &self.timacmp1())
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
            .field("mstcmp4", &self.mstcmp4())
            .field("mstcmp3", &self.mstcmp3())
            .field("mstcmp2", &self.mstcmp2())
            .field("mstcmp1", &self.mstcmp1())
            .field("mstper", &self.mstper())
            .field("cmp4", &self.cmp4())
            .field("cmp2", &self.cmp2())
            .field("updt", &self.updt())
            .finish()
    }
}
impl W {
    ///Bit 1 - Timer A Update reset
    #[inline(always)]
    #[must_use]
    pub fn updt(&mut self) -> UPDT_W<RSTCRrs> {
        UPDT_W::new(self, 1)
    }
    ///Bit 2 - Timer A compare 2 reset
    #[inline(always)]
    #[must_use]
    pub fn cmp2(&mut self) -> CMP2_W<RSTCRrs> {
        CMP2_W::new(self, 2)
    }
    ///Bit 3 - Timer A compare 4 reset
    #[inline(always)]
    #[must_use]
    pub fn cmp4(&mut self) -> CMP4_W<RSTCRrs> {
        CMP4_W::new(self, 3)
    }
    ///Bit 4 - Master timer Period
    #[inline(always)]
    #[must_use]
    pub fn mstper(&mut self) -> MSTPER_W<RSTCRrs> {
        MSTPER_W::new(self, 4)
    }
    ///Bit 5 - Master compare 1
    #[inline(always)]
    #[must_use]
    pub fn mstcmp1(&mut self) -> MSTCMP1_W<RSTCRrs> {
        MSTCMP1_W::new(self, 5)
    }
    ///Bit 6 - Master compare 2
    #[inline(always)]
    #[must_use]
    pub fn mstcmp2(&mut self) -> MSTCMP2_W<RSTCRrs> {
        MSTCMP2_W::new(self, 6)
    }
    ///Bit 7 - Master compare 3
    #[inline(always)]
    #[must_use]
    pub fn mstcmp3(&mut self) -> MSTCMP3_W<RSTCRrs> {
        MSTCMP3_W::new(self, 7)
    }
    ///Bit 8 - Master compare 4
    #[inline(always)]
    #[must_use]
    pub fn mstcmp4(&mut self) -> MSTCMP4_W<RSTCRrs> {
        MSTCMP4_W::new(self, 8)
    }
    ///Bit 9 - External Event 1
    #[inline(always)]
    #[must_use]
    pub fn extevnt1(&mut self) -> EXTEVNT1_W<RSTCRrs> {
        EXTEVNT1_W::new(self, 9)
    }
    ///Bit 10 - External Event 2
    #[inline(always)]
    #[must_use]
    pub fn extevnt2(&mut self) -> EXTEVNT2_W<RSTCRrs> {
        EXTEVNT2_W::new(self, 10)
    }
    ///Bit 11 - External Event 3
    #[inline(always)]
    #[must_use]
    pub fn extevnt3(&mut self) -> EXTEVNT3_W<RSTCRrs> {
        EXTEVNT3_W::new(self, 11)
    }
    ///Bit 12 - External Event 4
    #[inline(always)]
    #[must_use]
    pub fn extevnt4(&mut self) -> EXTEVNT4_W<RSTCRrs> {
        EXTEVNT4_W::new(self, 12)
    }
    ///Bit 13 - External Event 5
    #[inline(always)]
    #[must_use]
    pub fn extevnt5(&mut self) -> EXTEVNT5_W<RSTCRrs> {
        EXTEVNT5_W::new(self, 13)
    }
    ///Bit 14 - External Event 6
    #[inline(always)]
    #[must_use]
    pub fn extevnt6(&mut self) -> EXTEVNT6_W<RSTCRrs> {
        EXTEVNT6_W::new(self, 14)
    }
    ///Bit 15 - External Event 7
    #[inline(always)]
    #[must_use]
    pub fn extevnt7(&mut self) -> EXTEVNT7_W<RSTCRrs> {
        EXTEVNT7_W::new(self, 15)
    }
    ///Bit 16 - External Event 8
    #[inline(always)]
    #[must_use]
    pub fn extevnt8(&mut self) -> EXTEVNT8_W<RSTCRrs> {
        EXTEVNT8_W::new(self, 16)
    }
    ///Bit 17 - External Event 9
    #[inline(always)]
    #[must_use]
    pub fn extevnt9(&mut self) -> EXTEVNT9_W<RSTCRrs> {
        EXTEVNT9_W::new(self, 17)
    }
    ///Bit 18 - External Event 10
    #[inline(always)]
    #[must_use]
    pub fn extevnt10(&mut self) -> EXTEVNT10_W<RSTCRrs> {
        EXTEVNT10_W::new(self, 18)
    }
    ///Bit 19 - Timer A Compare 1
    #[inline(always)]
    #[must_use]
    pub fn timacmp1(&mut self) -> TIMACMP1_W<RSTCRrs> {
        TIMACMP1_W::new(self, 19)
    }
    ///Bit 20 - Timer A Compare 2
    #[inline(always)]
    #[must_use]
    pub fn timacmp2(&mut self) -> TIMACMP2_W<RSTCRrs> {
        TIMACMP2_W::new(self, 20)
    }
    ///Bit 21 - Timer A Compare 4
    #[inline(always)]
    #[must_use]
    pub fn timacmp4(&mut self) -> TIMACMP4_W<RSTCRrs> {
        TIMACMP4_W::new(self, 21)
    }
    ///Bit 22 - Timer B Compare 1
    #[inline(always)]
    #[must_use]
    pub fn timbcmp1(&mut self) -> TIMBCMP1_W<RSTCRrs> {
        TIMBCMP1_W::new(self, 22)
    }
    ///Bit 23 - Timer B Compare 2
    #[inline(always)]
    #[must_use]
    pub fn timbcmp2(&mut self) -> TIMBCMP2_W<RSTCRrs> {
        TIMBCMP2_W::new(self, 23)
    }
    ///Bit 24 - Timer B Compare 4
    #[inline(always)]
    #[must_use]
    pub fn timbcmp4(&mut self) -> TIMBCMP4_W<RSTCRrs> {
        TIMBCMP4_W::new(self, 24)
    }
    ///Bit 25 - Timer D Compare 1
    #[inline(always)]
    #[must_use]
    pub fn timdcmp1(&mut self) -> TIMDCMP1_W<RSTCRrs> {
        TIMDCMP1_W::new(self, 25)
    }
    ///Bit 26 - Timer D Compare 2
    #[inline(always)]
    #[must_use]
    pub fn timdcmp2(&mut self) -> TIMDCMP2_W<RSTCRrs> {
        TIMDCMP2_W::new(self, 26)
    }
    ///Bit 27 - Timer D Compare 4
    #[inline(always)]
    #[must_use]
    pub fn timdcmp4(&mut self) -> TIMDCMP4_W<RSTCRrs> {
        TIMDCMP4_W::new(self, 27)
    }
    ///Bit 28 - Timer E Compare 1
    #[inline(always)]
    #[must_use]
    pub fn timecmp1(&mut self) -> TIMECMP1_W<RSTCRrs> {
        TIMECMP1_W::new(self, 28)
    }
    ///Bit 29 - Timer E Compare 2
    #[inline(always)]
    #[must_use]
    pub fn timecmp2(&mut self) -> TIMECMP2_W<RSTCRrs> {
        TIMECMP2_W::new(self, 29)
    }
    ///Bit 30 - Timer E Compare 4
    #[inline(always)]
    #[must_use]
    pub fn timecmp4(&mut self) -> TIMECMP4_W<RSTCRrs> {
        TIMECMP4_W::new(self, 30)
    }
}
/**TimerA Reset Register

You can [`read`](crate::Reg::read) this register and get [`rstcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM7.html#HRTIM_TIMC:RSTCR)*/
pub struct RSTCRrs;
impl crate::RegisterSpec for RSTCRrs {
    type Ux = u32;
}
///`read()` method returns [`rstcr::R`](R) reader structure
impl crate::Readable for RSTCRrs {}
///`write(|w| ..)` method takes [`rstcr::W`](W) writer structure
impl crate::Writable for RSTCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RSTCR to value 0
impl crate::Resettable for RSTCRrs {
    const RESET_VALUE: u32 = 0;
}

///Register `TZENR1` reader
pub type R = crate::R<TZENR1rs>;
///Register `TZENR1` writer
pub type W = crate::W<TZENR1rs>;
///Field `TZEN0` reader - TZEN0
pub type TZEN0_R = crate::BitReader;
///Field `TZEN0` writer - TZEN0
pub type TZEN0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TZEN1` reader - TZEN1
pub type TZEN1_R = crate::BitReader;
///Field `TZEN1` writer - TZEN1
pub type TZEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TZEN2` reader - TZEN2
pub type TZEN2_R = crate::BitReader;
///Field `TZEN2` writer - TZEN2
pub type TZEN2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TZEN3` reader - TZEN3
pub type TZEN3_R = crate::BitReader;
///Field `TZEN3` writer - TZEN3
pub type TZEN3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TZEN4` reader - TZEN4
pub type TZEN4_R = crate::BitReader;
///Field `TZEN4` writer - TZEN4
pub type TZEN4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TZEN5` reader - TZEN5
pub type TZEN5_R = crate::BitReader;
///Field `TZEN5` writer - TZEN5
pub type TZEN5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TZEN6` reader - TZEN6
pub type TZEN6_R = crate::BitReader;
///Field `TZEN6` writer - TZEN6
pub type TZEN6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TZEN7` reader - TZEN7
pub type TZEN7_R = crate::BitReader;
///Field `TZEN7` writer - TZEN7
pub type TZEN7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TZEN8` reader - TZEN8
pub type TZEN8_R = crate::BitReader;
///Field `TZEN8` writer - TZEN8
pub type TZEN8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TZEN9` reader - TZEN9
pub type TZEN9_R = crate::BitReader;
///Field `TZEN9` writer - TZEN9
pub type TZEN9_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TZEN10` reader - TZEN10
pub type TZEN10_R = crate::BitReader;
///Field `TZEN10` writer - TZEN10
pub type TZEN10_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TZEN11` reader - TZEN11
pub type TZEN11_R = crate::BitReader;
///Field `TZEN11` writer - TZEN11
pub type TZEN11_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TZEN12` reader - TZEN12
pub type TZEN12_R = crate::BitReader;
///Field `TZEN12` writer - TZEN12
pub type TZEN12_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TZEN13` reader - TZEN13
pub type TZEN13_R = crate::BitReader;
///Field `TZEN13` writer - TZEN13
pub type TZEN13_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TZEN14` reader - TZEN14
pub type TZEN14_R = crate::BitReader;
///Field `TZEN14` writer - TZEN14
pub type TZEN14_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TZEN15` reader - TZEN15
pub type TZEN15_R = crate::BitReader;
///Field `TZEN15` writer - TZEN15
pub type TZEN15_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TZEN17` reader - TZEN17
pub type TZEN17_R = crate::BitReader;
///Field `TZEN17` writer - TZEN17
pub type TZEN17_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TZEN18` reader - TZEN18
pub type TZEN18_R = crate::BitReader;
///Field `TZEN18` writer - TZEN18
pub type TZEN18_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TZEN19` reader - TZEN19
pub type TZEN19_R = crate::BitReader;
///Field `TZEN19` writer - TZEN19
pub type TZEN19_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TZEN24` reader - TZEN24
pub type TZEN24_R = crate::BitReader;
///Field `TZEN24` writer - TZEN24
pub type TZEN24_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TZEN26` reader - TZEN26
pub type TZEN26_R = crate::BitReader;
///Field `TZEN26` writer - TZEN26
pub type TZEN26_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TZEN0
    #[inline(always)]
    pub fn tzen0(&self) -> TZEN0_R {
        TZEN0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TZEN1
    #[inline(always)]
    pub fn tzen1(&self) -> TZEN1_R {
        TZEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TZEN2
    #[inline(always)]
    pub fn tzen2(&self) -> TZEN2_R {
        TZEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TZEN3
    #[inline(always)]
    pub fn tzen3(&self) -> TZEN3_R {
        TZEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TZEN4
    #[inline(always)]
    pub fn tzen4(&self) -> TZEN4_R {
        TZEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TZEN5
    #[inline(always)]
    pub fn tzen5(&self) -> TZEN5_R {
        TZEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - TZEN6
    #[inline(always)]
    pub fn tzen6(&self) -> TZEN6_R {
        TZEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - TZEN7
    #[inline(always)]
    pub fn tzen7(&self) -> TZEN7_R {
        TZEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - TZEN8
    #[inline(always)]
    pub fn tzen8(&self) -> TZEN8_R {
        TZEN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - TZEN9
    #[inline(always)]
    pub fn tzen9(&self) -> TZEN9_R {
        TZEN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - TZEN10
    #[inline(always)]
    pub fn tzen10(&self) -> TZEN10_R {
        TZEN10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - TZEN11
    #[inline(always)]
    pub fn tzen11(&self) -> TZEN11_R {
        TZEN11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - TZEN12
    #[inline(always)]
    pub fn tzen12(&self) -> TZEN12_R {
        TZEN12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - TZEN13
    #[inline(always)]
    pub fn tzen13(&self) -> TZEN13_R {
        TZEN13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - TZEN14
    #[inline(always)]
    pub fn tzen14(&self) -> TZEN14_R {
        TZEN14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - TZEN15
    #[inline(always)]
    pub fn tzen15(&self) -> TZEN15_R {
        TZEN15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 17 - TZEN17
    #[inline(always)]
    pub fn tzen17(&self) -> TZEN17_R {
        TZEN17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TZEN18
    #[inline(always)]
    pub fn tzen18(&self) -> TZEN18_R {
        TZEN18_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - TZEN19
    #[inline(always)]
    pub fn tzen19(&self) -> TZEN19_R {
        TZEN19_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 24 - TZEN24
    #[inline(always)]
    pub fn tzen24(&self) -> TZEN24_R {
        TZEN24_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 26 - TZEN26
    #[inline(always)]
    pub fn tzen26(&self) -> TZEN26_R {
        TZEN26_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TZENR1")
            .field("tzen0", &self.tzen0())
            .field("tzen1", &self.tzen1())
            .field("tzen2", &self.tzen2())
            .field("tzen3", &self.tzen3())
            .field("tzen4", &self.tzen4())
            .field("tzen5", &self.tzen5())
            .field("tzen6", &self.tzen6())
            .field("tzen7", &self.tzen7())
            .field("tzen8", &self.tzen8())
            .field("tzen9", &self.tzen9())
            .field("tzen10", &self.tzen10())
            .field("tzen11", &self.tzen11())
            .field("tzen12", &self.tzen12())
            .field("tzen13", &self.tzen13())
            .field("tzen14", &self.tzen14())
            .field("tzen15", &self.tzen15())
            .field("tzen17", &self.tzen17())
            .field("tzen18", &self.tzen18())
            .field("tzen19", &self.tzen19())
            .field("tzen24", &self.tzen24())
            .field("tzen26", &self.tzen26())
            .finish()
    }
}
impl W {
    ///Bit 0 - TZEN0
    #[inline(always)]
    pub fn tzen0(&mut self) -> TZEN0_W<'_, TZENR1rs> {
        TZEN0_W::new(self, 0)
    }
    ///Bit 1 - TZEN1
    #[inline(always)]
    pub fn tzen1(&mut self) -> TZEN1_W<'_, TZENR1rs> {
        TZEN1_W::new(self, 1)
    }
    ///Bit 2 - TZEN2
    #[inline(always)]
    pub fn tzen2(&mut self) -> TZEN2_W<'_, TZENR1rs> {
        TZEN2_W::new(self, 2)
    }
    ///Bit 3 - TZEN3
    #[inline(always)]
    pub fn tzen3(&mut self) -> TZEN3_W<'_, TZENR1rs> {
        TZEN3_W::new(self, 3)
    }
    ///Bit 4 - TZEN4
    #[inline(always)]
    pub fn tzen4(&mut self) -> TZEN4_W<'_, TZENR1rs> {
        TZEN4_W::new(self, 4)
    }
    ///Bit 5 - TZEN5
    #[inline(always)]
    pub fn tzen5(&mut self) -> TZEN5_W<'_, TZENR1rs> {
        TZEN5_W::new(self, 5)
    }
    ///Bit 6 - TZEN6
    #[inline(always)]
    pub fn tzen6(&mut self) -> TZEN6_W<'_, TZENR1rs> {
        TZEN6_W::new(self, 6)
    }
    ///Bit 7 - TZEN7
    #[inline(always)]
    pub fn tzen7(&mut self) -> TZEN7_W<'_, TZENR1rs> {
        TZEN7_W::new(self, 7)
    }
    ///Bit 8 - TZEN8
    #[inline(always)]
    pub fn tzen8(&mut self) -> TZEN8_W<'_, TZENR1rs> {
        TZEN8_W::new(self, 8)
    }
    ///Bit 9 - TZEN9
    #[inline(always)]
    pub fn tzen9(&mut self) -> TZEN9_W<'_, TZENR1rs> {
        TZEN9_W::new(self, 9)
    }
    ///Bit 10 - TZEN10
    #[inline(always)]
    pub fn tzen10(&mut self) -> TZEN10_W<'_, TZENR1rs> {
        TZEN10_W::new(self, 10)
    }
    ///Bit 11 - TZEN11
    #[inline(always)]
    pub fn tzen11(&mut self) -> TZEN11_W<'_, TZENR1rs> {
        TZEN11_W::new(self, 11)
    }
    ///Bit 12 - TZEN12
    #[inline(always)]
    pub fn tzen12(&mut self) -> TZEN12_W<'_, TZENR1rs> {
        TZEN12_W::new(self, 12)
    }
    ///Bit 13 - TZEN13
    #[inline(always)]
    pub fn tzen13(&mut self) -> TZEN13_W<'_, TZENR1rs> {
        TZEN13_W::new(self, 13)
    }
    ///Bit 14 - TZEN14
    #[inline(always)]
    pub fn tzen14(&mut self) -> TZEN14_W<'_, TZENR1rs> {
        TZEN14_W::new(self, 14)
    }
    ///Bit 15 - TZEN15
    #[inline(always)]
    pub fn tzen15(&mut self) -> TZEN15_W<'_, TZENR1rs> {
        TZEN15_W::new(self, 15)
    }
    ///Bit 17 - TZEN17
    #[inline(always)]
    pub fn tzen17(&mut self) -> TZEN17_W<'_, TZENR1rs> {
        TZEN17_W::new(self, 17)
    }
    ///Bit 18 - TZEN18
    #[inline(always)]
    pub fn tzen18(&mut self) -> TZEN18_W<'_, TZENR1rs> {
        TZEN18_W::new(self, 18)
    }
    ///Bit 19 - TZEN19
    #[inline(always)]
    pub fn tzen19(&mut self) -> TZEN19_W<'_, TZENR1rs> {
        TZEN19_W::new(self, 19)
    }
    ///Bit 24 - TZEN24
    #[inline(always)]
    pub fn tzen24(&mut self) -> TZEN24_W<'_, TZENR1rs> {
        TZEN24_W::new(self, 24)
    }
    ///Bit 26 - TZEN26
    #[inline(always)]
    pub fn tzen26(&mut self) -> TZEN26_W<'_, TZENR1rs> {
        TZEN26_W::new(self, 26)
    }
}
/**This register provides TrustZone Write access security, a non-secure write access will generate a bus error. A non-secure read will return the register data. Contains only register bits for TrustZone capable Input events.

You can [`read`](crate::Reg::read) this register and get [`tzenr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzenr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#EXTI:TZENR1)*/
pub struct TZENR1rs;
impl crate::RegisterSpec for TZENR1rs {
    type Ux = u32;
}
///`read()` method returns [`tzenr1::R`](R) reader structure
impl crate::Readable for TZENR1rs {}
///`write(|w| ..)` method takes [`tzenr1::W`](W) writer structure
impl crate::Writable for TZENR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TZENR1 to value 0
impl crate::Resettable for TZENR1rs {}

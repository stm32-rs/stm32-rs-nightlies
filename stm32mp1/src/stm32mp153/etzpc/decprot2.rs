///Register `DECPROT2` reader
pub type R = crate::R<DECPROT2rs>;
///Register `DECPROT2` writer
pub type W = crate::W<DECPROT2rs>;
///Field `DECPROT0` reader - DECPROT0
pub type DECPROT0_R = crate::FieldReader;
///Field `DECPROT0` writer - DECPROT0
pub type DECPROT0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DECPROT1` reader - DECPROT1
pub type DECPROT1_R = crate::FieldReader;
///Field `DECPROT1` writer - DECPROT1
pub type DECPROT1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DECPROT2` reader - DECPROT2
pub type DECPROT2_R = crate::FieldReader;
///Field `DECPROT2` writer - DECPROT2
pub type DECPROT2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DECPROT3` reader - DECPROT3
pub type DECPROT3_R = crate::FieldReader;
///Field `DECPROT3` writer - DECPROT3
pub type DECPROT3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DECPROT4` reader - DECPROT4
pub type DECPROT4_R = crate::FieldReader;
///Field `DECPROT4` writer - DECPROT4
pub type DECPROT4_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DECPROT5` reader - DECPROT5
pub type DECPROT5_R = crate::FieldReader;
///Field `DECPROT5` writer - DECPROT5
pub type DECPROT5_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DECPROT6` reader - DECPROT6
pub type DECPROT6_R = crate::FieldReader;
///Field `DECPROT6` writer - DECPROT6
pub type DECPROT6_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DECPROT7` reader - DECPROT7
pub type DECPROT7_R = crate::FieldReader;
///Field `DECPROT7` writer - DECPROT7
pub type DECPROT7_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DECPROT8` reader - DECPROT8
pub type DECPROT8_R = crate::FieldReader;
///Field `DECPROT8` writer - DECPROT8
pub type DECPROT8_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DECPROT9` reader - DECPROT9
pub type DECPROT9_R = crate::FieldReader;
///Field `DECPROT9` writer - DECPROT9
pub type DECPROT9_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DECPROT10` reader - DECPROT10
pub type DECPROT10_R = crate::FieldReader;
///Field `DECPROT10` writer - DECPROT10
pub type DECPROT10_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DECPROT11` reader - DECPROT11
pub type DECPROT11_R = crate::FieldReader;
///Field `DECPROT11` writer - DECPROT11
pub type DECPROT11_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DECPROT12` reader - DECPROT12
pub type DECPROT12_R = crate::FieldReader;
///Field `DECPROT12` writer - DECPROT12
pub type DECPROT12_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DECPROT13` reader - DECPROT13
pub type DECPROT13_R = crate::FieldReader;
///Field `DECPROT13` writer - DECPROT13
pub type DECPROT13_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DECPROT14` reader - DECPROT14
pub type DECPROT14_R = crate::FieldReader;
///Field `DECPROT14` writer - DECPROT14
pub type DECPROT14_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DECPROT15` reader - DECPROT15
pub type DECPROT15_R = crate::FieldReader;
///Field `DECPROT15` writer - DECPROT15
pub type DECPROT15_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - DECPROT0
    #[inline(always)]
    pub fn decprot0(&self) -> DECPROT0_R {
        DECPROT0_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - DECPROT1
    #[inline(always)]
    pub fn decprot1(&self) -> DECPROT1_R {
        DECPROT1_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - DECPROT2
    #[inline(always)]
    pub fn decprot2(&self) -> DECPROT2_R {
        DECPROT2_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - DECPROT3
    #[inline(always)]
    pub fn decprot3(&self) -> DECPROT3_R {
        DECPROT3_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - DECPROT4
    #[inline(always)]
    pub fn decprot4(&self) -> DECPROT4_R {
        DECPROT4_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - DECPROT5
    #[inline(always)]
    pub fn decprot5(&self) -> DECPROT5_R {
        DECPROT5_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - DECPROT6
    #[inline(always)]
    pub fn decprot6(&self) -> DECPROT6_R {
        DECPROT6_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - DECPROT7
    #[inline(always)]
    pub fn decprot7(&self) -> DECPROT7_R {
        DECPROT7_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:17 - DECPROT8
    #[inline(always)]
    pub fn decprot8(&self) -> DECPROT8_R {
        DECPROT8_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - DECPROT9
    #[inline(always)]
    pub fn decprot9(&self) -> DECPROT9_R {
        DECPROT9_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:21 - DECPROT10
    #[inline(always)]
    pub fn decprot10(&self) -> DECPROT10_R {
        DECPROT10_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23 - DECPROT11
    #[inline(always)]
    pub fn decprot11(&self) -> DECPROT11_R {
        DECPROT11_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 24:25 - DECPROT12
    #[inline(always)]
    pub fn decprot12(&self) -> DECPROT12_R {
        DECPROT12_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:27 - DECPROT13
    #[inline(always)]
    pub fn decprot13(&self) -> DECPROT13_R {
        DECPROT13_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 28:29 - DECPROT14
    #[inline(always)]
    pub fn decprot14(&self) -> DECPROT14_R {
        DECPROT14_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bits 30:31 - DECPROT15
    #[inline(always)]
    pub fn decprot15(&self) -> DECPROT15_R {
        DECPROT15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DECPROT2")
            .field("decprot0", &self.decprot0())
            .field("decprot1", &self.decprot1())
            .field("decprot2", &self.decprot2())
            .field("decprot3", &self.decprot3())
            .field("decprot4", &self.decprot4())
            .field("decprot5", &self.decprot5())
            .field("decprot6", &self.decprot6())
            .field("decprot7", &self.decprot7())
            .field("decprot8", &self.decprot8())
            .field("decprot9", &self.decprot9())
            .field("decprot10", &self.decprot10())
            .field("decprot11", &self.decprot11())
            .field("decprot12", &self.decprot12())
            .field("decprot13", &self.decprot13())
            .field("decprot14", &self.decprot14())
            .field("decprot15", &self.decprot15())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - DECPROT0
    #[inline(always)]
    pub fn decprot0(&mut self) -> DECPROT0_W<'_, DECPROT2rs> {
        DECPROT0_W::new(self, 0)
    }
    ///Bits 2:3 - DECPROT1
    #[inline(always)]
    pub fn decprot1(&mut self) -> DECPROT1_W<'_, DECPROT2rs> {
        DECPROT1_W::new(self, 2)
    }
    ///Bits 4:5 - DECPROT2
    #[inline(always)]
    pub fn decprot2(&mut self) -> DECPROT2_W<'_, DECPROT2rs> {
        DECPROT2_W::new(self, 4)
    }
    ///Bits 6:7 - DECPROT3
    #[inline(always)]
    pub fn decprot3(&mut self) -> DECPROT3_W<'_, DECPROT2rs> {
        DECPROT3_W::new(self, 6)
    }
    ///Bits 8:9 - DECPROT4
    #[inline(always)]
    pub fn decprot4(&mut self) -> DECPROT4_W<'_, DECPROT2rs> {
        DECPROT4_W::new(self, 8)
    }
    ///Bits 10:11 - DECPROT5
    #[inline(always)]
    pub fn decprot5(&mut self) -> DECPROT5_W<'_, DECPROT2rs> {
        DECPROT5_W::new(self, 10)
    }
    ///Bits 12:13 - DECPROT6
    #[inline(always)]
    pub fn decprot6(&mut self) -> DECPROT6_W<'_, DECPROT2rs> {
        DECPROT6_W::new(self, 12)
    }
    ///Bits 14:15 - DECPROT7
    #[inline(always)]
    pub fn decprot7(&mut self) -> DECPROT7_W<'_, DECPROT2rs> {
        DECPROT7_W::new(self, 14)
    }
    ///Bits 16:17 - DECPROT8
    #[inline(always)]
    pub fn decprot8(&mut self) -> DECPROT8_W<'_, DECPROT2rs> {
        DECPROT8_W::new(self, 16)
    }
    ///Bits 18:19 - DECPROT9
    #[inline(always)]
    pub fn decprot9(&mut self) -> DECPROT9_W<'_, DECPROT2rs> {
        DECPROT9_W::new(self, 18)
    }
    ///Bits 20:21 - DECPROT10
    #[inline(always)]
    pub fn decprot10(&mut self) -> DECPROT10_W<'_, DECPROT2rs> {
        DECPROT10_W::new(self, 20)
    }
    ///Bits 22:23 - DECPROT11
    #[inline(always)]
    pub fn decprot11(&mut self) -> DECPROT11_W<'_, DECPROT2rs> {
        DECPROT11_W::new(self, 22)
    }
    ///Bits 24:25 - DECPROT12
    #[inline(always)]
    pub fn decprot12(&mut self) -> DECPROT12_W<'_, DECPROT2rs> {
        DECPROT12_W::new(self, 24)
    }
    ///Bits 26:27 - DECPROT13
    #[inline(always)]
    pub fn decprot13(&mut self) -> DECPROT13_W<'_, DECPROT2rs> {
        DECPROT13_W::new(self, 26)
    }
    ///Bits 28:29 - DECPROT14
    #[inline(always)]
    pub fn decprot14(&mut self) -> DECPROT14_W<'_, DECPROT2rs> {
        DECPROT14_W::new(self, 28)
    }
    ///Bits 30:31 - DECPROT15
    #[inline(always)]
    pub fn decprot15(&mut self) -> DECPROT15_W<'_, DECPROT2rs> {
        DECPROT15_W::new(self, 30)
    }
}
/**Register reset values

You can [`read`](crate::Reg::read) this register and get [`decprot2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`decprot2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETZPC:DECPROT2)*/
pub struct DECPROT2rs;
impl crate::RegisterSpec for DECPROT2rs {
    type Ux = u32;
}
///`read()` method returns [`decprot2::R`](R) reader structure
impl crate::Readable for DECPROT2rs {}
///`write(|w| ..)` method takes [`decprot2::W`](W) writer structure
impl crate::Writable for DECPROT2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DECPROT2 to value 0
impl crate::Resettable for DECPROT2rs {}

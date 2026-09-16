///Register `FA1R` reader
pub type R = crate::R<FA1Rrs>;
///Register `FA1R` writer
pub type W = crate::W<FA1Rrs>;
///Field `FACT0` reader - Filter active
pub type FACT0_R = crate::BitReader;
///Field `FACT0` writer - Filter active
pub type FACT0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FACT1` reader - Filter active
pub type FACT1_R = crate::BitReader;
///Field `FACT1` writer - Filter active
pub type FACT1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FACT2` reader - Filter active
pub type FACT2_R = crate::BitReader;
///Field `FACT2` writer - Filter active
pub type FACT2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FACT3` reader - Filter active
pub type FACT3_R = crate::BitReader;
///Field `FACT3` writer - Filter active
pub type FACT3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FACT4` reader - Filter active
pub type FACT4_R = crate::BitReader;
///Field `FACT4` writer - Filter active
pub type FACT4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FACT5` reader - Filter active
pub type FACT5_R = crate::BitReader;
///Field `FACT5` writer - Filter active
pub type FACT5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FACT6` reader - Filter active
pub type FACT6_R = crate::BitReader;
///Field `FACT6` writer - Filter active
pub type FACT6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FACT7` reader - Filter active
pub type FACT7_R = crate::BitReader;
///Field `FACT7` writer - Filter active
pub type FACT7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FACT8` reader - Filter active
pub type FACT8_R = crate::BitReader;
///Field `FACT8` writer - Filter active
pub type FACT8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FACT9` reader - Filter active
pub type FACT9_R = crate::BitReader;
///Field `FACT9` writer - Filter active
pub type FACT9_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FACT10` reader - Filter active
pub type FACT10_R = crate::BitReader;
///Field `FACT10` writer - Filter active
pub type FACT10_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FACT11` reader - Filter active
pub type FACT11_R = crate::BitReader;
///Field `FACT11` writer - Filter active
pub type FACT11_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FACT12` reader - Filter active
pub type FACT12_R = crate::BitReader;
///Field `FACT12` writer - Filter active
pub type FACT12_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FACT13` reader - Filter active
pub type FACT13_R = crate::BitReader;
///Field `FACT13` writer - Filter active
pub type FACT13_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FACT14` reader - Filter active
pub type FACT14_R = crate::BitReader;
///Field `FACT14` writer - Filter active
pub type FACT14_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FACT15` reader - Filter active
pub type FACT15_R = crate::BitReader;
///Field `FACT15` writer - Filter active
pub type FACT15_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FACT16` reader - Filter active
pub type FACT16_R = crate::BitReader;
///Field `FACT16` writer - Filter active
pub type FACT16_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FACT17` reader - Filter active
pub type FACT17_R = crate::BitReader;
///Field `FACT17` writer - Filter active
pub type FACT17_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FACT18` reader - Filter active
pub type FACT18_R = crate::BitReader;
///Field `FACT18` writer - Filter active
pub type FACT18_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FACT19` reader - Filter active
pub type FACT19_R = crate::BitReader;
///Field `FACT19` writer - Filter active
pub type FACT19_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FACT20` reader - Filter active
pub type FACT20_R = crate::BitReader;
///Field `FACT20` writer - Filter active
pub type FACT20_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FACT21` reader - Filter active
pub type FACT21_R = crate::BitReader;
///Field `FACT21` writer - Filter active
pub type FACT21_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FACT22` reader - Filter active
pub type FACT22_R = crate::BitReader;
///Field `FACT22` writer - Filter active
pub type FACT22_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FACT23` reader - Filter active
pub type FACT23_R = crate::BitReader;
///Field `FACT23` writer - Filter active
pub type FACT23_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FACT24` reader - Filter active
pub type FACT24_R = crate::BitReader;
///Field `FACT24` writer - Filter active
pub type FACT24_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FACT25` reader - Filter active
pub type FACT25_R = crate::BitReader;
///Field `FACT25` writer - Filter active
pub type FACT25_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FACT26` reader - Filter active
pub type FACT26_R = crate::BitReader;
///Field `FACT26` writer - Filter active
pub type FACT26_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FACT27` reader - Filter active
pub type FACT27_R = crate::BitReader;
///Field `FACT27` writer - Filter active
pub type FACT27_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Filter active
    #[inline(always)]
    pub fn fact0(&self) -> FACT0_R {
        FACT0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Filter active
    #[inline(always)]
    pub fn fact1(&self) -> FACT1_R {
        FACT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Filter active
    #[inline(always)]
    pub fn fact2(&self) -> FACT2_R {
        FACT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Filter active
    #[inline(always)]
    pub fn fact3(&self) -> FACT3_R {
        FACT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Filter active
    #[inline(always)]
    pub fn fact4(&self) -> FACT4_R {
        FACT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Filter active
    #[inline(always)]
    pub fn fact5(&self) -> FACT5_R {
        FACT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Filter active
    #[inline(always)]
    pub fn fact6(&self) -> FACT6_R {
        FACT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Filter active
    #[inline(always)]
    pub fn fact7(&self) -> FACT7_R {
        FACT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Filter active
    #[inline(always)]
    pub fn fact8(&self) -> FACT8_R {
        FACT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Filter active
    #[inline(always)]
    pub fn fact9(&self) -> FACT9_R {
        FACT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Filter active
    #[inline(always)]
    pub fn fact10(&self) -> FACT10_R {
        FACT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Filter active
    #[inline(always)]
    pub fn fact11(&self) -> FACT11_R {
        FACT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Filter active
    #[inline(always)]
    pub fn fact12(&self) -> FACT12_R {
        FACT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Filter active
    #[inline(always)]
    pub fn fact13(&self) -> FACT13_R {
        FACT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Filter active
    #[inline(always)]
    pub fn fact14(&self) -> FACT14_R {
        FACT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Filter active
    #[inline(always)]
    pub fn fact15(&self) -> FACT15_R {
        FACT15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Filter active
    #[inline(always)]
    pub fn fact16(&self) -> FACT16_R {
        FACT16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Filter active
    #[inline(always)]
    pub fn fact17(&self) -> FACT17_R {
        FACT17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Filter active
    #[inline(always)]
    pub fn fact18(&self) -> FACT18_R {
        FACT18_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Filter active
    #[inline(always)]
    pub fn fact19(&self) -> FACT19_R {
        FACT19_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Filter active
    #[inline(always)]
    pub fn fact20(&self) -> FACT20_R {
        FACT20_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Filter active
    #[inline(always)]
    pub fn fact21(&self) -> FACT21_R {
        FACT21_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Filter active
    #[inline(always)]
    pub fn fact22(&self) -> FACT22_R {
        FACT22_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Filter active
    #[inline(always)]
    pub fn fact23(&self) -> FACT23_R {
        FACT23_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Filter active
    #[inline(always)]
    pub fn fact24(&self) -> FACT24_R {
        FACT24_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Filter active
    #[inline(always)]
    pub fn fact25(&self) -> FACT25_R {
        FACT25_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Filter active
    #[inline(always)]
    pub fn fact26(&self) -> FACT26_R {
        FACT26_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Filter active
    #[inline(always)]
    pub fn fact27(&self) -> FACT27_R {
        FACT27_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FA1R")
            .field("fact0", &self.fact0())
            .field("fact1", &self.fact1())
            .field("fact2", &self.fact2())
            .field("fact3", &self.fact3())
            .field("fact4", &self.fact4())
            .field("fact5", &self.fact5())
            .field("fact6", &self.fact6())
            .field("fact7", &self.fact7())
            .field("fact8", &self.fact8())
            .field("fact9", &self.fact9())
            .field("fact10", &self.fact10())
            .field("fact11", &self.fact11())
            .field("fact12", &self.fact12())
            .field("fact13", &self.fact13())
            .field("fact14", &self.fact14())
            .field("fact15", &self.fact15())
            .field("fact16", &self.fact16())
            .field("fact17", &self.fact17())
            .field("fact18", &self.fact18())
            .field("fact19", &self.fact19())
            .field("fact20", &self.fact20())
            .field("fact21", &self.fact21())
            .field("fact22", &self.fact22())
            .field("fact23", &self.fact23())
            .field("fact24", &self.fact24())
            .field("fact25", &self.fact25())
            .field("fact26", &self.fact26())
            .field("fact27", &self.fact27())
            .finish()
    }
}
impl W {
    ///Bit 0 - Filter active
    #[inline(always)]
    pub fn fact0(&mut self) -> FACT0_W<'_, FA1Rrs> {
        FACT0_W::new(self, 0)
    }
    ///Bit 1 - Filter active
    #[inline(always)]
    pub fn fact1(&mut self) -> FACT1_W<'_, FA1Rrs> {
        FACT1_W::new(self, 1)
    }
    ///Bit 2 - Filter active
    #[inline(always)]
    pub fn fact2(&mut self) -> FACT2_W<'_, FA1Rrs> {
        FACT2_W::new(self, 2)
    }
    ///Bit 3 - Filter active
    #[inline(always)]
    pub fn fact3(&mut self) -> FACT3_W<'_, FA1Rrs> {
        FACT3_W::new(self, 3)
    }
    ///Bit 4 - Filter active
    #[inline(always)]
    pub fn fact4(&mut self) -> FACT4_W<'_, FA1Rrs> {
        FACT4_W::new(self, 4)
    }
    ///Bit 5 - Filter active
    #[inline(always)]
    pub fn fact5(&mut self) -> FACT5_W<'_, FA1Rrs> {
        FACT5_W::new(self, 5)
    }
    ///Bit 6 - Filter active
    #[inline(always)]
    pub fn fact6(&mut self) -> FACT6_W<'_, FA1Rrs> {
        FACT6_W::new(self, 6)
    }
    ///Bit 7 - Filter active
    #[inline(always)]
    pub fn fact7(&mut self) -> FACT7_W<'_, FA1Rrs> {
        FACT7_W::new(self, 7)
    }
    ///Bit 8 - Filter active
    #[inline(always)]
    pub fn fact8(&mut self) -> FACT8_W<'_, FA1Rrs> {
        FACT8_W::new(self, 8)
    }
    ///Bit 9 - Filter active
    #[inline(always)]
    pub fn fact9(&mut self) -> FACT9_W<'_, FA1Rrs> {
        FACT9_W::new(self, 9)
    }
    ///Bit 10 - Filter active
    #[inline(always)]
    pub fn fact10(&mut self) -> FACT10_W<'_, FA1Rrs> {
        FACT10_W::new(self, 10)
    }
    ///Bit 11 - Filter active
    #[inline(always)]
    pub fn fact11(&mut self) -> FACT11_W<'_, FA1Rrs> {
        FACT11_W::new(self, 11)
    }
    ///Bit 12 - Filter active
    #[inline(always)]
    pub fn fact12(&mut self) -> FACT12_W<'_, FA1Rrs> {
        FACT12_W::new(self, 12)
    }
    ///Bit 13 - Filter active
    #[inline(always)]
    pub fn fact13(&mut self) -> FACT13_W<'_, FA1Rrs> {
        FACT13_W::new(self, 13)
    }
    ///Bit 14 - Filter active
    #[inline(always)]
    pub fn fact14(&mut self) -> FACT14_W<'_, FA1Rrs> {
        FACT14_W::new(self, 14)
    }
    ///Bit 15 - Filter active
    #[inline(always)]
    pub fn fact15(&mut self) -> FACT15_W<'_, FA1Rrs> {
        FACT15_W::new(self, 15)
    }
    ///Bit 16 - Filter active
    #[inline(always)]
    pub fn fact16(&mut self) -> FACT16_W<'_, FA1Rrs> {
        FACT16_W::new(self, 16)
    }
    ///Bit 17 - Filter active
    #[inline(always)]
    pub fn fact17(&mut self) -> FACT17_W<'_, FA1Rrs> {
        FACT17_W::new(self, 17)
    }
    ///Bit 18 - Filter active
    #[inline(always)]
    pub fn fact18(&mut self) -> FACT18_W<'_, FA1Rrs> {
        FACT18_W::new(self, 18)
    }
    ///Bit 19 - Filter active
    #[inline(always)]
    pub fn fact19(&mut self) -> FACT19_W<'_, FA1Rrs> {
        FACT19_W::new(self, 19)
    }
    ///Bit 20 - Filter active
    #[inline(always)]
    pub fn fact20(&mut self) -> FACT20_W<'_, FA1Rrs> {
        FACT20_W::new(self, 20)
    }
    ///Bit 21 - Filter active
    #[inline(always)]
    pub fn fact21(&mut self) -> FACT21_W<'_, FA1Rrs> {
        FACT21_W::new(self, 21)
    }
    ///Bit 22 - Filter active
    #[inline(always)]
    pub fn fact22(&mut self) -> FACT22_W<'_, FA1Rrs> {
        FACT22_W::new(self, 22)
    }
    ///Bit 23 - Filter active
    #[inline(always)]
    pub fn fact23(&mut self) -> FACT23_W<'_, FA1Rrs> {
        FACT23_W::new(self, 23)
    }
    ///Bit 24 - Filter active
    #[inline(always)]
    pub fn fact24(&mut self) -> FACT24_W<'_, FA1Rrs> {
        FACT24_W::new(self, 24)
    }
    ///Bit 25 - Filter active
    #[inline(always)]
    pub fn fact25(&mut self) -> FACT25_W<'_, FA1Rrs> {
        FACT25_W::new(self, 25)
    }
    ///Bit 26 - Filter active
    #[inline(always)]
    pub fn fact26(&mut self) -> FACT26_W<'_, FA1Rrs> {
        FACT26_W::new(self, 26)
    }
    ///Bit 27 - Filter active
    #[inline(always)]
    pub fn fact27(&mut self) -> FACT27_W<'_, FA1Rrs> {
        FACT27_W::new(self, 27)
    }
}
/**filter activation register

You can [`read`](crate::Reg::read) this register and get [`fa1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fa1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#CAN1:FA1R)*/
pub struct FA1Rrs;
impl crate::RegisterSpec for FA1Rrs {
    type Ux = u32;
}
///`read()` method returns [`fa1r::R`](R) reader structure
impl crate::Readable for FA1Rrs {}
///`write(|w| ..)` method takes [`fa1r::W`](W) writer structure
impl crate::Writable for FA1Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FA1R to value 0
impl crate::Resettable for FA1Rrs {}

///Register `C2EMR1` reader
pub type R = crate::R<C2EMR1rs>;
///Register `C2EMR1` writer
pub type W = crate::W<C2EMR1rs>;
///Field `EM0` reader - EM0
pub type EM0_R = crate::BitReader;
///Field `EM0` writer - EM0
pub type EM0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM1` reader - EM1
pub type EM1_R = crate::BitReader;
///Field `EM1` writer - EM1
pub type EM1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM2` reader - EM2
pub type EM2_R = crate::BitReader;
///Field `EM2` writer - EM2
pub type EM2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM3` reader - EM3
pub type EM3_R = crate::BitReader;
///Field `EM3` writer - EM3
pub type EM3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM4` reader - EM4
pub type EM4_R = crate::BitReader;
///Field `EM4` writer - EM4
pub type EM4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM5` reader - EM5
pub type EM5_R = crate::BitReader;
///Field `EM5` writer - EM5
pub type EM5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM6` reader - EM6
pub type EM6_R = crate::BitReader;
///Field `EM6` writer - EM6
pub type EM6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM7` reader - EM7
pub type EM7_R = crate::BitReader;
///Field `EM7` writer - EM7
pub type EM7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM8` reader - EM8
pub type EM8_R = crate::BitReader;
///Field `EM8` writer - EM8
pub type EM8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM9` reader - EM9
pub type EM9_R = crate::BitReader;
///Field `EM9` writer - EM9
pub type EM9_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM10` reader - EM10
pub type EM10_R = crate::BitReader;
///Field `EM10` writer - EM10
pub type EM10_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM11` reader - EM11
pub type EM11_R = crate::BitReader;
///Field `EM11` writer - EM11
pub type EM11_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM12` reader - EM12
pub type EM12_R = crate::BitReader;
///Field `EM12` writer - EM12
pub type EM12_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM13` reader - EM13
pub type EM13_R = crate::BitReader;
///Field `EM13` writer - EM13
pub type EM13_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM14` reader - EM14
pub type EM14_R = crate::BitReader;
///Field `EM14` writer - EM14
pub type EM14_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM15` reader - EM15
pub type EM15_R = crate::BitReader;
///Field `EM15` writer - EM15
pub type EM15_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM17` reader - EM17
pub type EM17_R = crate::BitReader;
///Field `EM17` writer - EM17
pub type EM17_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM18` reader - EM18
pub type EM18_R = crate::BitReader;
///Field `EM18` writer - EM18
pub type EM18_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM19` reader - EM19
pub type EM19_R = crate::BitReader;
///Field `EM19` writer - EM19
pub type EM19_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - EM0
    #[inline(always)]
    pub fn em0(&self) -> EM0_R {
        EM0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - EM1
    #[inline(always)]
    pub fn em1(&self) -> EM1_R {
        EM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - EM2
    #[inline(always)]
    pub fn em2(&self) -> EM2_R {
        EM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - EM3
    #[inline(always)]
    pub fn em3(&self) -> EM3_R {
        EM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - EM4
    #[inline(always)]
    pub fn em4(&self) -> EM4_R {
        EM4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - EM5
    #[inline(always)]
    pub fn em5(&self) -> EM5_R {
        EM5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - EM6
    #[inline(always)]
    pub fn em6(&self) -> EM6_R {
        EM6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - EM7
    #[inline(always)]
    pub fn em7(&self) -> EM7_R {
        EM7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - EM8
    #[inline(always)]
    pub fn em8(&self) -> EM8_R {
        EM8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - EM9
    #[inline(always)]
    pub fn em9(&self) -> EM9_R {
        EM9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - EM10
    #[inline(always)]
    pub fn em10(&self) -> EM10_R {
        EM10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - EM11
    #[inline(always)]
    pub fn em11(&self) -> EM11_R {
        EM11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - EM12
    #[inline(always)]
    pub fn em12(&self) -> EM12_R {
        EM12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - EM13
    #[inline(always)]
    pub fn em13(&self) -> EM13_R {
        EM13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - EM14
    #[inline(always)]
    pub fn em14(&self) -> EM14_R {
        EM14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - EM15
    #[inline(always)]
    pub fn em15(&self) -> EM15_R {
        EM15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 17 - EM17
    #[inline(always)]
    pub fn em17(&self) -> EM17_R {
        EM17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - EM18
    #[inline(always)]
    pub fn em18(&self) -> EM18_R {
        EM18_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - EM19
    #[inline(always)]
    pub fn em19(&self) -> EM19_R {
        EM19_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C2EMR1")
            .field("em0", &self.em0())
            .field("em1", &self.em1())
            .field("em2", &self.em2())
            .field("em3", &self.em3())
            .field("em4", &self.em4())
            .field("em5", &self.em5())
            .field("em6", &self.em6())
            .field("em7", &self.em7())
            .field("em8", &self.em8())
            .field("em9", &self.em9())
            .field("em10", &self.em10())
            .field("em11", &self.em11())
            .field("em12", &self.em12())
            .field("em13", &self.em13())
            .field("em14", &self.em14())
            .field("em15", &self.em15())
            .field("em17", &self.em17())
            .field("em18", &self.em18())
            .field("em19", &self.em19())
            .finish()
    }
}
impl W {
    ///Bit 0 - EM0
    #[inline(always)]
    pub fn em0(&mut self) -> EM0_W<'_, C2EMR1rs> {
        EM0_W::new(self, 0)
    }
    ///Bit 1 - EM1
    #[inline(always)]
    pub fn em1(&mut self) -> EM1_W<'_, C2EMR1rs> {
        EM1_W::new(self, 1)
    }
    ///Bit 2 - EM2
    #[inline(always)]
    pub fn em2(&mut self) -> EM2_W<'_, C2EMR1rs> {
        EM2_W::new(self, 2)
    }
    ///Bit 3 - EM3
    #[inline(always)]
    pub fn em3(&mut self) -> EM3_W<'_, C2EMR1rs> {
        EM3_W::new(self, 3)
    }
    ///Bit 4 - EM4
    #[inline(always)]
    pub fn em4(&mut self) -> EM4_W<'_, C2EMR1rs> {
        EM4_W::new(self, 4)
    }
    ///Bit 5 - EM5
    #[inline(always)]
    pub fn em5(&mut self) -> EM5_W<'_, C2EMR1rs> {
        EM5_W::new(self, 5)
    }
    ///Bit 6 - EM6
    #[inline(always)]
    pub fn em6(&mut self) -> EM6_W<'_, C2EMR1rs> {
        EM6_W::new(self, 6)
    }
    ///Bit 7 - EM7
    #[inline(always)]
    pub fn em7(&mut self) -> EM7_W<'_, C2EMR1rs> {
        EM7_W::new(self, 7)
    }
    ///Bit 8 - EM8
    #[inline(always)]
    pub fn em8(&mut self) -> EM8_W<'_, C2EMR1rs> {
        EM8_W::new(self, 8)
    }
    ///Bit 9 - EM9
    #[inline(always)]
    pub fn em9(&mut self) -> EM9_W<'_, C2EMR1rs> {
        EM9_W::new(self, 9)
    }
    ///Bit 10 - EM10
    #[inline(always)]
    pub fn em10(&mut self) -> EM10_W<'_, C2EMR1rs> {
        EM10_W::new(self, 10)
    }
    ///Bit 11 - EM11
    #[inline(always)]
    pub fn em11(&mut self) -> EM11_W<'_, C2EMR1rs> {
        EM11_W::new(self, 11)
    }
    ///Bit 12 - EM12
    #[inline(always)]
    pub fn em12(&mut self) -> EM12_W<'_, C2EMR1rs> {
        EM12_W::new(self, 12)
    }
    ///Bit 13 - EM13
    #[inline(always)]
    pub fn em13(&mut self) -> EM13_W<'_, C2EMR1rs> {
        EM13_W::new(self, 13)
    }
    ///Bit 14 - EM14
    #[inline(always)]
    pub fn em14(&mut self) -> EM14_W<'_, C2EMR1rs> {
        EM14_W::new(self, 14)
    }
    ///Bit 15 - EM15
    #[inline(always)]
    pub fn em15(&mut self) -> EM15_W<'_, C2EMR1rs> {
        EM15_W::new(self, 15)
    }
    ///Bit 17 - EM17
    #[inline(always)]
    pub fn em17(&mut self) -> EM17_W<'_, C2EMR1rs> {
        EM17_W::new(self, 17)
    }
    ///Bit 18 - EM18
    #[inline(always)]
    pub fn em18(&mut self) -> EM18_W<'_, C2EMR1rs> {
        EM18_W::new(self, 18)
    }
    ///Bit 19 - EM19
    #[inline(always)]
    pub fn em19(&mut self) -> EM19_W<'_, C2EMR1rs> {
        EM19_W::new(self, 19)
    }
}
/**EXTI CPU2 wakeup with event mask register

You can [`read`](crate::Reg::read) this register and get [`c2emr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2emr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#EXTI:C2EMR1)*/
pub struct C2EMR1rs;
impl crate::RegisterSpec for C2EMR1rs {
    type Ux = u32;
}
///`read()` method returns [`c2emr1::R`](R) reader structure
impl crate::Readable for C2EMR1rs {}
///`write(|w| ..)` method takes [`c2emr1::W`](W) writer structure
impl crate::Writable for C2EMR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C2EMR1 to value 0
impl crate::Resettable for C2EMR1rs {}

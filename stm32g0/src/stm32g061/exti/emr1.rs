///Register `EMR1` reader
pub type R = crate::R<EMR1rs>;
///Register `EMR1` writer
pub type W = crate::W<EMR1rs>;
/**CPU wakeup with event mask on event input

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EVENT_MASK {
    ///0: Event request line is masked
    Masked = 0,
    ///1: Event request line is unmasked
    Unmasked = 1,
}
impl From<EVENT_MASK> for bool {
    #[inline(always)]
    fn from(variant: EVENT_MASK) -> Self {
        variant as u8 != 0
    }
}
///Field `EM0` reader - CPU wakeup with event mask on event input
pub type EM0_R = crate::BitReader<EVENT_MASK>;
impl EM0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EVENT_MASK {
        match self.bits {
            false => EVENT_MASK::Masked,
            true => EVENT_MASK::Unmasked,
        }
    }
    ///Event request line is masked
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == EVENT_MASK::Masked
    }
    ///Event request line is unmasked
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == EVENT_MASK::Unmasked
    }
}
///Field `EM0` writer - CPU wakeup with event mask on event input
pub type EM0_W<'a, REG> = crate::BitWriter<'a, REG, EVENT_MASK>;
impl<'a, REG> EM0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Event request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(EVENT_MASK::Masked)
    }
    ///Event request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(EVENT_MASK::Unmasked)
    }
}
///Field `EM1` reader - CPU wakeup with event mask on event input
pub use EM0_R as EM1_R;
///Field `EM2` reader - CPU wakeup with event mask on event input
pub use EM0_R as EM2_R;
///Field `EM3` reader - CPU wakeup with event mask on event input
pub use EM0_R as EM3_R;
///Field `EM4` reader - CPU wakeup with event mask on event input
pub use EM0_R as EM4_R;
///Field `EM5` reader - CPU wakeup with event mask on event input
pub use EM0_R as EM5_R;
///Field `EM6` reader - CPU wakeup with event mask on event input
pub use EM0_R as EM6_R;
///Field `EM7` reader - CPU wakeup with event mask on event input
pub use EM0_R as EM7_R;
///Field `EM8` reader - CPU wakeup with event mask on event input
pub use EM0_R as EM8_R;
///Field `EM9` reader - CPU wakeup with event mask on event input
pub use EM0_R as EM9_R;
///Field `EM10` reader - CPU wakeup with event mask on event input
pub use EM0_R as EM10_R;
///Field `EM11` reader - CPU wakeup with event mask on event input
pub use EM0_R as EM11_R;
///Field `EM12` reader - CPU wakeup with event mask on event input
pub use EM0_R as EM12_R;
///Field `EM13` reader - CPU wakeup with event mask on event input
pub use EM0_R as EM13_R;
///Field `EM14` reader - CPU wakeup with event mask on event input
pub use EM0_R as EM14_R;
///Field `EM15` reader - CPU wakeup with event mask on event input
pub use EM0_R as EM15_R;
///Field `EM16` reader - CPU wakeup with event mask on event input
pub use EM0_R as EM16_R;
///Field `EM19` reader - CPU wakeup with event mask on event input
pub use EM0_R as EM19_R;
///Field `EM21` reader - CPU wakeup with event mask on event input
pub use EM0_R as EM21_R;
///Field `EM23` reader - CPU wakeup with event mask on event input
pub use EM0_R as EM23_R;
///Field `EM25` reader - CPU wakeup with event mask on event input
pub use EM0_R as EM25_R;
///Field `EM26` reader - CPU wakeup with event mask on event input
pub use EM0_R as EM26_R;
///Field `EM28` reader - CPU wakeup with event mask on event input
pub use EM0_R as EM28_R;
///Field `EM29` reader - CPU wakeup with event mask on event input
pub use EM0_R as EM29_R;
///Field `EM30` reader - CPU wakeup with event mask on event input
pub use EM0_R as EM30_R;
///Field `EM31` reader - CPU wakeup with event mask on event input
pub use EM0_R as EM31_R;
///Field `EM1` writer - CPU wakeup with event mask on event input
pub use EM0_W as EM1_W;
///Field `EM2` writer - CPU wakeup with event mask on event input
pub use EM0_W as EM2_W;
///Field `EM3` writer - CPU wakeup with event mask on event input
pub use EM0_W as EM3_W;
///Field `EM4` writer - CPU wakeup with event mask on event input
pub use EM0_W as EM4_W;
///Field `EM5` writer - CPU wakeup with event mask on event input
pub use EM0_W as EM5_W;
///Field `EM6` writer - CPU wakeup with event mask on event input
pub use EM0_W as EM6_W;
///Field `EM7` writer - CPU wakeup with event mask on event input
pub use EM0_W as EM7_W;
///Field `EM8` writer - CPU wakeup with event mask on event input
pub use EM0_W as EM8_W;
///Field `EM9` writer - CPU wakeup with event mask on event input
pub use EM0_W as EM9_W;
///Field `EM10` writer - CPU wakeup with event mask on event input
pub use EM0_W as EM10_W;
///Field `EM11` writer - CPU wakeup with event mask on event input
pub use EM0_W as EM11_W;
///Field `EM12` writer - CPU wakeup with event mask on event input
pub use EM0_W as EM12_W;
///Field `EM13` writer - CPU wakeup with event mask on event input
pub use EM0_W as EM13_W;
///Field `EM14` writer - CPU wakeup with event mask on event input
pub use EM0_W as EM14_W;
///Field `EM15` writer - CPU wakeup with event mask on event input
pub use EM0_W as EM15_W;
///Field `EM16` writer - CPU wakeup with event mask on event input
pub use EM0_W as EM16_W;
///Field `EM19` writer - CPU wakeup with event mask on event input
pub use EM0_W as EM19_W;
///Field `EM21` writer - CPU wakeup with event mask on event input
pub use EM0_W as EM21_W;
///Field `EM23` writer - CPU wakeup with event mask on event input
pub use EM0_W as EM23_W;
///Field `EM25` writer - CPU wakeup with event mask on event input
pub use EM0_W as EM25_W;
///Field `EM26` writer - CPU wakeup with event mask on event input
pub use EM0_W as EM26_W;
///Field `EM28` writer - CPU wakeup with event mask on event input
pub use EM0_W as EM28_W;
///Field `EM29` writer - CPU wakeup with event mask on event input
pub use EM0_W as EM29_W;
///Field `EM30` writer - CPU wakeup with event mask on event input
pub use EM0_W as EM30_W;
///Field `EM31` writer - CPU wakeup with event mask on event input
pub use EM0_W as EM31_W;
impl R {
    ///Bit 0 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em0(&self) -> EM0_R {
        EM0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em1(&self) -> EM1_R {
        EM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em2(&self) -> EM2_R {
        EM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em3(&self) -> EM3_R {
        EM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em4(&self) -> EM4_R {
        EM4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em5(&self) -> EM5_R {
        EM5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em6(&self) -> EM6_R {
        EM6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em7(&self) -> EM7_R {
        EM7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em8(&self) -> EM8_R {
        EM8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em9(&self) -> EM9_R {
        EM9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em10(&self) -> EM10_R {
        EM10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em11(&self) -> EM11_R {
        EM11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em12(&self) -> EM12_R {
        EM12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em13(&self) -> EM13_R {
        EM13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em14(&self) -> EM14_R {
        EM14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em15(&self) -> EM15_R {
        EM15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em16(&self) -> EM16_R {
        EM16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 19 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em19(&self) -> EM19_R {
        EM19_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 21 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em21(&self) -> EM21_R {
        EM21_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 23 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em23(&self) -> EM23_R {
        EM23_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 25 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em25(&self) -> EM25_R {
        EM25_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em26(&self) -> EM26_R {
        EM26_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 28 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em28(&self) -> EM28_R {
        EM28_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em29(&self) -> EM29_R {
        EM29_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em30(&self) -> EM30_R {
        EM30_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em31(&self) -> EM31_R {
        EM31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EMR1")
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
            .field("em16", &self.em16())
            .field("em19", &self.em19())
            .field("em21", &self.em21())
            .field("em23", &self.em23())
            .field("em25", &self.em25())
            .field("em26", &self.em26())
            .field("em28", &self.em28())
            .field("em29", &self.em29())
            .field("em30", &self.em30())
            .field("em31", &self.em31())
            .finish()
    }
}
impl W {
    ///Bit 0 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em0(&mut self) -> EM0_W<'_, EMR1rs> {
        EM0_W::new(self, 0)
    }
    ///Bit 1 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em1(&mut self) -> EM1_W<'_, EMR1rs> {
        EM1_W::new(self, 1)
    }
    ///Bit 2 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em2(&mut self) -> EM2_W<'_, EMR1rs> {
        EM2_W::new(self, 2)
    }
    ///Bit 3 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em3(&mut self) -> EM3_W<'_, EMR1rs> {
        EM3_W::new(self, 3)
    }
    ///Bit 4 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em4(&mut self) -> EM4_W<'_, EMR1rs> {
        EM4_W::new(self, 4)
    }
    ///Bit 5 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em5(&mut self) -> EM5_W<'_, EMR1rs> {
        EM5_W::new(self, 5)
    }
    ///Bit 6 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em6(&mut self) -> EM6_W<'_, EMR1rs> {
        EM6_W::new(self, 6)
    }
    ///Bit 7 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em7(&mut self) -> EM7_W<'_, EMR1rs> {
        EM7_W::new(self, 7)
    }
    ///Bit 8 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em8(&mut self) -> EM8_W<'_, EMR1rs> {
        EM8_W::new(self, 8)
    }
    ///Bit 9 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em9(&mut self) -> EM9_W<'_, EMR1rs> {
        EM9_W::new(self, 9)
    }
    ///Bit 10 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em10(&mut self) -> EM10_W<'_, EMR1rs> {
        EM10_W::new(self, 10)
    }
    ///Bit 11 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em11(&mut self) -> EM11_W<'_, EMR1rs> {
        EM11_W::new(self, 11)
    }
    ///Bit 12 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em12(&mut self) -> EM12_W<'_, EMR1rs> {
        EM12_W::new(self, 12)
    }
    ///Bit 13 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em13(&mut self) -> EM13_W<'_, EMR1rs> {
        EM13_W::new(self, 13)
    }
    ///Bit 14 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em14(&mut self) -> EM14_W<'_, EMR1rs> {
        EM14_W::new(self, 14)
    }
    ///Bit 15 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em15(&mut self) -> EM15_W<'_, EMR1rs> {
        EM15_W::new(self, 15)
    }
    ///Bit 16 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em16(&mut self) -> EM16_W<'_, EMR1rs> {
        EM16_W::new(self, 16)
    }
    ///Bit 19 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em19(&mut self) -> EM19_W<'_, EMR1rs> {
        EM19_W::new(self, 19)
    }
    ///Bit 21 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em21(&mut self) -> EM21_W<'_, EMR1rs> {
        EM21_W::new(self, 21)
    }
    ///Bit 23 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em23(&mut self) -> EM23_W<'_, EMR1rs> {
        EM23_W::new(self, 23)
    }
    ///Bit 25 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em25(&mut self) -> EM25_W<'_, EMR1rs> {
        EM25_W::new(self, 25)
    }
    ///Bit 26 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em26(&mut self) -> EM26_W<'_, EMR1rs> {
        EM26_W::new(self, 26)
    }
    ///Bit 28 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em28(&mut self) -> EM28_W<'_, EMR1rs> {
        EM28_W::new(self, 28)
    }
    ///Bit 29 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em29(&mut self) -> EM29_W<'_, EMR1rs> {
        EM29_W::new(self, 29)
    }
    ///Bit 30 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em30(&mut self) -> EM30_W<'_, EMR1rs> {
        EM30_W::new(self, 30)
    }
    ///Bit 31 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em31(&mut self) -> EM31_W<'_, EMR1rs> {
        EM31_W::new(self, 31)
    }
}
/**EXTI CPU wakeup with event mask register

You can [`read`](crate::Reg::read) this register and get [`emr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G061.html#EXTI:EMR1)*/
pub struct EMR1rs;
impl crate::RegisterSpec for EMR1rs {
    type Ux = u32;
}
///`read()` method returns [`emr1::R`](R) reader structure
impl crate::Readable for EMR1rs {}
///`write(|w| ..)` method takes [`emr1::W`](W) writer structure
impl crate::Writable for EMR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EMR1 to value 0
impl crate::Resettable for EMR1rs {}

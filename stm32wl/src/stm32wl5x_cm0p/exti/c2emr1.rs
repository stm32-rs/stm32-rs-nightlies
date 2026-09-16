///Register `C2EMR1` reader
pub type R = crate::R<C2EMR1rs>;
///Register `C2EMR1` writer
pub type W = crate::W<C2EMR1rs>;
/**Wake-up with event generation mask on event input 0

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
///Field `EM0` reader - Wake-up with event generation mask on event input 0
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
///Field `EM0` writer - Wake-up with event generation mask on event input 0
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
///Field `EM1` reader - Wake-up with event generation mask on event input 1
pub use EM0_R as EM1_R;
///Field `EM2` reader - Wake-up with event generation mask on event input 2
pub use EM0_R as EM2_R;
///Field `EM3` reader - Wake-up with event generation mask on event input 3
pub use EM0_R as EM3_R;
///Field `EM4` reader - Wake-up with event generation mask on event input 4
pub use EM0_R as EM4_R;
///Field `EM5` reader - Wake-up with event generation mask on event input 5
pub use EM0_R as EM5_R;
///Field `EM6` reader - Wake-up with event generation mask on event input 6
pub use EM0_R as EM6_R;
///Field `EM7` reader - Wake-up with event generation mask on event input 7
pub use EM0_R as EM7_R;
///Field `EM8` reader - Wake-up with event generation mask on event input 8
pub use EM0_R as EM8_R;
///Field `EM9` reader - Wake-up with event generation mask on event input 19
pub use EM0_R as EM9_R;
///Field `EM10` reader - Wake-up with event generation mask on event input 10
pub use EM0_R as EM10_R;
///Field `EM11` reader - Wake-up with event generation mask on event input 11
pub use EM0_R as EM11_R;
///Field `EM12` reader - Wake-up with event generation mask on event input 12
pub use EM0_R as EM12_R;
///Field `EM13` reader - Wake-up with event generation mask on event input 13
pub use EM0_R as EM13_R;
///Field `EM14` reader - Wake-up with event generation mask on event input 14
pub use EM0_R as EM14_R;
///Field `EM15` reader - Wake-up with event generation mask on event input 15
pub use EM0_R as EM15_R;
///Field `EM17` reader - Wake-up with event generation mask on event input 17
pub use EM0_R as EM17_R;
///Field `EM18` reader - Wake-up with event generation mask on event input 18
pub use EM0_R as EM18_R;
///Field `EM19` reader - Wake-up with event generation mask on event input 19
pub use EM0_R as EM19_R;
///Field `EM20` reader - Wake-up with event generation mask on event input 20
pub use EM0_R as EM20_R;
///Field `EM21` reader - Wake-up with event generation mask on event input 21
pub use EM0_R as EM21_R;
///Field `EM22` reader - Wake-up with event generation mask on event input 22
pub use EM0_R as EM22_R;
///Field `EM1` writer - Wake-up with event generation mask on event input 1
pub use EM0_W as EM1_W;
///Field `EM2` writer - Wake-up with event generation mask on event input 2
pub use EM0_W as EM2_W;
///Field `EM3` writer - Wake-up with event generation mask on event input 3
pub use EM0_W as EM3_W;
///Field `EM4` writer - Wake-up with event generation mask on event input 4
pub use EM0_W as EM4_W;
///Field `EM5` writer - Wake-up with event generation mask on event input 5
pub use EM0_W as EM5_W;
///Field `EM6` writer - Wake-up with event generation mask on event input 6
pub use EM0_W as EM6_W;
///Field `EM7` writer - Wake-up with event generation mask on event input 7
pub use EM0_W as EM7_W;
///Field `EM8` writer - Wake-up with event generation mask on event input 8
pub use EM0_W as EM8_W;
///Field `EM9` writer - Wake-up with event generation mask on event input 19
pub use EM0_W as EM9_W;
///Field `EM10` writer - Wake-up with event generation mask on event input 10
pub use EM0_W as EM10_W;
///Field `EM11` writer - Wake-up with event generation mask on event input 11
pub use EM0_W as EM11_W;
///Field `EM12` writer - Wake-up with event generation mask on event input 12
pub use EM0_W as EM12_W;
///Field `EM13` writer - Wake-up with event generation mask on event input 13
pub use EM0_W as EM13_W;
///Field `EM14` writer - Wake-up with event generation mask on event input 14
pub use EM0_W as EM14_W;
///Field `EM15` writer - Wake-up with event generation mask on event input 15
pub use EM0_W as EM15_W;
///Field `EM17` writer - Wake-up with event generation mask on event input 17
pub use EM0_W as EM17_W;
///Field `EM18` writer - Wake-up with event generation mask on event input 18
pub use EM0_W as EM18_W;
///Field `EM19` writer - Wake-up with event generation mask on event input 19
pub use EM0_W as EM19_W;
///Field `EM20` writer - Wake-up with event generation mask on event input 20
pub use EM0_W as EM20_W;
///Field `EM21` writer - Wake-up with event generation mask on event input 21
pub use EM0_W as EM21_W;
///Field `EM22` writer - Wake-up with event generation mask on event input 22
pub use EM0_W as EM22_W;
impl R {
    ///Bit 0 - Wake-up with event generation mask on event input 0
    #[inline(always)]
    pub fn em0(&self) -> EM0_R {
        EM0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Wake-up with event generation mask on event input 1
    #[inline(always)]
    pub fn em1(&self) -> EM1_R {
        EM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Wake-up with event generation mask on event input 2
    #[inline(always)]
    pub fn em2(&self) -> EM2_R {
        EM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Wake-up with event generation mask on event input 3
    #[inline(always)]
    pub fn em3(&self) -> EM3_R {
        EM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Wake-up with event generation mask on event input 4
    #[inline(always)]
    pub fn em4(&self) -> EM4_R {
        EM4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Wake-up with event generation mask on event input 5
    #[inline(always)]
    pub fn em5(&self) -> EM5_R {
        EM5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Wake-up with event generation mask on event input 6
    #[inline(always)]
    pub fn em6(&self) -> EM6_R {
        EM6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Wake-up with event generation mask on event input 7
    #[inline(always)]
    pub fn em7(&self) -> EM7_R {
        EM7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Wake-up with event generation mask on event input 8
    #[inline(always)]
    pub fn em8(&self) -> EM8_R {
        EM8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Wake-up with event generation mask on event input 19
    #[inline(always)]
    pub fn em9(&self) -> EM9_R {
        EM9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Wake-up with event generation mask on event input 10
    #[inline(always)]
    pub fn em10(&self) -> EM10_R {
        EM10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Wake-up with event generation mask on event input 11
    #[inline(always)]
    pub fn em11(&self) -> EM11_R {
        EM11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Wake-up with event generation mask on event input 12
    #[inline(always)]
    pub fn em12(&self) -> EM12_R {
        EM12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Wake-up with event generation mask on event input 13
    #[inline(always)]
    pub fn em13(&self) -> EM13_R {
        EM13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Wake-up with event generation mask on event input 14
    #[inline(always)]
    pub fn em14(&self) -> EM14_R {
        EM14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Wake-up with event generation mask on event input 15
    #[inline(always)]
    pub fn em15(&self) -> EM15_R {
        EM15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 17 - Wake-up with event generation mask on event input 17
    #[inline(always)]
    pub fn em17(&self) -> EM17_R {
        EM17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Wake-up with event generation mask on event input 18
    #[inline(always)]
    pub fn em18(&self) -> EM18_R {
        EM18_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Wake-up with event generation mask on event input 19
    #[inline(always)]
    pub fn em19(&self) -> EM19_R {
        EM19_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Wake-up with event generation mask on event input 20
    #[inline(always)]
    pub fn em20(&self) -> EM20_R {
        EM20_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Wake-up with event generation mask on event input 21
    #[inline(always)]
    pub fn em21(&self) -> EM21_R {
        EM21_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Wake-up with event generation mask on event input 22
    #[inline(always)]
    pub fn em22(&self) -> EM22_R {
        EM22_R::new(((self.bits >> 22) & 1) != 0)
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
            .field("em20", &self.em20())
            .field("em21", &self.em21())
            .field("em22", &self.em22())
            .finish()
    }
}
impl W {
    ///Bit 0 - Wake-up with event generation mask on event input 0
    #[inline(always)]
    pub fn em0(&mut self) -> EM0_W<'_, C2EMR1rs> {
        EM0_W::new(self, 0)
    }
    ///Bit 1 - Wake-up with event generation mask on event input 1
    #[inline(always)]
    pub fn em1(&mut self) -> EM1_W<'_, C2EMR1rs> {
        EM1_W::new(self, 1)
    }
    ///Bit 2 - Wake-up with event generation mask on event input 2
    #[inline(always)]
    pub fn em2(&mut self) -> EM2_W<'_, C2EMR1rs> {
        EM2_W::new(self, 2)
    }
    ///Bit 3 - Wake-up with event generation mask on event input 3
    #[inline(always)]
    pub fn em3(&mut self) -> EM3_W<'_, C2EMR1rs> {
        EM3_W::new(self, 3)
    }
    ///Bit 4 - Wake-up with event generation mask on event input 4
    #[inline(always)]
    pub fn em4(&mut self) -> EM4_W<'_, C2EMR1rs> {
        EM4_W::new(self, 4)
    }
    ///Bit 5 - Wake-up with event generation mask on event input 5
    #[inline(always)]
    pub fn em5(&mut self) -> EM5_W<'_, C2EMR1rs> {
        EM5_W::new(self, 5)
    }
    ///Bit 6 - Wake-up with event generation mask on event input 6
    #[inline(always)]
    pub fn em6(&mut self) -> EM6_W<'_, C2EMR1rs> {
        EM6_W::new(self, 6)
    }
    ///Bit 7 - Wake-up with event generation mask on event input 7
    #[inline(always)]
    pub fn em7(&mut self) -> EM7_W<'_, C2EMR1rs> {
        EM7_W::new(self, 7)
    }
    ///Bit 8 - Wake-up with event generation mask on event input 8
    #[inline(always)]
    pub fn em8(&mut self) -> EM8_W<'_, C2EMR1rs> {
        EM8_W::new(self, 8)
    }
    ///Bit 9 - Wake-up with event generation mask on event input 19
    #[inline(always)]
    pub fn em9(&mut self) -> EM9_W<'_, C2EMR1rs> {
        EM9_W::new(self, 9)
    }
    ///Bit 10 - Wake-up with event generation mask on event input 10
    #[inline(always)]
    pub fn em10(&mut self) -> EM10_W<'_, C2EMR1rs> {
        EM10_W::new(self, 10)
    }
    ///Bit 11 - Wake-up with event generation mask on event input 11
    #[inline(always)]
    pub fn em11(&mut self) -> EM11_W<'_, C2EMR1rs> {
        EM11_W::new(self, 11)
    }
    ///Bit 12 - Wake-up with event generation mask on event input 12
    #[inline(always)]
    pub fn em12(&mut self) -> EM12_W<'_, C2EMR1rs> {
        EM12_W::new(self, 12)
    }
    ///Bit 13 - Wake-up with event generation mask on event input 13
    #[inline(always)]
    pub fn em13(&mut self) -> EM13_W<'_, C2EMR1rs> {
        EM13_W::new(self, 13)
    }
    ///Bit 14 - Wake-up with event generation mask on event input 14
    #[inline(always)]
    pub fn em14(&mut self) -> EM14_W<'_, C2EMR1rs> {
        EM14_W::new(self, 14)
    }
    ///Bit 15 - Wake-up with event generation mask on event input 15
    #[inline(always)]
    pub fn em15(&mut self) -> EM15_W<'_, C2EMR1rs> {
        EM15_W::new(self, 15)
    }
    ///Bit 17 - Wake-up with event generation mask on event input 17
    #[inline(always)]
    pub fn em17(&mut self) -> EM17_W<'_, C2EMR1rs> {
        EM17_W::new(self, 17)
    }
    ///Bit 18 - Wake-up with event generation mask on event input 18
    #[inline(always)]
    pub fn em18(&mut self) -> EM18_W<'_, C2EMR1rs> {
        EM18_W::new(self, 18)
    }
    ///Bit 19 - Wake-up with event generation mask on event input 19
    #[inline(always)]
    pub fn em19(&mut self) -> EM19_W<'_, C2EMR1rs> {
        EM19_W::new(self, 19)
    }
    ///Bit 20 - Wake-up with event generation mask on event input 20
    #[inline(always)]
    pub fn em20(&mut self) -> EM20_W<'_, C2EMR1rs> {
        EM20_W::new(self, 20)
    }
    ///Bit 21 - Wake-up with event generation mask on event input 21
    #[inline(always)]
    pub fn em21(&mut self) -> EM21_W<'_, C2EMR1rs> {
        EM21_W::new(self, 21)
    }
    ///Bit 22 - Wake-up with event generation mask on event input 22
    #[inline(always)]
    pub fn em22(&mut self) -> EM22_W<'_, C2EMR1rs> {
        EM22_W::new(self, 22)
    }
}
/**EXTI event mask register

You can [`read`](crate::Reg::read) this register and get [`c2emr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2emr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#EXTI:C2EMR1)*/
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

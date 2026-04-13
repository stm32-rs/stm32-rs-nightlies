///Register `PR` reader
pub type R = crate::R<PRrs>;
///Register `PR` writer
pub type W = crate::W<PRrs>;
/**Pending bit 0

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PR0R {
    ///0: No trigger request occurred
    NotPending = 0,
    ///1: Selected trigger request occurred
    Pending = 1,
}
impl From<PR0R> for bool {
    #[inline(always)]
    fn from(variant: PR0R) -> Self {
        variant as u8 != 0
    }
}
///Field `PR0` reader - Pending bit 0
pub type PR0_R = crate::BitReader<PR0R>;
impl PR0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PR0R {
        match self.bits {
            false => PR0R::NotPending,
            true => PR0R::Pending,
        }
    }
    ///No trigger request occurred
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == PR0R::NotPending
    }
    ///Selected trigger request occurred
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == PR0R::Pending
    }
}
/**Pending bit 0

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PR0W {
    ///1: Clears pending bit
    Clear = 1,
}
impl From<PR0W> for bool {
    #[inline(always)]
    fn from(variant: PR0W) -> Self {
        variant as u8 != 0
    }
}
///Field `PR0` writer - Pending bit 0
pub type PR0_W<'a, REG> = crate::BitWriter1C<'a, REG, PR0W>;
impl<'a, REG> PR0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PR0W::Clear)
    }
}
///Field `PR1` reader - Pending bit 1
pub use PR0_R as PR1_R;
///Field `PR2` reader - Pending bit 2
pub use PR0_R as PR2_R;
///Field `PR3` reader - Pending bit 3
pub use PR0_R as PR3_R;
///Field `PR4` reader - Pending bit 4
pub use PR0_R as PR4_R;
///Field `PR5` reader - Pending bit 5
pub use PR0_R as PR5_R;
///Field `PR6` reader - Pending bit 6
pub use PR0_R as PR6_R;
///Field `PR7` reader - Pending bit 7
pub use PR0_R as PR7_R;
///Field `PR8` reader - Pending bit 8
pub use PR0_R as PR8_R;
///Field `PR9` reader - Pending bit 9
pub use PR0_R as PR9_R;
///Field `PR10` reader - Pending bit 10
pub use PR0_R as PR10_R;
///Field `PR11` reader - Pending bit 11
pub use PR0_R as PR11_R;
///Field `PR12` reader - Pending bit 12
pub use PR0_R as PR12_R;
///Field `PR13` reader - Pending bit 13
pub use PR0_R as PR13_R;
///Field `PR14` reader - Pending bit 14
pub use PR0_R as PR14_R;
///Field `PR15` reader - Pending bit 15
pub use PR0_R as PR15_R;
///Field `PR16` reader - Pending bit 16
pub use PR0_R as PR16_R;
///Field `PR17` reader - Pending bit 17
pub use PR0_R as PR17_R;
///Field `PR19` reader - Pending bit 19
pub use PR0_R as PR19_R;
///Field `PR1` writer - Pending bit 1
pub use PR0_W as PR1_W;
///Field `PR2` writer - Pending bit 2
pub use PR0_W as PR2_W;
///Field `PR3` writer - Pending bit 3
pub use PR0_W as PR3_W;
///Field `PR4` writer - Pending bit 4
pub use PR0_W as PR4_W;
///Field `PR5` writer - Pending bit 5
pub use PR0_W as PR5_W;
///Field `PR6` writer - Pending bit 6
pub use PR0_W as PR6_W;
///Field `PR7` writer - Pending bit 7
pub use PR0_W as PR7_W;
///Field `PR8` writer - Pending bit 8
pub use PR0_W as PR8_W;
///Field `PR9` writer - Pending bit 9
pub use PR0_W as PR9_W;
///Field `PR10` writer - Pending bit 10
pub use PR0_W as PR10_W;
///Field `PR11` writer - Pending bit 11
pub use PR0_W as PR11_W;
///Field `PR12` writer - Pending bit 12
pub use PR0_W as PR12_W;
///Field `PR13` writer - Pending bit 13
pub use PR0_W as PR13_W;
///Field `PR14` writer - Pending bit 14
pub use PR0_W as PR14_W;
///Field `PR15` writer - Pending bit 15
pub use PR0_W as PR15_W;
///Field `PR16` writer - Pending bit 16
pub use PR0_W as PR16_W;
///Field `PR17` writer - Pending bit 17
pub use PR0_W as PR17_W;
///Field `PR19` writer - Pending bit 19
pub use PR0_W as PR19_W;
impl R {
    ///Bit 0 - Pending bit 0
    #[inline(always)]
    pub fn pr0(&self) -> PR0_R {
        PR0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Pending bit 1
    #[inline(always)]
    pub fn pr1(&self) -> PR1_R {
        PR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Pending bit 2
    #[inline(always)]
    pub fn pr2(&self) -> PR2_R {
        PR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Pending bit 3
    #[inline(always)]
    pub fn pr3(&self) -> PR3_R {
        PR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Pending bit 4
    #[inline(always)]
    pub fn pr4(&self) -> PR4_R {
        PR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Pending bit 5
    #[inline(always)]
    pub fn pr5(&self) -> PR5_R {
        PR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Pending bit 6
    #[inline(always)]
    pub fn pr6(&self) -> PR6_R {
        PR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Pending bit 7
    #[inline(always)]
    pub fn pr7(&self) -> PR7_R {
        PR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Pending bit 8
    #[inline(always)]
    pub fn pr8(&self) -> PR8_R {
        PR8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Pending bit 9
    #[inline(always)]
    pub fn pr9(&self) -> PR9_R {
        PR9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Pending bit 10
    #[inline(always)]
    pub fn pr10(&self) -> PR10_R {
        PR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Pending bit 11
    #[inline(always)]
    pub fn pr11(&self) -> PR11_R {
        PR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Pending bit 12
    #[inline(always)]
    pub fn pr12(&self) -> PR12_R {
        PR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Pending bit 13
    #[inline(always)]
    pub fn pr13(&self) -> PR13_R {
        PR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Pending bit 14
    #[inline(always)]
    pub fn pr14(&self) -> PR14_R {
        PR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Pending bit 15
    #[inline(always)]
    pub fn pr15(&self) -> PR15_R {
        PR15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Pending bit 16
    #[inline(always)]
    pub fn pr16(&self) -> PR16_R {
        PR16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Pending bit 17
    #[inline(always)]
    pub fn pr17(&self) -> PR17_R {
        PR17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 19 - Pending bit 19
    #[inline(always)]
    pub fn pr19(&self) -> PR19_R {
        PR19_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PR")
            .field("pr0", &self.pr0())
            .field("pr1", &self.pr1())
            .field("pr2", &self.pr2())
            .field("pr3", &self.pr3())
            .field("pr4", &self.pr4())
            .field("pr5", &self.pr5())
            .field("pr6", &self.pr6())
            .field("pr7", &self.pr7())
            .field("pr8", &self.pr8())
            .field("pr9", &self.pr9())
            .field("pr10", &self.pr10())
            .field("pr11", &self.pr11())
            .field("pr12", &self.pr12())
            .field("pr13", &self.pr13())
            .field("pr14", &self.pr14())
            .field("pr15", &self.pr15())
            .field("pr16", &self.pr16())
            .field("pr17", &self.pr17())
            .field("pr19", &self.pr19())
            .finish()
    }
}
impl W {
    ///Bit 0 - Pending bit 0
    #[inline(always)]
    pub fn pr0(&mut self) -> PR0_W<'_, PRrs> {
        PR0_W::new(self, 0)
    }
    ///Bit 1 - Pending bit 1
    #[inline(always)]
    pub fn pr1(&mut self) -> PR1_W<'_, PRrs> {
        PR1_W::new(self, 1)
    }
    ///Bit 2 - Pending bit 2
    #[inline(always)]
    pub fn pr2(&mut self) -> PR2_W<'_, PRrs> {
        PR2_W::new(self, 2)
    }
    ///Bit 3 - Pending bit 3
    #[inline(always)]
    pub fn pr3(&mut self) -> PR3_W<'_, PRrs> {
        PR3_W::new(self, 3)
    }
    ///Bit 4 - Pending bit 4
    #[inline(always)]
    pub fn pr4(&mut self) -> PR4_W<'_, PRrs> {
        PR4_W::new(self, 4)
    }
    ///Bit 5 - Pending bit 5
    #[inline(always)]
    pub fn pr5(&mut self) -> PR5_W<'_, PRrs> {
        PR5_W::new(self, 5)
    }
    ///Bit 6 - Pending bit 6
    #[inline(always)]
    pub fn pr6(&mut self) -> PR6_W<'_, PRrs> {
        PR6_W::new(self, 6)
    }
    ///Bit 7 - Pending bit 7
    #[inline(always)]
    pub fn pr7(&mut self) -> PR7_W<'_, PRrs> {
        PR7_W::new(self, 7)
    }
    ///Bit 8 - Pending bit 8
    #[inline(always)]
    pub fn pr8(&mut self) -> PR8_W<'_, PRrs> {
        PR8_W::new(self, 8)
    }
    ///Bit 9 - Pending bit 9
    #[inline(always)]
    pub fn pr9(&mut self) -> PR9_W<'_, PRrs> {
        PR9_W::new(self, 9)
    }
    ///Bit 10 - Pending bit 10
    #[inline(always)]
    pub fn pr10(&mut self) -> PR10_W<'_, PRrs> {
        PR10_W::new(self, 10)
    }
    ///Bit 11 - Pending bit 11
    #[inline(always)]
    pub fn pr11(&mut self) -> PR11_W<'_, PRrs> {
        PR11_W::new(self, 11)
    }
    ///Bit 12 - Pending bit 12
    #[inline(always)]
    pub fn pr12(&mut self) -> PR12_W<'_, PRrs> {
        PR12_W::new(self, 12)
    }
    ///Bit 13 - Pending bit 13
    #[inline(always)]
    pub fn pr13(&mut self) -> PR13_W<'_, PRrs> {
        PR13_W::new(self, 13)
    }
    ///Bit 14 - Pending bit 14
    #[inline(always)]
    pub fn pr14(&mut self) -> PR14_W<'_, PRrs> {
        PR14_W::new(self, 14)
    }
    ///Bit 15 - Pending bit 15
    #[inline(always)]
    pub fn pr15(&mut self) -> PR15_W<'_, PRrs> {
        PR15_W::new(self, 15)
    }
    ///Bit 16 - Pending bit 16
    #[inline(always)]
    pub fn pr16(&mut self) -> PR16_W<'_, PRrs> {
        PR16_W::new(self, 16)
    }
    ///Bit 17 - Pending bit 17
    #[inline(always)]
    pub fn pr17(&mut self) -> PR17_W<'_, PRrs> {
        PR17_W::new(self, 17)
    }
    ///Bit 19 - Pending bit 19
    #[inline(always)]
    pub fn pr19(&mut self) -> PR19_W<'_, PRrs> {
        PR19_W::new(self, 19)
    }
}
/**Pending register (EXTI_PR)

You can [`read`](crate::Reg::read) this register and get [`pr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F0x1.html#EXTI:PR)*/
pub struct PRrs;
impl crate::RegisterSpec for PRrs {
    type Ux = u32;
}
///`read()` method returns [`pr::R`](R) reader structure
impl crate::Readable for PRrs {}
///`write(|w| ..)` method takes [`pr::W`](W) writer structure
impl crate::Writable for PRrs {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x000b_ffff;
}
///`reset()` method sets PR to value 0
impl crate::Resettable for PRrs {}

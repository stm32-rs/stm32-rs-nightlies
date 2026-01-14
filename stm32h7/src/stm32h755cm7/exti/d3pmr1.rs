///Register `D3PMR1` reader
pub type R = crate::R<D3PMR1rs>;
///Register `D3PMR1` writer
pub type W = crate::W<D3PMR1rs>;
/**Rising trigger event configuration bit of Configurable Event input

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTERRUPT_MASK {
    ///0: Interrupt request line is masked
    Masked = 0,
    ///1: Interrupt request line is unmasked
    Unmasked = 1,
}
impl From<INTERRUPT_MASK> for bool {
    #[inline(always)]
    fn from(variant: INTERRUPT_MASK) -> Self {
        variant as u8 != 0
    }
}
///Field `MR0` reader - Rising trigger event configuration bit of Configurable Event input
pub type MR0_R = crate::BitReader<INTERRUPT_MASK>;
impl MR0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> INTERRUPT_MASK {
        match self.bits {
            false => INTERRUPT_MASK::Masked,
            true => INTERRUPT_MASK::Unmasked,
        }
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == INTERRUPT_MASK::Masked
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == INTERRUPT_MASK::Unmasked
    }
}
///Field `MR0` writer - Rising trigger event configuration bit of Configurable Event input
pub type MR0_W<'a, REG> = crate::BitWriter<'a, REG, INTERRUPT_MASK>;
impl<'a, REG> MR0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(INTERRUPT_MASK::Masked)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(INTERRUPT_MASK::Unmasked)
    }
}
///Field `MR1` reader - Rising trigger event configuration bit of Configurable Event input
pub use MR0_R as MR1_R;
///Field `MR2` reader - Rising trigger event configuration bit of Configurable Event input
pub use MR0_R as MR2_R;
///Field `MR3` reader - Rising trigger event configuration bit of Configurable Event input
pub use MR0_R as MR3_R;
///Field `MR4` reader - Rising trigger event configuration bit of Configurable Event input
pub use MR0_R as MR4_R;
///Field `MR5` reader - Rising trigger event configuration bit of Configurable Event input
pub use MR0_R as MR5_R;
///Field `MR6` reader - Rising trigger event configuration bit of Configurable Event input
pub use MR0_R as MR6_R;
///Field `MR7` reader - Rising trigger event configuration bit of Configurable Event input
pub use MR0_R as MR7_R;
///Field `MR8` reader - Rising trigger event configuration bit of Configurable Event input
pub use MR0_R as MR8_R;
///Field `MR9` reader - Rising trigger event configuration bit of Configurable Event input
pub use MR0_R as MR9_R;
///Field `MR10` reader - Rising trigger event configuration bit of Configurable Event input
pub use MR0_R as MR10_R;
///Field `MR11` reader - Rising trigger event configuration bit of Configurable Event input
pub use MR0_R as MR11_R;
///Field `MR12` reader - Rising trigger event configuration bit of Configurable Event input
pub use MR0_R as MR12_R;
///Field `MR13` reader - Rising trigger event configuration bit of Configurable Event input
pub use MR0_R as MR13_R;
///Field `MR14` reader - Rising trigger event configuration bit of Configurable Event input
pub use MR0_R as MR14_R;
///Field `MR15` reader - Rising trigger event configuration bit of Configurable Event input
pub use MR0_R as MR15_R;
///Field `MR19` reader - Rising trigger event configuration bit of Configurable Event input
pub use MR0_R as MR19_R;
///Field `MR20` reader - Rising trigger event configuration bit of Configurable Event input
pub use MR0_R as MR20_R;
///Field `MR21` reader - Rising trigger event configuration bit of Configurable Event input
pub use MR0_R as MR21_R;
///Field `MR25` reader - Rising trigger event configuration bit of Configurable Event input
pub use MR0_R as MR25_R;
///Field `MR1` writer - Rising trigger event configuration bit of Configurable Event input
pub use MR0_W as MR1_W;
///Field `MR2` writer - Rising trigger event configuration bit of Configurable Event input
pub use MR0_W as MR2_W;
///Field `MR3` writer - Rising trigger event configuration bit of Configurable Event input
pub use MR0_W as MR3_W;
///Field `MR4` writer - Rising trigger event configuration bit of Configurable Event input
pub use MR0_W as MR4_W;
///Field `MR5` writer - Rising trigger event configuration bit of Configurable Event input
pub use MR0_W as MR5_W;
///Field `MR6` writer - Rising trigger event configuration bit of Configurable Event input
pub use MR0_W as MR6_W;
///Field `MR7` writer - Rising trigger event configuration bit of Configurable Event input
pub use MR0_W as MR7_W;
///Field `MR8` writer - Rising trigger event configuration bit of Configurable Event input
pub use MR0_W as MR8_W;
///Field `MR9` writer - Rising trigger event configuration bit of Configurable Event input
pub use MR0_W as MR9_W;
///Field `MR10` writer - Rising trigger event configuration bit of Configurable Event input
pub use MR0_W as MR10_W;
///Field `MR11` writer - Rising trigger event configuration bit of Configurable Event input
pub use MR0_W as MR11_W;
///Field `MR12` writer - Rising trigger event configuration bit of Configurable Event input
pub use MR0_W as MR12_W;
///Field `MR13` writer - Rising trigger event configuration bit of Configurable Event input
pub use MR0_W as MR13_W;
///Field `MR14` writer - Rising trigger event configuration bit of Configurable Event input
pub use MR0_W as MR14_W;
///Field `MR15` writer - Rising trigger event configuration bit of Configurable Event input
pub use MR0_W as MR15_W;
///Field `MR19` writer - Rising trigger event configuration bit of Configurable Event input
pub use MR0_W as MR19_W;
///Field `MR20` writer - Rising trigger event configuration bit of Configurable Event input
pub use MR0_W as MR20_W;
///Field `MR21` writer - Rising trigger event configuration bit of Configurable Event input
pub use MR0_W as MR21_W;
///Field `MR25` writer - Rising trigger event configuration bit of Configurable Event input
pub use MR0_W as MR25_W;
impl R {
    ///Bit 0 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr0(&self) -> MR0_R {
        MR0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr1(&self) -> MR1_R {
        MR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr2(&self) -> MR2_R {
        MR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr3(&self) -> MR3_R {
        MR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr4(&self) -> MR4_R {
        MR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr5(&self) -> MR5_R {
        MR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr6(&self) -> MR6_R {
        MR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr7(&self) -> MR7_R {
        MR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr8(&self) -> MR8_R {
        MR8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr9(&self) -> MR9_R {
        MR9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr10(&self) -> MR10_R {
        MR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr11(&self) -> MR11_R {
        MR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr12(&self) -> MR12_R {
        MR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr13(&self) -> MR13_R {
        MR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr14(&self) -> MR14_R {
        MR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr15(&self) -> MR15_R {
        MR15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 19 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr19(&self) -> MR19_R {
        MR19_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr20(&self) -> MR20_R {
        MR20_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr21(&self) -> MR21_R {
        MR21_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 25 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr25(&self) -> MR25_R {
        MR25_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("D3PMR1")
            .field("mr0", &self.mr0())
            .field("mr1", &self.mr1())
            .field("mr2", &self.mr2())
            .field("mr3", &self.mr3())
            .field("mr4", &self.mr4())
            .field("mr5", &self.mr5())
            .field("mr6", &self.mr6())
            .field("mr7", &self.mr7())
            .field("mr8", &self.mr8())
            .field("mr9", &self.mr9())
            .field("mr10", &self.mr10())
            .field("mr11", &self.mr11())
            .field("mr12", &self.mr12())
            .field("mr13", &self.mr13())
            .field("mr14", &self.mr14())
            .field("mr15", &self.mr15())
            .field("mr19", &self.mr19())
            .field("mr20", &self.mr20())
            .field("mr21", &self.mr21())
            .field("mr25", &self.mr25())
            .finish()
    }
}
impl W {
    ///Bit 0 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr0(&mut self) -> MR0_W<'_, D3PMR1rs> {
        MR0_W::new(self, 0)
    }
    ///Bit 1 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr1(&mut self) -> MR1_W<'_, D3PMR1rs> {
        MR1_W::new(self, 1)
    }
    ///Bit 2 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr2(&mut self) -> MR2_W<'_, D3PMR1rs> {
        MR2_W::new(self, 2)
    }
    ///Bit 3 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr3(&mut self) -> MR3_W<'_, D3PMR1rs> {
        MR3_W::new(self, 3)
    }
    ///Bit 4 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr4(&mut self) -> MR4_W<'_, D3PMR1rs> {
        MR4_W::new(self, 4)
    }
    ///Bit 5 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr5(&mut self) -> MR5_W<'_, D3PMR1rs> {
        MR5_W::new(self, 5)
    }
    ///Bit 6 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr6(&mut self) -> MR6_W<'_, D3PMR1rs> {
        MR6_W::new(self, 6)
    }
    ///Bit 7 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr7(&mut self) -> MR7_W<'_, D3PMR1rs> {
        MR7_W::new(self, 7)
    }
    ///Bit 8 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr8(&mut self) -> MR8_W<'_, D3PMR1rs> {
        MR8_W::new(self, 8)
    }
    ///Bit 9 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr9(&mut self) -> MR9_W<'_, D3PMR1rs> {
        MR9_W::new(self, 9)
    }
    ///Bit 10 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr10(&mut self) -> MR10_W<'_, D3PMR1rs> {
        MR10_W::new(self, 10)
    }
    ///Bit 11 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr11(&mut self) -> MR11_W<'_, D3PMR1rs> {
        MR11_W::new(self, 11)
    }
    ///Bit 12 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr12(&mut self) -> MR12_W<'_, D3PMR1rs> {
        MR12_W::new(self, 12)
    }
    ///Bit 13 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr13(&mut self) -> MR13_W<'_, D3PMR1rs> {
        MR13_W::new(self, 13)
    }
    ///Bit 14 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr14(&mut self) -> MR14_W<'_, D3PMR1rs> {
        MR14_W::new(self, 14)
    }
    ///Bit 15 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr15(&mut self) -> MR15_W<'_, D3PMR1rs> {
        MR15_W::new(self, 15)
    }
    ///Bit 19 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr19(&mut self) -> MR19_W<'_, D3PMR1rs> {
        MR19_W::new(self, 19)
    }
    ///Bit 20 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr20(&mut self) -> MR20_W<'_, D3PMR1rs> {
        MR20_W::new(self, 20)
    }
    ///Bit 21 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr21(&mut self) -> MR21_W<'_, D3PMR1rs> {
        MR21_W::new(self, 21)
    }
    ///Bit 25 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr25(&mut self) -> MR25_W<'_, D3PMR1rs> {
        MR25_W::new(self, 25)
    }
}
/**EXTI D3 pending mask register

You can [`read`](crate::Reg::read) this register and get [`d3pmr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d3pmr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM7.html#EXTI:D3PMR1)*/
pub struct D3PMR1rs;
impl crate::RegisterSpec for D3PMR1rs {
    type Ux = u32;
}
///`read()` method returns [`d3pmr1::R`](R) reader structure
impl crate::Readable for D3PMR1rs {}
///`write(|w| ..)` method takes [`d3pmr1::W`](W) writer structure
impl crate::Writable for D3PMR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets D3PMR1 to value 0
impl crate::Resettable for D3PMR1rs {}

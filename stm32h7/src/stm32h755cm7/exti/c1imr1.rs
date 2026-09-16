///Register `C1IMR1` reader
pub type R = crate::R<C1IMR1rs>;
///Register `C1IMR1` writer
pub type W = crate::W<C1IMR1rs>;
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
///Field `MR16` reader - Rising trigger event configuration bit of Configurable Event input
pub use MR0_R as MR16_R;
///Field `MR17` reader - Rising trigger event configuration bit of Configurable Event input
pub use MR0_R as MR17_R;
///Field `MR18` reader - Rising trigger event configuration bit of Configurable Event input
pub use MR0_R as MR18_R;
///Field `MR19` reader - Rising trigger event configuration bit of Configurable Event input
pub use MR0_R as MR19_R;
///Field `MR20` reader - Rising trigger event configuration bit of Configurable Event input
pub use MR0_R as MR20_R;
///Field `MR21` reader - Rising trigger event configuration bit of Configurable Event input
pub use MR0_R as MR21_R;
///Field `MR22` reader - Rising trigger event configuration bit of Configurable Event input
pub use MR0_R as MR22_R;
///Field `MR23` reader - Rising trigger event configuration bit of Configurable Event input
pub use MR0_R as MR23_R;
///Field `MR24` reader - Rising trigger event configuration bit of Configurable Event input
pub use MR0_R as MR24_R;
///Field `MR25` reader - Rising trigger event configuration bit of Configurable Event input
pub use MR0_R as MR25_R;
///Field `MR26` reader - Rising trigger event configuration bit of Configurable Event input
pub use MR0_R as MR26_R;
///Field `MR27` reader - Rising trigger event configuration bit of Configurable Event input
pub use MR0_R as MR27_R;
///Field `MR28` reader - Rising trigger event configuration bit of Configurable Event input
pub use MR0_R as MR28_R;
///Field `MR29` reader - Rising trigger event configuration bit of Configurable Event input
pub use MR0_R as MR29_R;
///Field `MR30` reader - Rising trigger event configuration bit of Configurable Event input
pub use MR0_R as MR30_R;
///Field `MR31` reader - Rising trigger event configuration bit of Configurable Event input
pub use MR0_R as MR31_R;
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
///Field `MR16` writer - Rising trigger event configuration bit of Configurable Event input
pub use MR0_W as MR16_W;
///Field `MR17` writer - Rising trigger event configuration bit of Configurable Event input
pub use MR0_W as MR17_W;
///Field `MR18` writer - Rising trigger event configuration bit of Configurable Event input
pub use MR0_W as MR18_W;
///Field `MR19` writer - Rising trigger event configuration bit of Configurable Event input
pub use MR0_W as MR19_W;
///Field `MR20` writer - Rising trigger event configuration bit of Configurable Event input
pub use MR0_W as MR20_W;
///Field `MR21` writer - Rising trigger event configuration bit of Configurable Event input
pub use MR0_W as MR21_W;
///Field `MR22` writer - Rising trigger event configuration bit of Configurable Event input
pub use MR0_W as MR22_W;
///Field `MR23` writer - Rising trigger event configuration bit of Configurable Event input
pub use MR0_W as MR23_W;
///Field `MR24` writer - Rising trigger event configuration bit of Configurable Event input
pub use MR0_W as MR24_W;
///Field `MR25` writer - Rising trigger event configuration bit of Configurable Event input
pub use MR0_W as MR25_W;
///Field `MR26` writer - Rising trigger event configuration bit of Configurable Event input
pub use MR0_W as MR26_W;
///Field `MR27` writer - Rising trigger event configuration bit of Configurable Event input
pub use MR0_W as MR27_W;
///Field `MR28` writer - Rising trigger event configuration bit of Configurable Event input
pub use MR0_W as MR28_W;
///Field `MR29` writer - Rising trigger event configuration bit of Configurable Event input
pub use MR0_W as MR29_W;
///Field `MR30` writer - Rising trigger event configuration bit of Configurable Event input
pub use MR0_W as MR30_W;
///Field `MR31` writer - Rising trigger event configuration bit of Configurable Event input
pub use MR0_W as MR31_W;
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
    ///Bit 16 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr16(&self) -> MR16_R {
        MR16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr17(&self) -> MR17_R {
        MR17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr18(&self) -> MR18_R {
        MR18_R::new(((self.bits >> 18) & 1) != 0)
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
    ///Bit 22 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr22(&self) -> MR22_R {
        MR22_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr23(&self) -> MR23_R {
        MR23_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr24(&self) -> MR24_R {
        MR24_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr25(&self) -> MR25_R {
        MR25_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr26(&self) -> MR26_R {
        MR26_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr27(&self) -> MR27_R {
        MR27_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr28(&self) -> MR28_R {
        MR28_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr29(&self) -> MR29_R {
        MR29_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr30(&self) -> MR30_R {
        MR30_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr31(&self) -> MR31_R {
        MR31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C1IMR1")
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
            .field("mr16", &self.mr16())
            .field("mr17", &self.mr17())
            .field("mr18", &self.mr18())
            .field("mr19", &self.mr19())
            .field("mr20", &self.mr20())
            .field("mr21", &self.mr21())
            .field("mr22", &self.mr22())
            .field("mr23", &self.mr23())
            .field("mr24", &self.mr24())
            .field("mr25", &self.mr25())
            .field("mr26", &self.mr26())
            .field("mr27", &self.mr27())
            .field("mr28", &self.mr28())
            .field("mr29", &self.mr29())
            .field("mr30", &self.mr30())
            .field("mr31", &self.mr31())
            .finish()
    }
}
impl W {
    ///Bit 0 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr0(&mut self) -> MR0_W<'_, C1IMR1rs> {
        MR0_W::new(self, 0)
    }
    ///Bit 1 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr1(&mut self) -> MR1_W<'_, C1IMR1rs> {
        MR1_W::new(self, 1)
    }
    ///Bit 2 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr2(&mut self) -> MR2_W<'_, C1IMR1rs> {
        MR2_W::new(self, 2)
    }
    ///Bit 3 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr3(&mut self) -> MR3_W<'_, C1IMR1rs> {
        MR3_W::new(self, 3)
    }
    ///Bit 4 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr4(&mut self) -> MR4_W<'_, C1IMR1rs> {
        MR4_W::new(self, 4)
    }
    ///Bit 5 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr5(&mut self) -> MR5_W<'_, C1IMR1rs> {
        MR5_W::new(self, 5)
    }
    ///Bit 6 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr6(&mut self) -> MR6_W<'_, C1IMR1rs> {
        MR6_W::new(self, 6)
    }
    ///Bit 7 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr7(&mut self) -> MR7_W<'_, C1IMR1rs> {
        MR7_W::new(self, 7)
    }
    ///Bit 8 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr8(&mut self) -> MR8_W<'_, C1IMR1rs> {
        MR8_W::new(self, 8)
    }
    ///Bit 9 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr9(&mut self) -> MR9_W<'_, C1IMR1rs> {
        MR9_W::new(self, 9)
    }
    ///Bit 10 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr10(&mut self) -> MR10_W<'_, C1IMR1rs> {
        MR10_W::new(self, 10)
    }
    ///Bit 11 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr11(&mut self) -> MR11_W<'_, C1IMR1rs> {
        MR11_W::new(self, 11)
    }
    ///Bit 12 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr12(&mut self) -> MR12_W<'_, C1IMR1rs> {
        MR12_W::new(self, 12)
    }
    ///Bit 13 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr13(&mut self) -> MR13_W<'_, C1IMR1rs> {
        MR13_W::new(self, 13)
    }
    ///Bit 14 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr14(&mut self) -> MR14_W<'_, C1IMR1rs> {
        MR14_W::new(self, 14)
    }
    ///Bit 15 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr15(&mut self) -> MR15_W<'_, C1IMR1rs> {
        MR15_W::new(self, 15)
    }
    ///Bit 16 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr16(&mut self) -> MR16_W<'_, C1IMR1rs> {
        MR16_W::new(self, 16)
    }
    ///Bit 17 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr17(&mut self) -> MR17_W<'_, C1IMR1rs> {
        MR17_W::new(self, 17)
    }
    ///Bit 18 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr18(&mut self) -> MR18_W<'_, C1IMR1rs> {
        MR18_W::new(self, 18)
    }
    ///Bit 19 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr19(&mut self) -> MR19_W<'_, C1IMR1rs> {
        MR19_W::new(self, 19)
    }
    ///Bit 20 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr20(&mut self) -> MR20_W<'_, C1IMR1rs> {
        MR20_W::new(self, 20)
    }
    ///Bit 21 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr21(&mut self) -> MR21_W<'_, C1IMR1rs> {
        MR21_W::new(self, 21)
    }
    ///Bit 22 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr22(&mut self) -> MR22_W<'_, C1IMR1rs> {
        MR22_W::new(self, 22)
    }
    ///Bit 23 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr23(&mut self) -> MR23_W<'_, C1IMR1rs> {
        MR23_W::new(self, 23)
    }
    ///Bit 24 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr24(&mut self) -> MR24_W<'_, C1IMR1rs> {
        MR24_W::new(self, 24)
    }
    ///Bit 25 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr25(&mut self) -> MR25_W<'_, C1IMR1rs> {
        MR25_W::new(self, 25)
    }
    ///Bit 26 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr26(&mut self) -> MR26_W<'_, C1IMR1rs> {
        MR26_W::new(self, 26)
    }
    ///Bit 27 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr27(&mut self) -> MR27_W<'_, C1IMR1rs> {
        MR27_W::new(self, 27)
    }
    ///Bit 28 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr28(&mut self) -> MR28_W<'_, C1IMR1rs> {
        MR28_W::new(self, 28)
    }
    ///Bit 29 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr29(&mut self) -> MR29_W<'_, C1IMR1rs> {
        MR29_W::new(self, 29)
    }
    ///Bit 30 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr30(&mut self) -> MR30_W<'_, C1IMR1rs> {
        MR30_W::new(self, 30)
    }
    ///Bit 31 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr31(&mut self) -> MR31_W<'_, C1IMR1rs> {
        MR31_W::new(self, 31)
    }
}
/**EXTI interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`c1imr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1imr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM7.html#EXTI:C1IMR1)*/
pub struct C1IMR1rs;
impl crate::RegisterSpec for C1IMR1rs {
    type Ux = u32;
}
///`read()` method returns [`c1imr1::R`](R) reader structure
impl crate::Readable for C1IMR1rs {}
///`write(|w| ..)` method takes [`c1imr1::W`](W) writer structure
impl crate::Writable for C1IMR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C1IMR1 to value 0xffc0_0000
impl crate::Resettable for C1IMR1rs {
    const RESET_VALUE: u32 = 0xffc0_0000;
}

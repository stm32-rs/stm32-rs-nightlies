///Register `PR1` reader
pub type R = crate::R<PR1rs>;
///Register `PR1` writer
pub type W = crate::W<PR1rs>;
/**pending bit on event input 0

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIF0R {
    ///0: No trigger request occurred
    NotPending = 0,
    ///1: Selected trigger request occurred
    Pending = 1,
}
impl From<PIF0R> for bool {
    #[inline(always)]
    fn from(variant: PIF0R) -> Self {
        variant as u8 != 0
    }
}
///Field `PIF0` reader - pending bit on event input 0
pub type PIF0_R = crate::BitReader<PIF0R>;
impl PIF0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIF0R {
        match self.bits {
            false => PIF0R::NotPending,
            true => PIF0R::Pending,
        }
    }
    ///No trigger request occurred
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == PIF0R::NotPending
    }
    ///Selected trigger request occurred
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == PIF0R::Pending
    }
}
/**pending bit on event input 0

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIF0W {
    ///1: Clears pending bit
    Clear = 1,
}
impl From<PIF0W> for bool {
    #[inline(always)]
    fn from(variant: PIF0W) -> Self {
        variant as u8 != 0
    }
}
///Field `PIF0` writer - pending bit on event input 0
pub type PIF0_W<'a, REG> = crate::BitWriter1C<'a, REG, PIF0W>;
impl<'a, REG> PIF0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PIF0W::Clear)
    }
}
///Field `PIF1` reader - pending bit on event input 1
pub use PIF0_R as PIF1_R;
///Field `PIF2` reader - pending bit on event input 2
pub use PIF0_R as PIF2_R;
///Field `PIF3` reader - pending bit on event input 3
pub use PIF0_R as PIF3_R;
///Field `PIF4` reader - pending bit on event input 4
pub use PIF0_R as PIF4_R;
///Field `PIF5` reader - pending bit on event input 5
pub use PIF0_R as PIF5_R;
///Field `PIF6` reader - pending bit on event input 6
pub use PIF0_R as PIF6_R;
///Field `PIF7` reader - pending bit on event input 7
pub use PIF0_R as PIF7_R;
///Field `PIF8` reader - pending bit on event input 8
pub use PIF0_R as PIF8_R;
///Field `PIF9` reader - pending bit on event input 9
pub use PIF0_R as PIF9_R;
///Field `PIF10` reader - pending bit on event input 10
pub use PIF0_R as PIF10_R;
///Field `PIF11` reader - pending bit on event input 11
pub use PIF0_R as PIF11_R;
///Field `PIF12` reader - pending bit on event input 12
pub use PIF0_R as PIF12_R;
///Field `PIF13` reader - pending bit on event input 13
pub use PIF0_R as PIF13_R;
///Field `PIF14` reader - pending bit on event input 14
pub use PIF0_R as PIF14_R;
///Field `PIF15` reader - pending bit on event input 15
pub use PIF0_R as PIF15_R;
///Field `PIF16` reader - pending bit on event input 16
pub use PIF0_R as PIF16_R;
///Field `PIF21` reader - pending bit on event input 21
pub use PIF0_R as PIF21_R;
///Field `PIF22` reader - pending bit on event input 22 These bits are set when the selected edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it.
pub use PIF0_R as PIF22_R;
///Field `PIF1` writer - pending bit on event input 1
pub use PIF0_W as PIF1_W;
///Field `PIF2` writer - pending bit on event input 2
pub use PIF0_W as PIF2_W;
///Field `PIF3` writer - pending bit on event input 3
pub use PIF0_W as PIF3_W;
///Field `PIF4` writer - pending bit on event input 4
pub use PIF0_W as PIF4_W;
///Field `PIF5` writer - pending bit on event input 5
pub use PIF0_W as PIF5_W;
///Field `PIF6` writer - pending bit on event input 6
pub use PIF0_W as PIF6_W;
///Field `PIF7` writer - pending bit on event input 7
pub use PIF0_W as PIF7_W;
///Field `PIF8` writer - pending bit on event input 8
pub use PIF0_W as PIF8_W;
///Field `PIF9` writer - pending bit on event input 9
pub use PIF0_W as PIF9_W;
///Field `PIF10` writer - pending bit on event input 10
pub use PIF0_W as PIF10_W;
///Field `PIF11` writer - pending bit on event input 11
pub use PIF0_W as PIF11_W;
///Field `PIF12` writer - pending bit on event input 12
pub use PIF0_W as PIF12_W;
///Field `PIF13` writer - pending bit on event input 13
pub use PIF0_W as PIF13_W;
///Field `PIF14` writer - pending bit on event input 14
pub use PIF0_W as PIF14_W;
///Field `PIF15` writer - pending bit on event input 15
pub use PIF0_W as PIF15_W;
///Field `PIF16` writer - pending bit on event input 16
pub use PIF0_W as PIF16_W;
///Field `PIF21` writer - pending bit on event input 21
pub use PIF0_W as PIF21_W;
///Field `PIF22` writer - pending bit on event input 22 These bits are set when the selected edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it.
pub use PIF0_W as PIF22_W;
impl R {
    ///Bit 0 - pending bit on event input 0
    #[inline(always)]
    pub fn pif0(&self) -> PIF0_R {
        PIF0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - pending bit on event input 1
    #[inline(always)]
    pub fn pif1(&self) -> PIF1_R {
        PIF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - pending bit on event input 2
    #[inline(always)]
    pub fn pif2(&self) -> PIF2_R {
        PIF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - pending bit on event input 3
    #[inline(always)]
    pub fn pif3(&self) -> PIF3_R {
        PIF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - pending bit on event input 4
    #[inline(always)]
    pub fn pif4(&self) -> PIF4_R {
        PIF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - pending bit on event input 5
    #[inline(always)]
    pub fn pif5(&self) -> PIF5_R {
        PIF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - pending bit on event input 6
    #[inline(always)]
    pub fn pif6(&self) -> PIF6_R {
        PIF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - pending bit on event input 7
    #[inline(always)]
    pub fn pif7(&self) -> PIF7_R {
        PIF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - pending bit on event input 8
    #[inline(always)]
    pub fn pif8(&self) -> PIF8_R {
        PIF8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - pending bit on event input 9
    #[inline(always)]
    pub fn pif9(&self) -> PIF9_R {
        PIF9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - pending bit on event input 10
    #[inline(always)]
    pub fn pif10(&self) -> PIF10_R {
        PIF10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - pending bit on event input 11
    #[inline(always)]
    pub fn pif11(&self) -> PIF11_R {
        PIF11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - pending bit on event input 12
    #[inline(always)]
    pub fn pif12(&self) -> PIF12_R {
        PIF12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - pending bit on event input 13
    #[inline(always)]
    pub fn pif13(&self) -> PIF13_R {
        PIF13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - pending bit on event input 14
    #[inline(always)]
    pub fn pif14(&self) -> PIF14_R {
        PIF14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - pending bit on event input 15
    #[inline(always)]
    pub fn pif15(&self) -> PIF15_R {
        PIF15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - pending bit on event input 16
    #[inline(always)]
    pub fn pif16(&self) -> PIF16_R {
        PIF16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 21 - pending bit on event input 21
    #[inline(always)]
    pub fn pif21(&self) -> PIF21_R {
        PIF21_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - pending bit on event input 22 These bits are set when the selected edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it.
    #[inline(always)]
    pub fn pif22(&self) -> PIF22_R {
        PIF22_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PR1")
            .field("pif0", &self.pif0())
            .field("pif1", &self.pif1())
            .field("pif2", &self.pif2())
            .field("pif3", &self.pif3())
            .field("pif4", &self.pif4())
            .field("pif5", &self.pif5())
            .field("pif6", &self.pif6())
            .field("pif7", &self.pif7())
            .field("pif8", &self.pif8())
            .field("pif9", &self.pif9())
            .field("pif10", &self.pif10())
            .field("pif11", &self.pif11())
            .field("pif12", &self.pif12())
            .field("pif13", &self.pif13())
            .field("pif14", &self.pif14())
            .field("pif15", &self.pif15())
            .field("pif16", &self.pif16())
            .field("pif21", &self.pif21())
            .field("pif22", &self.pif22())
            .finish()
    }
}
impl W {
    ///Bit 0 - pending bit on event input 0
    #[inline(always)]
    pub fn pif0(&mut self) -> PIF0_W<'_, PR1rs> {
        PIF0_W::new(self, 0)
    }
    ///Bit 1 - pending bit on event input 1
    #[inline(always)]
    pub fn pif1(&mut self) -> PIF1_W<'_, PR1rs> {
        PIF1_W::new(self, 1)
    }
    ///Bit 2 - pending bit on event input 2
    #[inline(always)]
    pub fn pif2(&mut self) -> PIF2_W<'_, PR1rs> {
        PIF2_W::new(self, 2)
    }
    ///Bit 3 - pending bit on event input 3
    #[inline(always)]
    pub fn pif3(&mut self) -> PIF3_W<'_, PR1rs> {
        PIF3_W::new(self, 3)
    }
    ///Bit 4 - pending bit on event input 4
    #[inline(always)]
    pub fn pif4(&mut self) -> PIF4_W<'_, PR1rs> {
        PIF4_W::new(self, 4)
    }
    ///Bit 5 - pending bit on event input 5
    #[inline(always)]
    pub fn pif5(&mut self) -> PIF5_W<'_, PR1rs> {
        PIF5_W::new(self, 5)
    }
    ///Bit 6 - pending bit on event input 6
    #[inline(always)]
    pub fn pif6(&mut self) -> PIF6_W<'_, PR1rs> {
        PIF6_W::new(self, 6)
    }
    ///Bit 7 - pending bit on event input 7
    #[inline(always)]
    pub fn pif7(&mut self) -> PIF7_W<'_, PR1rs> {
        PIF7_W::new(self, 7)
    }
    ///Bit 8 - pending bit on event input 8
    #[inline(always)]
    pub fn pif8(&mut self) -> PIF8_W<'_, PR1rs> {
        PIF8_W::new(self, 8)
    }
    ///Bit 9 - pending bit on event input 9
    #[inline(always)]
    pub fn pif9(&mut self) -> PIF9_W<'_, PR1rs> {
        PIF9_W::new(self, 9)
    }
    ///Bit 10 - pending bit on event input 10
    #[inline(always)]
    pub fn pif10(&mut self) -> PIF10_W<'_, PR1rs> {
        PIF10_W::new(self, 10)
    }
    ///Bit 11 - pending bit on event input 11
    #[inline(always)]
    pub fn pif11(&mut self) -> PIF11_W<'_, PR1rs> {
        PIF11_W::new(self, 11)
    }
    ///Bit 12 - pending bit on event input 12
    #[inline(always)]
    pub fn pif12(&mut self) -> PIF12_W<'_, PR1rs> {
        PIF12_W::new(self, 12)
    }
    ///Bit 13 - pending bit on event input 13
    #[inline(always)]
    pub fn pif13(&mut self) -> PIF13_W<'_, PR1rs> {
        PIF13_W::new(self, 13)
    }
    ///Bit 14 - pending bit on event input 14
    #[inline(always)]
    pub fn pif14(&mut self) -> PIF14_W<'_, PR1rs> {
        PIF14_W::new(self, 14)
    }
    ///Bit 15 - pending bit on event input 15
    #[inline(always)]
    pub fn pif15(&mut self) -> PIF15_W<'_, PR1rs> {
        PIF15_W::new(self, 15)
    }
    ///Bit 16 - pending bit on event input 16
    #[inline(always)]
    pub fn pif16(&mut self) -> PIF16_W<'_, PR1rs> {
        PIF16_W::new(self, 16)
    }
    ///Bit 21 - pending bit on event input 21
    #[inline(always)]
    pub fn pif21(&mut self) -> PIF21_W<'_, PR1rs> {
        PIF21_W::new(self, 21)
    }
    ///Bit 22 - pending bit on event input 22 These bits are set when the selected edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it.
    #[inline(always)]
    pub fn pif22(&mut self) -> PIF22_W<'_, PR1rs> {
        PIF22_W::new(self, 22)
    }
}
/**EXTI pending register

You can [`read`](crate::Reg::read) this register and get [`pr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#EXTI:PR1)*/
pub struct PR1rs;
impl crate::RegisterSpec for PR1rs {
    type Ux = u32;
}
///`read()` method returns [`pr1::R`](R) reader structure
impl crate::Readable for PR1rs {}
///`write(|w| ..)` method takes [`pr1::W`](W) writer structure
impl crate::Writable for PR1rs {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0061_ffff;
}
///`reset()` method sets PR1 to value 0
impl crate::Resettable for PR1rs {}

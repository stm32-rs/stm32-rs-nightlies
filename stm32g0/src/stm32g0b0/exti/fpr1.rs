///Register `FPR1` reader
pub type R = crate::R<FPR1rs>;
///Register `FPR1` writer
pub type W = crate::W<FPR1rs>;
/**Falling edge event pending for configurable line

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPIF0R {
    ///0: No trigger request occurred
    NotPending = 0,
    ///1: Selected trigger request occurred
    Pending = 1,
}
impl From<FPIF0R> for bool {
    #[inline(always)]
    fn from(variant: FPIF0R) -> Self {
        variant as u8 != 0
    }
}
///Field `FPIF0` reader - Falling edge event pending for configurable line
pub type FPIF0_R = crate::BitReader<FPIF0R>;
impl FPIF0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FPIF0R {
        match self.bits {
            false => FPIF0R::NotPending,
            true => FPIF0R::Pending,
        }
    }
    ///No trigger request occurred
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == FPIF0R::NotPending
    }
    ///Selected trigger request occurred
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == FPIF0R::Pending
    }
}
/**Falling edge event pending for configurable line

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPIF0W {
    ///1: Clears pending bit
    Clear = 1,
}
impl From<FPIF0W> for bool {
    #[inline(always)]
    fn from(variant: FPIF0W) -> Self {
        variant as u8 != 0
    }
}
///Field `FPIF0` writer - Falling edge event pending for configurable line
pub type FPIF0_W<'a, REG> = crate::BitWriter1C<'a, REG, FPIF0W>;
impl<'a, REG> FPIF0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(FPIF0W::Clear)
    }
}
///Field `FPIF1` reader - Falling edge event pending for configurable line
pub use FPIF0_R as FPIF1_R;
///Field `FPIF2` reader - Falling edge event pending for configurable line
pub use FPIF0_R as FPIF2_R;
///Field `FPIF3` reader - Falling edge event pending for configurable line
pub use FPIF0_R as FPIF3_R;
///Field `FPIF4` reader - Falling edge event pending for configurable line
pub use FPIF0_R as FPIF4_R;
///Field `FPIF5` reader - Falling edge event pending for configurable line
pub use FPIF0_R as FPIF5_R;
///Field `FPIF6` reader - Falling edge event pending for configurable line
pub use FPIF0_R as FPIF6_R;
///Field `FPIF7` reader - Falling edge event pending for configurable line
pub use FPIF0_R as FPIF7_R;
///Field `FPIF8` reader - Falling edge event pending for configurable line
pub use FPIF0_R as FPIF8_R;
///Field `FPIF9` reader - Falling edge event pending for configurable line
pub use FPIF0_R as FPIF9_R;
///Field `FPIF10` reader - Falling edge event pending for configurable line
pub use FPIF0_R as FPIF10_R;
///Field `FPIF11` reader - Falling edge event pending for configurable line
pub use FPIF0_R as FPIF11_R;
///Field `FPIF12` reader - Falling edge event pending for configurable line
pub use FPIF0_R as FPIF12_R;
///Field `FPIF13` reader - Falling edge event pending for configurable line
pub use FPIF0_R as FPIF13_R;
///Field `FPIF14` reader - Falling edge event pending for configurable line
pub use FPIF0_R as FPIF14_R;
///Field `FPIF15` reader - Falling edge event pending for configurable line
pub use FPIF0_R as FPIF15_R;
///Field `FPIF1` writer - Falling edge event pending for configurable line
pub use FPIF0_W as FPIF1_W;
///Field `FPIF2` writer - Falling edge event pending for configurable line
pub use FPIF0_W as FPIF2_W;
///Field `FPIF3` writer - Falling edge event pending for configurable line
pub use FPIF0_W as FPIF3_W;
///Field `FPIF4` writer - Falling edge event pending for configurable line
pub use FPIF0_W as FPIF4_W;
///Field `FPIF5` writer - Falling edge event pending for configurable line
pub use FPIF0_W as FPIF5_W;
///Field `FPIF6` writer - Falling edge event pending for configurable line
pub use FPIF0_W as FPIF6_W;
///Field `FPIF7` writer - Falling edge event pending for configurable line
pub use FPIF0_W as FPIF7_W;
///Field `FPIF8` writer - Falling edge event pending for configurable line
pub use FPIF0_W as FPIF8_W;
///Field `FPIF9` writer - Falling edge event pending for configurable line
pub use FPIF0_W as FPIF9_W;
///Field `FPIF10` writer - Falling edge event pending for configurable line
pub use FPIF0_W as FPIF10_W;
///Field `FPIF11` writer - Falling edge event pending for configurable line
pub use FPIF0_W as FPIF11_W;
///Field `FPIF12` writer - Falling edge event pending for configurable line
pub use FPIF0_W as FPIF12_W;
///Field `FPIF13` writer - Falling edge event pending for configurable line
pub use FPIF0_W as FPIF13_W;
///Field `FPIF14` writer - Falling edge event pending for configurable line
pub use FPIF0_W as FPIF14_W;
///Field `FPIF15` writer - Falling edge event pending for configurable line
pub use FPIF0_W as FPIF15_W;
impl R {
    ///Bit 0 - Falling edge event pending for configurable line
    #[inline(always)]
    pub fn fpif0(&self) -> FPIF0_R {
        FPIF0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Falling edge event pending for configurable line
    #[inline(always)]
    pub fn fpif1(&self) -> FPIF1_R {
        FPIF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Falling edge event pending for configurable line
    #[inline(always)]
    pub fn fpif2(&self) -> FPIF2_R {
        FPIF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Falling edge event pending for configurable line
    #[inline(always)]
    pub fn fpif3(&self) -> FPIF3_R {
        FPIF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Falling edge event pending for configurable line
    #[inline(always)]
    pub fn fpif4(&self) -> FPIF4_R {
        FPIF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Falling edge event pending for configurable line
    #[inline(always)]
    pub fn fpif5(&self) -> FPIF5_R {
        FPIF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Falling edge event pending for configurable line
    #[inline(always)]
    pub fn fpif6(&self) -> FPIF6_R {
        FPIF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Falling edge event pending for configurable line
    #[inline(always)]
    pub fn fpif7(&self) -> FPIF7_R {
        FPIF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Falling edge event pending for configurable line
    #[inline(always)]
    pub fn fpif8(&self) -> FPIF8_R {
        FPIF8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Falling edge event pending for configurable line
    #[inline(always)]
    pub fn fpif9(&self) -> FPIF9_R {
        FPIF9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Falling edge event pending for configurable line
    #[inline(always)]
    pub fn fpif10(&self) -> FPIF10_R {
        FPIF10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Falling edge event pending for configurable line
    #[inline(always)]
    pub fn fpif11(&self) -> FPIF11_R {
        FPIF11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Falling edge event pending for configurable line
    #[inline(always)]
    pub fn fpif12(&self) -> FPIF12_R {
        FPIF12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Falling edge event pending for configurable line
    #[inline(always)]
    pub fn fpif13(&self) -> FPIF13_R {
        FPIF13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Falling edge event pending for configurable line
    #[inline(always)]
    pub fn fpif14(&self) -> FPIF14_R {
        FPIF14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Falling edge event pending for configurable line
    #[inline(always)]
    pub fn fpif15(&self) -> FPIF15_R {
        FPIF15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FPR1")
            .field("fpif0", &self.fpif0())
            .field("fpif1", &self.fpif1())
            .field("fpif2", &self.fpif2())
            .field("fpif3", &self.fpif3())
            .field("fpif4", &self.fpif4())
            .field("fpif5", &self.fpif5())
            .field("fpif6", &self.fpif6())
            .field("fpif7", &self.fpif7())
            .field("fpif8", &self.fpif8())
            .field("fpif9", &self.fpif9())
            .field("fpif10", &self.fpif10())
            .field("fpif11", &self.fpif11())
            .field("fpif12", &self.fpif12())
            .field("fpif13", &self.fpif13())
            .field("fpif14", &self.fpif14())
            .field("fpif15", &self.fpif15())
            .finish()
    }
}
impl W {
    ///Bit 0 - Falling edge event pending for configurable line
    #[inline(always)]
    pub fn fpif0(&mut self) -> FPIF0_W<'_, FPR1rs> {
        FPIF0_W::new(self, 0)
    }
    ///Bit 1 - Falling edge event pending for configurable line
    #[inline(always)]
    pub fn fpif1(&mut self) -> FPIF1_W<'_, FPR1rs> {
        FPIF1_W::new(self, 1)
    }
    ///Bit 2 - Falling edge event pending for configurable line
    #[inline(always)]
    pub fn fpif2(&mut self) -> FPIF2_W<'_, FPR1rs> {
        FPIF2_W::new(self, 2)
    }
    ///Bit 3 - Falling edge event pending for configurable line
    #[inline(always)]
    pub fn fpif3(&mut self) -> FPIF3_W<'_, FPR1rs> {
        FPIF3_W::new(self, 3)
    }
    ///Bit 4 - Falling edge event pending for configurable line
    #[inline(always)]
    pub fn fpif4(&mut self) -> FPIF4_W<'_, FPR1rs> {
        FPIF4_W::new(self, 4)
    }
    ///Bit 5 - Falling edge event pending for configurable line
    #[inline(always)]
    pub fn fpif5(&mut self) -> FPIF5_W<'_, FPR1rs> {
        FPIF5_W::new(self, 5)
    }
    ///Bit 6 - Falling edge event pending for configurable line
    #[inline(always)]
    pub fn fpif6(&mut self) -> FPIF6_W<'_, FPR1rs> {
        FPIF6_W::new(self, 6)
    }
    ///Bit 7 - Falling edge event pending for configurable line
    #[inline(always)]
    pub fn fpif7(&mut self) -> FPIF7_W<'_, FPR1rs> {
        FPIF7_W::new(self, 7)
    }
    ///Bit 8 - Falling edge event pending for configurable line
    #[inline(always)]
    pub fn fpif8(&mut self) -> FPIF8_W<'_, FPR1rs> {
        FPIF8_W::new(self, 8)
    }
    ///Bit 9 - Falling edge event pending for configurable line
    #[inline(always)]
    pub fn fpif9(&mut self) -> FPIF9_W<'_, FPR1rs> {
        FPIF9_W::new(self, 9)
    }
    ///Bit 10 - Falling edge event pending for configurable line
    #[inline(always)]
    pub fn fpif10(&mut self) -> FPIF10_W<'_, FPR1rs> {
        FPIF10_W::new(self, 10)
    }
    ///Bit 11 - Falling edge event pending for configurable line
    #[inline(always)]
    pub fn fpif11(&mut self) -> FPIF11_W<'_, FPR1rs> {
        FPIF11_W::new(self, 11)
    }
    ///Bit 12 - Falling edge event pending for configurable line
    #[inline(always)]
    pub fn fpif12(&mut self) -> FPIF12_W<'_, FPR1rs> {
        FPIF12_W::new(self, 12)
    }
    ///Bit 13 - Falling edge event pending for configurable line
    #[inline(always)]
    pub fn fpif13(&mut self) -> FPIF13_W<'_, FPR1rs> {
        FPIF13_W::new(self, 13)
    }
    ///Bit 14 - Falling edge event pending for configurable line
    #[inline(always)]
    pub fn fpif14(&mut self) -> FPIF14_W<'_, FPR1rs> {
        FPIF14_W::new(self, 14)
    }
    ///Bit 15 - Falling edge event pending for configurable line
    #[inline(always)]
    pub fn fpif15(&mut self) -> FPIF15_W<'_, FPR1rs> {
        FPIF15_W::new(self, 15)
    }
}
/**EXTI falling edge pending register

You can [`read`](crate::Reg::read) this register and get [`fpr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0B0.html#EXTI:FPR1)*/
pub struct FPR1rs;
impl crate::RegisterSpec for FPR1rs {
    type Ux = u32;
}
///`read()` method returns [`fpr1::R`](R) reader structure
impl crate::Readable for FPR1rs {}
///`write(|w| ..)` method takes [`fpr1::W`](W) writer structure
impl crate::Writable for FPR1rs {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xffff;
}
///`reset()` method sets FPR1 to value 0
impl crate::Resettable for FPR1rs {}

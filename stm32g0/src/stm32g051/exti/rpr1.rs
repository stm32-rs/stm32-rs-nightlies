///Register `RPR1` reader
pub type R = crate::R<RPR1rs>;
///Register `RPR1` writer
pub type W = crate::W<RPR1rs>;
/**configurable event inputs x rising edge Pending bit.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPIF0R {
    ///0: No trigger request occurred
    NotPending = 0,
    ///1: Selected trigger request occurred
    Pending = 1,
}
impl From<RPIF0R> for bool {
    #[inline(always)]
    fn from(variant: RPIF0R) -> Self {
        variant as u8 != 0
    }
}
///Field `RPIF0` reader - configurable event inputs x rising edge Pending bit.
pub type RPIF0_R = crate::BitReader<RPIF0R>;
impl RPIF0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RPIF0R {
        match self.bits {
            false => RPIF0R::NotPending,
            true => RPIF0R::Pending,
        }
    }
    ///No trigger request occurred
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == RPIF0R::NotPending
    }
    ///Selected trigger request occurred
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RPIF0R::Pending
    }
}
/**configurable event inputs x rising edge Pending bit.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPIF0W {
    ///1: Clears pending bit
    Clear = 1,
}
impl From<RPIF0W> for bool {
    #[inline(always)]
    fn from(variant: RPIF0W) -> Self {
        variant as u8 != 0
    }
}
///Field `RPIF0` writer - configurable event inputs x rising edge Pending bit.
pub type RPIF0_W<'a, REG> = crate::BitWriter1C<'a, REG, RPIF0W>;
impl<'a, REG> RPIF0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RPIF0W::Clear)
    }
}
///Field `RPIF1` reader - configurable event inputs x rising edge Pending bit.
pub use RPIF0_R as RPIF1_R;
///Field `RPIF2` reader - configurable event inputs x rising edge Pending bit.
pub use RPIF0_R as RPIF2_R;
///Field `RPIF3` reader - configurable event inputs x rising edge Pending bit.
pub use RPIF0_R as RPIF3_R;
///Field `RPIF4` reader - configurable event inputs x rising edge Pending bit.
pub use RPIF0_R as RPIF4_R;
///Field `RPIF5` reader - configurable event inputs x rising edge Pending bit
pub use RPIF0_R as RPIF5_R;
///Field `RPIF6` reader - configurable event inputs x rising edge Pending bit.
pub use RPIF0_R as RPIF6_R;
///Field `RPIF7` reader - configurable event inputs x rising edge Pending bit.
pub use RPIF0_R as RPIF7_R;
///Field `RPIF8` reader - configurable event inputs x rising edge Pending bit.
pub use RPIF0_R as RPIF8_R;
///Field `RPIF9` reader - configurable event inputs x rising edge Pending bit.
pub use RPIF0_R as RPIF9_R;
///Field `RPIF10` reader - configurable event inputs x rising edge Pending bit.
pub use RPIF0_R as RPIF10_R;
///Field `RPIF11` reader - configurable event inputs x rising edge Pending bit.
pub use RPIF0_R as RPIF11_R;
///Field `RPIF12` reader - configurable event inputs x rising edge Pending bit.
pub use RPIF0_R as RPIF12_R;
///Field `RPIF13` reader - configurable event inputs x rising edge Pending bit.
pub use RPIF0_R as RPIF13_R;
///Field `RPIF14` reader - configurable event inputs x rising edge Pending bit.
pub use RPIF0_R as RPIF14_R;
///Field `RPIF15` reader - configurable event inputs x rising edge Pending bit.
pub use RPIF0_R as RPIF15_R;
///Field `RPIF16` reader - configurable event inputs x rising edge Pending bit.
pub use RPIF0_R as RPIF16_R;
///Field `RPIF1` writer - configurable event inputs x rising edge Pending bit.
pub use RPIF0_W as RPIF1_W;
///Field `RPIF2` writer - configurable event inputs x rising edge Pending bit.
pub use RPIF0_W as RPIF2_W;
///Field `RPIF3` writer - configurable event inputs x rising edge Pending bit.
pub use RPIF0_W as RPIF3_W;
///Field `RPIF4` writer - configurable event inputs x rising edge Pending bit.
pub use RPIF0_W as RPIF4_W;
///Field `RPIF5` writer - configurable event inputs x rising edge Pending bit
pub use RPIF0_W as RPIF5_W;
///Field `RPIF6` writer - configurable event inputs x rising edge Pending bit.
pub use RPIF0_W as RPIF6_W;
///Field `RPIF7` writer - configurable event inputs x rising edge Pending bit.
pub use RPIF0_W as RPIF7_W;
///Field `RPIF8` writer - configurable event inputs x rising edge Pending bit.
pub use RPIF0_W as RPIF8_W;
///Field `RPIF9` writer - configurable event inputs x rising edge Pending bit.
pub use RPIF0_W as RPIF9_W;
///Field `RPIF10` writer - configurable event inputs x rising edge Pending bit.
pub use RPIF0_W as RPIF10_W;
///Field `RPIF11` writer - configurable event inputs x rising edge Pending bit.
pub use RPIF0_W as RPIF11_W;
///Field `RPIF12` writer - configurable event inputs x rising edge Pending bit.
pub use RPIF0_W as RPIF12_W;
///Field `RPIF13` writer - configurable event inputs x rising edge Pending bit.
pub use RPIF0_W as RPIF13_W;
///Field `RPIF14` writer - configurable event inputs x rising edge Pending bit.
pub use RPIF0_W as RPIF14_W;
///Field `RPIF15` writer - configurable event inputs x rising edge Pending bit.
pub use RPIF0_W as RPIF15_W;
///Field `RPIF16` writer - configurable event inputs x rising edge Pending bit.
pub use RPIF0_W as RPIF16_W;
impl R {
    ///Bit 0 - configurable event inputs x rising edge Pending bit.
    #[inline(always)]
    pub fn rpif0(&self) -> RPIF0_R {
        RPIF0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - configurable event inputs x rising edge Pending bit.
    #[inline(always)]
    pub fn rpif1(&self) -> RPIF1_R {
        RPIF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - configurable event inputs x rising edge Pending bit.
    #[inline(always)]
    pub fn rpif2(&self) -> RPIF2_R {
        RPIF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - configurable event inputs x rising edge Pending bit.
    #[inline(always)]
    pub fn rpif3(&self) -> RPIF3_R {
        RPIF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - configurable event inputs x rising edge Pending bit.
    #[inline(always)]
    pub fn rpif4(&self) -> RPIF4_R {
        RPIF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - configurable event inputs x rising edge Pending bit
    #[inline(always)]
    pub fn rpif5(&self) -> RPIF5_R {
        RPIF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - configurable event inputs x rising edge Pending bit.
    #[inline(always)]
    pub fn rpif6(&self) -> RPIF6_R {
        RPIF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - configurable event inputs x rising edge Pending bit.
    #[inline(always)]
    pub fn rpif7(&self) -> RPIF7_R {
        RPIF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - configurable event inputs x rising edge Pending bit.
    #[inline(always)]
    pub fn rpif8(&self) -> RPIF8_R {
        RPIF8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - configurable event inputs x rising edge Pending bit.
    #[inline(always)]
    pub fn rpif9(&self) -> RPIF9_R {
        RPIF9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - configurable event inputs x rising edge Pending bit.
    #[inline(always)]
    pub fn rpif10(&self) -> RPIF10_R {
        RPIF10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - configurable event inputs x rising edge Pending bit.
    #[inline(always)]
    pub fn rpif11(&self) -> RPIF11_R {
        RPIF11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - configurable event inputs x rising edge Pending bit.
    #[inline(always)]
    pub fn rpif12(&self) -> RPIF12_R {
        RPIF12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - configurable event inputs x rising edge Pending bit.
    #[inline(always)]
    pub fn rpif13(&self) -> RPIF13_R {
        RPIF13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - configurable event inputs x rising edge Pending bit.
    #[inline(always)]
    pub fn rpif14(&self) -> RPIF14_R {
        RPIF14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - configurable event inputs x rising edge Pending bit.
    #[inline(always)]
    pub fn rpif15(&self) -> RPIF15_R {
        RPIF15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - configurable event inputs x rising edge Pending bit.
    #[inline(always)]
    pub fn rpif16(&self) -> RPIF16_R {
        RPIF16_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RPR1")
            .field("rpif0", &self.rpif0())
            .field("rpif1", &self.rpif1())
            .field("rpif2", &self.rpif2())
            .field("rpif3", &self.rpif3())
            .field("rpif4", &self.rpif4())
            .field("rpif5", &self.rpif5())
            .field("rpif6", &self.rpif6())
            .field("rpif7", &self.rpif7())
            .field("rpif8", &self.rpif8())
            .field("rpif9", &self.rpif9())
            .field("rpif10", &self.rpif10())
            .field("rpif11", &self.rpif11())
            .field("rpif12", &self.rpif12())
            .field("rpif13", &self.rpif13())
            .field("rpif14", &self.rpif14())
            .field("rpif15", &self.rpif15())
            .field("rpif16", &self.rpif16())
            .finish()
    }
}
impl W {
    ///Bit 0 - configurable event inputs x rising edge Pending bit.
    #[inline(always)]
    pub fn rpif0(&mut self) -> RPIF0_W<'_, RPR1rs> {
        RPIF0_W::new(self, 0)
    }
    ///Bit 1 - configurable event inputs x rising edge Pending bit.
    #[inline(always)]
    pub fn rpif1(&mut self) -> RPIF1_W<'_, RPR1rs> {
        RPIF1_W::new(self, 1)
    }
    ///Bit 2 - configurable event inputs x rising edge Pending bit.
    #[inline(always)]
    pub fn rpif2(&mut self) -> RPIF2_W<'_, RPR1rs> {
        RPIF2_W::new(self, 2)
    }
    ///Bit 3 - configurable event inputs x rising edge Pending bit.
    #[inline(always)]
    pub fn rpif3(&mut self) -> RPIF3_W<'_, RPR1rs> {
        RPIF3_W::new(self, 3)
    }
    ///Bit 4 - configurable event inputs x rising edge Pending bit.
    #[inline(always)]
    pub fn rpif4(&mut self) -> RPIF4_W<'_, RPR1rs> {
        RPIF4_W::new(self, 4)
    }
    ///Bit 5 - configurable event inputs x rising edge Pending bit
    #[inline(always)]
    pub fn rpif5(&mut self) -> RPIF5_W<'_, RPR1rs> {
        RPIF5_W::new(self, 5)
    }
    ///Bit 6 - configurable event inputs x rising edge Pending bit.
    #[inline(always)]
    pub fn rpif6(&mut self) -> RPIF6_W<'_, RPR1rs> {
        RPIF6_W::new(self, 6)
    }
    ///Bit 7 - configurable event inputs x rising edge Pending bit.
    #[inline(always)]
    pub fn rpif7(&mut self) -> RPIF7_W<'_, RPR1rs> {
        RPIF7_W::new(self, 7)
    }
    ///Bit 8 - configurable event inputs x rising edge Pending bit.
    #[inline(always)]
    pub fn rpif8(&mut self) -> RPIF8_W<'_, RPR1rs> {
        RPIF8_W::new(self, 8)
    }
    ///Bit 9 - configurable event inputs x rising edge Pending bit.
    #[inline(always)]
    pub fn rpif9(&mut self) -> RPIF9_W<'_, RPR1rs> {
        RPIF9_W::new(self, 9)
    }
    ///Bit 10 - configurable event inputs x rising edge Pending bit.
    #[inline(always)]
    pub fn rpif10(&mut self) -> RPIF10_W<'_, RPR1rs> {
        RPIF10_W::new(self, 10)
    }
    ///Bit 11 - configurable event inputs x rising edge Pending bit.
    #[inline(always)]
    pub fn rpif11(&mut self) -> RPIF11_W<'_, RPR1rs> {
        RPIF11_W::new(self, 11)
    }
    ///Bit 12 - configurable event inputs x rising edge Pending bit.
    #[inline(always)]
    pub fn rpif12(&mut self) -> RPIF12_W<'_, RPR1rs> {
        RPIF12_W::new(self, 12)
    }
    ///Bit 13 - configurable event inputs x rising edge Pending bit.
    #[inline(always)]
    pub fn rpif13(&mut self) -> RPIF13_W<'_, RPR1rs> {
        RPIF13_W::new(self, 13)
    }
    ///Bit 14 - configurable event inputs x rising edge Pending bit.
    #[inline(always)]
    pub fn rpif14(&mut self) -> RPIF14_W<'_, RPR1rs> {
        RPIF14_W::new(self, 14)
    }
    ///Bit 15 - configurable event inputs x rising edge Pending bit.
    #[inline(always)]
    pub fn rpif15(&mut self) -> RPIF15_W<'_, RPR1rs> {
        RPIF15_W::new(self, 15)
    }
    ///Bit 16 - configurable event inputs x rising edge Pending bit.
    #[inline(always)]
    pub fn rpif16(&mut self) -> RPIF16_W<'_, RPR1rs> {
        RPIF16_W::new(self, 16)
    }
}
/**EXTI rising edge pending register

You can [`read`](crate::Reg::read) this register and get [`rpr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rpr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G051.html#EXTI:RPR1)*/
pub struct RPR1rs;
impl crate::RegisterSpec for RPR1rs {
    type Ux = u32;
}
///`read()` method returns [`rpr1::R`](R) reader structure
impl crate::Readable for RPR1rs {}
///`write(|w| ..)` method takes [`rpr1::W`](W) writer structure
impl crate::Writable for RPR1rs {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0001_ffff;
}
///`reset()` method sets RPR1 to value 0
impl crate::Resettable for RPR1rs {}

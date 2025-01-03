///Register `ODR` reader
pub type R = crate::R<ODRrs>;
///Register `ODR` writer
pub type W = crate::W<ODRrs>;
/**Port output data (y = 0..15)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ODR0 {
    ///0: Set output to logic low
    Low = 0,
    ///1: Set output to logic high
    High = 1,
}
impl From<ODR0> for bool {
    #[inline(always)]
    fn from(variant: ODR0) -> Self {
        variant as u8 != 0
    }
}
///Field `ODR0` reader - Port output data (y = 0..15)
pub type ODR0_R = crate::BitReader<ODR0>;
impl ODR0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ODR0 {
        match self.bits {
            false => ODR0::Low,
            true => ODR0::High,
        }
    }
    ///Set output to logic low
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == ODR0::Low
    }
    ///Set output to logic high
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == ODR0::High
    }
}
///Field `ODR0` writer - Port output data (y = 0..15)
pub type ODR0_W<'a, REG> = crate::BitWriter<'a, REG, ODR0>;
impl<'a, REG> ODR0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Set output to logic low
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(ODR0::Low)
    }
    ///Set output to logic high
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(ODR0::High)
    }
}
///Field `ODR1` reader - Port output data (y = 0..15)
pub use ODR0_R as ODR1_R;
///Field `ODR2` reader - Port output data (y = 0..15)
pub use ODR0_R as ODR2_R;
///Field `ODR3` reader - Port output data (y = 0..15)
pub use ODR0_R as ODR3_R;
///Field `ODR4` reader - Port output data (y = 0..15)
pub use ODR0_R as ODR4_R;
///Field `ODR5` reader - Port output data (y = 0..15)
pub use ODR0_R as ODR5_R;
///Field `ODR6` reader - Port output data (y = 0..15)
pub use ODR0_R as ODR6_R;
///Field `ODR7` reader - Port output data (y = 0..15)
pub use ODR0_R as ODR7_R;
///Field `ODR8` reader - Port output data (y = 0..15)
pub use ODR0_R as ODR8_R;
///Field `ODR9` reader - Port output data (y = 0..15)
pub use ODR0_R as ODR9_R;
///Field `ODR10` reader - Port output data (y = 0..15)
pub use ODR0_R as ODR10_R;
///Field `ODR11` reader - Port output data (y = 0..15)
pub use ODR0_R as ODR11_R;
///Field `ODR12` reader - Port output data (y = 0..15)
pub use ODR0_R as ODR12_R;
///Field `ODR13` reader - Port output data (y = 0..15)
pub use ODR0_R as ODR13_R;
///Field `ODR14` reader - Port output data (y = 0..15)
pub use ODR0_R as ODR14_R;
///Field `ODR15` reader - Port output data (y = 0..15)
pub use ODR0_R as ODR15_R;
///Field `ODR1` writer - Port output data (y = 0..15)
pub use ODR0_W as ODR1_W;
///Field `ODR2` writer - Port output data (y = 0..15)
pub use ODR0_W as ODR2_W;
///Field `ODR3` writer - Port output data (y = 0..15)
pub use ODR0_W as ODR3_W;
///Field `ODR4` writer - Port output data (y = 0..15)
pub use ODR0_W as ODR4_W;
///Field `ODR5` writer - Port output data (y = 0..15)
pub use ODR0_W as ODR5_W;
///Field `ODR6` writer - Port output data (y = 0..15)
pub use ODR0_W as ODR6_W;
///Field `ODR7` writer - Port output data (y = 0..15)
pub use ODR0_W as ODR7_W;
///Field `ODR8` writer - Port output data (y = 0..15)
pub use ODR0_W as ODR8_W;
///Field `ODR9` writer - Port output data (y = 0..15)
pub use ODR0_W as ODR9_W;
///Field `ODR10` writer - Port output data (y = 0..15)
pub use ODR0_W as ODR10_W;
///Field `ODR11` writer - Port output data (y = 0..15)
pub use ODR0_W as ODR11_W;
///Field `ODR12` writer - Port output data (y = 0..15)
pub use ODR0_W as ODR12_W;
///Field `ODR13` writer - Port output data (y = 0..15)
pub use ODR0_W as ODR13_W;
///Field `ODR14` writer - Port output data (y = 0..15)
pub use ODR0_W as ODR14_W;
///Field `ODR15` writer - Port output data (y = 0..15)
pub use ODR0_W as ODR15_W;
impl R {
    ///Bit 0 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr0(&self) -> ODR0_R {
        ODR0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr1(&self) -> ODR1_R {
        ODR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr2(&self) -> ODR2_R {
        ODR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr3(&self) -> ODR3_R {
        ODR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr4(&self) -> ODR4_R {
        ODR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr5(&self) -> ODR5_R {
        ODR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr6(&self) -> ODR6_R {
        ODR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr7(&self) -> ODR7_R {
        ODR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr8(&self) -> ODR8_R {
        ODR8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr9(&self) -> ODR9_R {
        ODR9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr10(&self) -> ODR10_R {
        ODR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr11(&self) -> ODR11_R {
        ODR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr12(&self) -> ODR12_R {
        ODR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr13(&self) -> ODR13_R {
        ODR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr14(&self) -> ODR14_R {
        ODR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr15(&self) -> ODR15_R {
        ODR15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ODR")
            .field("odr0", &self.odr0())
            .field("odr15", &self.odr15())
            .field("odr14", &self.odr14())
            .field("odr13", &self.odr13())
            .field("odr12", &self.odr12())
            .field("odr11", &self.odr11())
            .field("odr10", &self.odr10())
            .field("odr9", &self.odr9())
            .field("odr8", &self.odr8())
            .field("odr7", &self.odr7())
            .field("odr6", &self.odr6())
            .field("odr5", &self.odr5())
            .field("odr4", &self.odr4())
            .field("odr3", &self.odr3())
            .field("odr2", &self.odr2())
            .field("odr1", &self.odr1())
            .finish()
    }
}
impl W {
    ///Bit 0 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr0(&mut self) -> ODR0_W<ODRrs> {
        ODR0_W::new(self, 0)
    }
    ///Bit 1 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr1(&mut self) -> ODR1_W<ODRrs> {
        ODR1_W::new(self, 1)
    }
    ///Bit 2 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr2(&mut self) -> ODR2_W<ODRrs> {
        ODR2_W::new(self, 2)
    }
    ///Bit 3 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr3(&mut self) -> ODR3_W<ODRrs> {
        ODR3_W::new(self, 3)
    }
    ///Bit 4 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr4(&mut self) -> ODR4_W<ODRrs> {
        ODR4_W::new(self, 4)
    }
    ///Bit 5 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr5(&mut self) -> ODR5_W<ODRrs> {
        ODR5_W::new(self, 5)
    }
    ///Bit 6 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr6(&mut self) -> ODR6_W<ODRrs> {
        ODR6_W::new(self, 6)
    }
    ///Bit 7 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr7(&mut self) -> ODR7_W<ODRrs> {
        ODR7_W::new(self, 7)
    }
    ///Bit 8 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr8(&mut self) -> ODR8_W<ODRrs> {
        ODR8_W::new(self, 8)
    }
    ///Bit 9 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr9(&mut self) -> ODR9_W<ODRrs> {
        ODR9_W::new(self, 9)
    }
    ///Bit 10 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr10(&mut self) -> ODR10_W<ODRrs> {
        ODR10_W::new(self, 10)
    }
    ///Bit 11 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr11(&mut self) -> ODR11_W<ODRrs> {
        ODR11_W::new(self, 11)
    }
    ///Bit 12 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr12(&mut self) -> ODR12_W<ODRrs> {
        ODR12_W::new(self, 12)
    }
    ///Bit 13 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr13(&mut self) -> ODR13_W<ODRrs> {
        ODR13_W::new(self, 13)
    }
    ///Bit 14 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr14(&mut self) -> ODR14_W<ODRrs> {
        ODR14_W::new(self, 14)
    }
    ///Bit 15 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr15(&mut self) -> ODR15_W<ODRrs> {
        ODR15_W::new(self, 15)
    }
}
/**GPIO port output data register

You can [`read`](crate::Reg::read) this register and get [`odr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`odr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#GPIOB:ODR)*/
pub struct ODRrs;
impl crate::RegisterSpec for ODRrs {
    type Ux = u32;
}
///`read()` method returns [`odr::R`](R) reader structure
impl crate::Readable for ODRrs {}
///`write(|w| ..)` method takes [`odr::W`](W) writer structure
impl crate::Writable for ODRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ODR to value 0
impl crate::Resettable for ODRrs {
    const RESET_VALUE: u32 = 0;
}

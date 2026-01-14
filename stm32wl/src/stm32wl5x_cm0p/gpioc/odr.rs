///Register `ODR` reader
pub type R = crate::R<ODRrs>;
///Register `ODR` writer
pub type W = crate::W<ODRrs>;
/**Port output data (y = 0..15)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OUTPUT_DATA {
    ///0: Set output to logic low
    Low = 0,
    ///1: Set output to logic high
    High = 1,
}
impl From<OUTPUT_DATA> for bool {
    #[inline(always)]
    fn from(variant: OUTPUT_DATA) -> Self {
        variant as u8 != 0
    }
}
///Field `ODR0` reader - Port output data (y = 0..15)
pub type ODR0_R = crate::BitReader<OUTPUT_DATA>;
impl ODR0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OUTPUT_DATA {
        match self.bits {
            false => OUTPUT_DATA::Low,
            true => OUTPUT_DATA::High,
        }
    }
    ///Set output to logic low
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == OUTPUT_DATA::Low
    }
    ///Set output to logic high
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == OUTPUT_DATA::High
    }
}
///Field `ODR0` writer - Port output data (y = 0..15)
pub type ODR0_W<'a, REG> = crate::BitWriter<'a, REG, OUTPUT_DATA>;
impl<'a, REG> ODR0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Set output to logic low
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(OUTPUT_DATA::Low)
    }
    ///Set output to logic high
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(OUTPUT_DATA::High)
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
    pub fn odr0(&mut self) -> ODR0_W<'_, ODRrs> {
        ODR0_W::new(self, 0)
    }
    ///Bit 1 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr1(&mut self) -> ODR1_W<'_, ODRrs> {
        ODR1_W::new(self, 1)
    }
    ///Bit 2 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr2(&mut self) -> ODR2_W<'_, ODRrs> {
        ODR2_W::new(self, 2)
    }
    ///Bit 3 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr3(&mut self) -> ODR3_W<'_, ODRrs> {
        ODR3_W::new(self, 3)
    }
    ///Bit 4 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr4(&mut self) -> ODR4_W<'_, ODRrs> {
        ODR4_W::new(self, 4)
    }
    ///Bit 5 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr5(&mut self) -> ODR5_W<'_, ODRrs> {
        ODR5_W::new(self, 5)
    }
    ///Bit 6 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr6(&mut self) -> ODR6_W<'_, ODRrs> {
        ODR6_W::new(self, 6)
    }
    ///Bit 13 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr13(&mut self) -> ODR13_W<'_, ODRrs> {
        ODR13_W::new(self, 13)
    }
    ///Bit 14 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr14(&mut self) -> ODR14_W<'_, ODRrs> {
        ODR14_W::new(self, 14)
    }
    ///Bit 15 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr15(&mut self) -> ODR15_W<'_, ODRrs> {
        ODR15_W::new(self, 15)
    }
}
/**GPIO port output data register

You can [`read`](crate::Reg::read) this register and get [`odr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`odr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#GPIOC:ODR)*/
pub struct ODRrs;
impl crate::RegisterSpec for ODRrs {
    type Ux = u32;
}
///`read()` method returns [`odr::R`](R) reader structure
impl crate::Readable for ODRrs {}
///`write(|w| ..)` method takes [`odr::W`](W) writer structure
impl crate::Writable for ODRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ODR to value 0
impl crate::Resettable for ODRrs {}

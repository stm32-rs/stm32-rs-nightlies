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
///Field `ODR3` reader - Port output data (y = 0..15)
pub use ODR0_R as ODR3_R;
///Field `ODR1` writer - Port output data (y = 0..15)
pub use ODR0_W as ODR1_W;
///Field `ODR3` writer - Port output data (y = 0..15)
pub use ODR0_W as ODR3_W;
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
    ///Bit 3 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr3(&self) -> ODR3_R {
        ODR3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ODR")
            .field("odr0", &self.odr0())
            .field("odr3", &self.odr3())
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
    ///Bit 3 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr3(&mut self) -> ODR3_W<'_, ODRrs> {
        ODR3_W::new(self, 3)
    }
}
/**GPIO port output data register

You can [`read`](crate::Reg::read) this register and get [`odr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`odr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#GPIOH:ODR)*/
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

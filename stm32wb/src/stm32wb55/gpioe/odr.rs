///Register `ODR` reader
pub type R = crate::R<ODRrs>;
///Register `ODR` writer
pub type W = crate::W<ODRrs>;
/**Port output data pin %s

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
///Field `ODR(0-4)` reader - Port output data pin %s
pub type ODR_R = crate::BitReader<ODR0>;
impl ODR_R {
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
///Field `ODR(0-4)` writer - Port output data pin %s
pub type ODR_W<'a, REG> = crate::BitWriter<'a, REG, ODR0>;
impl<'a, REG> ODR_W<'a, REG>
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
impl R {
    ///Port output data pin (0-4)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `ODR0` field.</div>
    #[inline(always)]
    pub fn odr(&self, n: u8) -> ODR_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        ODR_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Port output data pin (0-4)
    #[inline(always)]
    pub fn odr_iter(&self) -> impl Iterator<Item = ODR_R> + '_ {
        (0..5).map(move |n| ODR_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - Port output data pin 0
    #[inline(always)]
    pub fn odr0(&self) -> ODR_R {
        ODR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port output data pin 1
    #[inline(always)]
    pub fn odr1(&self) -> ODR_R {
        ODR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Port output data pin 2
    #[inline(always)]
    pub fn odr2(&self) -> ODR_R {
        ODR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Port output data pin 3
    #[inline(always)]
    pub fn odr3(&self) -> ODR_R {
        ODR_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Port output data pin 4
    #[inline(always)]
    pub fn odr4(&self) -> ODR_R {
        ODR_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ODR")
            .field("odr0", &self.odr0())
            .field("odr1", &self.odr1())
            .field("odr2", &self.odr2())
            .field("odr3", &self.odr3())
            .field("odr4", &self.odr4())
            .finish()
    }
}
impl W {
    ///Port output data pin (0-4)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `ODR0` field.</div>
    #[inline(always)]
    pub fn odr(&mut self, n: u8) -> ODR_W<ODRrs> {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        ODR_W::new(self, n)
    }
    ///Bit 0 - Port output data pin 0
    #[inline(always)]
    pub fn odr0(&mut self) -> ODR_W<ODRrs> {
        ODR_W::new(self, 0)
    }
    ///Bit 1 - Port output data pin 1
    #[inline(always)]
    pub fn odr1(&mut self) -> ODR_W<ODRrs> {
        ODR_W::new(self, 1)
    }
    ///Bit 2 - Port output data pin 2
    #[inline(always)]
    pub fn odr2(&mut self) -> ODR_W<ODRrs> {
        ODR_W::new(self, 2)
    }
    ///Bit 3 - Port output data pin 3
    #[inline(always)]
    pub fn odr3(&mut self) -> ODR_W<ODRrs> {
        ODR_W::new(self, 3)
    }
    ///Bit 4 - Port output data pin 4
    #[inline(always)]
    pub fn odr4(&mut self) -> ODR_W<ODRrs> {
        ODR_W::new(self, 4)
    }
}
/**GPIO port output data register

You can [`read`](crate::Reg::read) this register and get [`odr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`odr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#GPIOE:ODR)*/
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

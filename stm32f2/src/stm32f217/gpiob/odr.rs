#[doc = "Register `ODR` reader"]
pub type R = crate::R<ODRrs>;
#[doc = "Register `ODR` writer"]
pub type W = crate::W<ODRrs>;
#[doc = "Port output data pin %s\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ODR0 {
    #[doc = "0: Set output to logic low"]
    Low = 0,
    #[doc = "1: Set output to logic high"]
    High = 1,
}
impl From<ODR0> for bool {
    #[inline(always)]
    fn from(variant: ODR0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ODR(0-15)` reader - Port output data pin %s"]
pub type ODR_R = crate::BitReader<ODR0>;
impl ODR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ODR0 {
        match self.bits {
            false => ODR0::Low,
            true => ODR0::High,
        }
    }
    #[doc = "Set output to logic low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == ODR0::Low
    }
    #[doc = "Set output to logic high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == ODR0::High
    }
}
#[doc = "Field `ODR(0-15)` writer - Port output data pin %s"]
pub type ODR_W<'a, REG> = crate::BitWriter<'a, REG, ODR0>;
impl<'a, REG> ODR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set output to logic low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(ODR0::Low)
    }
    #[doc = "Set output to logic high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(ODR0::High)
    }
}
impl R {
    #[doc = "Port output data pin (0-15)"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `ODR0` field"]
    #[inline(always)]
    pub fn odr(&self, n: u8) -> ODR_R {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        ODR_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Port output data pin (0-15)"]
    #[inline(always)]
    pub fn odr_iter(&self) -> impl Iterator<Item = ODR_R> + '_ {
        (0..16).map(move |n| ODR_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Port output data pin 0"]
    #[inline(always)]
    pub fn odr0(&self) -> ODR_R {
        ODR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port output data pin 1"]
    #[inline(always)]
    pub fn odr1(&self) -> ODR_R {
        ODR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port output data pin 2"]
    #[inline(always)]
    pub fn odr2(&self) -> ODR_R {
        ODR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port output data pin 3"]
    #[inline(always)]
    pub fn odr3(&self) -> ODR_R {
        ODR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port output data pin 4"]
    #[inline(always)]
    pub fn odr4(&self) -> ODR_R {
        ODR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port output data pin 5"]
    #[inline(always)]
    pub fn odr5(&self) -> ODR_R {
        ODR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port output data pin 6"]
    #[inline(always)]
    pub fn odr6(&self) -> ODR_R {
        ODR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port output data pin 7"]
    #[inline(always)]
    pub fn odr7(&self) -> ODR_R {
        ODR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port output data pin 8"]
    #[inline(always)]
    pub fn odr8(&self) -> ODR_R {
        ODR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port output data pin 9"]
    #[inline(always)]
    pub fn odr9(&self) -> ODR_R {
        ODR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port output data pin 10"]
    #[inline(always)]
    pub fn odr10(&self) -> ODR_R {
        ODR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port output data pin 11"]
    #[inline(always)]
    pub fn odr11(&self) -> ODR_R {
        ODR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port output data pin 12"]
    #[inline(always)]
    pub fn odr12(&self) -> ODR_R {
        ODR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port output data pin 13"]
    #[inline(always)]
    pub fn odr13(&self) -> ODR_R {
        ODR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port output data pin 14"]
    #[inline(always)]
    pub fn odr14(&self) -> ODR_R {
        ODR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port output data pin 15"]
    #[inline(always)]
    pub fn odr15(&self) -> ODR_R {
        ODR_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Port output data pin (0-15)"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `ODR0` field"]
    #[inline(always)]
    #[must_use]
    pub fn odr(&mut self, n: u8) -> ODR_W<ODRrs> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        ODR_W::new(self, n)
    }
    #[doc = "Bit 0 - Port output data pin 0"]
    #[inline(always)]
    #[must_use]
    pub fn odr0(&mut self) -> ODR_W<ODRrs> {
        ODR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Port output data pin 1"]
    #[inline(always)]
    #[must_use]
    pub fn odr1(&mut self) -> ODR_W<ODRrs> {
        ODR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Port output data pin 2"]
    #[inline(always)]
    #[must_use]
    pub fn odr2(&mut self) -> ODR_W<ODRrs> {
        ODR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Port output data pin 3"]
    #[inline(always)]
    #[must_use]
    pub fn odr3(&mut self) -> ODR_W<ODRrs> {
        ODR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Port output data pin 4"]
    #[inline(always)]
    #[must_use]
    pub fn odr4(&mut self) -> ODR_W<ODRrs> {
        ODR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Port output data pin 5"]
    #[inline(always)]
    #[must_use]
    pub fn odr5(&mut self) -> ODR_W<ODRrs> {
        ODR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Port output data pin 6"]
    #[inline(always)]
    #[must_use]
    pub fn odr6(&mut self) -> ODR_W<ODRrs> {
        ODR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Port output data pin 7"]
    #[inline(always)]
    #[must_use]
    pub fn odr7(&mut self) -> ODR_W<ODRrs> {
        ODR_W::new(self, 7)
    }
    #[doc = "Bit 8 - Port output data pin 8"]
    #[inline(always)]
    #[must_use]
    pub fn odr8(&mut self) -> ODR_W<ODRrs> {
        ODR_W::new(self, 8)
    }
    #[doc = "Bit 9 - Port output data pin 9"]
    #[inline(always)]
    #[must_use]
    pub fn odr9(&mut self) -> ODR_W<ODRrs> {
        ODR_W::new(self, 9)
    }
    #[doc = "Bit 10 - Port output data pin 10"]
    #[inline(always)]
    #[must_use]
    pub fn odr10(&mut self) -> ODR_W<ODRrs> {
        ODR_W::new(self, 10)
    }
    #[doc = "Bit 11 - Port output data pin 11"]
    #[inline(always)]
    #[must_use]
    pub fn odr11(&mut self) -> ODR_W<ODRrs> {
        ODR_W::new(self, 11)
    }
    #[doc = "Bit 12 - Port output data pin 12"]
    #[inline(always)]
    #[must_use]
    pub fn odr12(&mut self) -> ODR_W<ODRrs> {
        ODR_W::new(self, 12)
    }
    #[doc = "Bit 13 - Port output data pin 13"]
    #[inline(always)]
    #[must_use]
    pub fn odr13(&mut self) -> ODR_W<ODRrs> {
        ODR_W::new(self, 13)
    }
    #[doc = "Bit 14 - Port output data pin 14"]
    #[inline(always)]
    #[must_use]
    pub fn odr14(&mut self) -> ODR_W<ODRrs> {
        ODR_W::new(self, 14)
    }
    #[doc = "Bit 15 - Port output data pin 15"]
    #[inline(always)]
    #[must_use]
    pub fn odr15(&mut self) -> ODR_W<ODRrs> {
        ODR_W::new(self, 15)
    }
}
#[doc = "GPIO port output data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`odr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`odr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ODRrs;
impl crate::RegisterSpec for ODRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`odr::R`](R) reader structure"]
impl crate::Readable for ODRrs {}
#[doc = "`write(|w| ..)` method takes [`odr::W`](W) writer structure"]
impl crate::Writable for ODRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ODR to value 0"]
impl crate::Resettable for ODRrs {
    const RESET_VALUE: u32 = 0;
}

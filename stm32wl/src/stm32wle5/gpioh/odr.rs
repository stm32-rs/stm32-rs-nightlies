#[doc = "Register `ODR` reader"]
pub type R = crate::R<ODRrs>;
#[doc = "Register `ODR` writer"]
pub type W = crate::W<ODRrs>;
#[doc = "Port output data (y = 0..15)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ODR3 {
    #[doc = "0: Set output to logic low"]
    Low = 0,
    #[doc = "1: Set output to logic high"]
    High = 1,
}
impl From<ODR3> for bool {
    #[inline(always)]
    fn from(variant: ODR3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ODR3` reader - Port output data (y = 0..15)"]
pub type ODR3_R = crate::BitReader<ODR3>;
impl ODR3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ODR3 {
        match self.bits {
            false => ODR3::Low,
            true => ODR3::High,
        }
    }
    #[doc = "Set output to logic low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == ODR3::Low
    }
    #[doc = "Set output to logic high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == ODR3::High
    }
}
#[doc = "Field `ODR3` writer - Port output data (y = 0..15)"]
pub type ODR3_W<'a, REG> = crate::BitWriter<'a, REG, ODR3>;
impl<'a, REG> ODR3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set output to logic low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(ODR3::Low)
    }
    #[doc = "Set output to logic high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(ODR3::High)
    }
}
impl R {
    #[doc = "Bit 3 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr3(&self) -> ODR3_R {
        ODR3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Port output data (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn odr3(&mut self) -> ODR3_W<ODRrs> {
        ODR3_W::new(self, 3)
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

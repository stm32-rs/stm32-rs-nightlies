#[doc = "Register `ODR` reader"]
pub type R = crate::R<ODRrs>;
#[doc = "Register `ODR` writer"]
pub type W = crate::W<ODRrs>;
#[doc = "Field `ODR0` reader - Port output data (y = 0..15)"]
pub type ODR0_R = crate::BitReader;
#[doc = "Field `ODR0` writer - Port output data (y = 0..15)"]
pub type ODR0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODR1` reader - Port output data (y = 0..15)"]
pub type ODR1_R = crate::BitReader;
#[doc = "Field `ODR1` writer - Port output data (y = 0..15)"]
pub type ODR1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODR3` reader - Port output data (y = 0..15)"]
pub type ODR3_R = crate::BitReader;
#[doc = "Field `ODR3` writer - Port output data (y = 0..15)"]
pub type ODR3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr0(&self) -> ODR0_R {
        ODR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr1(&self) -> ODR1_R {
        ODR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr3(&self) -> ODR3_R {
        ODR3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port output data (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn odr0(&mut self) -> ODR0_W<ODRrs> {
        ODR0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Port output data (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn odr1(&mut self) -> ODR1_W<ODRrs> {
        ODR1_W::new(self, 1)
    }
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

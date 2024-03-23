#[doc = "Register `RNG_SR` reader"]
pub type R = crate::R<RNG_SRrs>;
#[doc = "Register `RNG_SR` writer"]
pub type W = crate::W<RNG_SRrs>;
#[doc = "Field `DRDY` reader - DRDY"]
pub type DRDY_R = crate::BitReader;
#[doc = "Field `CECS` reader - CECS"]
pub type CECS_R = crate::BitReader;
#[doc = "Field `SECS` reader - SECS"]
pub type SECS_R = crate::BitReader;
#[doc = "Field `CEIS` reader - CEIS"]
pub type CEIS_R = crate::BitReader;
#[doc = "Field `CEIS` writer - CEIS"]
pub type CEIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEIS` reader - SEIS"]
pub type SEIS_R = crate::BitReader;
#[doc = "Field `SEIS` writer - SEIS"]
pub type SEIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DRDY"]
    #[inline(always)]
    pub fn drdy(&self) -> DRDY_R {
        DRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CECS"]
    #[inline(always)]
    pub fn cecs(&self) -> CECS_R {
        CECS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SECS"]
    #[inline(always)]
    pub fn secs(&self) -> SECS_R {
        SECS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - CEIS"]
    #[inline(always)]
    pub fn ceis(&self) -> CEIS_R {
        CEIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SEIS"]
    #[inline(always)]
    pub fn seis(&self) -> SEIS_R {
        SEIS_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - CEIS"]
    #[inline(always)]
    #[must_use]
    pub fn ceis(&mut self) -> CEIS_W<RNG_SRrs> {
        CEIS_W::new(self, 5)
    }
    #[doc = "Bit 6 - SEIS"]
    #[inline(always)]
    #[must_use]
    pub fn seis(&mut self) -> SEIS_W<RNG_SRrs> {
        SEIS_W::new(self, 6)
    }
}
#[doc = "RNG status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rng_sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rng_sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RNG_SRrs;
impl crate::RegisterSpec for RNG_SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng_sr::R`](R) reader structure"]
impl crate::Readable for RNG_SRrs {}
#[doc = "`write(|w| ..)` method takes [`rng_sr::W`](W) writer structure"]
impl crate::Writable for RNG_SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RNG_SR to value 0"]
impl crate::Resettable for RNG_SRrs {
    const RESET_VALUE: u32 = 0;
}

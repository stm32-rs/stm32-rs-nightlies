#[doc = "Register `LCKR` reader"]
pub type R = crate::R<LCKRrs>;
#[doc = "Register `LCKR` writer"]
pub type W = crate::W<LCKRrs>;
#[doc = "Field `LCK0` reader - Port x lock bit y (y= 0..15)"]
pub type LCK0_R = crate::BitReader;
#[doc = "Field `LCK0` writer - Port x lock bit y (y= 0..15)"]
pub type LCK0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCK1` reader - Port x lock bit y (y= 0..15)"]
pub type LCK1_R = crate::BitReader;
#[doc = "Field `LCK1` writer - Port x lock bit y (y= 0..15)"]
pub type LCK1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCK3` reader - Port x lock bit y (y= 0..15)"]
pub type LCK3_R = crate::BitReader;
#[doc = "Field `LCK3` writer - Port x lock bit y (y= 0..15)"]
pub type LCK3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCKK` reader - Port x lock bit y (y= 0..15)"]
pub type LCKK_R = crate::BitReader;
#[doc = "Field `LCKK` writer - Port x lock bit y (y= 0..15)"]
pub type LCKK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck0(&self) -> LCK0_R {
        LCK0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck1(&self) -> LCK1_R {
        LCK1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck3(&self) -> LCK3_R {
        LCK3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lckk(&self) -> LCKK_R {
        LCKK_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn lck0(&mut self) -> LCK0_W<LCKRrs> {
        LCK0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn lck1(&mut self) -> LCK1_W<LCKRrs> {
        LCK1_W::new(self, 1)
    }
    #[doc = "Bit 3 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn lck3(&mut self) -> LCK3_W<LCKRrs> {
        LCK3_W::new(self, 3)
    }
    #[doc = "Bit 16 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn lckk(&mut self) -> LCKK_W<LCKRrs> {
        LCKK_W::new(self, 16)
    }
}
#[doc = "GPIO port configuration lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lckr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lckr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCKRrs;
impl crate::RegisterSpec for LCKRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lckr::R`](R) reader structure"]
impl crate::Readable for LCKRrs {}
#[doc = "`write(|w| ..)` method takes [`lckr::W`](W) writer structure"]
impl crate::Writable for LCKRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LCKR to value 0"]
impl crate::Resettable for LCKRrs {
    const RESET_VALUE: u32 = 0;
}

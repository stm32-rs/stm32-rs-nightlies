#[doc = "Register `OTYPER` reader"]
pub type R = crate::R<OTYPERrs>;
#[doc = "Register `OTYPER` writer"]
pub type W = crate::W<OTYPERrs>;
#[doc = "Field `OT0` reader - Port x configuration bits (y = 0..15)"]
pub type OT0_R = crate::BitReader;
#[doc = "Field `OT0` writer - Port x configuration bits (y = 0..15)"]
pub type OT0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OT1` reader - Port x configuration bits (y = 0..15)"]
pub type OT1_R = crate::BitReader;
#[doc = "Field `OT1` writer - Port x configuration bits (y = 0..15)"]
pub type OT1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OT2` reader - Port x configuration bits (y = 0..15)"]
pub type OT2_R = crate::BitReader;
#[doc = "Field `OT2` writer - Port x configuration bits (y = 0..15)"]
pub type OT2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OT3` reader - Port x configuration bits (y = 0..15)"]
pub type OT3_R = crate::BitReader;
#[doc = "Field `OT3` writer - Port x configuration bits (y = 0..15)"]
pub type OT3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OT4` reader - Port x configuration bits (y = 0..15)"]
pub type OT4_R = crate::BitReader;
#[doc = "Field `OT4` writer - Port x configuration bits (y = 0..15)"]
pub type OT4_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot0(&self) -> OT0_R {
        OT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot1(&self) -> OT1_R {
        OT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot2(&self) -> OT2_R {
        OT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot3(&self) -> OT3_R {
        OT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot4(&self) -> OT4_R {
        OT4_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ot0(&mut self) -> OT0_W<OTYPERrs> {
        OT0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ot1(&mut self) -> OT1_W<OTYPERrs> {
        OT1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ot2(&mut self) -> OT2_W<OTYPERrs> {
        OT2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ot3(&mut self) -> OT3_W<OTYPERrs> {
        OT3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ot4(&mut self) -> OT4_W<OTYPERrs> {
        OT4_W::new(self, 4)
    }
}
#[doc = "GPIO port output type register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otyper::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otyper::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTYPERrs;
impl crate::RegisterSpec for OTYPERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otyper::R`](R) reader structure"]
impl crate::Readable for OTYPERrs {}
#[doc = "`write(|w| ..)` method takes [`otyper::W`](W) writer structure"]
impl crate::Writable for OTYPERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTYPER to value 0"]
impl crate::Resettable for OTYPERrs {
    const RESET_VALUE: u32 = 0;
}

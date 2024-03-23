#[doc = "Register `EXTI_C2IMR3` reader"]
pub type R = crate::R<EXTI_C2IMR3rs>;
#[doc = "Register `EXTI_C2IMR3` writer"]
pub type W = crate::W<EXTI_C2IMR3rs>;
#[doc = "Field `IM64` reader - IM64"]
pub type IM64_R = crate::BitReader;
#[doc = "Field `IM64` writer - IM64"]
pub type IM64_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM65` reader - IM65"]
pub type IM65_R = crate::BitReader;
#[doc = "Field `IM65` writer - IM65"]
pub type IM65_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM66` reader - IM66"]
pub type IM66_R = crate::BitReader;
#[doc = "Field `IM66` writer - IM66"]
pub type IM66_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM67` reader - IM67"]
pub type IM67_R = crate::BitReader;
#[doc = "Field `IM67` writer - IM67"]
pub type IM67_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM68` reader - IM68"]
pub type IM68_R = crate::BitReader;
#[doc = "Field `IM68` writer - IM68"]
pub type IM68_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM69` reader - IM69"]
pub type IM69_R = crate::BitReader;
#[doc = "Field `IM69` writer - IM69"]
pub type IM69_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM70` reader - IM70"]
pub type IM70_R = crate::BitReader;
#[doc = "Field `IM70` writer - IM70"]
pub type IM70_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM71` reader - IM71"]
pub type IM71_R = crate::BitReader;
#[doc = "Field `IM71` writer - IM71"]
pub type IM71_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM72` reader - IM72"]
pub type IM72_R = crate::BitReader;
#[doc = "Field `IM72` writer - IM72"]
pub type IM72_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM73` reader - IM73"]
pub type IM73_R = crate::BitReader;
#[doc = "Field `IM73` writer - IM73"]
pub type IM73_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM74` reader - IM74"]
pub type IM74_R = crate::BitReader;
#[doc = "Field `IM74` writer - IM74"]
pub type IM74_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM75` reader - IM75"]
pub type IM75_R = crate::BitReader;
#[doc = "Field `IM75` writer - IM75"]
pub type IM75_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - IM64"]
    #[inline(always)]
    pub fn im64(&self) -> IM64_R {
        IM64_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IM65"]
    #[inline(always)]
    pub fn im65(&self) -> IM65_R {
        IM65_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IM66"]
    #[inline(always)]
    pub fn im66(&self) -> IM66_R {
        IM66_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IM67"]
    #[inline(always)]
    pub fn im67(&self) -> IM67_R {
        IM67_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IM68"]
    #[inline(always)]
    pub fn im68(&self) -> IM68_R {
        IM68_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IM69"]
    #[inline(always)]
    pub fn im69(&self) -> IM69_R {
        IM69_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IM70"]
    #[inline(always)]
    pub fn im70(&self) -> IM70_R {
        IM70_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IM71"]
    #[inline(always)]
    pub fn im71(&self) -> IM71_R {
        IM71_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - IM72"]
    #[inline(always)]
    pub fn im72(&self) -> IM72_R {
        IM72_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - IM73"]
    #[inline(always)]
    pub fn im73(&self) -> IM73_R {
        IM73_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - IM74"]
    #[inline(always)]
    pub fn im74(&self) -> IM74_R {
        IM74_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - IM75"]
    #[inline(always)]
    pub fn im75(&self) -> IM75_R {
        IM75_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IM64"]
    #[inline(always)]
    #[must_use]
    pub fn im64(&mut self) -> IM64_W<EXTI_C2IMR3rs> {
        IM64_W::new(self, 0)
    }
    #[doc = "Bit 1 - IM65"]
    #[inline(always)]
    #[must_use]
    pub fn im65(&mut self) -> IM65_W<EXTI_C2IMR3rs> {
        IM65_W::new(self, 1)
    }
    #[doc = "Bit 2 - IM66"]
    #[inline(always)]
    #[must_use]
    pub fn im66(&mut self) -> IM66_W<EXTI_C2IMR3rs> {
        IM66_W::new(self, 2)
    }
    #[doc = "Bit 3 - IM67"]
    #[inline(always)]
    #[must_use]
    pub fn im67(&mut self) -> IM67_W<EXTI_C2IMR3rs> {
        IM67_W::new(self, 3)
    }
    #[doc = "Bit 4 - IM68"]
    #[inline(always)]
    #[must_use]
    pub fn im68(&mut self) -> IM68_W<EXTI_C2IMR3rs> {
        IM68_W::new(self, 4)
    }
    #[doc = "Bit 5 - IM69"]
    #[inline(always)]
    #[must_use]
    pub fn im69(&mut self) -> IM69_W<EXTI_C2IMR3rs> {
        IM69_W::new(self, 5)
    }
    #[doc = "Bit 6 - IM70"]
    #[inline(always)]
    #[must_use]
    pub fn im70(&mut self) -> IM70_W<EXTI_C2IMR3rs> {
        IM70_W::new(self, 6)
    }
    #[doc = "Bit 7 - IM71"]
    #[inline(always)]
    #[must_use]
    pub fn im71(&mut self) -> IM71_W<EXTI_C2IMR3rs> {
        IM71_W::new(self, 7)
    }
    #[doc = "Bit 8 - IM72"]
    #[inline(always)]
    #[must_use]
    pub fn im72(&mut self) -> IM72_W<EXTI_C2IMR3rs> {
        IM72_W::new(self, 8)
    }
    #[doc = "Bit 9 - IM73"]
    #[inline(always)]
    #[must_use]
    pub fn im73(&mut self) -> IM73_W<EXTI_C2IMR3rs> {
        IM73_W::new(self, 9)
    }
    #[doc = "Bit 10 - IM74"]
    #[inline(always)]
    #[must_use]
    pub fn im74(&mut self) -> IM74_W<EXTI_C2IMR3rs> {
        IM74_W::new(self, 10)
    }
    #[doc = "Bit 11 - IM75"]
    #[inline(always)]
    #[must_use]
    pub fn im75(&mut self) -> IM75_W<EXTI_C2IMR3rs> {
        IM75_W::new(self, 11)
    }
}
#[doc = "Contains register bits for configurable events and direct events.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_c2imr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_c2imr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTI_C2IMR3rs;
impl crate::RegisterSpec for EXTI_C2IMR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exti_c2imr3::R`](R) reader structure"]
impl crate::Readable for EXTI_C2IMR3rs {}
#[doc = "`write(|w| ..)` method takes [`exti_c2imr3::W`](W) writer structure"]
impl crate::Writable for EXTI_C2IMR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTI_C2IMR3 to value 0x0de9"]
impl crate::Resettable for EXTI_C2IMR3rs {
    const RESET_VALUE: u32 = 0x0de9;
}

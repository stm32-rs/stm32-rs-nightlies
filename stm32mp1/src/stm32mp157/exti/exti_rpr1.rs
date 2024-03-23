#[doc = "Register `EXTI_RPR1` reader"]
pub type R = crate::R<EXTI_RPR1rs>;
#[doc = "Register `EXTI_RPR1` writer"]
pub type W = crate::W<EXTI_RPR1rs>;
#[doc = "Field `RPIF0` reader - RPIF0"]
pub type RPIF0_R = crate::BitReader;
#[doc = "Field `RPIF0` writer - RPIF0"]
pub type RPIF0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPIF1` reader - RPIF1"]
pub type RPIF1_R = crate::BitReader;
#[doc = "Field `RPIF1` writer - RPIF1"]
pub type RPIF1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPIF2` reader - RPIF2"]
pub type RPIF2_R = crate::BitReader;
#[doc = "Field `RPIF2` writer - RPIF2"]
pub type RPIF2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPIF3` reader - RPIF3"]
pub type RPIF3_R = crate::BitReader;
#[doc = "Field `RPIF3` writer - RPIF3"]
pub type RPIF3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPIF4` reader - RPIF4"]
pub type RPIF4_R = crate::BitReader;
#[doc = "Field `RPIF4` writer - RPIF4"]
pub type RPIF4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPIF5` reader - RPIF5"]
pub type RPIF5_R = crate::BitReader;
#[doc = "Field `RPIF5` writer - RPIF5"]
pub type RPIF5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPIF6` reader - RPIF6"]
pub type RPIF6_R = crate::BitReader;
#[doc = "Field `RPIF6` writer - RPIF6"]
pub type RPIF6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPIF7` reader - RPIF7"]
pub type RPIF7_R = crate::BitReader;
#[doc = "Field `RPIF7` writer - RPIF7"]
pub type RPIF7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPIF8` reader - RPIF8"]
pub type RPIF8_R = crate::BitReader;
#[doc = "Field `RPIF8` writer - RPIF8"]
pub type RPIF8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPIF9` reader - RPIF9"]
pub type RPIF9_R = crate::BitReader;
#[doc = "Field `RPIF9` writer - RPIF9"]
pub type RPIF9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPIF10` reader - RPIF10"]
pub type RPIF10_R = crate::BitReader;
#[doc = "Field `RPIF10` writer - RPIF10"]
pub type RPIF10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPIF11` reader - RPIF11"]
pub type RPIF11_R = crate::BitReader;
#[doc = "Field `RPIF11` writer - RPIF11"]
pub type RPIF11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPIF12` reader - RPIF12"]
pub type RPIF12_R = crate::BitReader;
#[doc = "Field `RPIF12` writer - RPIF12"]
pub type RPIF12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPIF13` reader - RPIF13"]
pub type RPIF13_R = crate::BitReader;
#[doc = "Field `RPIF13` writer - RPIF13"]
pub type RPIF13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPIF14` reader - RPIF14"]
pub type RPIF14_R = crate::BitReader;
#[doc = "Field `RPIF14` writer - RPIF14"]
pub type RPIF14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPIF15` reader - RPIF15"]
pub type RPIF15_R = crate::BitReader;
#[doc = "Field `RPIF15` writer - RPIF15"]
pub type RPIF15_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPIF16` reader - RPIF16"]
pub type RPIF16_R = crate::BitReader;
#[doc = "Field `RPIF16` writer - RPIF16"]
pub type RPIF16_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RPIF0"]
    #[inline(always)]
    pub fn rpif0(&self) -> RPIF0_R {
        RPIF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RPIF1"]
    #[inline(always)]
    pub fn rpif1(&self) -> RPIF1_R {
        RPIF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RPIF2"]
    #[inline(always)]
    pub fn rpif2(&self) -> RPIF2_R {
        RPIF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RPIF3"]
    #[inline(always)]
    pub fn rpif3(&self) -> RPIF3_R {
        RPIF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RPIF4"]
    #[inline(always)]
    pub fn rpif4(&self) -> RPIF4_R {
        RPIF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RPIF5"]
    #[inline(always)]
    pub fn rpif5(&self) -> RPIF5_R {
        RPIF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RPIF6"]
    #[inline(always)]
    pub fn rpif6(&self) -> RPIF6_R {
        RPIF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RPIF7"]
    #[inline(always)]
    pub fn rpif7(&self) -> RPIF7_R {
        RPIF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RPIF8"]
    #[inline(always)]
    pub fn rpif8(&self) -> RPIF8_R {
        RPIF8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RPIF9"]
    #[inline(always)]
    pub fn rpif9(&self) -> RPIF9_R {
        RPIF9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RPIF10"]
    #[inline(always)]
    pub fn rpif10(&self) -> RPIF10_R {
        RPIF10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RPIF11"]
    #[inline(always)]
    pub fn rpif11(&self) -> RPIF11_R {
        RPIF11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RPIF12"]
    #[inline(always)]
    pub fn rpif12(&self) -> RPIF12_R {
        RPIF12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - RPIF13"]
    #[inline(always)]
    pub fn rpif13(&self) -> RPIF13_R {
        RPIF13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RPIF14"]
    #[inline(always)]
    pub fn rpif14(&self) -> RPIF14_R {
        RPIF14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - RPIF15"]
    #[inline(always)]
    pub fn rpif15(&self) -> RPIF15_R {
        RPIF15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RPIF16"]
    #[inline(always)]
    pub fn rpif16(&self) -> RPIF16_R {
        RPIF16_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RPIF0"]
    #[inline(always)]
    #[must_use]
    pub fn rpif0(&mut self) -> RPIF0_W<EXTI_RPR1rs> {
        RPIF0_W::new(self, 0)
    }
    #[doc = "Bit 1 - RPIF1"]
    #[inline(always)]
    #[must_use]
    pub fn rpif1(&mut self) -> RPIF1_W<EXTI_RPR1rs> {
        RPIF1_W::new(self, 1)
    }
    #[doc = "Bit 2 - RPIF2"]
    #[inline(always)]
    #[must_use]
    pub fn rpif2(&mut self) -> RPIF2_W<EXTI_RPR1rs> {
        RPIF2_W::new(self, 2)
    }
    #[doc = "Bit 3 - RPIF3"]
    #[inline(always)]
    #[must_use]
    pub fn rpif3(&mut self) -> RPIF3_W<EXTI_RPR1rs> {
        RPIF3_W::new(self, 3)
    }
    #[doc = "Bit 4 - RPIF4"]
    #[inline(always)]
    #[must_use]
    pub fn rpif4(&mut self) -> RPIF4_W<EXTI_RPR1rs> {
        RPIF4_W::new(self, 4)
    }
    #[doc = "Bit 5 - RPIF5"]
    #[inline(always)]
    #[must_use]
    pub fn rpif5(&mut self) -> RPIF5_W<EXTI_RPR1rs> {
        RPIF5_W::new(self, 5)
    }
    #[doc = "Bit 6 - RPIF6"]
    #[inline(always)]
    #[must_use]
    pub fn rpif6(&mut self) -> RPIF6_W<EXTI_RPR1rs> {
        RPIF6_W::new(self, 6)
    }
    #[doc = "Bit 7 - RPIF7"]
    #[inline(always)]
    #[must_use]
    pub fn rpif7(&mut self) -> RPIF7_W<EXTI_RPR1rs> {
        RPIF7_W::new(self, 7)
    }
    #[doc = "Bit 8 - RPIF8"]
    #[inline(always)]
    #[must_use]
    pub fn rpif8(&mut self) -> RPIF8_W<EXTI_RPR1rs> {
        RPIF8_W::new(self, 8)
    }
    #[doc = "Bit 9 - RPIF9"]
    #[inline(always)]
    #[must_use]
    pub fn rpif9(&mut self) -> RPIF9_W<EXTI_RPR1rs> {
        RPIF9_W::new(self, 9)
    }
    #[doc = "Bit 10 - RPIF10"]
    #[inline(always)]
    #[must_use]
    pub fn rpif10(&mut self) -> RPIF10_W<EXTI_RPR1rs> {
        RPIF10_W::new(self, 10)
    }
    #[doc = "Bit 11 - RPIF11"]
    #[inline(always)]
    #[must_use]
    pub fn rpif11(&mut self) -> RPIF11_W<EXTI_RPR1rs> {
        RPIF11_W::new(self, 11)
    }
    #[doc = "Bit 12 - RPIF12"]
    #[inline(always)]
    #[must_use]
    pub fn rpif12(&mut self) -> RPIF12_W<EXTI_RPR1rs> {
        RPIF12_W::new(self, 12)
    }
    #[doc = "Bit 13 - RPIF13"]
    #[inline(always)]
    #[must_use]
    pub fn rpif13(&mut self) -> RPIF13_W<EXTI_RPR1rs> {
        RPIF13_W::new(self, 13)
    }
    #[doc = "Bit 14 - RPIF14"]
    #[inline(always)]
    #[must_use]
    pub fn rpif14(&mut self) -> RPIF14_W<EXTI_RPR1rs> {
        RPIF14_W::new(self, 14)
    }
    #[doc = "Bit 15 - RPIF15"]
    #[inline(always)]
    #[must_use]
    pub fn rpif15(&mut self) -> RPIF15_W<EXTI_RPR1rs> {
        RPIF15_W::new(self, 15)
    }
    #[doc = "Bit 16 - RPIF16"]
    #[inline(always)]
    #[must_use]
    pub fn rpif16(&mut self) -> RPIF16_W<EXTI_RPR1rs> {
        RPIF16_W::new(self, 16)
    }
}
#[doc = "Contains only register bits for configurable events.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_rpr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_rpr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTI_RPR1rs;
impl crate::RegisterSpec for EXTI_RPR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exti_rpr1::R`](R) reader structure"]
impl crate::Readable for EXTI_RPR1rs {}
#[doc = "`write(|w| ..)` method takes [`exti_rpr1::W`](W) writer structure"]
impl crate::Writable for EXTI_RPR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTI_RPR1 to value 0"]
impl crate::Resettable for EXTI_RPR1rs {
    const RESET_VALUE: u32 = 0;
}

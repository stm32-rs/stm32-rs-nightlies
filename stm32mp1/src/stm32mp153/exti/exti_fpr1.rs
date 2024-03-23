#[doc = "Register `EXTI_FPR1` reader"]
pub type R = crate::R<EXTI_FPR1rs>;
#[doc = "Register `EXTI_FPR1` writer"]
pub type W = crate::W<EXTI_FPR1rs>;
#[doc = "Field `FPIF0` reader - FPIF0"]
pub type FPIF0_R = crate::BitReader;
#[doc = "Field `FPIF0` writer - FPIF0"]
pub type FPIF0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIF1` reader - FPIF1"]
pub type FPIF1_R = crate::BitReader;
#[doc = "Field `FPIF1` writer - FPIF1"]
pub type FPIF1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIF2` reader - FPIF2"]
pub type FPIF2_R = crate::BitReader;
#[doc = "Field `FPIF2` writer - FPIF2"]
pub type FPIF2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIF3` reader - FPIF3"]
pub type FPIF3_R = crate::BitReader;
#[doc = "Field `FPIF3` writer - FPIF3"]
pub type FPIF3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIF4` reader - FPIF4"]
pub type FPIF4_R = crate::BitReader;
#[doc = "Field `FPIF4` writer - FPIF4"]
pub type FPIF4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIF5` reader - FPIF5"]
pub type FPIF5_R = crate::BitReader;
#[doc = "Field `FPIF5` writer - FPIF5"]
pub type FPIF5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIF6` reader - FPIF6"]
pub type FPIF6_R = crate::BitReader;
#[doc = "Field `FPIF6` writer - FPIF6"]
pub type FPIF6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIF7` reader - FPIF7"]
pub type FPIF7_R = crate::BitReader;
#[doc = "Field `FPIF7` writer - FPIF7"]
pub type FPIF7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIF8` reader - FPIF8"]
pub type FPIF8_R = crate::BitReader;
#[doc = "Field `FPIF8` writer - FPIF8"]
pub type FPIF8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIF9` reader - FPIF9"]
pub type FPIF9_R = crate::BitReader;
#[doc = "Field `FPIF9` writer - FPIF9"]
pub type FPIF9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIF10` reader - FPIF10"]
pub type FPIF10_R = crate::BitReader;
#[doc = "Field `FPIF10` writer - FPIF10"]
pub type FPIF10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIF11` reader - FPIF11"]
pub type FPIF11_R = crate::BitReader;
#[doc = "Field `FPIF11` writer - FPIF11"]
pub type FPIF11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIF12` reader - FPIF12"]
pub type FPIF12_R = crate::BitReader;
#[doc = "Field `FPIF12` writer - FPIF12"]
pub type FPIF12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIF13` reader - FPIF13"]
pub type FPIF13_R = crate::BitReader;
#[doc = "Field `FPIF13` writer - FPIF13"]
pub type FPIF13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIF14` reader - FPIF14"]
pub type FPIF14_R = crate::BitReader;
#[doc = "Field `FPIF14` writer - FPIF14"]
pub type FPIF14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIF15` reader - FPIF15"]
pub type FPIF15_R = crate::BitReader;
#[doc = "Field `FPIF15` writer - FPIF15"]
pub type FPIF15_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIF16` reader - FPIF16"]
pub type FPIF16_R = crate::BitReader;
#[doc = "Field `FPIF16` writer - FPIF16"]
pub type FPIF16_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - FPIF0"]
    #[inline(always)]
    pub fn fpif0(&self) -> FPIF0_R {
        FPIF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FPIF1"]
    #[inline(always)]
    pub fn fpif1(&self) -> FPIF1_R {
        FPIF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FPIF2"]
    #[inline(always)]
    pub fn fpif2(&self) -> FPIF2_R {
        FPIF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FPIF3"]
    #[inline(always)]
    pub fn fpif3(&self) -> FPIF3_R {
        FPIF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FPIF4"]
    #[inline(always)]
    pub fn fpif4(&self) -> FPIF4_R {
        FPIF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FPIF5"]
    #[inline(always)]
    pub fn fpif5(&self) -> FPIF5_R {
        FPIF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FPIF6"]
    #[inline(always)]
    pub fn fpif6(&self) -> FPIF6_R {
        FPIF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - FPIF7"]
    #[inline(always)]
    pub fn fpif7(&self) -> FPIF7_R {
        FPIF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - FPIF8"]
    #[inline(always)]
    pub fn fpif8(&self) -> FPIF8_R {
        FPIF8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - FPIF9"]
    #[inline(always)]
    pub fn fpif9(&self) -> FPIF9_R {
        FPIF9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - FPIF10"]
    #[inline(always)]
    pub fn fpif10(&self) -> FPIF10_R {
        FPIF10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - FPIF11"]
    #[inline(always)]
    pub fn fpif11(&self) -> FPIF11_R {
        FPIF11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - FPIF12"]
    #[inline(always)]
    pub fn fpif12(&self) -> FPIF12_R {
        FPIF12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - FPIF13"]
    #[inline(always)]
    pub fn fpif13(&self) -> FPIF13_R {
        FPIF13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - FPIF14"]
    #[inline(always)]
    pub fn fpif14(&self) -> FPIF14_R {
        FPIF14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - FPIF15"]
    #[inline(always)]
    pub fn fpif15(&self) -> FPIF15_R {
        FPIF15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - FPIF16"]
    #[inline(always)]
    pub fn fpif16(&self) -> FPIF16_R {
        FPIF16_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FPIF0"]
    #[inline(always)]
    #[must_use]
    pub fn fpif0(&mut self) -> FPIF0_W<EXTI_FPR1rs> {
        FPIF0_W::new(self, 0)
    }
    #[doc = "Bit 1 - FPIF1"]
    #[inline(always)]
    #[must_use]
    pub fn fpif1(&mut self) -> FPIF1_W<EXTI_FPR1rs> {
        FPIF1_W::new(self, 1)
    }
    #[doc = "Bit 2 - FPIF2"]
    #[inline(always)]
    #[must_use]
    pub fn fpif2(&mut self) -> FPIF2_W<EXTI_FPR1rs> {
        FPIF2_W::new(self, 2)
    }
    #[doc = "Bit 3 - FPIF3"]
    #[inline(always)]
    #[must_use]
    pub fn fpif3(&mut self) -> FPIF3_W<EXTI_FPR1rs> {
        FPIF3_W::new(self, 3)
    }
    #[doc = "Bit 4 - FPIF4"]
    #[inline(always)]
    #[must_use]
    pub fn fpif4(&mut self) -> FPIF4_W<EXTI_FPR1rs> {
        FPIF4_W::new(self, 4)
    }
    #[doc = "Bit 5 - FPIF5"]
    #[inline(always)]
    #[must_use]
    pub fn fpif5(&mut self) -> FPIF5_W<EXTI_FPR1rs> {
        FPIF5_W::new(self, 5)
    }
    #[doc = "Bit 6 - FPIF6"]
    #[inline(always)]
    #[must_use]
    pub fn fpif6(&mut self) -> FPIF6_W<EXTI_FPR1rs> {
        FPIF6_W::new(self, 6)
    }
    #[doc = "Bit 7 - FPIF7"]
    #[inline(always)]
    #[must_use]
    pub fn fpif7(&mut self) -> FPIF7_W<EXTI_FPR1rs> {
        FPIF7_W::new(self, 7)
    }
    #[doc = "Bit 8 - FPIF8"]
    #[inline(always)]
    #[must_use]
    pub fn fpif8(&mut self) -> FPIF8_W<EXTI_FPR1rs> {
        FPIF8_W::new(self, 8)
    }
    #[doc = "Bit 9 - FPIF9"]
    #[inline(always)]
    #[must_use]
    pub fn fpif9(&mut self) -> FPIF9_W<EXTI_FPR1rs> {
        FPIF9_W::new(self, 9)
    }
    #[doc = "Bit 10 - FPIF10"]
    #[inline(always)]
    #[must_use]
    pub fn fpif10(&mut self) -> FPIF10_W<EXTI_FPR1rs> {
        FPIF10_W::new(self, 10)
    }
    #[doc = "Bit 11 - FPIF11"]
    #[inline(always)]
    #[must_use]
    pub fn fpif11(&mut self) -> FPIF11_W<EXTI_FPR1rs> {
        FPIF11_W::new(self, 11)
    }
    #[doc = "Bit 12 - FPIF12"]
    #[inline(always)]
    #[must_use]
    pub fn fpif12(&mut self) -> FPIF12_W<EXTI_FPR1rs> {
        FPIF12_W::new(self, 12)
    }
    #[doc = "Bit 13 - FPIF13"]
    #[inline(always)]
    #[must_use]
    pub fn fpif13(&mut self) -> FPIF13_W<EXTI_FPR1rs> {
        FPIF13_W::new(self, 13)
    }
    #[doc = "Bit 14 - FPIF14"]
    #[inline(always)]
    #[must_use]
    pub fn fpif14(&mut self) -> FPIF14_W<EXTI_FPR1rs> {
        FPIF14_W::new(self, 14)
    }
    #[doc = "Bit 15 - FPIF15"]
    #[inline(always)]
    #[must_use]
    pub fn fpif15(&mut self) -> FPIF15_W<EXTI_FPR1rs> {
        FPIF15_W::new(self, 15)
    }
    #[doc = "Bit 16 - FPIF16"]
    #[inline(always)]
    #[must_use]
    pub fn fpif16(&mut self) -> FPIF16_W<EXTI_FPR1rs> {
        FPIF16_W::new(self, 16)
    }
}
#[doc = "Contains only register bits for configurable events.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_fpr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_fpr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTI_FPR1rs;
impl crate::RegisterSpec for EXTI_FPR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exti_fpr1::R`](R) reader structure"]
impl crate::Readable for EXTI_FPR1rs {}
#[doc = "`write(|w| ..)` method takes [`exti_fpr1::W`](W) writer structure"]
impl crate::Writable for EXTI_FPR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTI_FPR1 to value 0"]
impl crate::Resettable for EXTI_FPR1rs {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `IMR` reader"]
pub type R = crate::R<IMRrs>;
#[doc = "Register `IMR` writer"]
pub type W = crate::W<IMRrs>;
#[doc = "Interrupt Mask on line 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM0 {
    #[doc = "0: Interrupt request line is masked"]
    Masked = 0,
    #[doc = "1: Interrupt request line is unmasked"]
    Unmasked = 1,
}
impl From<IM0> for bool {
    #[inline(always)]
    fn from(variant: IM0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM0` reader - Interrupt Mask on line 0"]
pub type IM0_R = crate::BitReader<IM0>;
impl IM0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IM0 {
        match self.bits {
            false => IM0::Masked,
            true => IM0::Unmasked,
        }
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == IM0::Masked
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == IM0::Unmasked
    }
}
#[doc = "Field `IM0` writer - Interrupt Mask on line 0"]
pub type IM0_W<'a, REG> = crate::BitWriter<'a, REG, IM0>;
impl<'a, REG> IM0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(IM0::Masked)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(IM0::Unmasked)
    }
}
#[doc = "Field `IM1` reader - Interrupt Mask on line 1"]
pub use IM0_R as IM1_R;
#[doc = "Field `IM2` reader - Interrupt Mask on line 2"]
pub use IM0_R as IM2_R;
#[doc = "Field `IM3` reader - Interrupt Mask on line 3"]
pub use IM0_R as IM3_R;
#[doc = "Field `IM4` reader - Interrupt Mask on line 4"]
pub use IM0_R as IM4_R;
#[doc = "Field `IM5` reader - Interrupt Mask on line 5"]
pub use IM0_R as IM5_R;
#[doc = "Field `IM6` reader - Interrupt Mask on line 6"]
pub use IM0_R as IM6_R;
#[doc = "Field `IM7` reader - Interrupt Mask on line 7"]
pub use IM0_R as IM7_R;
#[doc = "Field `IM8` reader - Interrupt Mask on line 8"]
pub use IM0_R as IM8_R;
#[doc = "Field `IM9` reader - Interrupt Mask on line 9"]
pub use IM0_R as IM9_R;
#[doc = "Field `IM10` reader - Interrupt Mask on line 10"]
pub use IM0_R as IM10_R;
#[doc = "Field `IM11` reader - Interrupt Mask on line 11"]
pub use IM0_R as IM11_R;
#[doc = "Field `IM12` reader - Interrupt Mask on line 12"]
pub use IM0_R as IM12_R;
#[doc = "Field `IM13` reader - Interrupt Mask on line 13"]
pub use IM0_R as IM13_R;
#[doc = "Field `IM14` reader - Interrupt Mask on line 14"]
pub use IM0_R as IM14_R;
#[doc = "Field `IM15` reader - Interrupt Mask on line 15"]
pub use IM0_R as IM15_R;
#[doc = "Field `IM16` reader - Interrupt Mask on line 16"]
pub use IM0_R as IM16_R;
#[doc = "Field `IM17` reader - Interrupt Mask on line 17"]
pub use IM0_R as IM17_R;
#[doc = "Field `IM18` reader - Interrupt Mask on line 18"]
pub use IM0_R as IM18_R;
#[doc = "Field `IM19` reader - Interrupt Mask on line 19"]
pub use IM0_R as IM19_R;
#[doc = "Field `IM20` reader - Interrupt Mask on line 20"]
pub use IM0_R as IM20_R;
#[doc = "Field `IM21` reader - Interrupt Mask on line 21"]
pub use IM0_R as IM21_R;
#[doc = "Field `IM22` reader - Interrupt Mask on line 22"]
pub use IM0_R as IM22_R;
#[doc = "Field `IM23` reader - Interrupt Mask on line 23"]
pub use IM0_R as IM23_R;
#[doc = "Field `IM1` writer - Interrupt Mask on line 1"]
pub use IM0_W as IM1_W;
#[doc = "Field `IM2` writer - Interrupt Mask on line 2"]
pub use IM0_W as IM2_W;
#[doc = "Field `IM3` writer - Interrupt Mask on line 3"]
pub use IM0_W as IM3_W;
#[doc = "Field `IM4` writer - Interrupt Mask on line 4"]
pub use IM0_W as IM4_W;
#[doc = "Field `IM5` writer - Interrupt Mask on line 5"]
pub use IM0_W as IM5_W;
#[doc = "Field `IM6` writer - Interrupt Mask on line 6"]
pub use IM0_W as IM6_W;
#[doc = "Field `IM7` writer - Interrupt Mask on line 7"]
pub use IM0_W as IM7_W;
#[doc = "Field `IM8` writer - Interrupt Mask on line 8"]
pub use IM0_W as IM8_W;
#[doc = "Field `IM9` writer - Interrupt Mask on line 9"]
pub use IM0_W as IM9_W;
#[doc = "Field `IM10` writer - Interrupt Mask on line 10"]
pub use IM0_W as IM10_W;
#[doc = "Field `IM11` writer - Interrupt Mask on line 11"]
pub use IM0_W as IM11_W;
#[doc = "Field `IM12` writer - Interrupt Mask on line 12"]
pub use IM0_W as IM12_W;
#[doc = "Field `IM13` writer - Interrupt Mask on line 13"]
pub use IM0_W as IM13_W;
#[doc = "Field `IM14` writer - Interrupt Mask on line 14"]
pub use IM0_W as IM14_W;
#[doc = "Field `IM15` writer - Interrupt Mask on line 15"]
pub use IM0_W as IM15_W;
#[doc = "Field `IM16` writer - Interrupt Mask on line 16"]
pub use IM0_W as IM16_W;
#[doc = "Field `IM17` writer - Interrupt Mask on line 17"]
pub use IM0_W as IM17_W;
#[doc = "Field `IM18` writer - Interrupt Mask on line 18"]
pub use IM0_W as IM18_W;
#[doc = "Field `IM19` writer - Interrupt Mask on line 19"]
pub use IM0_W as IM19_W;
#[doc = "Field `IM20` writer - Interrupt Mask on line 20"]
pub use IM0_W as IM20_W;
#[doc = "Field `IM21` writer - Interrupt Mask on line 21"]
pub use IM0_W as IM21_W;
#[doc = "Field `IM22` writer - Interrupt Mask on line 22"]
pub use IM0_W as IM22_W;
#[doc = "Field `IM23` writer - Interrupt Mask on line 23"]
pub use IM0_W as IM23_W;
impl R {
    #[doc = "Bit 0 - Interrupt Mask on line 0"]
    #[inline(always)]
    pub fn im0(&self) -> IM0_R {
        IM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Mask on line 1"]
    #[inline(always)]
    pub fn im1(&self) -> IM1_R {
        IM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt Mask on line 2"]
    #[inline(always)]
    pub fn im2(&self) -> IM2_R {
        IM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Mask on line 3"]
    #[inline(always)]
    pub fn im3(&self) -> IM3_R {
        IM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt Mask on line 4"]
    #[inline(always)]
    pub fn im4(&self) -> IM4_R {
        IM4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt Mask on line 5"]
    #[inline(always)]
    pub fn im5(&self) -> IM5_R {
        IM5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt Mask on line 6"]
    #[inline(always)]
    pub fn im6(&self) -> IM6_R {
        IM6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt Mask on line 7"]
    #[inline(always)]
    pub fn im7(&self) -> IM7_R {
        IM7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt Mask on line 8"]
    #[inline(always)]
    pub fn im8(&self) -> IM8_R {
        IM8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt Mask on line 9"]
    #[inline(always)]
    pub fn im9(&self) -> IM9_R {
        IM9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt Mask on line 10"]
    #[inline(always)]
    pub fn im10(&self) -> IM10_R {
        IM10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt Mask on line 11"]
    #[inline(always)]
    pub fn im11(&self) -> IM11_R {
        IM11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt Mask on line 12"]
    #[inline(always)]
    pub fn im12(&self) -> IM12_R {
        IM12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt Mask on line 13"]
    #[inline(always)]
    pub fn im13(&self) -> IM13_R {
        IM13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt Mask on line 14"]
    #[inline(always)]
    pub fn im14(&self) -> IM14_R {
        IM14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt Mask on line 15"]
    #[inline(always)]
    pub fn im15(&self) -> IM15_R {
        IM15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt Mask on line 16"]
    #[inline(always)]
    pub fn im16(&self) -> IM16_R {
        IM16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt Mask on line 17"]
    #[inline(always)]
    pub fn im17(&self) -> IM17_R {
        IM17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt Mask on line 18"]
    #[inline(always)]
    pub fn im18(&self) -> IM18_R {
        IM18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt Mask on line 19"]
    #[inline(always)]
    pub fn im19(&self) -> IM19_R {
        IM19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Interrupt Mask on line 20"]
    #[inline(always)]
    pub fn im20(&self) -> IM20_R {
        IM20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt Mask on line 21"]
    #[inline(always)]
    pub fn im21(&self) -> IM21_R {
        IM21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Interrupt Mask on line 22"]
    #[inline(always)]
    pub fn im22(&self) -> IM22_R {
        IM22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt Mask on line 23"]
    #[inline(always)]
    pub fn im23(&self) -> IM23_R {
        IM23_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Mask on line 0"]
    #[inline(always)]
    #[must_use]
    pub fn im0(&mut self) -> IM0_W<IMRrs> {
        IM0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Interrupt Mask on line 1"]
    #[inline(always)]
    #[must_use]
    pub fn im1(&mut self) -> IM1_W<IMRrs> {
        IM1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt Mask on line 2"]
    #[inline(always)]
    #[must_use]
    pub fn im2(&mut self) -> IM2_W<IMRrs> {
        IM2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt Mask on line 3"]
    #[inline(always)]
    #[must_use]
    pub fn im3(&mut self) -> IM3_W<IMRrs> {
        IM3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Interrupt Mask on line 4"]
    #[inline(always)]
    #[must_use]
    pub fn im4(&mut self) -> IM4_W<IMRrs> {
        IM4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt Mask on line 5"]
    #[inline(always)]
    #[must_use]
    pub fn im5(&mut self) -> IM5_W<IMRrs> {
        IM5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Interrupt Mask on line 6"]
    #[inline(always)]
    #[must_use]
    pub fn im6(&mut self) -> IM6_W<IMRrs> {
        IM6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Interrupt Mask on line 7"]
    #[inline(always)]
    #[must_use]
    pub fn im7(&mut self) -> IM7_W<IMRrs> {
        IM7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Interrupt Mask on line 8"]
    #[inline(always)]
    #[must_use]
    pub fn im8(&mut self) -> IM8_W<IMRrs> {
        IM8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Interrupt Mask on line 9"]
    #[inline(always)]
    #[must_use]
    pub fn im9(&mut self) -> IM9_W<IMRrs> {
        IM9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Interrupt Mask on line 10"]
    #[inline(always)]
    #[must_use]
    pub fn im10(&mut self) -> IM10_W<IMRrs> {
        IM10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Interrupt Mask on line 11"]
    #[inline(always)]
    #[must_use]
    pub fn im11(&mut self) -> IM11_W<IMRrs> {
        IM11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Interrupt Mask on line 12"]
    #[inline(always)]
    #[must_use]
    pub fn im12(&mut self) -> IM12_W<IMRrs> {
        IM12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Interrupt Mask on line 13"]
    #[inline(always)]
    #[must_use]
    pub fn im13(&mut self) -> IM13_W<IMRrs> {
        IM13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Interrupt Mask on line 14"]
    #[inline(always)]
    #[must_use]
    pub fn im14(&mut self) -> IM14_W<IMRrs> {
        IM14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Interrupt Mask on line 15"]
    #[inline(always)]
    #[must_use]
    pub fn im15(&mut self) -> IM15_W<IMRrs> {
        IM15_W::new(self, 15)
    }
    #[doc = "Bit 16 - Interrupt Mask on line 16"]
    #[inline(always)]
    #[must_use]
    pub fn im16(&mut self) -> IM16_W<IMRrs> {
        IM16_W::new(self, 16)
    }
    #[doc = "Bit 17 - Interrupt Mask on line 17"]
    #[inline(always)]
    #[must_use]
    pub fn im17(&mut self) -> IM17_W<IMRrs> {
        IM17_W::new(self, 17)
    }
    #[doc = "Bit 18 - Interrupt Mask on line 18"]
    #[inline(always)]
    #[must_use]
    pub fn im18(&mut self) -> IM18_W<IMRrs> {
        IM18_W::new(self, 18)
    }
    #[doc = "Bit 19 - Interrupt Mask on line 19"]
    #[inline(always)]
    #[must_use]
    pub fn im19(&mut self) -> IM19_W<IMRrs> {
        IM19_W::new(self, 19)
    }
    #[doc = "Bit 20 - Interrupt Mask on line 20"]
    #[inline(always)]
    #[must_use]
    pub fn im20(&mut self) -> IM20_W<IMRrs> {
        IM20_W::new(self, 20)
    }
    #[doc = "Bit 21 - Interrupt Mask on line 21"]
    #[inline(always)]
    #[must_use]
    pub fn im21(&mut self) -> IM21_W<IMRrs> {
        IM21_W::new(self, 21)
    }
    #[doc = "Bit 22 - Interrupt Mask on line 22"]
    #[inline(always)]
    #[must_use]
    pub fn im22(&mut self) -> IM22_W<IMRrs> {
        IM22_W::new(self, 22)
    }
    #[doc = "Bit 23 - Interrupt Mask on line 23"]
    #[inline(always)]
    #[must_use]
    pub fn im23(&mut self) -> IM23_W<IMRrs> {
        IM23_W::new(self, 23)
    }
}
#[doc = "Interrupt mask register (EXTI_IMR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IMRrs;
impl crate::RegisterSpec for IMRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for IMRrs {}
#[doc = "`write(|w| ..)` method takes [`imr::W`](W) writer structure"]
impl crate::Writable for IMRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for IMRrs {
    const RESET_VALUE: u32 = 0;
}

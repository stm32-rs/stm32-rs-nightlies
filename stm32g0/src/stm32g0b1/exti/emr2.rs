#[doc = "Register `EMR2` reader"]
pub type R = crate::R<EMR2rs>;
#[doc = "Register `EMR2` writer"]
pub type W = crate::W<EMR2rs>;
#[doc = "CPU wakeup with event mask on event input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM32 {
    #[doc = "0: Interrupt request line is masked"]
    Masked = 0,
    #[doc = "1: Interrupt request line is unmasked"]
    Unmasked = 1,
}
impl From<EM32> for bool {
    #[inline(always)]
    fn from(variant: EM32) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM32` reader - CPU wakeup with event mask on event input"]
pub type EM32_R = crate::BitReader<EM32>;
impl EM32_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EM32 {
        match self.bits {
            false => EM32::Masked,
            true => EM32::Unmasked,
        }
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == EM32::Masked
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == EM32::Unmasked
    }
}
#[doc = "Field `EM32` writer - CPU wakeup with event mask on event input"]
pub type EM32_W<'a, REG> = crate::BitWriter<'a, REG, EM32>;
impl<'a, REG> EM32_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(EM32::Masked)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(EM32::Unmasked)
    }
}
#[doc = "Field `EM33` reader - CPU wakeup with event mask on event input"]
pub use EM32_R as EM33_R;
#[doc = "Field `EM34` reader - CPU wakeup with event mask on event input"]
pub use EM32_R as EM34_R;
#[doc = "Field `EM35` reader - CPU wakeup with event mask on event input"]
pub use EM32_R as EM35_R;
#[doc = "Field `EM33` writer - CPU wakeup with event mask on event input"]
pub use EM32_W as EM33_W;
#[doc = "Field `EM34` writer - CPU wakeup with event mask on event input"]
pub use EM32_W as EM34_W;
#[doc = "Field `EM35` writer - CPU wakeup with event mask on event input"]
pub use EM32_W as EM35_W;
impl R {
    #[doc = "Bit 0 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em32(&self) -> EM32_R {
        EM32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em33(&self) -> EM33_R {
        EM33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em34(&self) -> EM34_R {
        EM34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em35(&self) -> EM35_R {
        EM35_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn em32(&mut self) -> EM32_W<EMR2rs> {
        EM32_W::new(self, 0)
    }
    #[doc = "Bit 1 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn em33(&mut self) -> EM33_W<EMR2rs> {
        EM33_W::new(self, 1)
    }
    #[doc = "Bit 2 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn em34(&mut self) -> EM34_W<EMR2rs> {
        EM34_W::new(self, 2)
    }
    #[doc = "Bit 3 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn em35(&mut self) -> EM35_W<EMR2rs> {
        EM35_W::new(self, 3)
    }
}
#[doc = "EXTI CPU wakeup with event mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMR2rs;
impl crate::RegisterSpec for EMR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emr2::R`](R) reader structure"]
impl crate::Readable for EMR2rs {}
#[doc = "`write(|w| ..)` method takes [`emr2::W`](W) writer structure"]
impl crate::Writable for EMR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EMR2 to value 0"]
impl crate::Resettable for EMR2rs {
    const RESET_VALUE: u32 = 0;
}

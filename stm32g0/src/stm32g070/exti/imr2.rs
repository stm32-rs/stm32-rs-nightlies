#[doc = "Register `IMR2` reader"]
pub type R = crate::R<IMR2rs>;
#[doc = "Register `IMR2` writer"]
pub type W = crate::W<IMR2rs>;
#[doc = "CPU wakeup with interrupt mask on event input\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM32 {
    #[doc = "0: Interrupt request line is masked"]
    Masked = 0,
    #[doc = "1: Interrupt request line is unmasked"]
    Unmasked = 1,
}
impl From<IM32> for bool {
    #[inline(always)]
    fn from(variant: IM32) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM32` reader - CPU wakeup with interrupt mask on event input"]
pub type IM32_R = crate::BitReader<IM32>;
impl IM32_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IM32 {
        match self.bits {
            false => IM32::Masked,
            true => IM32::Unmasked,
        }
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == IM32::Masked
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == IM32::Unmasked
    }
}
#[doc = "Field `IM32` writer - CPU wakeup with interrupt mask on event input"]
pub type IM32_W<'a, REG> = crate::BitWriter<'a, REG, IM32>;
impl<'a, REG> IM32_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(IM32::Masked)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(IM32::Unmasked)
    }
}
#[doc = "Field `IM33` reader - CPU wakeup with interrupt mask on event input"]
pub use IM32_R as IM33_R;
#[doc = "Field `IM33` writer - CPU wakeup with interrupt mask on event input"]
pub use IM32_W as IM33_W;
impl R {
    #[doc = "Bit 0 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im32(&self) -> IM32_R {
        IM32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im33(&self) -> IM33_R {
        IM33_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn im32(&mut self) -> IM32_W<IMR2rs> {
        IM32_W::new(self, 0)
    }
    #[doc = "Bit 1 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn im33(&mut self) -> IM33_W<IMR2rs> {
        IM33_W::new(self, 1)
    }
}
#[doc = "EXTI CPU wakeup with interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IMR2rs;
impl crate::RegisterSpec for IMR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr2::R`](R) reader structure"]
impl crate::Readable for IMR2rs {}
#[doc = "`write(|w| ..)` method takes [`imr2::W`](W) writer structure"]
impl crate::Writable for IMR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMR2 to value 0xffff_ffff"]
impl crate::Resettable for IMR2rs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}

#[doc = "Register `RTSR2` reader"]
pub type R = crate::R<RTSR2rs>;
#[doc = "Register `RTSR2` writer"]
pub type W = crate::W<RTSR2rs>;
#[doc = "Rising trigger event configuration bit of line 32\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RT32 {
    #[doc = "0: Rising edge trigger is disabled"]
    Disabled = 0,
    #[doc = "1: Rising edge trigger is enabled"]
    Enabled = 1,
}
impl From<RT32> for bool {
    #[inline(always)]
    fn from(variant: RT32) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT32` reader - Rising trigger event configuration bit of line 32"]
pub type RT32_R = crate::BitReader<RT32>;
impl RT32_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RT32 {
        match self.bits {
            false => RT32::Disabled,
            true => RT32::Enabled,
        }
    }
    #[doc = "Rising edge trigger is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RT32::Disabled
    }
    #[doc = "Rising edge trigger is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RT32::Enabled
    }
}
#[doc = "Field `RT32` writer - Rising trigger event configuration bit of line 32"]
pub type RT32_W<'a, REG> = crate::BitWriter<'a, REG, RT32>;
impl<'a, REG> RT32_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rising edge trigger is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RT32::Disabled)
    }
    #[doc = "Rising edge trigger is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RT32::Enabled)
    }
}
#[doc = "Field `RT33` reader - Rising trigger event configuration bit of line 32"]
pub use RT32_R as RT33_R;
#[doc = "Field `RT40` reader - Rising trigger event configuration bit of line 40"]
pub use RT32_R as RT40_R;
#[doc = "Field `RT41` reader - Rising trigger event configuration bit of line 41"]
pub use RT32_R as RT41_R;
#[doc = "Field `RT33` writer - Rising trigger event configuration bit of line 32"]
pub use RT32_W as RT33_W;
#[doc = "Field `RT40` writer - Rising trigger event configuration bit of line 40"]
pub use RT32_W as RT40_W;
#[doc = "Field `RT41` writer - Rising trigger event configuration bit of line 41"]
pub use RT32_W as RT41_W;
impl R {
    #[doc = "Bit 0 - Rising trigger event configuration bit of line 32"]
    #[inline(always)]
    pub fn rt32(&self) -> RT32_R {
        RT32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rising trigger event configuration bit of line 32"]
    #[inline(always)]
    pub fn rt33(&self) -> RT33_R {
        RT33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Rising trigger event configuration bit of line 40"]
    #[inline(always)]
    pub fn rt40(&self) -> RT40_R {
        RT40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Rising trigger event configuration bit of line 41"]
    #[inline(always)]
    pub fn rt41(&self) -> RT41_R {
        RT41_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rising trigger event configuration bit of line 32"]
    #[inline(always)]
    #[must_use]
    pub fn rt32(&mut self) -> RT32_W<RTSR2rs> {
        RT32_W::new(self, 0)
    }
    #[doc = "Bit 1 - Rising trigger event configuration bit of line 32"]
    #[inline(always)]
    #[must_use]
    pub fn rt33(&mut self) -> RT33_W<RTSR2rs> {
        RT33_W::new(self, 1)
    }
    #[doc = "Bit 8 - Rising trigger event configuration bit of line 40"]
    #[inline(always)]
    #[must_use]
    pub fn rt40(&mut self) -> RT40_W<RTSR2rs> {
        RT40_W::new(self, 8)
    }
    #[doc = "Bit 9 - Rising trigger event configuration bit of line 41"]
    #[inline(always)]
    #[must_use]
    pub fn rt41(&mut self) -> RT41_W<RTSR2rs> {
        RT41_W::new(self, 9)
    }
}
#[doc = "Rising Trigger selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtsr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtsr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTSR2rs;
impl crate::RegisterSpec for RTSR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtsr2::R`](R) reader structure"]
impl crate::Readable for RTSR2rs {}
#[doc = "`write(|w| ..)` method takes [`rtsr2::W`](W) writer structure"]
impl crate::Writable for RTSR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTSR2 to value 0"]
impl crate::Resettable for RTSR2rs {
    const RESET_VALUE: u32 = 0;
}

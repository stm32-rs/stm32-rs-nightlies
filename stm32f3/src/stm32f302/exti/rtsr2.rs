#[doc = "Register `RTSR2` reader"]
pub type R = crate::R<RTSR2rs>;
#[doc = "Register `RTSR2` writer"]
pub type W = crate::W<RTSR2rs>;
#[doc = "Rising trigger event configuration bit of line 32\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TR32 {
    #[doc = "0: Rising edge trigger is disabled"]
    Disabled = 0,
    #[doc = "1: Rising edge trigger is enabled"]
    Enabled = 1,
}
impl From<TR32> for bool {
    #[inline(always)]
    fn from(variant: TR32) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TR32` reader - Rising trigger event configuration bit of line 32"]
pub type TR32_R = crate::BitReader<TR32>;
impl TR32_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TR32 {
        match self.bits {
            false => TR32::Disabled,
            true => TR32::Enabled,
        }
    }
    #[doc = "Rising edge trigger is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TR32::Disabled
    }
    #[doc = "Rising edge trigger is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TR32::Enabled
    }
}
#[doc = "Field `TR32` writer - Rising trigger event configuration bit of line 32"]
pub type TR32_W<'a, REG> = crate::BitWriter<'a, REG, TR32>;
impl<'a, REG> TR32_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rising edge trigger is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TR32::Disabled)
    }
    #[doc = "Rising edge trigger is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TR32::Enabled)
    }
}
#[doc = "Field `TR33` reader - Rising trigger event configuration bit of line 33"]
pub use TR32_R as TR33_R;
#[doc = "Field `TR33` writer - Rising trigger event configuration bit of line 33"]
pub use TR32_W as TR33_W;
impl R {
    #[doc = "Bit 0 - Rising trigger event configuration bit of line 32"]
    #[inline(always)]
    pub fn tr32(&self) -> TR32_R {
        TR32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rising trigger event configuration bit of line 33"]
    #[inline(always)]
    pub fn tr33(&self) -> TR33_R {
        TR33_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rising trigger event configuration bit of line 32"]
    #[inline(always)]
    #[must_use]
    pub fn tr32(&mut self) -> TR32_W<RTSR2rs> {
        TR32_W::new(self, 0)
    }
    #[doc = "Bit 1 - Rising trigger event configuration bit of line 33"]
    #[inline(always)]
    #[must_use]
    pub fn tr33(&mut self) -> TR33_W<RTSR2rs> {
        TR33_W::new(self, 1)
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

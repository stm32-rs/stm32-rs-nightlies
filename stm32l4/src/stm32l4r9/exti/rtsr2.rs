#[doc = "Register `RTSR2` reader"]
pub type R = crate::R<RTSR2rs>;
#[doc = "Register `RTSR2` writer"]
pub type W = crate::W<RTSR2rs>;
#[doc = "Rising trigger event configuration bit of line 35\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RT35 {
    #[doc = "0: Rising edge trigger is disabled"]
    Disabled = 0,
    #[doc = "1: Rising edge trigger is enabled"]
    Enabled = 1,
}
impl From<RT35> for bool {
    #[inline(always)]
    fn from(variant: RT35) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT35` reader - Rising trigger event configuration bit of line 35"]
pub type RT35_R = crate::BitReader<RT35>;
impl RT35_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RT35 {
        match self.bits {
            false => RT35::Disabled,
            true => RT35::Enabled,
        }
    }
    #[doc = "Rising edge trigger is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RT35::Disabled
    }
    #[doc = "Rising edge trigger is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RT35::Enabled
    }
}
#[doc = "Field `RT35` writer - Rising trigger event configuration bit of line 35"]
pub type RT35_W<'a, REG> = crate::BitWriter<'a, REG, RT35>;
impl<'a, REG> RT35_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rising edge trigger is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RT35::Disabled)
    }
    #[doc = "Rising edge trigger is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RT35::Enabled)
    }
}
#[doc = "Field `RT36` reader - Rising trigger event configuration bit of line 36"]
pub use RT35_R as RT36_R;
#[doc = "Field `RT37` reader - Rising trigger event configuration bit of line 37"]
pub use RT35_R as RT37_R;
#[doc = "Field `RT38` reader - Rising trigger event configuration bit of line 38"]
pub use RT35_R as RT38_R;
#[doc = "Field `RT36` writer - Rising trigger event configuration bit of line 36"]
pub use RT35_W as RT36_W;
#[doc = "Field `RT37` writer - Rising trigger event configuration bit of line 37"]
pub use RT35_W as RT37_W;
#[doc = "Field `RT38` writer - Rising trigger event configuration bit of line 38"]
pub use RT35_W as RT38_W;
impl R {
    #[doc = "Bit 3 - Rising trigger event configuration bit of line 35"]
    #[inline(always)]
    pub fn rt35(&self) -> RT35_R {
        RT35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rising trigger event configuration bit of line 36"]
    #[inline(always)]
    pub fn rt36(&self) -> RT36_R {
        RT36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rising trigger event configuration bit of line 37"]
    #[inline(always)]
    pub fn rt37(&self) -> RT37_R {
        RT37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rising trigger event configuration bit of line 38"]
    #[inline(always)]
    pub fn rt38(&self) -> RT38_R {
        RT38_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Rising trigger event configuration bit of line 35"]
    #[inline(always)]
    #[must_use]
    pub fn rt35(&mut self) -> RT35_W<RTSR2rs> {
        RT35_W::new(self, 3)
    }
    #[doc = "Bit 4 - Rising trigger event configuration bit of line 36"]
    #[inline(always)]
    #[must_use]
    pub fn rt36(&mut self) -> RT36_W<RTSR2rs> {
        RT36_W::new(self, 4)
    }
    #[doc = "Bit 5 - Rising trigger event configuration bit of line 37"]
    #[inline(always)]
    #[must_use]
    pub fn rt37(&mut self) -> RT37_W<RTSR2rs> {
        RT37_W::new(self, 5)
    }
    #[doc = "Bit 6 - Rising trigger event configuration bit of line 38"]
    #[inline(always)]
    #[must_use]
    pub fn rt38(&mut self) -> RT38_W<RTSR2rs> {
        RT38_W::new(self, 6)
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

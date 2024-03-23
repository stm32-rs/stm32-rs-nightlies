#[doc = "Register `RTSR2` reader"]
pub type R = crate::R<RTSR2rs>;
#[doc = "Register `RTSR2` writer"]
pub type W = crate::W<RTSR2rs>;
#[doc = "Rising trigger event configuration bit of Configurable Event input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RT34 {
    #[doc = "0: Rising edge trigger is disabled"]
    Disabled = 0,
    #[doc = "1: Rising edge trigger is enabled"]
    Enabled = 1,
}
impl From<RT34> for bool {
    #[inline(always)]
    fn from(variant: RT34) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT34` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type RT34_R = crate::BitReader<RT34>;
impl RT34_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RT34 {
        match self.bits {
            false => RT34::Disabled,
            true => RT34::Enabled,
        }
    }
    #[doc = "Rising edge trigger is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RT34::Disabled
    }
    #[doc = "Rising edge trigger is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RT34::Enabled
    }
}
#[doc = "Field `RT34` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type RT34_W<'a, REG> = crate::BitWriter<'a, REG, RT34>;
impl<'a, REG> RT34_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rising edge trigger is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RT34::Disabled)
    }
    #[doc = "Rising edge trigger is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RT34::Enabled)
    }
}
#[doc = "Field `RT40` reader - Rising trigger event configuration bit of Configurable Event input"]
pub use RT34_R as RT40_R;
#[doc = "Field `RT41` reader - Rising trigger event configuration bit of Configurable Event input"]
pub use RT34_R as RT41_R;
#[doc = "Field `RT45` reader - Rising trigger event configuration bit of Configurable Event input"]
pub use RT34_R as RT45_R;
#[doc = "Field `RT40` writer - Rising trigger event configuration bit of Configurable Event input"]
pub use RT34_W as RT40_W;
#[doc = "Field `RT41` writer - Rising trigger event configuration bit of Configurable Event input"]
pub use RT34_W as RT41_W;
#[doc = "Field `RT45` writer - Rising trigger event configuration bit of Configurable Event input"]
pub use RT34_W as RT45_W;
impl R {
    #[doc = "Bit 2 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn rt34(&self) -> RT34_R {
        RT34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn rt40(&self) -> RT40_R {
        RT40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn rt41(&self) -> RT41_R {
        RT41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn rt45(&self) -> RT45_R {
        RT45_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    #[must_use]
    pub fn rt34(&mut self) -> RT34_W<RTSR2rs> {
        RT34_W::new(self, 2)
    }
    #[doc = "Bit 8 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    #[must_use]
    pub fn rt40(&mut self) -> RT40_W<RTSR2rs> {
        RT40_W::new(self, 8)
    }
    #[doc = "Bit 9 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    #[must_use]
    pub fn rt41(&mut self) -> RT41_W<RTSR2rs> {
        RT41_W::new(self, 9)
    }
    #[doc = "Bit 13 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    #[must_use]
    pub fn rt45(&mut self) -> RT45_W<RTSR2rs> {
        RT45_W::new(self, 13)
    }
}
#[doc = "rising trigger selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtsr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtsr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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

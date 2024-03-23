#[doc = "Register `RTSR3` reader"]
pub type R = crate::R<RTSR3rs>;
#[doc = "Register `RTSR3` writer"]
pub type W = crate::W<RTSR3rs>;
#[doc = "Rising trigger event configuration bit of Configurable Event input x+64\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TR82 {
    #[doc = "0: Rising edge trigger is disabled"]
    Disabled = 0,
    #[doc = "1: Rising edge trigger is enabled"]
    Enabled = 1,
}
impl From<TR82> for bool {
    #[inline(always)]
    fn from(variant: TR82) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TR82` reader - Rising trigger event configuration bit of Configurable Event input x+64"]
pub type TR82_R = crate::BitReader<TR82>;
impl TR82_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TR82 {
        match self.bits {
            false => TR82::Disabled,
            true => TR82::Enabled,
        }
    }
    #[doc = "Rising edge trigger is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TR82::Disabled
    }
    #[doc = "Rising edge trigger is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TR82::Enabled
    }
}
#[doc = "Field `TR82` writer - Rising trigger event configuration bit of Configurable Event input x+64"]
pub type TR82_W<'a, REG> = crate::BitWriter<'a, REG, TR82>;
impl<'a, REG> TR82_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rising edge trigger is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TR82::Disabled)
    }
    #[doc = "Rising edge trigger is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TR82::Enabled)
    }
}
#[doc = "Field `TR84` reader - Rising trigger event configuration bit of Configurable Event input x+64"]
pub use TR82_R as TR84_R;
#[doc = "Field `TR85` reader - Rising trigger event configuration bit of Configurable Event input x+64"]
pub use TR82_R as TR85_R;
#[doc = "Field `TR86` reader - Rising trigger event configuration bit of Configurable Event input x+64"]
pub use TR82_R as TR86_R;
#[doc = "Field `TR84` writer - Rising trigger event configuration bit of Configurable Event input x+64"]
pub use TR82_W as TR84_W;
#[doc = "Field `TR85` writer - Rising trigger event configuration bit of Configurable Event input x+64"]
pub use TR82_W as TR85_W;
#[doc = "Field `TR86` writer - Rising trigger event configuration bit of Configurable Event input x+64"]
pub use TR82_W as TR86_W;
impl R {
    #[doc = "Bit 18 - Rising trigger event configuration bit of Configurable Event input x+64"]
    #[inline(always)]
    pub fn tr82(&self) -> TR82_R {
        TR82_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Rising trigger event configuration bit of Configurable Event input x+64"]
    #[inline(always)]
    pub fn tr84(&self) -> TR84_R {
        TR84_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Rising trigger event configuration bit of Configurable Event input x+64"]
    #[inline(always)]
    pub fn tr85(&self) -> TR85_R {
        TR85_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Rising trigger event configuration bit of Configurable Event input x+64"]
    #[inline(always)]
    pub fn tr86(&self) -> TR86_R {
        TR86_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - Rising trigger event configuration bit of Configurable Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn tr82(&mut self) -> TR82_W<RTSR3rs> {
        TR82_W::new(self, 18)
    }
    #[doc = "Bit 20 - Rising trigger event configuration bit of Configurable Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn tr84(&mut self) -> TR84_W<RTSR3rs> {
        TR84_W::new(self, 20)
    }
    #[doc = "Bit 21 - Rising trigger event configuration bit of Configurable Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn tr85(&mut self) -> TR85_W<RTSR3rs> {
        TR85_W::new(self, 21)
    }
    #[doc = "Bit 22 - Rising trigger event configuration bit of Configurable Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn tr86(&mut self) -> TR86_W<RTSR3rs> {
        TR86_W::new(self, 22)
    }
}
#[doc = "EXTI rising trigger selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtsr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtsr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTSR3rs;
impl crate::RegisterSpec for RTSR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtsr3::R`](R) reader structure"]
impl crate::Readable for RTSR3rs {}
#[doc = "`write(|w| ..)` method takes [`rtsr3::W`](W) writer structure"]
impl crate::Writable for RTSR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTSR3 to value 0"]
impl crate::Resettable for RTSR3rs {
    const RESET_VALUE: u32 = 0;
}

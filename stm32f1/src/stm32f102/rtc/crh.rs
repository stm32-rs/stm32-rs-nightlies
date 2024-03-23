#[doc = "Register `CRH` reader"]
pub type R = crate::R<CRHrs>;
#[doc = "Register `CRH` writer"]
pub type W = crate::W<CRHrs>;
#[doc = "Second interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SECIE {
    #[doc = "0: Second interrupt is masked"]
    Disabled = 0,
    #[doc = "1: Second interrupt is enabled"]
    Enabled = 1,
}
impl From<SECIE> for bool {
    #[inline(always)]
    fn from(variant: SECIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SECIE` reader - Second interrupt Enable"]
pub type SECIE_R = crate::BitReader<SECIE>;
impl SECIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SECIE {
        match self.bits {
            false => SECIE::Disabled,
            true => SECIE::Enabled,
        }
    }
    #[doc = "Second interrupt is masked"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SECIE::Disabled
    }
    #[doc = "Second interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SECIE::Enabled
    }
}
#[doc = "Field `SECIE` writer - Second interrupt Enable"]
pub type SECIE_W<'a, REG> = crate::BitWriter<'a, REG, SECIE>;
impl<'a, REG> SECIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Second interrupt is masked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SECIE::Disabled)
    }
    #[doc = "Second interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SECIE::Enabled)
    }
}
#[doc = "Alarm interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALRIE {
    #[doc = "0: Alarm interrupt is masked"]
    Disabled = 0,
    #[doc = "1: Alarm interrupt is enabled"]
    Enabled = 1,
}
impl From<ALRIE> for bool {
    #[inline(always)]
    fn from(variant: ALRIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALRIE` reader - Alarm interrupt Enable"]
pub type ALRIE_R = crate::BitReader<ALRIE>;
impl ALRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ALRIE {
        match self.bits {
            false => ALRIE::Disabled,
            true => ALRIE::Enabled,
        }
    }
    #[doc = "Alarm interrupt is masked"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ALRIE::Disabled
    }
    #[doc = "Alarm interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ALRIE::Enabled
    }
}
#[doc = "Field `ALRIE` writer - Alarm interrupt Enable"]
pub type ALRIE_W<'a, REG> = crate::BitWriter<'a, REG, ALRIE>;
impl<'a, REG> ALRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Alarm interrupt is masked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ALRIE::Disabled)
    }
    #[doc = "Alarm interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ALRIE::Enabled)
    }
}
#[doc = "Overflow interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OWIE {
    #[doc = "0: Overflow interrupt is masked"]
    Disabled = 0,
    #[doc = "1: Overflow interrupt is enabled"]
    Enabled = 1,
}
impl From<OWIE> for bool {
    #[inline(always)]
    fn from(variant: OWIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OWIE` reader - Overflow interrupt Enable"]
pub type OWIE_R = crate::BitReader<OWIE>;
impl OWIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OWIE {
        match self.bits {
            false => OWIE::Disabled,
            true => OWIE::Enabled,
        }
    }
    #[doc = "Overflow interrupt is masked"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OWIE::Disabled
    }
    #[doc = "Overflow interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OWIE::Enabled
    }
}
#[doc = "Field `OWIE` writer - Overflow interrupt Enable"]
pub type OWIE_W<'a, REG> = crate::BitWriter<'a, REG, OWIE>;
impl<'a, REG> OWIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Overflow interrupt is masked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OWIE::Disabled)
    }
    #[doc = "Overflow interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OWIE::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Second interrupt Enable"]
    #[inline(always)]
    pub fn secie(&self) -> SECIE_R {
        SECIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm interrupt Enable"]
    #[inline(always)]
    pub fn alrie(&self) -> ALRIE_R {
        ALRIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Overflow interrupt Enable"]
    #[inline(always)]
    pub fn owie(&self) -> OWIE_R {
        OWIE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Second interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn secie(&mut self) -> SECIE_W<CRHrs> {
        SECIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Alarm interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn alrie(&mut self) -> ALRIE_W<CRHrs> {
        ALRIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Overflow interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn owie(&mut self) -> OWIE_W<CRHrs> {
        OWIE_W::new(self, 2)
    }
}
#[doc = "RTC Control Register High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRHrs;
impl crate::RegisterSpec for CRHrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crh::R`](R) reader structure"]
impl crate::Readable for CRHrs {}
#[doc = "`write(|w| ..)` method takes [`crh::W`](W) writer structure"]
impl crate::Writable for CRHrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRH to value 0"]
impl crate::Resettable for CRHrs {
    const RESET_VALUE: u32 = 0;
}

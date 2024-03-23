#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Random number generator enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RNGEN {
    #[doc = "0: Random number generator is disabled"]
    Disabled = 0,
    #[doc = "1: Random number generator is enabled"]
    Enabled = 1,
}
impl From<RNGEN> for bool {
    #[inline(always)]
    fn from(variant: RNGEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RNGEN` reader - Random number generator enable"]
pub type RNGEN_R = crate::BitReader<RNGEN>;
impl RNGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RNGEN {
        match self.bits {
            false => RNGEN::Disabled,
            true => RNGEN::Enabled,
        }
    }
    #[doc = "Random number generator is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RNGEN::Disabled
    }
    #[doc = "Random number generator is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RNGEN::Enabled
    }
}
#[doc = "Field `RNGEN` writer - Random number generator enable"]
pub type RNGEN_W<'a, REG> = crate::BitWriter<'a, REG, RNGEN>;
impl<'a, REG> RNGEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Random number generator is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RNGEN::Disabled)
    }
    #[doc = "Random number generator is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RNGEN::Enabled)
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IE {
    #[doc = "0: RNG interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: RNG interrupt is enabled"]
    Enabled = 1,
}
impl From<IE> for bool {
    #[inline(always)]
    fn from(variant: IE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IE` reader - Interrupt enable"]
pub type IE_R = crate::BitReader<IE>;
impl IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IE {
        match self.bits {
            false => IE::Disabled,
            true => IE::Enabled,
        }
    }
    #[doc = "RNG interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IE::Disabled
    }
    #[doc = "RNG interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IE::Enabled
    }
}
#[doc = "Field `IE` writer - Interrupt enable"]
pub type IE_W<'a, REG> = crate::BitWriter<'a, REG, IE>;
impl<'a, REG> IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RNG interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(IE::Disabled)
    }
    #[doc = "RNG interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(IE::Enabled)
    }
}
#[doc = "Clock error detection Note: The clock error detection can be used only when ck_rc48 or ck_pll1_q (ck_pll1_q = 48MHz) source is selected otherwise, CED bit must be equal to 1. The clock error detection cannot be enabled nor disabled on the fly when RNG peripheral is enabled, to enable or disable CED the RNG must be disabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CED {
    #[doc = "0: Clock error detection is enabled"]
    Enabled = 0,
    #[doc = "1: Clock error detection is disabled"]
    Disabled = 1,
}
impl From<CED> for bool {
    #[inline(always)]
    fn from(variant: CED) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CED` reader - Clock error detection Note: The clock error detection can be used only when ck_rc48 or ck_pll1_q (ck_pll1_q = 48MHz) source is selected otherwise, CED bit must be equal to 1. The clock error detection cannot be enabled nor disabled on the fly when RNG peripheral is enabled, to enable or disable CED the RNG must be disabled."]
pub type CED_R = crate::BitReader<CED>;
impl CED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CED {
        match self.bits {
            false => CED::Enabled,
            true => CED::Disabled,
        }
    }
    #[doc = "Clock error detection is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CED::Enabled
    }
    #[doc = "Clock error detection is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CED::Disabled
    }
}
#[doc = "Field `CED` writer - Clock error detection Note: The clock error detection can be used only when ck_rc48 or ck_pll1_q (ck_pll1_q = 48MHz) source is selected otherwise, CED bit must be equal to 1. The clock error detection cannot be enabled nor disabled on the fly when RNG peripheral is enabled, to enable or disable CED the RNG must be disabled."]
pub type CED_W<'a, REG> = crate::BitWriter<'a, REG, CED>;
impl<'a, REG> CED_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock error detection is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CED::Enabled)
    }
    #[doc = "Clock error detection is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CED::Disabled)
    }
}
impl R {
    #[doc = "Bit 2 - Random number generator enable"]
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt enable"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Clock error detection Note: The clock error detection can be used only when ck_rc48 or ck_pll1_q (ck_pll1_q = 48MHz) source is selected otherwise, CED bit must be equal to 1. The clock error detection cannot be enabled nor disabled on the fly when RNG peripheral is enabled, to enable or disable CED the RNG must be disabled."]
    #[inline(always)]
    pub fn ced(&self) -> CED_R {
        CED_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Random number generator enable"]
    #[inline(always)]
    #[must_use]
    pub fn rngen(&mut self) -> RNGEN_W<CRrs> {
        RNGEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ie(&mut self) -> IE_W<CRrs> {
        IE_W::new(self, 3)
    }
    #[doc = "Bit 5 - Clock error detection Note: The clock error detection can be used only when ck_rc48 or ck_pll1_q (ck_pll1_q = 48MHz) source is selected otherwise, CED bit must be equal to 1. The clock error detection cannot be enabled nor disabled on the fly when RNG peripheral is enabled, to enable or disable CED the RNG must be disabled."]
    #[inline(always)]
    #[must_use]
    pub fn ced(&mut self) -> CED_W<CRrs> {
        CED_W::new(self, 5)
    }
}
#[doc = "RNG control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CRrs {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}

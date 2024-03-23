#[doc = "Register `CCR` reader"]
pub type R = crate::R<CCRrs>;
#[doc = "Register `CCR` writer"]
pub type W = crate::W<CCRrs>;
#[doc = "Temperature sensor and VREFINT enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VREFEN {
    #[doc = "0: V_REFINT channel disabled"]
    Disabled = 0,
    #[doc = "1: V_REFINT channel enabled"]
    Enabled = 1,
}
impl From<VREFEN> for bool {
    #[inline(always)]
    fn from(variant: VREFEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VREFEN` reader - Temperature sensor and VREFINT enable"]
pub type VREFEN_R = crate::BitReader<VREFEN>;
impl VREFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VREFEN {
        match self.bits {
            false => VREFEN::Disabled,
            true => VREFEN::Enabled,
        }
    }
    #[doc = "V_REFINT channel disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VREFEN::Disabled
    }
    #[doc = "V_REFINT channel enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VREFEN::Enabled
    }
}
#[doc = "Field `VREFEN` writer - Temperature sensor and VREFINT enable"]
pub type VREFEN_W<'a, REG> = crate::BitWriter<'a, REG, VREFEN>;
impl<'a, REG> VREFEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "V_REFINT channel disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(VREFEN::Disabled)
    }
    #[doc = "V_REFINT channel enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(VREFEN::Enabled)
    }
}
#[doc = "Temperature sensor enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSEN {
    #[doc = "0: Temperature sensor disabled"]
    Disabled = 0,
    #[doc = "1: Temperature sensor enabled"]
    Enabled = 1,
}
impl From<TSEN> for bool {
    #[inline(always)]
    fn from(variant: TSEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSEN` reader - Temperature sensor enable"]
pub type TSEN_R = crate::BitReader<TSEN>;
impl TSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TSEN {
        match self.bits {
            false => TSEN::Disabled,
            true => TSEN::Enabled,
        }
    }
    #[doc = "Temperature sensor disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TSEN::Disabled
    }
    #[doc = "Temperature sensor enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TSEN::Enabled
    }
}
#[doc = "Field `TSEN` writer - Temperature sensor enable"]
pub type TSEN_W<'a, REG> = crate::BitWriter<'a, REG, TSEN>;
impl<'a, REG> TSEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Temperature sensor disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TSEN::Disabled)
    }
    #[doc = "Temperature sensor enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TSEN::Enabled)
    }
}
#[doc = "VBAT enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBATEN {
    #[doc = "0: V_BAT channel disabled"]
    Disabled = 0,
    #[doc = "1: V_BAT channel enabled"]
    Enabled = 1,
}
impl From<VBATEN> for bool {
    #[inline(always)]
    fn from(variant: VBATEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBATEN` reader - VBAT enable"]
pub type VBATEN_R = crate::BitReader<VBATEN>;
impl VBATEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VBATEN {
        match self.bits {
            false => VBATEN::Disabled,
            true => VBATEN::Enabled,
        }
    }
    #[doc = "V_BAT channel disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VBATEN::Disabled
    }
    #[doc = "V_BAT channel enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VBATEN::Enabled
    }
}
#[doc = "Field `VBATEN` writer - VBAT enable"]
pub type VBATEN_W<'a, REG> = crate::BitWriter<'a, REG, VBATEN>;
impl<'a, REG> VBATEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "V_BAT channel disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(VBATEN::Disabled)
    }
    #[doc = "V_BAT channel enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(VBATEN::Enabled)
    }
}
impl R {
    #[doc = "Bit 22 - Temperature sensor and VREFINT enable"]
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Temperature sensor enable"]
    #[inline(always)]
    pub fn tsen(&self) -> TSEN_R {
        TSEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - VBAT enable"]
    #[inline(always)]
    pub fn vbaten(&self) -> VBATEN_R {
        VBATEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 22 - Temperature sensor and VREFINT enable"]
    #[inline(always)]
    #[must_use]
    pub fn vrefen(&mut self) -> VREFEN_W<CCRrs> {
        VREFEN_W::new(self, 22)
    }
    #[doc = "Bit 23 - Temperature sensor enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsen(&mut self) -> TSEN_W<CCRrs> {
        TSEN_W::new(self, 23)
    }
    #[doc = "Bit 24 - VBAT enable"]
    #[inline(always)]
    #[must_use]
    pub fn vbaten(&mut self) -> VBATEN_W<CCRrs> {
        VBATEN_W::new(self, 24)
    }
}
#[doc = "common configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCRrs;
impl crate::RegisterSpec for CCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr::R`](R) reader structure"]
impl crate::Readable for CCRrs {}
#[doc = "`write(|w| ..)` method takes [`ccr::W`](W) writer structure"]
impl crate::Writable for CCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCR to value 0"]
impl crate::Resettable for CCRrs {
    const RESET_VALUE: u32 = 0;
}

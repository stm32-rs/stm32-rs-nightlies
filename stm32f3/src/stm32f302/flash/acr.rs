#[doc = "Register `ACR` reader"]
pub type R = crate::R<ACRrs>;
#[doc = "Register `ACR` writer"]
pub type W = crate::W<ACRrs>;
#[doc = "LATENCY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LATENCY {
    #[doc = "0: 0 wait states, if 0 &lt; HCLK &lt;= 24 MHz"]
    Ws0 = 0,
    #[doc = "1: 1 wait state, if 24 &lt; HCLK &lt;= 48 MHz"]
    Ws1 = 1,
    #[doc = "2: 2 wait states, if 48 &lt; HCLK &lt;= 72 MHz"]
    Ws2 = 2,
}
impl From<LATENCY> for u8 {
    #[inline(always)]
    fn from(variant: LATENCY) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LATENCY {
    type Ux = u8;
}
#[doc = "Field `LATENCY` reader - LATENCY"]
pub type LATENCY_R = crate::FieldReader<LATENCY>;
impl LATENCY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LATENCY> {
        match self.bits {
            0 => Some(LATENCY::Ws0),
            1 => Some(LATENCY::Ws1),
            2 => Some(LATENCY::Ws2),
            _ => None,
        }
    }
    #[doc = "0 wait states, if 0 &lt; HCLK &lt;= 24 MHz"]
    #[inline(always)]
    pub fn is_ws0(&self) -> bool {
        *self == LATENCY::Ws0
    }
    #[doc = "1 wait state, if 24 &lt; HCLK &lt;= 48 MHz"]
    #[inline(always)]
    pub fn is_ws1(&self) -> bool {
        *self == LATENCY::Ws1
    }
    #[doc = "2 wait states, if 48 &lt; HCLK &lt;= 72 MHz"]
    #[inline(always)]
    pub fn is_ws2(&self) -> bool {
        *self == LATENCY::Ws2
    }
}
#[doc = "Field `LATENCY` writer - LATENCY"]
pub type LATENCY_W<'a, REG> = crate::FieldWriter<'a, REG, 3, LATENCY>;
impl<'a, REG> LATENCY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0 wait states, if 0 &lt; HCLK &lt;= 24 MHz"]
    #[inline(always)]
    pub fn ws0(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws0)
    }
    #[doc = "1 wait state, if 24 &lt; HCLK &lt;= 48 MHz"]
    #[inline(always)]
    pub fn ws1(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws1)
    }
    #[doc = "2 wait states, if 48 &lt; HCLK &lt;= 72 MHz"]
    #[inline(always)]
    pub fn ws2(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws2)
    }
}
#[doc = "Flash half cycle access enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HLFCYA {
    #[doc = "0: Half cycle is disabled"]
    Disabled = 0,
    #[doc = "1: Half cycle is enabled"]
    Enabled = 1,
}
impl From<HLFCYA> for bool {
    #[inline(always)]
    fn from(variant: HLFCYA) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HLFCYA` reader - Flash half cycle access enable"]
pub type HLFCYA_R = crate::BitReader<HLFCYA>;
impl HLFCYA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HLFCYA {
        match self.bits {
            false => HLFCYA::Disabled,
            true => HLFCYA::Enabled,
        }
    }
    #[doc = "Half cycle is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HLFCYA::Disabled
    }
    #[doc = "Half cycle is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HLFCYA::Enabled
    }
}
#[doc = "Field `HLFCYA` writer - Flash half cycle access enable"]
pub type HLFCYA_W<'a, REG> = crate::BitWriter<'a, REG, HLFCYA>;
impl<'a, REG> HLFCYA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Half cycle is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HLFCYA::Disabled)
    }
    #[doc = "Half cycle is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HLFCYA::Enabled)
    }
}
#[doc = "PRFTBE\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRFTBE {
    #[doc = "0: Prefetch is disabled"]
    Disabled = 0,
    #[doc = "1: Prefetch is enabled"]
    Enabled = 1,
}
impl From<PRFTBE> for bool {
    #[inline(always)]
    fn from(variant: PRFTBE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRFTBE` reader - PRFTBE"]
pub type PRFTBE_R = crate::BitReader<PRFTBE>;
impl PRFTBE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRFTBE {
        match self.bits {
            false => PRFTBE::Disabled,
            true => PRFTBE::Enabled,
        }
    }
    #[doc = "Prefetch is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PRFTBE::Disabled
    }
    #[doc = "Prefetch is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PRFTBE::Enabled
    }
}
#[doc = "Field `PRFTBE` writer - PRFTBE"]
pub type PRFTBE_W<'a, REG> = crate::BitWriter<'a, REG, PRFTBE>;
impl<'a, REG> PRFTBE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Prefetch is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PRFTBE::Disabled)
    }
    #[doc = "Prefetch is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PRFTBE::Enabled)
    }
}
#[doc = "PRFTBS\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRFTBS {
    #[doc = "0: Prefetch buffer is disabled"]
    Disabled = 0,
    #[doc = "1: Prefetch buffer is enabled"]
    Enabled = 1,
}
impl From<PRFTBS> for bool {
    #[inline(always)]
    fn from(variant: PRFTBS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRFTBS` reader - PRFTBS"]
pub type PRFTBS_R = crate::BitReader<PRFTBS>;
impl PRFTBS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRFTBS {
        match self.bits {
            false => PRFTBS::Disabled,
            true => PRFTBS::Enabled,
        }
    }
    #[doc = "Prefetch buffer is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PRFTBS::Disabled
    }
    #[doc = "Prefetch buffer is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PRFTBS::Enabled
    }
}
impl R {
    #[doc = "Bits 0:2 - LATENCY"]
    #[inline(always)]
    pub fn latency(&self) -> LATENCY_R {
        LATENCY_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Flash half cycle access enable"]
    #[inline(always)]
    pub fn hlfcya(&self) -> HLFCYA_R {
        HLFCYA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PRFTBE"]
    #[inline(always)]
    pub fn prftbe(&self) -> PRFTBE_R {
        PRFTBE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PRFTBS"]
    #[inline(always)]
    pub fn prftbs(&self) -> PRFTBS_R {
        PRFTBS_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - LATENCY"]
    #[inline(always)]
    #[must_use]
    pub fn latency(&mut self) -> LATENCY_W<ACRrs> {
        LATENCY_W::new(self, 0)
    }
    #[doc = "Bit 3 - Flash half cycle access enable"]
    #[inline(always)]
    #[must_use]
    pub fn hlfcya(&mut self) -> HLFCYA_W<ACRrs> {
        HLFCYA_W::new(self, 3)
    }
    #[doc = "Bit 4 - PRFTBE"]
    #[inline(always)]
    #[must_use]
    pub fn prftbe(&mut self) -> PRFTBE_W<ACRrs> {
        PRFTBE_W::new(self, 4)
    }
}
#[doc = "Flash access control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACRrs;
impl crate::RegisterSpec for ACRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acr::R`](R) reader structure"]
impl crate::Readable for ACRrs {}
#[doc = "`write(|w| ..)` method takes [`acr::W`](W) writer structure"]
impl crate::Writable for ACRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ACR to value 0x30"]
impl crate::Resettable for ACRrs {
    const RESET_VALUE: u32 = 0x30;
}

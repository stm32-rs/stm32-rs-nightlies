#[doc = "Register `IER` reader"]
pub type R = crate::R<IERrs>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IERrs>;
#[doc = "ADC ready interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADRDYIE {
    #[doc = "0: ADC ready interrupt disabled"]
    Disabled = 0,
    #[doc = "1: ADC ready interrupt enabled"]
    Enabled = 1,
}
impl From<ADRDYIE> for bool {
    #[inline(always)]
    fn from(variant: ADRDYIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADRDYIE` reader - ADC ready interrupt enable"]
pub type ADRDYIE_R = crate::BitReader<ADRDYIE>;
impl ADRDYIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADRDYIE {
        match self.bits {
            false => ADRDYIE::Disabled,
            true => ADRDYIE::Enabled,
        }
    }
    #[doc = "ADC ready interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADRDYIE::Disabled
    }
    #[doc = "ADC ready interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADRDYIE::Enabled
    }
}
#[doc = "Field `ADRDYIE` writer - ADC ready interrupt enable"]
pub type ADRDYIE_W<'a, REG> = crate::BitWriter<'a, REG, ADRDYIE>;
impl<'a, REG> ADRDYIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC ready interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADRDYIE::Disabled)
    }
    #[doc = "ADC ready interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADRDYIE::Enabled)
    }
}
#[doc = "End of sampling flag interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSMPIE {
    #[doc = "0: End of sampling interrupt disabled"]
    Disabled = 0,
    #[doc = "1: End of sampling interrupt enabled"]
    Enabled = 1,
}
impl From<EOSMPIE> for bool {
    #[inline(always)]
    fn from(variant: EOSMPIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOSMPIE` reader - End of sampling flag interrupt enable"]
pub type EOSMPIE_R = crate::BitReader<EOSMPIE>;
impl EOSMPIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EOSMPIE {
        match self.bits {
            false => EOSMPIE::Disabled,
            true => EOSMPIE::Enabled,
        }
    }
    #[doc = "End of sampling interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EOSMPIE::Disabled
    }
    #[doc = "End of sampling interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EOSMPIE::Enabled
    }
}
#[doc = "Field `EOSMPIE` writer - End of sampling flag interrupt enable"]
pub type EOSMPIE_W<'a, REG> = crate::BitWriter<'a, REG, EOSMPIE>;
impl<'a, REG> EOSMPIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "End of sampling interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOSMPIE::Disabled)
    }
    #[doc = "End of sampling interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOSMPIE::Enabled)
    }
}
#[doc = "End of conversion interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCIE {
    #[doc = "0: End of conversion interrupt disabled"]
    Disabled = 0,
    #[doc = "1: End of conversion interrupt enabled"]
    Enabled = 1,
}
impl From<EOCIE> for bool {
    #[inline(always)]
    fn from(variant: EOCIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOCIE` reader - End of conversion interrupt enable"]
pub type EOCIE_R = crate::BitReader<EOCIE>;
impl EOCIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EOCIE {
        match self.bits {
            false => EOCIE::Disabled,
            true => EOCIE::Enabled,
        }
    }
    #[doc = "End of conversion interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EOCIE::Disabled
    }
    #[doc = "End of conversion interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EOCIE::Enabled
    }
}
#[doc = "Field `EOCIE` writer - End of conversion interrupt enable"]
pub type EOCIE_W<'a, REG> = crate::BitWriter<'a, REG, EOCIE>;
impl<'a, REG> EOCIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "End of conversion interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOCIE::Disabled)
    }
    #[doc = "End of conversion interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOCIE::Enabled)
    }
}
#[doc = "End of conversion sequence interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSEQIE {
    #[doc = "0: End of conversion sequence interrupt disabled"]
    Disabled = 0,
    #[doc = "1: End of conversion sequence interrupt enabled"]
    Enabled = 1,
}
impl From<EOSEQIE> for bool {
    #[inline(always)]
    fn from(variant: EOSEQIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOSEQIE` reader - End of conversion sequence interrupt enable"]
pub type EOSEQIE_R = crate::BitReader<EOSEQIE>;
impl EOSEQIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EOSEQIE {
        match self.bits {
            false => EOSEQIE::Disabled,
            true => EOSEQIE::Enabled,
        }
    }
    #[doc = "End of conversion sequence interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EOSEQIE::Disabled
    }
    #[doc = "End of conversion sequence interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EOSEQIE::Enabled
    }
}
#[doc = "Field `EOSEQIE` writer - End of conversion sequence interrupt enable"]
pub type EOSEQIE_W<'a, REG> = crate::BitWriter<'a, REG, EOSEQIE>;
impl<'a, REG> EOSEQIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "End of conversion sequence interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOSEQIE::Disabled)
    }
    #[doc = "End of conversion sequence interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOSEQIE::Enabled)
    }
}
#[doc = "Overrun interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRIE {
    #[doc = "0: Overrun interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Overrun interrupt enabled"]
    Enabled = 1,
}
impl From<OVRIE> for bool {
    #[inline(always)]
    fn from(variant: OVRIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVRIE` reader - Overrun interrupt enable"]
pub type OVRIE_R = crate::BitReader<OVRIE>;
impl OVRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OVRIE {
        match self.bits {
            false => OVRIE::Disabled,
            true => OVRIE::Enabled,
        }
    }
    #[doc = "Overrun interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OVRIE::Disabled
    }
    #[doc = "Overrun interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OVRIE::Enabled
    }
}
#[doc = "Field `OVRIE` writer - Overrun interrupt enable"]
pub type OVRIE_W<'a, REG> = crate::BitWriter<'a, REG, OVRIE>;
impl<'a, REG> OVRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Overrun interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OVRIE::Disabled)
    }
    #[doc = "Overrun interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OVRIE::Enabled)
    }
}
#[doc = "Analog watchdog interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWDIE {
    #[doc = "0: Analog watchdog interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Analog watchdog interrupt enabled"]
    Enabled = 1,
}
impl From<AWDIE> for bool {
    #[inline(always)]
    fn from(variant: AWDIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWDIE` reader - Analog watchdog interrupt enable"]
pub type AWDIE_R = crate::BitReader<AWDIE>;
impl AWDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AWDIE {
        match self.bits {
            false => AWDIE::Disabled,
            true => AWDIE::Enabled,
        }
    }
    #[doc = "Analog watchdog interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AWDIE::Disabled
    }
    #[doc = "Analog watchdog interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AWDIE::Enabled
    }
}
#[doc = "Field `AWDIE` writer - Analog watchdog interrupt enable"]
pub type AWDIE_W<'a, REG> = crate::BitWriter<'a, REG, AWDIE>;
impl<'a, REG> AWDIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Analog watchdog interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AWDIE::Disabled)
    }
    #[doc = "Analog watchdog interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AWDIE::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - ADC ready interrupt enable"]
    #[inline(always)]
    pub fn adrdyie(&self) -> ADRDYIE_R {
        ADRDYIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End of sampling flag interrupt enable"]
    #[inline(always)]
    pub fn eosmpie(&self) -> EOSMPIE_R {
        EOSMPIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - End of conversion interrupt enable"]
    #[inline(always)]
    pub fn eocie(&self) -> EOCIE_R {
        EOCIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End of conversion sequence interrupt enable"]
    #[inline(always)]
    pub fn eoseqie(&self) -> EOSEQIE_R {
        EOSEQIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Overrun interrupt enable"]
    #[inline(always)]
    pub fn ovrie(&self) -> OVRIE_R {
        OVRIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Analog watchdog interrupt enable"]
    #[inline(always)]
    pub fn awdie(&self) -> AWDIE_R {
        AWDIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn adrdyie(&mut self) -> ADRDYIE_W<IERrs> {
        ADRDYIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - End of sampling flag interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eosmpie(&mut self) -> EOSMPIE_W<IERrs> {
        EOSMPIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - End of conversion interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eocie(&mut self) -> EOCIE_W<IERrs> {
        EOCIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - End of conversion sequence interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eoseqie(&mut self) -> EOSEQIE_W<IERrs> {
        EOSEQIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Overrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovrie(&mut self) -> OVRIE_W<IERrs> {
        OVRIE_W::new(self, 4)
    }
    #[doc = "Bit 7 - Analog watchdog interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn awdie(&mut self) -> AWDIE_W<IERrs> {
        AWDIE_W::new(self, 7)
    }
}
#[doc = "interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IERrs;
impl crate::RegisterSpec for IERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IERrs {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IERrs {
    const RESET_VALUE: u32 = 0;
}

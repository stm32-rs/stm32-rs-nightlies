#[doc = "Register `IER` reader"]
pub type R = crate::R<IERrs>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IERrs>;
#[doc = "ADC ready interrupt\n\nValue on reset: 0"]
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
#[doc = "Field `ADRDYIE` reader - ADC ready interrupt"]
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
#[doc = "Field `ADRDYIE` writer - ADC ready interrupt"]
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
#[doc = "ADC group regular end of sampling interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSMPIE {
    #[doc = "0: End of regular conversion sampling phase interrupt disabled"]
    Disabled = 0,
    #[doc = "1: End of regular conversion sampling phase interrupt enabled"]
    Enabled = 1,
}
impl From<EOSMPIE> for bool {
    #[inline(always)]
    fn from(variant: EOSMPIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOSMPIE` reader - ADC group regular end of sampling interrupt"]
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
    #[doc = "End of regular conversion sampling phase interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EOSMPIE::Disabled
    }
    #[doc = "End of regular conversion sampling phase interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EOSMPIE::Enabled
    }
}
#[doc = "Field `EOSMPIE` writer - ADC group regular end of sampling interrupt"]
pub type EOSMPIE_W<'a, REG> = crate::BitWriter<'a, REG, EOSMPIE>;
impl<'a, REG> EOSMPIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "End of regular conversion sampling phase interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOSMPIE::Disabled)
    }
    #[doc = "End of regular conversion sampling phase interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOSMPIE::Enabled)
    }
}
#[doc = "ADC group regular end of unitary conversion interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCIE {
    #[doc = "0: End of regular conversion interrupt disabled"]
    Disabled = 0,
    #[doc = "1: End of regular conversion interrupt enabled"]
    Enabled = 1,
}
impl From<EOCIE> for bool {
    #[inline(always)]
    fn from(variant: EOCIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOCIE` reader - ADC group regular end of unitary conversion interrupt"]
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
    #[doc = "End of regular conversion interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EOCIE::Disabled
    }
    #[doc = "End of regular conversion interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EOCIE::Enabled
    }
}
#[doc = "Field `EOCIE` writer - ADC group regular end of unitary conversion interrupt"]
pub type EOCIE_W<'a, REG> = crate::BitWriter<'a, REG, EOCIE>;
impl<'a, REG> EOCIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "End of regular conversion interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOCIE::Disabled)
    }
    #[doc = "End of regular conversion interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOCIE::Enabled)
    }
}
#[doc = "ADC group regular end of sequence conversions interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSIE {
    #[doc = "0: End of regular sequence interrupt disabled"]
    Disabled = 0,
    #[doc = "1: End of regular sequence interrupt enabled"]
    Enabled = 1,
}
impl From<EOSIE> for bool {
    #[inline(always)]
    fn from(variant: EOSIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOSIE` reader - ADC group regular end of sequence conversions interrupt"]
pub type EOSIE_R = crate::BitReader<EOSIE>;
impl EOSIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EOSIE {
        match self.bits {
            false => EOSIE::Disabled,
            true => EOSIE::Enabled,
        }
    }
    #[doc = "End of regular sequence interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EOSIE::Disabled
    }
    #[doc = "End of regular sequence interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EOSIE::Enabled
    }
}
#[doc = "Field `EOSIE` writer - ADC group regular end of sequence conversions interrupt"]
pub type EOSIE_W<'a, REG> = crate::BitWriter<'a, REG, EOSIE>;
impl<'a, REG> EOSIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "End of regular sequence interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOSIE::Disabled)
    }
    #[doc = "End of regular sequence interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOSIE::Enabled)
    }
}
#[doc = "ADC group regular overrun interrupt\n\nValue on reset: 0"]
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
#[doc = "Field `OVRIE` reader - ADC group regular overrun interrupt"]
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
#[doc = "Field `OVRIE` writer - ADC group regular overrun interrupt"]
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
#[doc = "ADC group injected end of unitary conversion interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEOCIE {
    #[doc = "0: End of injected conversion interrupt disabled"]
    Disabled = 0,
    #[doc = "1: End of injected conversion interrupt enabled"]
    Enabled = 1,
}
impl From<JEOCIE> for bool {
    #[inline(always)]
    fn from(variant: JEOCIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JEOCIE` reader - ADC group injected end of unitary conversion interrupt"]
pub type JEOCIE_R = crate::BitReader<JEOCIE>;
impl JEOCIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> JEOCIE {
        match self.bits {
            false => JEOCIE::Disabled,
            true => JEOCIE::Enabled,
        }
    }
    #[doc = "End of injected conversion interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JEOCIE::Disabled
    }
    #[doc = "End of injected conversion interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JEOCIE::Enabled
    }
}
#[doc = "Field `JEOCIE` writer - ADC group injected end of unitary conversion interrupt"]
pub type JEOCIE_W<'a, REG> = crate::BitWriter<'a, REG, JEOCIE>;
impl<'a, REG> JEOCIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "End of injected conversion interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(JEOCIE::Disabled)
    }
    #[doc = "End of injected conversion interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(JEOCIE::Enabled)
    }
}
#[doc = "ADC group injected end of sequence conversions interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEOSIE {
    #[doc = "0: End of injected sequence interrupt disabled"]
    Disabled = 0,
    #[doc = "1: End of injected sequence interrupt enabled"]
    Enabled = 1,
}
impl From<JEOSIE> for bool {
    #[inline(always)]
    fn from(variant: JEOSIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JEOSIE` reader - ADC group injected end of sequence conversions interrupt"]
pub type JEOSIE_R = crate::BitReader<JEOSIE>;
impl JEOSIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> JEOSIE {
        match self.bits {
            false => JEOSIE::Disabled,
            true => JEOSIE::Enabled,
        }
    }
    #[doc = "End of injected sequence interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JEOSIE::Disabled
    }
    #[doc = "End of injected sequence interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JEOSIE::Enabled
    }
}
#[doc = "Field `JEOSIE` writer - ADC group injected end of sequence conversions interrupt"]
pub type JEOSIE_W<'a, REG> = crate::BitWriter<'a, REG, JEOSIE>;
impl<'a, REG> JEOSIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "End of injected sequence interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(JEOSIE::Disabled)
    }
    #[doc = "End of injected sequence interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(JEOSIE::Enabled)
    }
}
#[doc = "ADC analog watchdog 1 interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD1IE {
    #[doc = "0: Analog watchdog interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Analog watchdog interrupt enabled"]
    Enabled = 1,
}
impl From<AWD1IE> for bool {
    #[inline(always)]
    fn from(variant: AWD1IE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD1IE` reader - ADC analog watchdog 1 interrupt"]
pub type AWD1IE_R = crate::BitReader<AWD1IE>;
impl AWD1IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AWD1IE {
        match self.bits {
            false => AWD1IE::Disabled,
            true => AWD1IE::Enabled,
        }
    }
    #[doc = "Analog watchdog interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AWD1IE::Disabled
    }
    #[doc = "Analog watchdog interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AWD1IE::Enabled
    }
}
#[doc = "Field `AWD1IE` writer - ADC analog watchdog 1 interrupt"]
pub type AWD1IE_W<'a, REG> = crate::BitWriter<'a, REG, AWD1IE>;
impl<'a, REG> AWD1IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Analog watchdog interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1IE::Disabled)
    }
    #[doc = "Analog watchdog interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1IE::Enabled)
    }
}
#[doc = "Field `AWD2IE` reader - ADC analog watchdog 2 interrupt"]
pub use AWD1IE_R as AWD2IE_R;
#[doc = "Field `AWD3IE` reader - ADC analog watchdog 3 interrupt"]
pub use AWD1IE_R as AWD3IE_R;
#[doc = "Field `AWD2IE` writer - ADC analog watchdog 2 interrupt"]
pub use AWD1IE_W as AWD2IE_W;
#[doc = "Field `AWD3IE` writer - ADC analog watchdog 3 interrupt"]
pub use AWD1IE_W as AWD3IE_W;
#[doc = "ADC group injected contexts queue overflow interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JQOVFIE {
    #[doc = "0: Injected context queue overflow interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Injected context queue overflow interrupt enabled"]
    Enabled = 1,
}
impl From<JQOVFIE> for bool {
    #[inline(always)]
    fn from(variant: JQOVFIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JQOVFIE` reader - ADC group injected contexts queue overflow interrupt"]
pub type JQOVFIE_R = crate::BitReader<JQOVFIE>;
impl JQOVFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> JQOVFIE {
        match self.bits {
            false => JQOVFIE::Disabled,
            true => JQOVFIE::Enabled,
        }
    }
    #[doc = "Injected context queue overflow interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JQOVFIE::Disabled
    }
    #[doc = "Injected context queue overflow interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JQOVFIE::Enabled
    }
}
#[doc = "Field `JQOVFIE` writer - ADC group injected contexts queue overflow interrupt"]
pub type JQOVFIE_W<'a, REG> = crate::BitWriter<'a, REG, JQOVFIE>;
impl<'a, REG> JQOVFIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Injected context queue overflow interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(JQOVFIE::Disabled)
    }
    #[doc = "Injected context queue overflow interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(JQOVFIE::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - ADC ready interrupt"]
    #[inline(always)]
    pub fn adrdyie(&self) -> ADRDYIE_R {
        ADRDYIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC group regular end of sampling interrupt"]
    #[inline(always)]
    pub fn eosmpie(&self) -> EOSMPIE_R {
        EOSMPIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC group regular end of unitary conversion interrupt"]
    #[inline(always)]
    pub fn eocie(&self) -> EOCIE_R {
        EOCIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC group regular end of sequence conversions interrupt"]
    #[inline(always)]
    pub fn eosie(&self) -> EOSIE_R {
        EOSIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC group regular overrun interrupt"]
    #[inline(always)]
    pub fn ovrie(&self) -> OVRIE_R {
        OVRIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC group injected end of unitary conversion interrupt"]
    #[inline(always)]
    pub fn jeocie(&self) -> JEOCIE_R {
        JEOCIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ADC group injected end of sequence conversions interrupt"]
    #[inline(always)]
    pub fn jeosie(&self) -> JEOSIE_R {
        JEOSIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC analog watchdog 1 interrupt"]
    #[inline(always)]
    pub fn awd1ie(&self) -> AWD1IE_R {
        AWD1IE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC analog watchdog 2 interrupt"]
    #[inline(always)]
    pub fn awd2ie(&self) -> AWD2IE_R {
        AWD2IE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC analog watchdog 3 interrupt"]
    #[inline(always)]
    pub fn awd3ie(&self) -> AWD3IE_R {
        AWD3IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC group injected contexts queue overflow interrupt"]
    #[inline(always)]
    pub fn jqovfie(&self) -> JQOVFIE_R {
        JQOVFIE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC ready interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn adrdyie(&mut self) -> ADRDYIE_W<IERrs> {
        ADRDYIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - ADC group regular end of sampling interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn eosmpie(&mut self) -> EOSMPIE_W<IERrs> {
        EOSMPIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - ADC group regular end of unitary conversion interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn eocie(&mut self) -> EOCIE_W<IERrs> {
        EOCIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - ADC group regular end of sequence conversions interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn eosie(&mut self) -> EOSIE_W<IERrs> {
        EOSIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - ADC group regular overrun interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ovrie(&mut self) -> OVRIE_W<IERrs> {
        OVRIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - ADC group injected end of unitary conversion interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn jeocie(&mut self) -> JEOCIE_W<IERrs> {
        JEOCIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - ADC group injected end of sequence conversions interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn jeosie(&mut self) -> JEOSIE_W<IERrs> {
        JEOSIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - ADC analog watchdog 1 interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn awd1ie(&mut self) -> AWD1IE_W<IERrs> {
        AWD1IE_W::new(self, 7)
    }
    #[doc = "Bit 8 - ADC analog watchdog 2 interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn awd2ie(&mut self) -> AWD2IE_W<IERrs> {
        AWD2IE_W::new(self, 8)
    }
    #[doc = "Bit 9 - ADC analog watchdog 3 interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn awd3ie(&mut self) -> AWD3IE_W<IERrs> {
        AWD3IE_W::new(self, 9)
    }
    #[doc = "Bit 10 - ADC group injected contexts queue overflow interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn jqovfie(&mut self) -> JQOVFIE_W<IERrs> {
        JQOVFIE_W::new(self, 10)
    }
}
#[doc = "ADC interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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

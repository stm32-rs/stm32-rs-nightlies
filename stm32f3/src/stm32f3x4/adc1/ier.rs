///Register `IER` reader
pub type R = crate::R<IERrs>;
///Register `IER` writer
pub type W = crate::W<IERrs>;
/**ADRDYIE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADRDYIE {
    ///0: ADC ready interrupt disabled
    Disabled = 0,
    ///1: ADC ready interrupt enabled
    Enabled = 1,
}
impl From<ADRDYIE> for bool {
    #[inline(always)]
    fn from(variant: ADRDYIE) -> Self {
        variant as u8 != 0
    }
}
///Field `ADRDYIE` reader - ADRDYIE
pub type ADRDYIE_R = crate::BitReader<ADRDYIE>;
impl ADRDYIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADRDYIE {
        match self.bits {
            false => ADRDYIE::Disabled,
            true => ADRDYIE::Enabled,
        }
    }
    ///ADC ready interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADRDYIE::Disabled
    }
    ///ADC ready interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADRDYIE::Enabled
    }
}
///Field `ADRDYIE` writer - ADRDYIE
pub type ADRDYIE_W<'a, REG> = crate::BitWriter<'a, REG, ADRDYIE>;
impl<'a, REG> ADRDYIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC ready interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADRDYIE::Disabled)
    }
    ///ADC ready interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADRDYIE::Enabled)
    }
}
/**EOSMPIE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSMPIE {
    ///0: End of regular conversion sampling phase interrupt disabled
    Disabled = 0,
    ///1: End of regular conversion sampling phase interrupt enabled
    Enabled = 1,
}
impl From<EOSMPIE> for bool {
    #[inline(always)]
    fn from(variant: EOSMPIE) -> Self {
        variant as u8 != 0
    }
}
///Field `EOSMPIE` reader - EOSMPIE
pub type EOSMPIE_R = crate::BitReader<EOSMPIE>;
impl EOSMPIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EOSMPIE {
        match self.bits {
            false => EOSMPIE::Disabled,
            true => EOSMPIE::Enabled,
        }
    }
    ///End of regular conversion sampling phase interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EOSMPIE::Disabled
    }
    ///End of regular conversion sampling phase interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EOSMPIE::Enabled
    }
}
///Field `EOSMPIE` writer - EOSMPIE
pub type EOSMPIE_W<'a, REG> = crate::BitWriter<'a, REG, EOSMPIE>;
impl<'a, REG> EOSMPIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///End of regular conversion sampling phase interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOSMPIE::Disabled)
    }
    ///End of regular conversion sampling phase interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOSMPIE::Enabled)
    }
}
/**EOCIE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCIE {
    ///0: End of regular conversion interrupt disabled
    Disabled = 0,
    ///1: End of regular conversion interrupt enabled
    Enabled = 1,
}
impl From<EOCIE> for bool {
    #[inline(always)]
    fn from(variant: EOCIE) -> Self {
        variant as u8 != 0
    }
}
///Field `EOCIE` reader - EOCIE
pub type EOCIE_R = crate::BitReader<EOCIE>;
impl EOCIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EOCIE {
        match self.bits {
            false => EOCIE::Disabled,
            true => EOCIE::Enabled,
        }
    }
    ///End of regular conversion interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EOCIE::Disabled
    }
    ///End of regular conversion interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EOCIE::Enabled
    }
}
///Field `EOCIE` writer - EOCIE
pub type EOCIE_W<'a, REG> = crate::BitWriter<'a, REG, EOCIE>;
impl<'a, REG> EOCIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///End of regular conversion interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOCIE::Disabled)
    }
    ///End of regular conversion interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOCIE::Enabled)
    }
}
/**EOSIE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSIE {
    ///0: End of regular sequence interrupt disabled
    Disabled = 0,
    ///1: End of regular sequence interrupt enabled
    Enabled = 1,
}
impl From<EOSIE> for bool {
    #[inline(always)]
    fn from(variant: EOSIE) -> Self {
        variant as u8 != 0
    }
}
///Field `EOSIE` reader - EOSIE
pub type EOSIE_R = crate::BitReader<EOSIE>;
impl EOSIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EOSIE {
        match self.bits {
            false => EOSIE::Disabled,
            true => EOSIE::Enabled,
        }
    }
    ///End of regular sequence interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EOSIE::Disabled
    }
    ///End of regular sequence interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EOSIE::Enabled
    }
}
///Field `EOSIE` writer - EOSIE
pub type EOSIE_W<'a, REG> = crate::BitWriter<'a, REG, EOSIE>;
impl<'a, REG> EOSIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///End of regular sequence interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOSIE::Disabled)
    }
    ///End of regular sequence interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOSIE::Enabled)
    }
}
/**OVRIE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRIE {
    ///0: Overrun interrupt disabled
    Disabled = 0,
    ///1: Overrun interrupt enabled
    Enabled = 1,
}
impl From<OVRIE> for bool {
    #[inline(always)]
    fn from(variant: OVRIE) -> Self {
        variant as u8 != 0
    }
}
///Field `OVRIE` reader - OVRIE
pub type OVRIE_R = crate::BitReader<OVRIE>;
impl OVRIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OVRIE {
        match self.bits {
            false => OVRIE::Disabled,
            true => OVRIE::Enabled,
        }
    }
    ///Overrun interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OVRIE::Disabled
    }
    ///Overrun interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OVRIE::Enabled
    }
}
///Field `OVRIE` writer - OVRIE
pub type OVRIE_W<'a, REG> = crate::BitWriter<'a, REG, OVRIE>;
impl<'a, REG> OVRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Overrun interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OVRIE::Disabled)
    }
    ///Overrun interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OVRIE::Enabled)
    }
}
/**JEOCIE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEOCIE {
    ///0: End of injected conversion interrupt disabled
    Disabled = 0,
    ///1: End of injected conversion interrupt enabled
    Enabled = 1,
}
impl From<JEOCIE> for bool {
    #[inline(always)]
    fn from(variant: JEOCIE) -> Self {
        variant as u8 != 0
    }
}
///Field `JEOCIE` reader - JEOCIE
pub type JEOCIE_R = crate::BitReader<JEOCIE>;
impl JEOCIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JEOCIE {
        match self.bits {
            false => JEOCIE::Disabled,
            true => JEOCIE::Enabled,
        }
    }
    ///End of injected conversion interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JEOCIE::Disabled
    }
    ///End of injected conversion interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JEOCIE::Enabled
    }
}
///Field `JEOCIE` writer - JEOCIE
pub type JEOCIE_W<'a, REG> = crate::BitWriter<'a, REG, JEOCIE>;
impl<'a, REG> JEOCIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///End of injected conversion interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(JEOCIE::Disabled)
    }
    ///End of injected conversion interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(JEOCIE::Enabled)
    }
}
/**JEOSIE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEOSIE {
    ///0: End of injected sequence interrupt disabled
    Disabled = 0,
    ///1: End of injected sequence interrupt enabled
    Enabled = 1,
}
impl From<JEOSIE> for bool {
    #[inline(always)]
    fn from(variant: JEOSIE) -> Self {
        variant as u8 != 0
    }
}
///Field `JEOSIE` reader - JEOSIE
pub type JEOSIE_R = crate::BitReader<JEOSIE>;
impl JEOSIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JEOSIE {
        match self.bits {
            false => JEOSIE::Disabled,
            true => JEOSIE::Enabled,
        }
    }
    ///End of injected sequence interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JEOSIE::Disabled
    }
    ///End of injected sequence interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JEOSIE::Enabled
    }
}
///Field `JEOSIE` writer - JEOSIE
pub type JEOSIE_W<'a, REG> = crate::BitWriter<'a, REG, JEOSIE>;
impl<'a, REG> JEOSIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///End of injected sequence interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(JEOSIE::Disabled)
    }
    ///End of injected sequence interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(JEOSIE::Enabled)
    }
}
/**Analog watchdog %s interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD1IE {
    ///0: Analog watchdog interrupt disabled
    Disabled = 0,
    ///1: Analog watchdog interrupt enabled
    Enabled = 1,
}
impl From<AWD1IE> for bool {
    #[inline(always)]
    fn from(variant: AWD1IE) -> Self {
        variant as u8 != 0
    }
}
///Field `AWDIE(1-3)` reader - Analog watchdog %s interrupt enable
pub type AWDIE_R = crate::BitReader<AWD1IE>;
impl AWDIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD1IE {
        match self.bits {
            false => AWD1IE::Disabled,
            true => AWD1IE::Enabled,
        }
    }
    ///Analog watchdog interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AWD1IE::Disabled
    }
    ///Analog watchdog interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AWD1IE::Enabled
    }
}
///Field `AWDIE(1-3)` writer - Analog watchdog %s interrupt enable
pub type AWDIE_W<'a, REG> = crate::BitWriter<'a, REG, AWD1IE>;
impl<'a, REG> AWDIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Analog watchdog interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1IE::Disabled)
    }
    ///Analog watchdog interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1IE::Enabled)
    }
}
/**JQOVFIE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JQOVFIE {
    ///0: Injected context queue overflow interrupt disabled
    Disabled = 0,
    ///1: Injected context queue overflow interrupt enabled
    Enabled = 1,
}
impl From<JQOVFIE> for bool {
    #[inline(always)]
    fn from(variant: JQOVFIE) -> Self {
        variant as u8 != 0
    }
}
///Field `JQOVFIE` reader - JQOVFIE
pub type JQOVFIE_R = crate::BitReader<JQOVFIE>;
impl JQOVFIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JQOVFIE {
        match self.bits {
            false => JQOVFIE::Disabled,
            true => JQOVFIE::Enabled,
        }
    }
    ///Injected context queue overflow interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JQOVFIE::Disabled
    }
    ///Injected context queue overflow interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JQOVFIE::Enabled
    }
}
///Field `JQOVFIE` writer - JQOVFIE
pub type JQOVFIE_W<'a, REG> = crate::BitWriter<'a, REG, JQOVFIE>;
impl<'a, REG> JQOVFIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Injected context queue overflow interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(JQOVFIE::Disabled)
    }
    ///Injected context queue overflow interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(JQOVFIE::Enabled)
    }
}
impl R {
    ///Bit 0 - ADRDYIE
    #[inline(always)]
    pub fn adrdyie(&self) -> ADRDYIE_R {
        ADRDYIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - EOSMPIE
    #[inline(always)]
    pub fn eosmpie(&self) -> EOSMPIE_R {
        EOSMPIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - EOCIE
    #[inline(always)]
    pub fn eocie(&self) -> EOCIE_R {
        EOCIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - EOSIE
    #[inline(always)]
    pub fn eosie(&self) -> EOSIE_R {
        EOSIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - OVRIE
    #[inline(always)]
    pub fn ovrie(&self) -> OVRIE_R {
        OVRIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - JEOCIE
    #[inline(always)]
    pub fn jeocie(&self) -> JEOCIE_R {
        JEOCIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - JEOSIE
    #[inline(always)]
    pub fn jeosie(&self) -> JEOSIE_R {
        JEOSIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Analog watchdog (1-3) interrupt enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `AWD1IE` field.</div>
    #[inline(always)]
    pub fn awdie(&self, n: u8) -> AWDIE_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        AWDIE_R::new(((self.bits >> (n + 7)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Analog watchdog (1-3) interrupt enable
    #[inline(always)]
    pub fn awdie_iter(&self) -> impl Iterator<Item = AWDIE_R> + '_ {
        (0..3).map(move |n| AWDIE_R::new(((self.bits >> (n + 7)) & 1) != 0))
    }
    ///Bit 7 - Analog watchdog 1 interrupt enable
    #[inline(always)]
    pub fn awd1ie(&self) -> AWDIE_R {
        AWDIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Analog watchdog 2 interrupt enable
    #[inline(always)]
    pub fn awd2ie(&self) -> AWDIE_R {
        AWDIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Analog watchdog 3 interrupt enable
    #[inline(always)]
    pub fn awd3ie(&self) -> AWDIE_R {
        AWDIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - JQOVFIE
    #[inline(always)]
    pub fn jqovfie(&self) -> JQOVFIE_R {
        JQOVFIE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER")
            .field("jqovfie", &self.jqovfie())
            .field("awd1ie", &self.awd1ie())
            .field("awd2ie", &self.awd2ie())
            .field("awd3ie", &self.awd3ie())
            .field("jeosie", &self.jeosie())
            .field("jeocie", &self.jeocie())
            .field("ovrie", &self.ovrie())
            .field("eosie", &self.eosie())
            .field("eocie", &self.eocie())
            .field("eosmpie", &self.eosmpie())
            .field("adrdyie", &self.adrdyie())
            .finish()
    }
}
impl W {
    ///Bit 0 - ADRDYIE
    #[inline(always)]
    pub fn adrdyie(&mut self) -> ADRDYIE_W<IERrs> {
        ADRDYIE_W::new(self, 0)
    }
    ///Bit 1 - EOSMPIE
    #[inline(always)]
    pub fn eosmpie(&mut self) -> EOSMPIE_W<IERrs> {
        EOSMPIE_W::new(self, 1)
    }
    ///Bit 2 - EOCIE
    #[inline(always)]
    pub fn eocie(&mut self) -> EOCIE_W<IERrs> {
        EOCIE_W::new(self, 2)
    }
    ///Bit 3 - EOSIE
    #[inline(always)]
    pub fn eosie(&mut self) -> EOSIE_W<IERrs> {
        EOSIE_W::new(self, 3)
    }
    ///Bit 4 - OVRIE
    #[inline(always)]
    pub fn ovrie(&mut self) -> OVRIE_W<IERrs> {
        OVRIE_W::new(self, 4)
    }
    ///Bit 5 - JEOCIE
    #[inline(always)]
    pub fn jeocie(&mut self) -> JEOCIE_W<IERrs> {
        JEOCIE_W::new(self, 5)
    }
    ///Bit 6 - JEOSIE
    #[inline(always)]
    pub fn jeosie(&mut self) -> JEOSIE_W<IERrs> {
        JEOSIE_W::new(self, 6)
    }
    ///Analog watchdog (1-3) interrupt enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `AWD1IE` field.</div>
    #[inline(always)]
    pub fn awdie(&mut self, n: u8) -> AWDIE_W<IERrs> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        AWDIE_W::new(self, n + 7)
    }
    ///Bit 7 - Analog watchdog 1 interrupt enable
    #[inline(always)]
    pub fn awd1ie(&mut self) -> AWDIE_W<IERrs> {
        AWDIE_W::new(self, 7)
    }
    ///Bit 8 - Analog watchdog 2 interrupt enable
    #[inline(always)]
    pub fn awd2ie(&mut self) -> AWDIE_W<IERrs> {
        AWDIE_W::new(self, 8)
    }
    ///Bit 9 - Analog watchdog 3 interrupt enable
    #[inline(always)]
    pub fn awd3ie(&mut self) -> AWDIE_W<IERrs> {
        AWDIE_W::new(self, 9)
    }
    ///Bit 10 - JQOVFIE
    #[inline(always)]
    pub fn jqovfie(&mut self) -> JQOVFIE_W<IERrs> {
        JQOVFIE_W::new(self, 10)
    }
}
/**interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#ADC1:IER)*/
pub struct IERrs;
impl crate::RegisterSpec for IERrs {
    type Ux = u32;
}
///`read()` method returns [`ier::R`](R) reader structure
impl crate::Readable for IERrs {}
///`write(|w| ..)` method takes [`ier::W`](W) writer structure
impl crate::Writable for IERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IER to value 0
impl crate::Resettable for IERrs {}

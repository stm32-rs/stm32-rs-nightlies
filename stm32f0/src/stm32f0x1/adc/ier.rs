///Register `IER` reader
pub type R = crate::R<IERrs>;
///Register `IER` writer
pub type W = crate::W<IERrs>;
/**ADC ready interrupt enable

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
///Field `ADRDYIE` reader - ADC ready interrupt enable
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
///Field `ADRDYIE` writer - ADC ready interrupt enable
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
/**End of sampling flag interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSMPIE {
    ///0: End of sampling interrupt disabled
    Disabled = 0,
    ///1: End of sampling interrupt enabled
    Enabled = 1,
}
impl From<EOSMPIE> for bool {
    #[inline(always)]
    fn from(variant: EOSMPIE) -> Self {
        variant as u8 != 0
    }
}
///Field `EOSMPIE` reader - End of sampling flag interrupt enable
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
    ///End of sampling interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EOSMPIE::Disabled
    }
    ///End of sampling interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EOSMPIE::Enabled
    }
}
///Field `EOSMPIE` writer - End of sampling flag interrupt enable
pub type EOSMPIE_W<'a, REG> = crate::BitWriter<'a, REG, EOSMPIE>;
impl<'a, REG> EOSMPIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///End of sampling interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOSMPIE::Disabled)
    }
    ///End of sampling interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOSMPIE::Enabled)
    }
}
/**End of conversion interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCIE {
    ///0: End of conversion interrupt disabled
    Disabled = 0,
    ///1: End of conversion interrupt enabled
    Enabled = 1,
}
impl From<EOCIE> for bool {
    #[inline(always)]
    fn from(variant: EOCIE) -> Self {
        variant as u8 != 0
    }
}
///Field `EOCIE` reader - End of conversion interrupt enable
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
    ///End of conversion interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EOCIE::Disabled
    }
    ///End of conversion interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EOCIE::Enabled
    }
}
///Field `EOCIE` writer - End of conversion interrupt enable
pub type EOCIE_W<'a, REG> = crate::BitWriter<'a, REG, EOCIE>;
impl<'a, REG> EOCIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///End of conversion interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOCIE::Disabled)
    }
    ///End of conversion interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOCIE::Enabled)
    }
}
/**End of conversion sequence interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSEQIE {
    ///0: End of conversion sequence interrupt disabled
    Disabled = 0,
    ///1: End of conversion sequence interrupt enabled
    Enabled = 1,
}
impl From<EOSEQIE> for bool {
    #[inline(always)]
    fn from(variant: EOSEQIE) -> Self {
        variant as u8 != 0
    }
}
///Field `EOSEQIE` reader - End of conversion sequence interrupt enable
pub type EOSEQIE_R = crate::BitReader<EOSEQIE>;
impl EOSEQIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EOSEQIE {
        match self.bits {
            false => EOSEQIE::Disabled,
            true => EOSEQIE::Enabled,
        }
    }
    ///End of conversion sequence interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EOSEQIE::Disabled
    }
    ///End of conversion sequence interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EOSEQIE::Enabled
    }
}
///Field `EOSEQIE` writer - End of conversion sequence interrupt enable
pub type EOSEQIE_W<'a, REG> = crate::BitWriter<'a, REG, EOSEQIE>;
impl<'a, REG> EOSEQIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///End of conversion sequence interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOSEQIE::Disabled)
    }
    ///End of conversion sequence interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOSEQIE::Enabled)
    }
}
/**Overrun interrupt enable

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
///Field `OVRIE` reader - Overrun interrupt enable
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
///Field `OVRIE` writer - Overrun interrupt enable
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
/**Analog watchdog interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWDIE {
    ///0: Analog watchdog interrupt disabled
    Disabled = 0,
    ///1: Analog watchdog interrupt enabled
    Enabled = 1,
}
impl From<AWDIE> for bool {
    #[inline(always)]
    fn from(variant: AWDIE) -> Self {
        variant as u8 != 0
    }
}
///Field `AWDIE` reader - Analog watchdog interrupt enable
pub type AWDIE_R = crate::BitReader<AWDIE>;
impl AWDIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWDIE {
        match self.bits {
            false => AWDIE::Disabled,
            true => AWDIE::Enabled,
        }
    }
    ///Analog watchdog interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AWDIE::Disabled
    }
    ///Analog watchdog interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AWDIE::Enabled
    }
}
///Field `AWDIE` writer - Analog watchdog interrupt enable
pub type AWDIE_W<'a, REG> = crate::BitWriter<'a, REG, AWDIE>;
impl<'a, REG> AWDIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Analog watchdog interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AWDIE::Disabled)
    }
    ///Analog watchdog interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AWDIE::Enabled)
    }
}
impl R {
    ///Bit 0 - ADC ready interrupt enable
    #[inline(always)]
    pub fn adrdyie(&self) -> ADRDYIE_R {
        ADRDYIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - End of sampling flag interrupt enable
    #[inline(always)]
    pub fn eosmpie(&self) -> EOSMPIE_R {
        EOSMPIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - End of conversion interrupt enable
    #[inline(always)]
    pub fn eocie(&self) -> EOCIE_R {
        EOCIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - End of conversion sequence interrupt enable
    #[inline(always)]
    pub fn eoseqie(&self) -> EOSEQIE_R {
        EOSEQIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Overrun interrupt enable
    #[inline(always)]
    pub fn ovrie(&self) -> OVRIE_R {
        OVRIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - Analog watchdog interrupt enable
    #[inline(always)]
    pub fn awdie(&self) -> AWDIE_R {
        AWDIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER")
            .field("awdie", &self.awdie())
            .field("ovrie", &self.ovrie())
            .field("eoseqie", &self.eoseqie())
            .field("eocie", &self.eocie())
            .field("eosmpie", &self.eosmpie())
            .field("adrdyie", &self.adrdyie())
            .finish()
    }
}
impl W {
    ///Bit 0 - ADC ready interrupt enable
    #[inline(always)]
    pub fn adrdyie(&mut self) -> ADRDYIE_W<'_, IERrs> {
        ADRDYIE_W::new(self, 0)
    }
    ///Bit 1 - End of sampling flag interrupt enable
    #[inline(always)]
    pub fn eosmpie(&mut self) -> EOSMPIE_W<'_, IERrs> {
        EOSMPIE_W::new(self, 1)
    }
    ///Bit 2 - End of conversion interrupt enable
    #[inline(always)]
    pub fn eocie(&mut self) -> EOCIE_W<'_, IERrs> {
        EOCIE_W::new(self, 2)
    }
    ///Bit 3 - End of conversion sequence interrupt enable
    #[inline(always)]
    pub fn eoseqie(&mut self) -> EOSEQIE_W<'_, IERrs> {
        EOSEQIE_W::new(self, 3)
    }
    ///Bit 4 - Overrun interrupt enable
    #[inline(always)]
    pub fn ovrie(&mut self) -> OVRIE_W<'_, IERrs> {
        OVRIE_W::new(self, 4)
    }
    ///Bit 7 - Analog watchdog interrupt enable
    #[inline(always)]
    pub fn awdie(&mut self) -> AWDIE_W<'_, IERrs> {
        AWDIE_W::new(self, 7)
    }
}
/**interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F0x1.html#ADC:IER)*/
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

///Register `IER` reader
pub type R = crate::R<IERrs>;
///Register `IER` writer
pub type W = crate::W<IERrs>;
/**ADC ready interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADRDYIE {
    ///0: ADRDY interrupt disabled
    Disabled = 0,
    ///1: ADRDY interrupt enabled. An interrupt is generated when the ADRDY bit is set.
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
    ///ADRDY interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADRDYIE::Disabled
    }
    ///ADRDY interrupt enabled. An interrupt is generated when the ADRDY bit is set.
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
    ///ADRDY interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADRDYIE::Disabled)
    }
    ///ADRDY interrupt enabled. An interrupt is generated when the ADRDY bit is set.
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
    ///0: EOSMP interrupt disabled
    Disabled = 0,
    ///1: EOSMP interrupt enabled. An interrupt is generated when the EOSMP bit is set.
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
    ///EOSMP interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EOSMPIE::Disabled
    }
    ///EOSMP interrupt enabled. An interrupt is generated when the EOSMP bit is set.
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
    ///EOSMP interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOSMPIE::Disabled)
    }
    ///EOSMP interrupt enabled. An interrupt is generated when the EOSMP bit is set.
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
    ///0: EOC interrupt disabled
    Disabled = 0,
    ///1: EOC interrupt enabled. An interrupt is generated when the EOC bit is set.
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
    ///EOC interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EOCIE::Disabled
    }
    ///EOC interrupt enabled. An interrupt is generated when the EOC bit is set.
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
    ///EOC interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOCIE::Disabled)
    }
    ///EOC interrupt enabled. An interrupt is generated when the EOC bit is set.
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOCIE::Enabled)
    }
}
/**End of conversion sequence interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSIE {
    ///0: EOS interrupt disabled
    Disabled = 0,
    ///1: EOS interrupt enabled. An interrupt is generated when the EOS bit is set.
    Enabled = 1,
}
impl From<EOSIE> for bool {
    #[inline(always)]
    fn from(variant: EOSIE) -> Self {
        variant as u8 != 0
    }
}
///Field `EOSIE` reader - End of conversion sequence interrupt enable
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
    ///EOS interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EOSIE::Disabled
    }
    ///EOS interrupt enabled. An interrupt is generated when the EOS bit is set.
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EOSIE::Enabled
    }
}
///Field `EOSIE` writer - End of conversion sequence interrupt enable
pub type EOSIE_W<'a, REG> = crate::BitWriter<'a, REG, EOSIE>;
impl<'a, REG> EOSIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///EOS interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOSIE::Disabled)
    }
    ///EOS interrupt enabled. An interrupt is generated when the EOS bit is set.
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOSIE::Enabled)
    }
}
/**Overrun interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRIE {
    ///0: Overrun interrupt disabled
    Disabled = 0,
    ///1: Overrun interrupt enabled. An interrupt is generated when the OVR bit is set.
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
    ///Overrun interrupt enabled. An interrupt is generated when the OVR bit is set.
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
    ///Overrun interrupt enabled. An interrupt is generated when the OVR bit is set.
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
/**End of calibration interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCALIE {
    ///0: End of calibration interrupt disabled
    Disabled = 0,
    ///1: End of calibration interrupt enabled
    Enabled = 1,
}
impl From<EOCALIE> for bool {
    #[inline(always)]
    fn from(variant: EOCALIE) -> Self {
        variant as u8 != 0
    }
}
///Field `EOCALIE` reader - End of calibration interrupt enable
pub type EOCALIE_R = crate::BitReader<EOCALIE>;
impl EOCALIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EOCALIE {
        match self.bits {
            false => EOCALIE::Disabled,
            true => EOCALIE::Enabled,
        }
    }
    ///End of calibration interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EOCALIE::Disabled
    }
    ///End of calibration interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EOCALIE::Enabled
    }
}
///Field `EOCALIE` writer - End of calibration interrupt enable
pub type EOCALIE_W<'a, REG> = crate::BitWriter<'a, REG, EOCALIE>;
impl<'a, REG> EOCALIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///End of calibration interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOCALIE::Disabled)
    }
    ///End of calibration interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOCALIE::Enabled)
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
    pub fn eosie(&self) -> EOSIE_R {
        EOSIE_R::new(((self.bits >> 3) & 1) != 0)
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
    ///Bit 11 - End of calibration interrupt enable
    #[inline(always)]
    pub fn eocalie(&self) -> EOCALIE_R {
        EOCALIE_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER")
            .field("adrdyie", &self.adrdyie())
            .field("eosmpie", &self.eosmpie())
            .field("eocie", &self.eocie())
            .field("eosie", &self.eosie())
            .field("ovrie", &self.ovrie())
            .field("awdie", &self.awdie())
            .field("eocalie", &self.eocalie())
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
    pub fn eosie(&mut self) -> EOSIE_W<'_, IERrs> {
        EOSIE_W::new(self, 3)
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
    ///Bit 11 - End of calibration interrupt enable
    #[inline(always)]
    pub fn eocalie(&mut self) -> EOCALIE_W<'_, IERrs> {
        EOCALIE_W::new(self, 11)
    }
}
/**interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x1.html#ADC:IER)*/
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

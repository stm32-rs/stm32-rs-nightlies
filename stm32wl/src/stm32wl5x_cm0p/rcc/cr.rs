///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
/**MSI clock enable

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSION {
    ///0: MSI oscillator off
    Disabled = 0,
    ///1: MSI oscillator on
    Enabled = 1,
}
impl From<MSION> for bool {
    #[inline(always)]
    fn from(variant: MSION) -> Self {
        variant as u8 != 0
    }
}
///Field `MSION` reader - MSI clock enable
pub type MSION_R = crate::BitReader<MSION>;
impl MSION_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSION {
        match self.bits {
            false => MSION::Disabled,
            true => MSION::Enabled,
        }
    }
    ///MSI oscillator off
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MSION::Disabled
    }
    ///MSI oscillator on
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MSION::Enabled
    }
}
///Field `MSION` writer - MSI clock enable
pub type MSION_W<'a, REG> = crate::BitWriter<'a, REG, MSION>;
impl<'a, REG> MSION_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///MSI oscillator off
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MSION::Disabled)
    }
    ///MSI oscillator on
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MSION::Enabled)
    }
}
/**MSI clock ready flag (After reset this bit will be read 1 once the MSI is ready)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSIRDY {
    ///0: MSI oscillator not ready
    NotReady = 0,
    ///1: MSI oscillator ready
    Ready = 1,
}
impl From<MSIRDY> for bool {
    #[inline(always)]
    fn from(variant: MSIRDY) -> Self {
        variant as u8 != 0
    }
}
///Field `MSIRDY` reader - MSI clock ready flag (After reset this bit will be read 1 once the MSI is ready)
pub type MSIRDY_R = crate::BitReader<MSIRDY>;
impl MSIRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSIRDY {
        match self.bits {
            false => MSIRDY::NotReady,
            true => MSIRDY::Ready,
        }
    }
    ///MSI oscillator not ready
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == MSIRDY::NotReady
    }
    ///MSI oscillator ready
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == MSIRDY::Ready
    }
}
/**MSI clock PLL enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSIPLLEN {
    ///0: MSI PLL Off
    Off = 0,
    ///1: MSI PLL On
    On = 1,
}
impl From<MSIPLLEN> for bool {
    #[inline(always)]
    fn from(variant: MSIPLLEN) -> Self {
        variant as u8 != 0
    }
}
///Field `MSIPLLEN` reader - MSI clock PLL enable
pub type MSIPLLEN_R = crate::BitReader<MSIPLLEN>;
impl MSIPLLEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSIPLLEN {
        match self.bits {
            false => MSIPLLEN::Off,
            true => MSIPLLEN::On,
        }
    }
    ///MSI PLL Off
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == MSIPLLEN::Off
    }
    ///MSI PLL On
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == MSIPLLEN::On
    }
}
///Field `MSIPLLEN` writer - MSI clock PLL enable
pub type MSIPLLEN_W<'a, REG> = crate::BitWriter<'a, REG, MSIPLLEN>;
impl<'a, REG> MSIPLLEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///MSI PLL Off
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(MSIPLLEN::Off)
    }
    ///MSI PLL On
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(MSIPLLEN::On)
    }
}
/**MSI range control selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSIRGSEL {
    ///0: MSI frequency range defined by MSISRANGE\[3:0\] in the RCC_CSR register
    Csr = 0,
    ///1: MSI frequency range defined by MSIRANGE\[3:0\] in the RCC_CR register
    Cr = 1,
}
impl From<MSIRGSEL> for bool {
    #[inline(always)]
    fn from(variant: MSIRGSEL) -> Self {
        variant as u8 != 0
    }
}
///Field `MSIRGSEL` reader - MSI range control selection
pub type MSIRGSEL_R = crate::BitReader<MSIRGSEL>;
impl MSIRGSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSIRGSEL {
        match self.bits {
            false => MSIRGSEL::Csr,
            true => MSIRGSEL::Cr,
        }
    }
    ///MSI frequency range defined by MSISRANGE\[3:0\] in the RCC_CSR register
    #[inline(always)]
    pub fn is_csr(&self) -> bool {
        *self == MSIRGSEL::Csr
    }
    ///MSI frequency range defined by MSIRANGE\[3:0\] in the RCC_CR register
    #[inline(always)]
    pub fn is_cr(&self) -> bool {
        *self == MSIRGSEL::Cr
    }
}
///Field `MSIRGSEL` writer - MSI range control selection
pub type MSIRGSEL_W<'a, REG> = crate::BitWriter<'a, REG, MSIRGSEL>;
impl<'a, REG> MSIRGSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///MSI frequency range defined by MSISRANGE\[3:0\] in the RCC_CSR register
    #[inline(always)]
    pub fn csr(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRGSEL::Csr)
    }
    ///MSI frequency range defined by MSIRANGE\[3:0\] in the RCC_CR register
    #[inline(always)]
    pub fn cr(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRGSEL::Cr)
    }
}
/**MSI clock ranges

Value on reset: 6*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MSIRANGE {
    ///0: range 0 around 100 kHz
    Range100k = 0,
    ///1: range 1 around 200 kHz
    Range200k = 1,
    ///2: range 2 around 400 kHz
    Range400k = 2,
    ///3: range 3 around 800 kHz
    Range800k = 3,
    ///4: range 4 around 1 MHz
    Range1m = 4,
    ///5: range 5 around 2 MHz
    Range2m = 5,
    ///6: range 6 around 4 MHz (reset value)
    Range4m = 6,
    ///7: range 7 around 8 MHz
    Range8m = 7,
    ///8: range 8 around 16 MHz
    Range16m = 8,
    ///9: range 9 around 24 MHz
    Range24m = 9,
    ///10: range 10 around 32 MHz
    Range32m = 10,
    ///11: range 11 around 48 MHz
    Range48m = 11,
}
impl From<MSIRANGE> for u8 {
    #[inline(always)]
    fn from(variant: MSIRANGE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MSIRANGE {
    type Ux = u8;
}
impl crate::IsEnum for MSIRANGE {}
///Field `MSIRANGE` reader - MSI clock ranges
pub type MSIRANGE_R = crate::FieldReader<MSIRANGE>;
impl MSIRANGE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<MSIRANGE> {
        match self.bits {
            0 => Some(MSIRANGE::Range100k),
            1 => Some(MSIRANGE::Range200k),
            2 => Some(MSIRANGE::Range400k),
            3 => Some(MSIRANGE::Range800k),
            4 => Some(MSIRANGE::Range1m),
            5 => Some(MSIRANGE::Range2m),
            6 => Some(MSIRANGE::Range4m),
            7 => Some(MSIRANGE::Range8m),
            8 => Some(MSIRANGE::Range16m),
            9 => Some(MSIRANGE::Range24m),
            10 => Some(MSIRANGE::Range32m),
            11 => Some(MSIRANGE::Range48m),
            _ => None,
        }
    }
    ///range 0 around 100 kHz
    #[inline(always)]
    pub fn is_range100k(&self) -> bool {
        *self == MSIRANGE::Range100k
    }
    ///range 1 around 200 kHz
    #[inline(always)]
    pub fn is_range200k(&self) -> bool {
        *self == MSIRANGE::Range200k
    }
    ///range 2 around 400 kHz
    #[inline(always)]
    pub fn is_range400k(&self) -> bool {
        *self == MSIRANGE::Range400k
    }
    ///range 3 around 800 kHz
    #[inline(always)]
    pub fn is_range800k(&self) -> bool {
        *self == MSIRANGE::Range800k
    }
    ///range 4 around 1 MHz
    #[inline(always)]
    pub fn is_range1m(&self) -> bool {
        *self == MSIRANGE::Range1m
    }
    ///range 5 around 2 MHz
    #[inline(always)]
    pub fn is_range2m(&self) -> bool {
        *self == MSIRANGE::Range2m
    }
    ///range 6 around 4 MHz (reset value)
    #[inline(always)]
    pub fn is_range4m(&self) -> bool {
        *self == MSIRANGE::Range4m
    }
    ///range 7 around 8 MHz
    #[inline(always)]
    pub fn is_range8m(&self) -> bool {
        *self == MSIRANGE::Range8m
    }
    ///range 8 around 16 MHz
    #[inline(always)]
    pub fn is_range16m(&self) -> bool {
        *self == MSIRANGE::Range16m
    }
    ///range 9 around 24 MHz
    #[inline(always)]
    pub fn is_range24m(&self) -> bool {
        *self == MSIRANGE::Range24m
    }
    ///range 10 around 32 MHz
    #[inline(always)]
    pub fn is_range32m(&self) -> bool {
        *self == MSIRANGE::Range32m
    }
    ///range 11 around 48 MHz
    #[inline(always)]
    pub fn is_range48m(&self) -> bool {
        *self == MSIRANGE::Range48m
    }
}
///Field `MSIRANGE` writer - MSI clock ranges
pub type MSIRANGE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, MSIRANGE>;
impl<'a, REG> MSIRANGE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///range 0 around 100 kHz
    #[inline(always)]
    pub fn range100k(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRANGE::Range100k)
    }
    ///range 1 around 200 kHz
    #[inline(always)]
    pub fn range200k(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRANGE::Range200k)
    }
    ///range 2 around 400 kHz
    #[inline(always)]
    pub fn range400k(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRANGE::Range400k)
    }
    ///range 3 around 800 kHz
    #[inline(always)]
    pub fn range800k(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRANGE::Range800k)
    }
    ///range 4 around 1 MHz
    #[inline(always)]
    pub fn range1m(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRANGE::Range1m)
    }
    ///range 5 around 2 MHz
    #[inline(always)]
    pub fn range2m(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRANGE::Range2m)
    }
    ///range 6 around 4 MHz (reset value)
    #[inline(always)]
    pub fn range4m(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRANGE::Range4m)
    }
    ///range 7 around 8 MHz
    #[inline(always)]
    pub fn range8m(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRANGE::Range8m)
    }
    ///range 8 around 16 MHz
    #[inline(always)]
    pub fn range16m(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRANGE::Range16m)
    }
    ///range 9 around 24 MHz
    #[inline(always)]
    pub fn range24m(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRANGE::Range24m)
    }
    ///range 10 around 32 MHz
    #[inline(always)]
    pub fn range32m(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRANGE::Range32m)
    }
    ///range 11 around 48 MHz
    #[inline(always)]
    pub fn range48m(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRANGE::Range48m)
    }
}
/**HSI16 clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSION {
    ///0: HSI16 oscillator off
    Disabled = 0,
    ///1: HSI16 oscillator on
    Enabled = 1,
}
impl From<HSION> for bool {
    #[inline(always)]
    fn from(variant: HSION) -> Self {
        variant as u8 != 0
    }
}
///Field `HSION` reader - HSI16 clock enable
pub type HSION_R = crate::BitReader<HSION>;
impl HSION_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSION {
        match self.bits {
            false => HSION::Disabled,
            true => HSION::Enabled,
        }
    }
    ///HSI16 oscillator off
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HSION::Disabled
    }
    ///HSI16 oscillator on
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HSION::Enabled
    }
}
///Field `HSION` writer - HSI16 clock enable
pub type HSION_W<'a, REG> = crate::BitWriter<'a, REG, HSION>;
impl<'a, REG> HSION_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///HSI16 oscillator off
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSION::Disabled)
    }
    ///HSI16 oscillator on
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSION::Enabled)
    }
}
/**HSI16 always enable for peripheral kernel clocks.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIKERON {
    ///0: No effect on HSI16 oscillator
    NotForced = 0,
    ///1: HSI16 oscillator forced on even in Stop modes
    Forced = 1,
}
impl From<HSIKERON> for bool {
    #[inline(always)]
    fn from(variant: HSIKERON) -> Self {
        variant as u8 != 0
    }
}
///Field `HSIKERON` reader - HSI16 always enable for peripheral kernel clocks.
pub type HSIKERON_R = crate::BitReader<HSIKERON>;
impl HSIKERON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSIKERON {
        match self.bits {
            false => HSIKERON::NotForced,
            true => HSIKERON::Forced,
        }
    }
    ///No effect on HSI16 oscillator
    #[inline(always)]
    pub fn is_not_forced(&self) -> bool {
        *self == HSIKERON::NotForced
    }
    ///HSI16 oscillator forced on even in Stop modes
    #[inline(always)]
    pub fn is_forced(&self) -> bool {
        *self == HSIKERON::Forced
    }
}
///Field `HSIKERON` writer - HSI16 always enable for peripheral kernel clocks.
pub type HSIKERON_W<'a, REG> = crate::BitWriter<'a, REG, HSIKERON>;
impl<'a, REG> HSIKERON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on HSI16 oscillator
    #[inline(always)]
    pub fn not_forced(self) -> &'a mut crate::W<REG> {
        self.variant(HSIKERON::NotForced)
    }
    ///HSI16 oscillator forced on even in Stop modes
    #[inline(always)]
    pub fn forced(self) -> &'a mut crate::W<REG> {
        self.variant(HSIKERON::Forced)
    }
}
/**HSI16 clock ready flag. (After wakeup from Stop this bit will be read 1 once the HSI16 is ready)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIRDY {
    ///0: HSI16 oscillator not ready
    NotReady = 0,
    ///1: HSI16 oscillator ready
    Ready = 1,
}
impl From<HSIRDY> for bool {
    #[inline(always)]
    fn from(variant: HSIRDY) -> Self {
        variant as u8 != 0
    }
}
///Field `HSIRDY` reader - HSI16 clock ready flag. (After wakeup from Stop this bit will be read 1 once the HSI16 is ready)
pub type HSIRDY_R = crate::BitReader<HSIRDY>;
impl HSIRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSIRDY {
        match self.bits {
            false => HSIRDY::NotReady,
            true => HSIRDY::Ready,
        }
    }
    ///HSI16 oscillator not ready
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == HSIRDY::NotReady
    }
    ///HSI16 oscillator ready
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == HSIRDY::Ready
    }
}
/**HSI16 automatic start from Stop

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIASFS {
    ///0: HSI16 not enabled by hardware when exiting Stop modes with MSI as wakeup clock
    Disabled = 0,
    ///1: HSI16 enabled by hardware when exiting Stop mode with MSI as wakeup clock
    Enabled = 1,
}
impl From<HSIASFS> for bool {
    #[inline(always)]
    fn from(variant: HSIASFS) -> Self {
        variant as u8 != 0
    }
}
///Field `HSIASFS` reader - HSI16 automatic start from Stop
pub type HSIASFS_R = crate::BitReader<HSIASFS>;
impl HSIASFS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSIASFS {
        match self.bits {
            false => HSIASFS::Disabled,
            true => HSIASFS::Enabled,
        }
    }
    ///HSI16 not enabled by hardware when exiting Stop modes with MSI as wakeup clock
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HSIASFS::Disabled
    }
    ///HSI16 enabled by hardware when exiting Stop mode with MSI as wakeup clock
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HSIASFS::Enabled
    }
}
///Field `HSIASFS` writer - HSI16 automatic start from Stop
pub type HSIASFS_W<'a, REG> = crate::BitWriter<'a, REG, HSIASFS>;
impl<'a, REG> HSIASFS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///HSI16 not enabled by hardware when exiting Stop modes with MSI as wakeup clock
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSIASFS::Disabled)
    }
    ///HSI16 enabled by hardware when exiting Stop mode with MSI as wakeup clock
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSIASFS::Enabled)
    }
}
/**HSI16 kernel clock ready flag for peripherals requests.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIKERDY {
    ///0: HSI16 oscillator not ready
    NotReady = 0,
    ///1: HSI16 oscillator ready
    Ready = 1,
}
impl From<HSIKERDY> for bool {
    #[inline(always)]
    fn from(variant: HSIKERDY) -> Self {
        variant as u8 != 0
    }
}
///Field `HSIKERDY` reader - HSI16 kernel clock ready flag for peripherals requests.
pub type HSIKERDY_R = crate::BitReader<HSIKERDY>;
impl HSIKERDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSIKERDY {
        match self.bits {
            false => HSIKERDY::NotReady,
            true => HSIKERDY::Ready,
        }
    }
    ///HSI16 oscillator not ready
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == HSIKERDY::NotReady
    }
    ///HSI16 oscillator ready
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == HSIKERDY::Ready
    }
}
/**HSE32 clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSEON {
    ///0: HSE32 oscillator for CPU disabled
    Disabled = 0,
    ///1: HSE32 oscillator for CPU enabled
    Enabled = 1,
}
impl From<HSEON> for bool {
    #[inline(always)]
    fn from(variant: HSEON) -> Self {
        variant as u8 != 0
    }
}
///Field `HSEON` reader - HSE32 clock enable
pub type HSEON_R = crate::BitReader<HSEON>;
impl HSEON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSEON {
        match self.bits {
            false => HSEON::Disabled,
            true => HSEON::Enabled,
        }
    }
    ///HSE32 oscillator for CPU disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HSEON::Disabled
    }
    ///HSE32 oscillator for CPU enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HSEON::Enabled
    }
}
///Field `HSEON` writer - HSE32 clock enable
pub type HSEON_W<'a, REG> = crate::BitWriter<'a, REG, HSEON>;
impl<'a, REG> HSEON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///HSE32 oscillator for CPU disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSEON::Disabled)
    }
    ///HSE32 oscillator for CPU enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSEON::Enabled)
    }
}
/**HSE32 clock ready flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSERDY {
    ///0: HSE32 oscillator not ready
    NotReady = 0,
    ///1: HSE32 oscillator ready
    Ready = 1,
}
impl From<HSERDY> for bool {
    #[inline(always)]
    fn from(variant: HSERDY) -> Self {
        variant as u8 != 0
    }
}
///Field `HSERDY` reader - HSE32 clock ready flag
pub type HSERDY_R = crate::BitReader<HSERDY>;
impl HSERDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSERDY {
        match self.bits {
            false => HSERDY::NotReady,
            true => HSERDY::Ready,
        }
    }
    ///HSE32 oscillator not ready
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == HSERDY::NotReady
    }
    ///HSE32 oscillator ready
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == HSERDY::Ready
    }
}
/**HSE32 Clock security system enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSON {
    ///0: HSE32 CSS off
    Disabled = 0,
    ///1: HSE32 CSS on if the HSE32 oscillator is stable and off if not
    Enabled = 1,
}
impl From<CSSON> for bool {
    #[inline(always)]
    fn from(variant: CSSON) -> Self {
        variant as u8 != 0
    }
}
///Field `CSSON` reader - HSE32 Clock security system enable
pub type CSSON_R = crate::BitReader<CSSON>;
impl CSSON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSSON {
        match self.bits {
            false => CSSON::Disabled,
            true => CSSON::Enabled,
        }
    }
    ///HSE32 CSS off
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CSSON::Disabled
    }
    ///HSE32 CSS on if the HSE32 oscillator is stable and off if not
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CSSON::Enabled
    }
}
///Field `CSSON` writer - HSE32 Clock security system enable
pub type CSSON_W<'a, REG> = crate::BitWriter<'a, REG, CSSON>;
impl<'a, REG> CSSON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///HSE32 CSS off
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CSSON::Disabled)
    }
    ///HSE32 CSS on if the HSE32 oscillator is stable and off if not
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CSSON::Enabled)
    }
}
/**HSE32 sysclk prescaler

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSEPRE {
    ///0: SYSCLK not divided (HSE32)
    Div1 = 0,
    ///1: SYSCLK divided by two (HSE32/2)
    Div2 = 1,
}
impl From<HSEPRE> for bool {
    #[inline(always)]
    fn from(variant: HSEPRE) -> Self {
        variant as u8 != 0
    }
}
///Field `HSEPRE` reader - HSE32 sysclk prescaler
pub type HSEPRE_R = crate::BitReader<HSEPRE>;
impl HSEPRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSEPRE {
        match self.bits {
            false => HSEPRE::Div1,
            true => HSEPRE::Div2,
        }
    }
    ///SYSCLK not divided (HSE32)
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == HSEPRE::Div1
    }
    ///SYSCLK divided by two (HSE32/2)
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == HSEPRE::Div2
    }
}
///Field `HSEPRE` writer - HSE32 sysclk prescaler
pub type HSEPRE_W<'a, REG> = crate::BitWriter<'a, REG, HSEPRE>;
impl<'a, REG> HSEPRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SYSCLK not divided (HSE32)
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(HSEPRE::Div1)
    }
    ///SYSCLK divided by two (HSE32/2)
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(HSEPRE::Div2)
    }
}
/**Enable HSE32 VDDTCXO output on package pin PB0-VDDTCXO.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSEBYPPWR {
    ///0: PB0 selected
    Pb0 = 0,
    ///1: VDDTCXO selected
    Vddtcxo = 1,
}
impl From<HSEBYPPWR> for bool {
    #[inline(always)]
    fn from(variant: HSEBYPPWR) -> Self {
        variant as u8 != 0
    }
}
///Field `HSEBYPPWR` reader - Enable HSE32 VDDTCXO output on package pin PB0-VDDTCXO.
pub type HSEBYPPWR_R = crate::BitReader<HSEBYPPWR>;
impl HSEBYPPWR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSEBYPPWR {
        match self.bits {
            false => HSEBYPPWR::Pb0,
            true => HSEBYPPWR::Vddtcxo,
        }
    }
    ///PB0 selected
    #[inline(always)]
    pub fn is_pb0(&self) -> bool {
        *self == HSEBYPPWR::Pb0
    }
    ///VDDTCXO selected
    #[inline(always)]
    pub fn is_vddtcxo(&self) -> bool {
        *self == HSEBYPPWR::Vddtcxo
    }
}
///Field `HSEBYPPWR` writer - Enable HSE32 VDDTCXO output on package pin PB0-VDDTCXO.
pub type HSEBYPPWR_W<'a, REG> = crate::BitWriter<'a, REG, HSEBYPPWR>;
impl<'a, REG> HSEBYPPWR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PB0 selected
    #[inline(always)]
    pub fn pb0(self) -> &'a mut crate::W<REG> {
        self.variant(HSEBYPPWR::Pb0)
    }
    ///VDDTCXO selected
    #[inline(always)]
    pub fn vddtcxo(self) -> &'a mut crate::W<REG> {
        self.variant(HSEBYPPWR::Vddtcxo)
    }
}
/**Main PLL enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLON {
    ///0: Main PLL Off
    Off = 0,
    ///1: Main PLL On
    On = 1,
}
impl From<PLLON> for bool {
    #[inline(always)]
    fn from(variant: PLLON) -> Self {
        variant as u8 != 0
    }
}
///Field `PLLON` reader - Main PLL enable
pub type PLLON_R = crate::BitReader<PLLON>;
impl PLLON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLLON {
        match self.bits {
            false => PLLON::Off,
            true => PLLON::On,
        }
    }
    ///Main PLL Off
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == PLLON::Off
    }
    ///Main PLL On
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == PLLON::On
    }
}
///Field `PLLON` writer - Main PLL enable
pub type PLLON_W<'a, REG> = crate::BitWriter<'a, REG, PLLON>;
impl<'a, REG> PLLON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Main PLL Off
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(PLLON::Off)
    }
    ///Main PLL On
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(PLLON::On)
    }
}
/**Main PLL clock ready flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLRDY {
    ///0: PLL unlocked
    Unlocked = 0,
    ///1: PLL Locked
    Locked = 1,
}
impl From<PLLRDY> for bool {
    #[inline(always)]
    fn from(variant: PLLRDY) -> Self {
        variant as u8 != 0
    }
}
///Field `PLLRDY` reader - Main PLL clock ready flag
pub type PLLRDY_R = crate::BitReader<PLLRDY>;
impl PLLRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLLRDY {
        match self.bits {
            false => PLLRDY::Unlocked,
            true => PLLRDY::Locked,
        }
    }
    ///PLL unlocked
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == PLLRDY::Unlocked
    }
    ///PLL Locked
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == PLLRDY::Locked
    }
}
impl R {
    ///Bit 0 - MSI clock enable
    #[inline(always)]
    pub fn msion(&self) -> MSION_R {
        MSION_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - MSI clock ready flag (After reset this bit will be read 1 once the MSI is ready)
    #[inline(always)]
    pub fn msirdy(&self) -> MSIRDY_R {
        MSIRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - MSI clock PLL enable
    #[inline(always)]
    pub fn msipllen(&self) -> MSIPLLEN_R {
        MSIPLLEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - MSI range control selection
    #[inline(always)]
    pub fn msirgsel(&self) -> MSIRGSEL_R {
        MSIRGSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:7 - MSI clock ranges
    #[inline(always)]
    pub fn msirange(&self) -> MSIRANGE_R {
        MSIRANGE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bit 8 - HSI16 clock enable
    #[inline(always)]
    pub fn hsion(&self) -> HSION_R {
        HSION_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - HSI16 always enable for peripheral kernel clocks.
    #[inline(always)]
    pub fn hsikeron(&self) -> HSIKERON_R {
        HSIKERON_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - HSI16 clock ready flag. (After wakeup from Stop this bit will be read 1 once the HSI16 is ready)
    #[inline(always)]
    pub fn hsirdy(&self) -> HSIRDY_R {
        HSIRDY_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - HSI16 automatic start from Stop
    #[inline(always)]
    pub fn hsiasfs(&self) -> HSIASFS_R {
        HSIASFS_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - HSI16 kernel clock ready flag for peripherals requests.
    #[inline(always)]
    pub fn hsikerdy(&self) -> HSIKERDY_R {
        HSIKERDY_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - HSE32 clock enable
    #[inline(always)]
    pub fn hseon(&self) -> HSEON_R {
        HSEON_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - HSE32 clock ready flag
    #[inline(always)]
    pub fn hserdy(&self) -> HSERDY_R {
        HSERDY_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 19 - HSE32 Clock security system enable
    #[inline(always)]
    pub fn csson(&self) -> CSSON_R {
        CSSON_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - HSE32 sysclk prescaler
    #[inline(always)]
    pub fn hsepre(&self) -> HSEPRE_R {
        HSEPRE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Enable HSE32 VDDTCXO output on package pin PB0-VDDTCXO.
    #[inline(always)]
    pub fn hsebyppwr(&self) -> HSEBYPPWR_R {
        HSEBYPPWR_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 24 - Main PLL enable
    #[inline(always)]
    pub fn pllon(&self) -> PLLON_R {
        PLLON_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Main PLL clock ready flag
    #[inline(always)]
    pub fn pllrdy(&self) -> PLLRDY_R {
        PLLRDY_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("pllrdy", &self.pllrdy())
            .field("pllon", &self.pllon())
            .field("hsebyppwr", &self.hsebyppwr())
            .field("hsepre", &self.hsepre())
            .field("csson", &self.csson())
            .field("hserdy", &self.hserdy())
            .field("hseon", &self.hseon())
            .field("hsikerdy", &self.hsikerdy())
            .field("hsiasfs", &self.hsiasfs())
            .field("hsirdy", &self.hsirdy())
            .field("hsikeron", &self.hsikeron())
            .field("hsion", &self.hsion())
            .field("msirange", &self.msirange())
            .field("msirgsel", &self.msirgsel())
            .field("msipllen", &self.msipllen())
            .field("msirdy", &self.msirdy())
            .field("msion", &self.msion())
            .finish()
    }
}
impl W {
    ///Bit 0 - MSI clock enable
    #[inline(always)]
    pub fn msion(&mut self) -> MSION_W<'_, CRrs> {
        MSION_W::new(self, 0)
    }
    ///Bit 2 - MSI clock PLL enable
    #[inline(always)]
    pub fn msipllen(&mut self) -> MSIPLLEN_W<'_, CRrs> {
        MSIPLLEN_W::new(self, 2)
    }
    ///Bit 3 - MSI range control selection
    #[inline(always)]
    pub fn msirgsel(&mut self) -> MSIRGSEL_W<'_, CRrs> {
        MSIRGSEL_W::new(self, 3)
    }
    ///Bits 4:7 - MSI clock ranges
    #[inline(always)]
    pub fn msirange(&mut self) -> MSIRANGE_W<'_, CRrs> {
        MSIRANGE_W::new(self, 4)
    }
    ///Bit 8 - HSI16 clock enable
    #[inline(always)]
    pub fn hsion(&mut self) -> HSION_W<'_, CRrs> {
        HSION_W::new(self, 8)
    }
    ///Bit 9 - HSI16 always enable for peripheral kernel clocks.
    #[inline(always)]
    pub fn hsikeron(&mut self) -> HSIKERON_W<'_, CRrs> {
        HSIKERON_W::new(self, 9)
    }
    ///Bit 11 - HSI16 automatic start from Stop
    #[inline(always)]
    pub fn hsiasfs(&mut self) -> HSIASFS_W<'_, CRrs> {
        HSIASFS_W::new(self, 11)
    }
    ///Bit 16 - HSE32 clock enable
    #[inline(always)]
    pub fn hseon(&mut self) -> HSEON_W<'_, CRrs> {
        HSEON_W::new(self, 16)
    }
    ///Bit 19 - HSE32 Clock security system enable
    #[inline(always)]
    pub fn csson(&mut self) -> CSSON_W<'_, CRrs> {
        CSSON_W::new(self, 19)
    }
    ///Bit 20 - HSE32 sysclk prescaler
    #[inline(always)]
    pub fn hsepre(&mut self) -> HSEPRE_W<'_, CRrs> {
        HSEPRE_W::new(self, 20)
    }
    ///Bit 21 - Enable HSE32 VDDTCXO output on package pin PB0-VDDTCXO.
    #[inline(always)]
    pub fn hsebyppwr(&mut self) -> HSEBYPPWR_W<'_, CRrs> {
        HSEBYPPWR_W::new(self, 21)
    }
    ///Bit 24 - Main PLL enable
    #[inline(always)]
    pub fn pllon(&mut self) -> PLLON_W<'_, CRrs> {
        PLLON_W::new(self, 24)
    }
}
/**Clock control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0x61
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0x61;
}

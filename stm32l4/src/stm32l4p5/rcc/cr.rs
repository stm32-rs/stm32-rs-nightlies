///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
/**MSI clock enable

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSION {
    ///0: MSI oscillator OFF
    Disabled = 0,
    ///1: MSI oscillator ON
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
    ///MSI oscillator OFF
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MSION::Disabled
    }
    ///MSI oscillator ON
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
    ///MSI oscillator OFF
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MSION::Disabled)
    }
    ///MSI oscillator ON
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MSION::Enabled)
    }
}
/**MSI clock ready flag

Value on reset: 1*/
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
///Field `MSIRDY` reader - MSI clock ready flag
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
    ///0: MSI PLL OFF
    Disabled = 0,
    ///1: MSI PLL ON
    Enabled = 1,
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
            false => MSIPLLEN::Disabled,
            true => MSIPLLEN::Enabled,
        }
    }
    ///MSI PLL OFF
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MSIPLLEN::Disabled
    }
    ///MSI PLL ON
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MSIPLLEN::Enabled
    }
}
///Field `MSIPLLEN` writer - MSI clock PLL enable
pub type MSIPLLEN_W<'a, REG> = crate::BitWriter<'a, REG, MSIPLLEN>;
impl<'a, REG> MSIPLLEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///MSI PLL OFF
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MSIPLLEN::Disabled)
    }
    ///MSI PLL ON
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MSIPLLEN::Enabled)
    }
}
/**MSI clock range selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSIRGSEL {
    ///0: MSI Range is provided by MSISRANGE\[3:0\] in RCC_CSR register
    Csr = 0,
    ///1: MSI Range is provided by MSIRANGE\[3:0\] in the RCC_CR register
    Cr = 1,
}
impl From<MSIRGSEL> for bool {
    #[inline(always)]
    fn from(variant: MSIRGSEL) -> Self {
        variant as u8 != 0
    }
}
///Field `MSIRGSEL` writer - MSI clock range selection
pub type MSIRGSEL_W<'a, REG> = crate::BitWriter<'a, REG, MSIRGSEL>;
impl<'a, REG> MSIRGSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///MSI Range is provided by MSISRANGE\[3:0\] in RCC_CSR register
    #[inline(always)]
    pub fn csr(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRGSEL::Csr)
    }
    ///MSI Range is provided by MSIRANGE\[3:0\] in the RCC_CR register
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
    ///6: range 6 around 4 MHz
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
    ///range 6 around 4 MHz
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
    ///range 6 around 4 MHz
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
/**HSI clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSION {
    ///0: HSI16 oscillator OFF
    Disabled = 0,
    ///1: HSI16 oscillator ON
    Enabled = 1,
}
impl From<HSION> for bool {
    #[inline(always)]
    fn from(variant: HSION) -> Self {
        variant as u8 != 0
    }
}
///Field `HSION` reader - HSI clock enable
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
    ///HSI16 oscillator OFF
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HSION::Disabled
    }
    ///HSI16 oscillator ON
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HSION::Enabled
    }
}
///Field `HSION` writer - HSI clock enable
pub type HSION_W<'a, REG> = crate::BitWriter<'a, REG, HSION>;
impl<'a, REG> HSION_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///HSI16 oscillator OFF
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSION::Disabled)
    }
    ///HSI16 oscillator ON
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSION::Enabled)
    }
}
/**HSI always enable for peripheral kernels

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIKERON {
    ///0: No effect on HSI16 oscillator
    Disabled = 0,
    ///1: HSI16 oscillator is forced ON even in Stop mode
    Enabled = 1,
}
impl From<HSIKERON> for bool {
    #[inline(always)]
    fn from(variant: HSIKERON) -> Self {
        variant as u8 != 0
    }
}
///Field `HSIKERON` reader - HSI always enable for peripheral kernels
pub type HSIKERON_R = crate::BitReader<HSIKERON>;
impl HSIKERON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSIKERON {
        match self.bits {
            false => HSIKERON::Disabled,
            true => HSIKERON::Enabled,
        }
    }
    ///No effect on HSI16 oscillator
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HSIKERON::Disabled
    }
    ///HSI16 oscillator is forced ON even in Stop mode
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HSIKERON::Enabled
    }
}
///Field `HSIKERON` writer - HSI always enable for peripheral kernels
pub type HSIKERON_W<'a, REG> = crate::BitWriter<'a, REG, HSIKERON>;
impl<'a, REG> HSIKERON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on HSI16 oscillator
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSIKERON::Disabled)
    }
    ///HSI16 oscillator is forced ON even in Stop mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSIKERON::Enabled)
    }
}
/**HSI clock ready flag

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
///Field `HSIRDY` reader - HSI clock ready flag
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
/**HSI automatic start from Stop

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIASFS {
    ///0: HSI16 oscillator is not enabled by hardware when exiting Stop mode with MSI as wakeup clock
    Disabled = 0,
    ///1: HSI16 oscillator is enabled by hardware when exiting Stop mode with MSI as wakeup clock
    Enabled = 1,
}
impl From<HSIASFS> for bool {
    #[inline(always)]
    fn from(variant: HSIASFS) -> Self {
        variant as u8 != 0
    }
}
///Field `HSIASFS` reader - HSI automatic start from Stop
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
    ///HSI16 oscillator is not enabled by hardware when exiting Stop mode with MSI as wakeup clock
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HSIASFS::Disabled
    }
    ///HSI16 oscillator is enabled by hardware when exiting Stop mode with MSI as wakeup clock
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HSIASFS::Enabled
    }
}
///Field `HSIASFS` writer - HSI automatic start from Stop
pub type HSIASFS_W<'a, REG> = crate::BitWriter<'a, REG, HSIASFS>;
impl<'a, REG> HSIASFS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///HSI16 oscillator is not enabled by hardware when exiting Stop mode with MSI as wakeup clock
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSIASFS::Disabled)
    }
    ///HSI16 oscillator is enabled by hardware when exiting Stop mode with MSI as wakeup clock
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSIASFS::Enabled)
    }
}
/**HSE clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSEON {
    ///0: HSE oscillator OFF
    Disabled = 0,
    ///1: HSE oscillator ON
    Enabled = 1,
}
impl From<HSEON> for bool {
    #[inline(always)]
    fn from(variant: HSEON) -> Self {
        variant as u8 != 0
    }
}
///Field `HSEON` reader - HSE clock enable
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
    ///HSE oscillator OFF
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HSEON::Disabled
    }
    ///HSE oscillator ON
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HSEON::Enabled
    }
}
///Field `HSEON` writer - HSE clock enable
pub type HSEON_W<'a, REG> = crate::BitWriter<'a, REG, HSEON>;
impl<'a, REG> HSEON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///HSE oscillator OFF
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSEON::Disabled)
    }
    ///HSE oscillator ON
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSEON::Enabled)
    }
}
/**HSE clock ready flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSERDY {
    ///0: HSE oscillator not ready
    NotReady = 0,
    ///1: HSE oscillator ready
    Ready = 1,
}
impl From<HSERDY> for bool {
    #[inline(always)]
    fn from(variant: HSERDY) -> Self {
        variant as u8 != 0
    }
}
///Field `HSERDY` reader - HSE clock ready flag
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
    ///HSE oscillator not ready
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == HSERDY::NotReady
    }
    ///HSE oscillator ready
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == HSERDY::Ready
    }
}
/**HSE crystal oscillator bypass

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSEBYP {
    ///0: HSE crystal oscillator not bypassed
    NotBypassed = 0,
    ///1: HSE crystal oscillator bypassed with external clock
    Bypassed = 1,
}
impl From<HSEBYP> for bool {
    #[inline(always)]
    fn from(variant: HSEBYP) -> Self {
        variant as u8 != 0
    }
}
///Field `HSEBYP` reader - HSE crystal oscillator bypass
pub type HSEBYP_R = crate::BitReader<HSEBYP>;
impl HSEBYP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSEBYP {
        match self.bits {
            false => HSEBYP::NotBypassed,
            true => HSEBYP::Bypassed,
        }
    }
    ///HSE crystal oscillator not bypassed
    #[inline(always)]
    pub fn is_not_bypassed(&self) -> bool {
        *self == HSEBYP::NotBypassed
    }
    ///HSE crystal oscillator bypassed with external clock
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        *self == HSEBYP::Bypassed
    }
}
///Field `HSEBYP` writer - HSE crystal oscillator bypass
pub type HSEBYP_W<'a, REG> = crate::BitWriter<'a, REG, HSEBYP>;
impl<'a, REG> HSEBYP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///HSE crystal oscillator not bypassed
    #[inline(always)]
    pub fn not_bypassed(self) -> &'a mut crate::W<REG> {
        self.variant(HSEBYP::NotBypassed)
    }
    ///HSE crystal oscillator bypassed with external clock
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut crate::W<REG> {
        self.variant(HSEBYP::Bypassed)
    }
}
/**Clock security system enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSON {
    ///0: Clock security system OFF (clock detector OFF)
    Disabled = 0,
    ///1: Clock security system ON (Clock detector ON if the HSE oscillator is stable, OFF if not)
    Enabled = 1,
}
impl From<CSSON> for bool {
    #[inline(always)]
    fn from(variant: CSSON) -> Self {
        variant as u8 != 0
    }
}
///Field `CSSON` writer - Clock security system enable
pub type CSSON_W<'a, REG> = crate::BitWriter<'a, REG, CSSON>;
impl<'a, REG> CSSON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clock security system OFF (clock detector OFF)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CSSON::Disabled)
    }
    ///Clock security system ON (Clock detector ON if the HSE oscillator is stable, OFF if not)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CSSON::Enabled)
    }
}
/**Main PLL enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLON {
    ///0: PLL OFF
    Disabled = 0,
    ///1: PLL ON
    Enabled = 1,
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
            false => PLLON::Disabled,
            true => PLLON::Enabled,
        }
    }
    ///PLL OFF
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLLON::Disabled
    }
    ///PLL ON
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLLON::Enabled
    }
}
///Field `PLLON` writer - Main PLL enable
pub type PLLON_W<'a, REG> = crate::BitWriter<'a, REG, PLLON>;
impl<'a, REG> PLLON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PLL OFF
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLLON::Disabled)
    }
    ///PLL ON
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLLON::Enabled)
    }
}
/**Main PLL clock ready flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLRDY {
    ///0: PLL unlocked
    Unlocked = 0,
    ///1: PLL locked
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
    ///PLL locked
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == PLLRDY::Locked
    }
}
/**SAI1 PLL enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSAI1ON {
    ///0: PLLSAI1 OFF
    Disabled = 0,
    ///1: PLLSAI1 ON
    Enabled = 1,
}
impl From<PLLSAI1ON> for bool {
    #[inline(always)]
    fn from(variant: PLLSAI1ON) -> Self {
        variant as u8 != 0
    }
}
///Field `PLLSAI1ON` reader - SAI1 PLL enable
pub type PLLSAI1ON_R = crate::BitReader<PLLSAI1ON>;
impl PLLSAI1ON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLLSAI1ON {
        match self.bits {
            false => PLLSAI1ON::Disabled,
            true => PLLSAI1ON::Enabled,
        }
    }
    ///PLLSAI1 OFF
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLLSAI1ON::Disabled
    }
    ///PLLSAI1 ON
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLLSAI1ON::Enabled
    }
}
///Field `PLLSAI1ON` writer - SAI1 PLL enable
pub type PLLSAI1ON_W<'a, REG> = crate::BitWriter<'a, REG, PLLSAI1ON>;
impl<'a, REG> PLLSAI1ON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PLLSAI1 OFF
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1ON::Disabled)
    }
    ///PLLSAI1 ON
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1ON::Enabled)
    }
}
/**SAI1 PLL clock ready flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSAI1RDY {
    ///0: PLLSAI1 unlocked
    Unlocked = 0,
    ///1: PLLSAI1 locked
    Locked = 1,
}
impl From<PLLSAI1RDY> for bool {
    #[inline(always)]
    fn from(variant: PLLSAI1RDY) -> Self {
        variant as u8 != 0
    }
}
///Field `PLLSAI1RDY` reader - SAI1 PLL clock ready flag
pub type PLLSAI1RDY_R = crate::BitReader<PLLSAI1RDY>;
impl PLLSAI1RDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLLSAI1RDY {
        match self.bits {
            false => PLLSAI1RDY::Unlocked,
            true => PLLSAI1RDY::Locked,
        }
    }
    ///PLLSAI1 unlocked
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == PLLSAI1RDY::Unlocked
    }
    ///PLLSAI1 locked
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == PLLSAI1RDY::Locked
    }
}
/**SAI2 PLL enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSAI2ON {
    ///0: PLLSAI2 OFF
    Disabled = 0,
    ///1: PLLSAI2 ON
    Enabled = 1,
}
impl From<PLLSAI2ON> for bool {
    #[inline(always)]
    fn from(variant: PLLSAI2ON) -> Self {
        variant as u8 != 0
    }
}
///Field `PLLSAI2ON` reader - SAI2 PLL enable
pub type PLLSAI2ON_R = crate::BitReader<PLLSAI2ON>;
impl PLLSAI2ON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLLSAI2ON {
        match self.bits {
            false => PLLSAI2ON::Disabled,
            true => PLLSAI2ON::Enabled,
        }
    }
    ///PLLSAI2 OFF
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLLSAI2ON::Disabled
    }
    ///PLLSAI2 ON
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLLSAI2ON::Enabled
    }
}
///Field `PLLSAI2ON` writer - SAI2 PLL enable
pub type PLLSAI2ON_W<'a, REG> = crate::BitWriter<'a, REG, PLLSAI2ON>;
impl<'a, REG> PLLSAI2ON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PLLSAI2 OFF
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2ON::Disabled)
    }
    ///PLLSAI2 ON
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2ON::Enabled)
    }
}
/**SAI2 PLL clock ready flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSAI2RDY {
    ///0: PLLSAI2 unlocked
    Unlocked = 0,
    ///1: PLLSAI2 locked
    Locked = 1,
}
impl From<PLLSAI2RDY> for bool {
    #[inline(always)]
    fn from(variant: PLLSAI2RDY) -> Self {
        variant as u8 != 0
    }
}
///Field `PLLSAI2RDY` reader - SAI2 PLL clock ready flag
pub type PLLSAI2RDY_R = crate::BitReader<PLLSAI2RDY>;
impl PLLSAI2RDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLLSAI2RDY {
        match self.bits {
            false => PLLSAI2RDY::Unlocked,
            true => PLLSAI2RDY::Locked,
        }
    }
    ///PLLSAI2 unlocked
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == PLLSAI2RDY::Unlocked
    }
    ///PLLSAI2 locked
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == PLLSAI2RDY::Locked
    }
}
impl R {
    ///Bit 0 - MSI clock enable
    #[inline(always)]
    pub fn msion(&self) -> MSION_R {
        MSION_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - MSI clock ready flag
    #[inline(always)]
    pub fn msirdy(&self) -> MSIRDY_R {
        MSIRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - MSI clock PLL enable
    #[inline(always)]
    pub fn msipllen(&self) -> MSIPLLEN_R {
        MSIPLLEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 4:7 - MSI clock ranges
    #[inline(always)]
    pub fn msirange(&self) -> MSIRANGE_R {
        MSIRANGE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bit 8 - HSI clock enable
    #[inline(always)]
    pub fn hsion(&self) -> HSION_R {
        HSION_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - HSI always enable for peripheral kernels
    #[inline(always)]
    pub fn hsikeron(&self) -> HSIKERON_R {
        HSIKERON_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - HSI clock ready flag
    #[inline(always)]
    pub fn hsirdy(&self) -> HSIRDY_R {
        HSIRDY_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - HSI automatic start from Stop
    #[inline(always)]
    pub fn hsiasfs(&self) -> HSIASFS_R {
        HSIASFS_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 16 - HSE clock enable
    #[inline(always)]
    pub fn hseon(&self) -> HSEON_R {
        HSEON_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - HSE clock ready flag
    #[inline(always)]
    pub fn hserdy(&self) -> HSERDY_R {
        HSERDY_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - HSE crystal oscillator bypass
    #[inline(always)]
    pub fn hsebyp(&self) -> HSEBYP_R {
        HSEBYP_R::new(((self.bits >> 18) & 1) != 0)
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
    ///Bit 26 - SAI1 PLL enable
    #[inline(always)]
    pub fn pllsai1on(&self) -> PLLSAI1ON_R {
        PLLSAI1ON_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - SAI1 PLL clock ready flag
    #[inline(always)]
    pub fn pllsai1rdy(&self) -> PLLSAI1RDY_R {
        PLLSAI1RDY_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - SAI2 PLL enable
    #[inline(always)]
    pub fn pllsai2on(&self) -> PLLSAI2ON_R {
        PLLSAI2ON_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - SAI2 PLL clock ready flag
    #[inline(always)]
    pub fn pllsai2rdy(&self) -> PLLSAI2RDY_R {
        PLLSAI2RDY_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("pllsai2rdy", &self.pllsai2rdy())
            .field("pllsai2on", &self.pllsai2on())
            .field("pllsai1rdy", &self.pllsai1rdy())
            .field("pllsai1on", &self.pllsai1on())
            .field("pllrdy", &self.pllrdy())
            .field("pllon", &self.pllon())
            .field("hsebyp", &self.hsebyp())
            .field("hserdy", &self.hserdy())
            .field("hseon", &self.hseon())
            .field("hsiasfs", &self.hsiasfs())
            .field("hsirdy", &self.hsirdy())
            .field("hsikeron", &self.hsikeron())
            .field("hsion", &self.hsion())
            .field("msirange", &self.msirange())
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
    ///Bit 3 - MSI clock range selection
    #[inline(always)]
    pub fn msirgsel(&mut self) -> MSIRGSEL_W<'_, CRrs> {
        MSIRGSEL_W::new(self, 3)
    }
    ///Bits 4:7 - MSI clock ranges
    #[inline(always)]
    pub fn msirange(&mut self) -> MSIRANGE_W<'_, CRrs> {
        MSIRANGE_W::new(self, 4)
    }
    ///Bit 8 - HSI clock enable
    #[inline(always)]
    pub fn hsion(&mut self) -> HSION_W<'_, CRrs> {
        HSION_W::new(self, 8)
    }
    ///Bit 9 - HSI always enable for peripheral kernels
    #[inline(always)]
    pub fn hsikeron(&mut self) -> HSIKERON_W<'_, CRrs> {
        HSIKERON_W::new(self, 9)
    }
    ///Bit 11 - HSI automatic start from Stop
    #[inline(always)]
    pub fn hsiasfs(&mut self) -> HSIASFS_W<'_, CRrs> {
        HSIASFS_W::new(self, 11)
    }
    ///Bit 16 - HSE clock enable
    #[inline(always)]
    pub fn hseon(&mut self) -> HSEON_W<'_, CRrs> {
        HSEON_W::new(self, 16)
    }
    ///Bit 18 - HSE crystal oscillator bypass
    #[inline(always)]
    pub fn hsebyp(&mut self) -> HSEBYP_W<'_, CRrs> {
        HSEBYP_W::new(self, 18)
    }
    ///Bit 19 - Clock security system enable
    #[inline(always)]
    pub fn csson(&mut self) -> CSSON_W<'_, CRrs> {
        CSSON_W::new(self, 19)
    }
    ///Bit 24 - Main PLL enable
    #[inline(always)]
    pub fn pllon(&mut self) -> PLLON_W<'_, CRrs> {
        PLLON_W::new(self, 24)
    }
    ///Bit 26 - SAI1 PLL enable
    #[inline(always)]
    pub fn pllsai1on(&mut self) -> PLLSAI1ON_W<'_, CRrs> {
        PLLSAI1ON_W::new(self, 26)
    }
    ///Bit 28 - SAI2 PLL enable
    #[inline(always)]
    pub fn pllsai2on(&mut self) -> PLLSAI2ON_W<'_, CRrs> {
        PLLSAI2ON_W::new(self, 28)
    }
}
/**Clock control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#RCC:CR)*/
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
///`reset()` method sets CR to value 0x63
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0x63;
}

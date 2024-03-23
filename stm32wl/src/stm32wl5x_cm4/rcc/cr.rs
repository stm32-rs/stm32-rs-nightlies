#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "MSI clock enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSION {
    #[doc = "0: MSI oscillator off"]
    Disabled = 0,
    #[doc = "1: MSI oscillator on"]
    Enabled = 1,
}
impl From<MSION> for bool {
    #[inline(always)]
    fn from(variant: MSION) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSION` reader - MSI clock enable"]
pub type MSION_R = crate::BitReader<MSION>;
impl MSION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSION {
        match self.bits {
            false => MSION::Disabled,
            true => MSION::Enabled,
        }
    }
    #[doc = "MSI oscillator off"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MSION::Disabled
    }
    #[doc = "MSI oscillator on"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MSION::Enabled
    }
}
#[doc = "Field `MSION` writer - MSI clock enable"]
pub type MSION_W<'a, REG> = crate::BitWriter<'a, REG, MSION>;
impl<'a, REG> MSION_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MSI oscillator off"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MSION::Disabled)
    }
    #[doc = "MSI oscillator on"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MSION::Enabled)
    }
}
#[doc = "MSI clock ready flag (After reset this bit will be read 1 once the MSI is ready)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSIRDY {
    #[doc = "0: MSI oscillator not ready"]
    NotReady = 0,
    #[doc = "1: MSI oscillator ready"]
    Ready = 1,
}
impl From<MSIRDY> for bool {
    #[inline(always)]
    fn from(variant: MSIRDY) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSIRDY` reader - MSI clock ready flag (After reset this bit will be read 1 once the MSI is ready)"]
pub type MSIRDY_R = crate::BitReader<MSIRDY>;
impl MSIRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSIRDY {
        match self.bits {
            false => MSIRDY::NotReady,
            true => MSIRDY::Ready,
        }
    }
    #[doc = "MSI oscillator not ready"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == MSIRDY::NotReady
    }
    #[doc = "MSI oscillator ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == MSIRDY::Ready
    }
}
#[doc = "MSI clock PLL enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSIPLLEN {
    #[doc = "0: MSI PLL Off"]
    Off = 0,
    #[doc = "1: MSI PLL On"]
    On = 1,
}
impl From<MSIPLLEN> for bool {
    #[inline(always)]
    fn from(variant: MSIPLLEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSIPLLEN` reader - MSI clock PLL enable"]
pub type MSIPLLEN_R = crate::BitReader<MSIPLLEN>;
impl MSIPLLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSIPLLEN {
        match self.bits {
            false => MSIPLLEN::Off,
            true => MSIPLLEN::On,
        }
    }
    #[doc = "MSI PLL Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == MSIPLLEN::Off
    }
    #[doc = "MSI PLL On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == MSIPLLEN::On
    }
}
#[doc = "Field `MSIPLLEN` writer - MSI clock PLL enable"]
pub type MSIPLLEN_W<'a, REG> = crate::BitWriter<'a, REG, MSIPLLEN>;
impl<'a, REG> MSIPLLEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MSI PLL Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(MSIPLLEN::Off)
    }
    #[doc = "MSI PLL On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(MSIPLLEN::On)
    }
}
#[doc = "MSI range control selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSIRGSEL {
    #[doc = "0: MSI frequency range defined by MSISRANGE\\[3:0\\]
in the RCC_CSR register"]
    Csr = 0,
    #[doc = "1: MSI frequency range defined by MSIRANGE\\[3:0\\]
in the RCC_CR register"]
    Cr = 1,
}
impl From<MSIRGSEL> for bool {
    #[inline(always)]
    fn from(variant: MSIRGSEL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSIRGSEL` reader - MSI range control selection"]
pub type MSIRGSEL_R = crate::BitReader<MSIRGSEL>;
impl MSIRGSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSIRGSEL {
        match self.bits {
            false => MSIRGSEL::Csr,
            true => MSIRGSEL::Cr,
        }
    }
    #[doc = "MSI frequency range defined by MSISRANGE\\[3:0\\]
in the RCC_CSR register"]
    #[inline(always)]
    pub fn is_csr(&self) -> bool {
        *self == MSIRGSEL::Csr
    }
    #[doc = "MSI frequency range defined by MSIRANGE\\[3:0\\]
in the RCC_CR register"]
    #[inline(always)]
    pub fn is_cr(&self) -> bool {
        *self == MSIRGSEL::Cr
    }
}
#[doc = "Field `MSIRGSEL` writer - MSI range control selection"]
pub type MSIRGSEL_W<'a, REG> = crate::BitWriter<'a, REG, MSIRGSEL>;
impl<'a, REG> MSIRGSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MSI frequency range defined by MSISRANGE\\[3:0\\]
in the RCC_CSR register"]
    #[inline(always)]
    pub fn csr(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRGSEL::Csr)
    }
    #[doc = "MSI frequency range defined by MSIRANGE\\[3:0\\]
in the RCC_CR register"]
    #[inline(always)]
    pub fn cr(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRGSEL::Cr)
    }
}
#[doc = "MSI clock ranges\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MSIRANGE {
    #[doc = "0: range 0 around 100 kHz"]
    Range100k = 0,
    #[doc = "1: range 1 around 200 kHz"]
    Range200k = 1,
    #[doc = "2: range 2 around 400 kHz"]
    Range400k = 2,
    #[doc = "3: range 3 around 800 kHz"]
    Range800k = 3,
    #[doc = "4: range 4 around 1 MHz"]
    Range1m = 4,
    #[doc = "5: range 5 around 2 MHz"]
    Range2m = 5,
    #[doc = "6: range 6 around 4 MHz (reset value)"]
    Range4m = 6,
    #[doc = "7: range 7 around 8 MHz"]
    Range8m = 7,
    #[doc = "8: range 8 around 16 MHz"]
    Range16m = 8,
    #[doc = "9: range 9 around 24 MHz"]
    Range24m = 9,
    #[doc = "10: range 10 around 32 MHz"]
    Range32m = 10,
    #[doc = "11: range 11 around 48 MHz"]
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
#[doc = "Field `MSIRANGE` reader - MSI clock ranges"]
pub type MSIRANGE_R = crate::FieldReader<MSIRANGE>;
impl MSIRANGE_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "range 0 around 100 kHz"]
    #[inline(always)]
    pub fn is_range100k(&self) -> bool {
        *self == MSIRANGE::Range100k
    }
    #[doc = "range 1 around 200 kHz"]
    #[inline(always)]
    pub fn is_range200k(&self) -> bool {
        *self == MSIRANGE::Range200k
    }
    #[doc = "range 2 around 400 kHz"]
    #[inline(always)]
    pub fn is_range400k(&self) -> bool {
        *self == MSIRANGE::Range400k
    }
    #[doc = "range 3 around 800 kHz"]
    #[inline(always)]
    pub fn is_range800k(&self) -> bool {
        *self == MSIRANGE::Range800k
    }
    #[doc = "range 4 around 1 MHz"]
    #[inline(always)]
    pub fn is_range1m(&self) -> bool {
        *self == MSIRANGE::Range1m
    }
    #[doc = "range 5 around 2 MHz"]
    #[inline(always)]
    pub fn is_range2m(&self) -> bool {
        *self == MSIRANGE::Range2m
    }
    #[doc = "range 6 around 4 MHz (reset value)"]
    #[inline(always)]
    pub fn is_range4m(&self) -> bool {
        *self == MSIRANGE::Range4m
    }
    #[doc = "range 7 around 8 MHz"]
    #[inline(always)]
    pub fn is_range8m(&self) -> bool {
        *self == MSIRANGE::Range8m
    }
    #[doc = "range 8 around 16 MHz"]
    #[inline(always)]
    pub fn is_range16m(&self) -> bool {
        *self == MSIRANGE::Range16m
    }
    #[doc = "range 9 around 24 MHz"]
    #[inline(always)]
    pub fn is_range24m(&self) -> bool {
        *self == MSIRANGE::Range24m
    }
    #[doc = "range 10 around 32 MHz"]
    #[inline(always)]
    pub fn is_range32m(&self) -> bool {
        *self == MSIRANGE::Range32m
    }
    #[doc = "range 11 around 48 MHz"]
    #[inline(always)]
    pub fn is_range48m(&self) -> bool {
        *self == MSIRANGE::Range48m
    }
}
#[doc = "Field `MSIRANGE` writer - MSI clock ranges"]
pub type MSIRANGE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, MSIRANGE>;
impl<'a, REG> MSIRANGE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "range 0 around 100 kHz"]
    #[inline(always)]
    pub fn range100k(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRANGE::Range100k)
    }
    #[doc = "range 1 around 200 kHz"]
    #[inline(always)]
    pub fn range200k(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRANGE::Range200k)
    }
    #[doc = "range 2 around 400 kHz"]
    #[inline(always)]
    pub fn range400k(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRANGE::Range400k)
    }
    #[doc = "range 3 around 800 kHz"]
    #[inline(always)]
    pub fn range800k(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRANGE::Range800k)
    }
    #[doc = "range 4 around 1 MHz"]
    #[inline(always)]
    pub fn range1m(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRANGE::Range1m)
    }
    #[doc = "range 5 around 2 MHz"]
    #[inline(always)]
    pub fn range2m(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRANGE::Range2m)
    }
    #[doc = "range 6 around 4 MHz (reset value)"]
    #[inline(always)]
    pub fn range4m(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRANGE::Range4m)
    }
    #[doc = "range 7 around 8 MHz"]
    #[inline(always)]
    pub fn range8m(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRANGE::Range8m)
    }
    #[doc = "range 8 around 16 MHz"]
    #[inline(always)]
    pub fn range16m(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRANGE::Range16m)
    }
    #[doc = "range 9 around 24 MHz"]
    #[inline(always)]
    pub fn range24m(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRANGE::Range24m)
    }
    #[doc = "range 10 around 32 MHz"]
    #[inline(always)]
    pub fn range32m(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRANGE::Range32m)
    }
    #[doc = "range 11 around 48 MHz"]
    #[inline(always)]
    pub fn range48m(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRANGE::Range48m)
    }
}
#[doc = "HSI16 clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSION {
    #[doc = "0: HSI16 oscillator off"]
    Disabled = 0,
    #[doc = "1: HSI16 oscillator on"]
    Enabled = 1,
}
impl From<HSION> for bool {
    #[inline(always)]
    fn from(variant: HSION) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSION` reader - HSI16 clock enable"]
pub type HSION_R = crate::BitReader<HSION>;
impl HSION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSION {
        match self.bits {
            false => HSION::Disabled,
            true => HSION::Enabled,
        }
    }
    #[doc = "HSI16 oscillator off"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HSION::Disabled
    }
    #[doc = "HSI16 oscillator on"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HSION::Enabled
    }
}
#[doc = "Field `HSION` writer - HSI16 clock enable"]
pub type HSION_W<'a, REG> = crate::BitWriter<'a, REG, HSION>;
impl<'a, REG> HSION_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSI16 oscillator off"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSION::Disabled)
    }
    #[doc = "HSI16 oscillator on"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSION::Enabled)
    }
}
#[doc = "HSI16 always enable for peripheral kernel clocks.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIKERON {
    #[doc = "0: No effect on HSI16 oscillator"]
    NotForced = 0,
    #[doc = "1: HSI16 oscillator forced on even in Stop modes"]
    Forced = 1,
}
impl From<HSIKERON> for bool {
    #[inline(always)]
    fn from(variant: HSIKERON) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSIKERON` reader - HSI16 always enable for peripheral kernel clocks."]
pub type HSIKERON_R = crate::BitReader<HSIKERON>;
impl HSIKERON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSIKERON {
        match self.bits {
            false => HSIKERON::NotForced,
            true => HSIKERON::Forced,
        }
    }
    #[doc = "No effect on HSI16 oscillator"]
    #[inline(always)]
    pub fn is_not_forced(&self) -> bool {
        *self == HSIKERON::NotForced
    }
    #[doc = "HSI16 oscillator forced on even in Stop modes"]
    #[inline(always)]
    pub fn is_forced(&self) -> bool {
        *self == HSIKERON::Forced
    }
}
#[doc = "Field `HSIKERON` writer - HSI16 always enable for peripheral kernel clocks."]
pub type HSIKERON_W<'a, REG> = crate::BitWriter<'a, REG, HSIKERON>;
impl<'a, REG> HSIKERON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on HSI16 oscillator"]
    #[inline(always)]
    pub fn not_forced(self) -> &'a mut crate::W<REG> {
        self.variant(HSIKERON::NotForced)
    }
    #[doc = "HSI16 oscillator forced on even in Stop modes"]
    #[inline(always)]
    pub fn forced(self) -> &'a mut crate::W<REG> {
        self.variant(HSIKERON::Forced)
    }
}
#[doc = "HSI16 clock ready flag. (After wakeup from Stop this bit will be read 1 once the HSI16 is ready)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIRDY {
    #[doc = "0: HSI16 oscillator not ready"]
    NotReady = 0,
    #[doc = "1: HSI16 oscillator ready"]
    Ready = 1,
}
impl From<HSIRDY> for bool {
    #[inline(always)]
    fn from(variant: HSIRDY) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSIRDY` reader - HSI16 clock ready flag. (After wakeup from Stop this bit will be read 1 once the HSI16 is ready)"]
pub type HSIRDY_R = crate::BitReader<HSIRDY>;
impl HSIRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSIRDY {
        match self.bits {
            false => HSIRDY::NotReady,
            true => HSIRDY::Ready,
        }
    }
    #[doc = "HSI16 oscillator not ready"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == HSIRDY::NotReady
    }
    #[doc = "HSI16 oscillator ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == HSIRDY::Ready
    }
}
#[doc = "HSI16 automatic start from Stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIASFS {
    #[doc = "0: HSI16 not enabled by hardware when exiting Stop modes with MSI as wakeup clock"]
    Disabled = 0,
    #[doc = "1: HSI16 enabled by hardware when exiting Stop mode with MSI as wakeup clock"]
    Enabled = 1,
}
impl From<HSIASFS> for bool {
    #[inline(always)]
    fn from(variant: HSIASFS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSIASFS` reader - HSI16 automatic start from Stop"]
pub type HSIASFS_R = crate::BitReader<HSIASFS>;
impl HSIASFS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSIASFS {
        match self.bits {
            false => HSIASFS::Disabled,
            true => HSIASFS::Enabled,
        }
    }
    #[doc = "HSI16 not enabled by hardware when exiting Stop modes with MSI as wakeup clock"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HSIASFS::Disabled
    }
    #[doc = "HSI16 enabled by hardware when exiting Stop mode with MSI as wakeup clock"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HSIASFS::Enabled
    }
}
#[doc = "Field `HSIASFS` writer - HSI16 automatic start from Stop"]
pub type HSIASFS_W<'a, REG> = crate::BitWriter<'a, REG, HSIASFS>;
impl<'a, REG> HSIASFS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSI16 not enabled by hardware when exiting Stop modes with MSI as wakeup clock"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSIASFS::Disabled)
    }
    #[doc = "HSI16 enabled by hardware when exiting Stop mode with MSI as wakeup clock"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSIASFS::Enabled)
    }
}
#[doc = "HSI16 kernel clock ready flag for peripherals requests.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIKERDY {
    #[doc = "0: HSI16 oscillator not ready"]
    NotReady = 0,
    #[doc = "1: HSI16 oscillator ready"]
    Ready = 1,
}
impl From<HSIKERDY> for bool {
    #[inline(always)]
    fn from(variant: HSIKERDY) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSIKERDY` reader - HSI16 kernel clock ready flag for peripherals requests."]
pub type HSIKERDY_R = crate::BitReader<HSIKERDY>;
impl HSIKERDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSIKERDY {
        match self.bits {
            false => HSIKERDY::NotReady,
            true => HSIKERDY::Ready,
        }
    }
    #[doc = "HSI16 oscillator not ready"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == HSIKERDY::NotReady
    }
    #[doc = "HSI16 oscillator ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == HSIKERDY::Ready
    }
}
#[doc = "HSE32 clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSEON {
    #[doc = "0: HSE32 oscillator for CPU disabled"]
    Disabled = 0,
    #[doc = "1: HSE32 oscillator for CPU enabled"]
    Enabled = 1,
}
impl From<HSEON> for bool {
    #[inline(always)]
    fn from(variant: HSEON) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSEON` reader - HSE32 clock enable"]
pub type HSEON_R = crate::BitReader<HSEON>;
impl HSEON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSEON {
        match self.bits {
            false => HSEON::Disabled,
            true => HSEON::Enabled,
        }
    }
    #[doc = "HSE32 oscillator for CPU disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HSEON::Disabled
    }
    #[doc = "HSE32 oscillator for CPU enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HSEON::Enabled
    }
}
#[doc = "Field `HSEON` writer - HSE32 clock enable"]
pub type HSEON_W<'a, REG> = crate::BitWriter<'a, REG, HSEON>;
impl<'a, REG> HSEON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSE32 oscillator for CPU disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSEON::Disabled)
    }
    #[doc = "HSE32 oscillator for CPU enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSEON::Enabled)
    }
}
#[doc = "HSE32 clock ready flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSERDY {
    #[doc = "0: HSE32 oscillator not ready"]
    NotReady = 0,
    #[doc = "1: HSE32 oscillator ready"]
    Ready = 1,
}
impl From<HSERDY> for bool {
    #[inline(always)]
    fn from(variant: HSERDY) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSERDY` reader - HSE32 clock ready flag"]
pub type HSERDY_R = crate::BitReader<HSERDY>;
impl HSERDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSERDY {
        match self.bits {
            false => HSERDY::NotReady,
            true => HSERDY::Ready,
        }
    }
    #[doc = "HSE32 oscillator not ready"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == HSERDY::NotReady
    }
    #[doc = "HSE32 oscillator ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == HSERDY::Ready
    }
}
#[doc = "HSE32 Clock security system enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSON {
    #[doc = "0: HSE32 CSS off"]
    Disabled = 0,
    #[doc = "1: HSE32 CSS on if the HSE32 oscillator is stable and off if not"]
    Enabled = 1,
}
impl From<CSSON> for bool {
    #[inline(always)]
    fn from(variant: CSSON) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSSON` reader - HSE32 Clock security system enable"]
pub type CSSON_R = crate::BitReader<CSSON>;
impl CSSON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSSON {
        match self.bits {
            false => CSSON::Disabled,
            true => CSSON::Enabled,
        }
    }
    #[doc = "HSE32 CSS off"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CSSON::Disabled
    }
    #[doc = "HSE32 CSS on if the HSE32 oscillator is stable and off if not"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CSSON::Enabled
    }
}
#[doc = "Field `CSSON` writer - HSE32 Clock security system enable"]
pub type CSSON_W<'a, REG> = crate::BitWriter<'a, REG, CSSON>;
impl<'a, REG> CSSON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSE32 CSS off"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CSSON::Disabled)
    }
    #[doc = "HSE32 CSS on if the HSE32 oscillator is stable and off if not"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CSSON::Enabled)
    }
}
#[doc = "HSE32 sysclk prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSEPRE {
    #[doc = "0: SYSCLK not divided (HSE32)"]
    Div1 = 0,
    #[doc = "1: SYSCLK divided by two (HSE32/2)"]
    Div2 = 1,
}
impl From<HSEPRE> for bool {
    #[inline(always)]
    fn from(variant: HSEPRE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSEPRE` reader - HSE32 sysclk prescaler"]
pub type HSEPRE_R = crate::BitReader<HSEPRE>;
impl HSEPRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSEPRE {
        match self.bits {
            false => HSEPRE::Div1,
            true => HSEPRE::Div2,
        }
    }
    #[doc = "SYSCLK not divided (HSE32)"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == HSEPRE::Div1
    }
    #[doc = "SYSCLK divided by two (HSE32/2)"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == HSEPRE::Div2
    }
}
#[doc = "Field `HSEPRE` writer - HSE32 sysclk prescaler"]
pub type HSEPRE_W<'a, REG> = crate::BitWriter<'a, REG, HSEPRE>;
impl<'a, REG> HSEPRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SYSCLK not divided (HSE32)"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(HSEPRE::Div1)
    }
    #[doc = "SYSCLK divided by two (HSE32/2)"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(HSEPRE::Div2)
    }
}
#[doc = "Enable HSE32 VDDTCXO output on package pin PB0-VDDTCXO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSEBYPPWR {
    #[doc = "0: PB0 selected"]
    Pb0 = 0,
    #[doc = "1: VDDTCXO selected"]
    Vddtcxo = 1,
}
impl From<HSEBYPPWR> for bool {
    #[inline(always)]
    fn from(variant: HSEBYPPWR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSEBYPPWR` reader - Enable HSE32 VDDTCXO output on package pin PB0-VDDTCXO."]
pub type HSEBYPPWR_R = crate::BitReader<HSEBYPPWR>;
impl HSEBYPPWR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSEBYPPWR {
        match self.bits {
            false => HSEBYPPWR::Pb0,
            true => HSEBYPPWR::Vddtcxo,
        }
    }
    #[doc = "PB0 selected"]
    #[inline(always)]
    pub fn is_pb0(&self) -> bool {
        *self == HSEBYPPWR::Pb0
    }
    #[doc = "VDDTCXO selected"]
    #[inline(always)]
    pub fn is_vddtcxo(&self) -> bool {
        *self == HSEBYPPWR::Vddtcxo
    }
}
#[doc = "Field `HSEBYPPWR` writer - Enable HSE32 VDDTCXO output on package pin PB0-VDDTCXO."]
pub type HSEBYPPWR_W<'a, REG> = crate::BitWriter<'a, REG, HSEBYPPWR>;
impl<'a, REG> HSEBYPPWR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PB0 selected"]
    #[inline(always)]
    pub fn pb0(self) -> &'a mut crate::W<REG> {
        self.variant(HSEBYPPWR::Pb0)
    }
    #[doc = "VDDTCXO selected"]
    #[inline(always)]
    pub fn vddtcxo(self) -> &'a mut crate::W<REG> {
        self.variant(HSEBYPPWR::Vddtcxo)
    }
}
#[doc = "Main PLL enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLON {
    #[doc = "0: Main PLL Off"]
    Off = 0,
    #[doc = "1: Main PLL On"]
    On = 1,
}
impl From<PLLON> for bool {
    #[inline(always)]
    fn from(variant: PLLON) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLON` reader - Main PLL enable"]
pub type PLLON_R = crate::BitReader<PLLON>;
impl PLLON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLLON {
        match self.bits {
            false => PLLON::Off,
            true => PLLON::On,
        }
    }
    #[doc = "Main PLL Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == PLLON::Off
    }
    #[doc = "Main PLL On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == PLLON::On
    }
}
#[doc = "Field `PLLON` writer - Main PLL enable"]
pub type PLLON_W<'a, REG> = crate::BitWriter<'a, REG, PLLON>;
impl<'a, REG> PLLON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Main PLL Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(PLLON::Off)
    }
    #[doc = "Main PLL On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(PLLON::On)
    }
}
#[doc = "Main PLL clock ready flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLRDY {
    #[doc = "0: PLL unlocked"]
    Unlocked = 0,
    #[doc = "1: PLL Locked"]
    Locked = 1,
}
impl From<PLLRDY> for bool {
    #[inline(always)]
    fn from(variant: PLLRDY) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLRDY` reader - Main PLL clock ready flag"]
pub type PLLRDY_R = crate::BitReader<PLLRDY>;
impl PLLRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLLRDY {
        match self.bits {
            false => PLLRDY::Unlocked,
            true => PLLRDY::Locked,
        }
    }
    #[doc = "PLL unlocked"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == PLLRDY::Unlocked
    }
    #[doc = "PLL Locked"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == PLLRDY::Locked
    }
}
impl R {
    #[doc = "Bit 0 - MSI clock enable"]
    #[inline(always)]
    pub fn msion(&self) -> MSION_R {
        MSION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MSI clock ready flag (After reset this bit will be read 1 once the MSI is ready)"]
    #[inline(always)]
    pub fn msirdy(&self) -> MSIRDY_R {
        MSIRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MSI clock PLL enable"]
    #[inline(always)]
    pub fn msipllen(&self) -> MSIPLLEN_R {
        MSIPLLEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MSI range control selection"]
    #[inline(always)]
    pub fn msirgsel(&self) -> MSIRGSEL_R {
        MSIRGSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - MSI clock ranges"]
    #[inline(always)]
    pub fn msirange(&self) -> MSIRANGE_R {
        MSIRANGE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - HSI16 clock enable"]
    #[inline(always)]
    pub fn hsion(&self) -> HSION_R {
        HSION_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HSI16 always enable for peripheral kernel clocks."]
    #[inline(always)]
    pub fn hsikeron(&self) -> HSIKERON_R {
        HSIKERON_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HSI16 clock ready flag. (After wakeup from Stop this bit will be read 1 once the HSI16 is ready)"]
    #[inline(always)]
    pub fn hsirdy(&self) -> HSIRDY_R {
        HSIRDY_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HSI16 automatic start from Stop"]
    #[inline(always)]
    pub fn hsiasfs(&self) -> HSIASFS_R {
        HSIASFS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - HSI16 kernel clock ready flag for peripherals requests."]
    #[inline(always)]
    pub fn hsikerdy(&self) -> HSIKERDY_R {
        HSIKERDY_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - HSE32 clock enable"]
    #[inline(always)]
    pub fn hseon(&self) -> HSEON_R {
        HSEON_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HSE32 clock ready flag"]
    #[inline(always)]
    pub fn hserdy(&self) -> HSERDY_R {
        HSERDY_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - HSE32 Clock security system enable"]
    #[inline(always)]
    pub fn csson(&self) -> CSSON_R {
        CSSON_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - HSE32 sysclk prescaler"]
    #[inline(always)]
    pub fn hsepre(&self) -> HSEPRE_R {
        HSEPRE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable HSE32 VDDTCXO output on package pin PB0-VDDTCXO."]
    #[inline(always)]
    pub fn hsebyppwr(&self) -> HSEBYPPWR_R {
        HSEBYPPWR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - Main PLL enable"]
    #[inline(always)]
    pub fn pllon(&self) -> PLLON_R {
        PLLON_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Main PLL clock ready flag"]
    #[inline(always)]
    pub fn pllrdy(&self) -> PLLRDY_R {
        PLLRDY_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MSI clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn msion(&mut self) -> MSION_W<CRrs> {
        MSION_W::new(self, 0)
    }
    #[doc = "Bit 2 - MSI clock PLL enable"]
    #[inline(always)]
    #[must_use]
    pub fn msipllen(&mut self) -> MSIPLLEN_W<CRrs> {
        MSIPLLEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - MSI range control selection"]
    #[inline(always)]
    #[must_use]
    pub fn msirgsel(&mut self) -> MSIRGSEL_W<CRrs> {
        MSIRGSEL_W::new(self, 3)
    }
    #[doc = "Bits 4:7 - MSI clock ranges"]
    #[inline(always)]
    #[must_use]
    pub fn msirange(&mut self) -> MSIRANGE_W<CRrs> {
        MSIRANGE_W::new(self, 4)
    }
    #[doc = "Bit 8 - HSI16 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsion(&mut self) -> HSION_W<CRrs> {
        HSION_W::new(self, 8)
    }
    #[doc = "Bit 9 - HSI16 always enable for peripheral kernel clocks."]
    #[inline(always)]
    #[must_use]
    pub fn hsikeron(&mut self) -> HSIKERON_W<CRrs> {
        HSIKERON_W::new(self, 9)
    }
    #[doc = "Bit 11 - HSI16 automatic start from Stop"]
    #[inline(always)]
    #[must_use]
    pub fn hsiasfs(&mut self) -> HSIASFS_W<CRrs> {
        HSIASFS_W::new(self, 11)
    }
    #[doc = "Bit 16 - HSE32 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn hseon(&mut self) -> HSEON_W<CRrs> {
        HSEON_W::new(self, 16)
    }
    #[doc = "Bit 19 - HSE32 Clock security system enable"]
    #[inline(always)]
    #[must_use]
    pub fn csson(&mut self) -> CSSON_W<CRrs> {
        CSSON_W::new(self, 19)
    }
    #[doc = "Bit 20 - HSE32 sysclk prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn hsepre(&mut self) -> HSEPRE_W<CRrs> {
        HSEPRE_W::new(self, 20)
    }
    #[doc = "Bit 21 - Enable HSE32 VDDTCXO output on package pin PB0-VDDTCXO."]
    #[inline(always)]
    #[must_use]
    pub fn hsebyppwr(&mut self) -> HSEBYPPWR_W<CRrs> {
        HSEBYPPWR_W::new(self, 21)
    }
    #[doc = "Bit 24 - Main PLL enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllon(&mut self) -> PLLON_W<CRrs> {
        PLLON_W::new(self, 24)
    }
}
#[doc = "Clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CR to value 0x61"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0x61;
}

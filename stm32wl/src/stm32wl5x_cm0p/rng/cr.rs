#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "True random number generator enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RNGEN {
    #[doc = "0: Random number generator is disabled"]
    Disabled = 0,
    #[doc = "1: Random number generator is enabled"]
    Enabled = 1,
}
impl From<RNGEN> for bool {
    #[inline(always)]
    fn from(variant: RNGEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RNGEN` reader - True random number generator enable"]
pub type RNGEN_R = crate::BitReader<RNGEN>;
impl RNGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RNGEN {
        match self.bits {
            false => RNGEN::Disabled,
            true => RNGEN::Enabled,
        }
    }
    #[doc = "Random number generator is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RNGEN::Disabled
    }
    #[doc = "Random number generator is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RNGEN::Enabled
    }
}
#[doc = "Field `RNGEN` writer - True random number generator enable"]
pub type RNGEN_W<'a, REG> = crate::BitWriter<'a, REG, RNGEN>;
impl<'a, REG> RNGEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Random number generator is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RNGEN::Disabled)
    }
    #[doc = "Random number generator is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RNGEN::Enabled)
    }
}
#[doc = "Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IE {
    #[doc = "0: RNG interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: RNG interrupt is enabled"]
    Enabled = 1,
}
impl From<IE> for bool {
    #[inline(always)]
    fn from(variant: IE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IE` reader - Interrupt Enable"]
pub type IE_R = crate::BitReader<IE>;
impl IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IE {
        match self.bits {
            false => IE::Disabled,
            true => IE::Enabled,
        }
    }
    #[doc = "RNG interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IE::Disabled
    }
    #[doc = "RNG interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IE::Enabled
    }
}
#[doc = "Field `IE` writer - Interrupt Enable"]
pub type IE_W<'a, REG> = crate::BitWriter<'a, REG, IE>;
impl<'a, REG> IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RNG interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(IE::Disabled)
    }
    #[doc = "RNG interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(IE::Enabled)
    }
}
#[doc = "Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CED {
    #[doc = "0: Clock error detection is enabled"]
    Enabled = 0,
    #[doc = "1: Clock error detection is disabled"]
    Disabled = 1,
}
impl From<CED> for bool {
    #[inline(always)]
    fn from(variant: CED) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CED` reader - Interrupt Enable"]
pub type CED_R = crate::BitReader<CED>;
impl CED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CED {
        match self.bits {
            false => CED::Enabled,
            true => CED::Disabled,
        }
    }
    #[doc = "Clock error detection is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CED::Enabled
    }
    #[doc = "Clock error detection is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CED::Disabled
    }
}
#[doc = "Field `CED` writer - Interrupt Enable"]
pub type CED_W<'a, REG> = crate::BitWriter<'a, REG, CED>;
impl<'a, REG> CED_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock error detection is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CED::Enabled)
    }
    #[doc = "Clock error detection is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CED::Disabled)
    }
}
#[doc = "RNG_CONFIG3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RNG_CONFIG3 {
    #[doc = "0: Recommended value for config B (not NIST certifiable)"]
    ConfigB = 0,
    #[doc = "13: Recommended value for config A (NIST certifiable)"]
    ConfigA = 13,
}
impl From<RNG_CONFIG3> for u8 {
    #[inline(always)]
    fn from(variant: RNG_CONFIG3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RNG_CONFIG3 {
    type Ux = u8;
}
#[doc = "Field `RNG_CONFIG3` reader - RNG_CONFIG3"]
pub type RNG_CONFIG3_R = crate::FieldReader<RNG_CONFIG3>;
impl RNG_CONFIG3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RNG_CONFIG3> {
        match self.bits {
            0 => Some(RNG_CONFIG3::ConfigB),
            13 => Some(RNG_CONFIG3::ConfigA),
            _ => None,
        }
    }
    #[doc = "Recommended value for config B (not NIST certifiable)"]
    #[inline(always)]
    pub fn is_config_b(&self) -> bool {
        *self == RNG_CONFIG3::ConfigB
    }
    #[doc = "Recommended value for config A (NIST certifiable)"]
    #[inline(always)]
    pub fn is_config_a(&self) -> bool {
        *self == RNG_CONFIG3::ConfigA
    }
}
#[doc = "Field `RNG_CONFIG3` writer - RNG_CONFIG3"]
pub type RNG_CONFIG3_W<'a, REG> = crate::FieldWriter<'a, REG, 4, RNG_CONFIG3>;
impl<'a, REG> RNG_CONFIG3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Recommended value for config B (not NIST certifiable)"]
    #[inline(always)]
    pub fn config_b(self) -> &'a mut crate::W<REG> {
        self.variant(RNG_CONFIG3::ConfigB)
    }
    #[doc = "Recommended value for config A (NIST certifiable)"]
    #[inline(always)]
    pub fn config_a(self) -> &'a mut crate::W<REG> {
        self.variant(RNG_CONFIG3::ConfigA)
    }
}
#[doc = "NISTC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NISTC {
    #[doc = "0: Hardware default values for NIST compliant RNG. In this configuration per 128-bit output two conditioning loops are performed and 256 bits of noise source are used"]
    Default = 0,
    #[doc = "1: Custom values for NIST compliant RNG"]
    Custom = 1,
}
impl From<NISTC> for bool {
    #[inline(always)]
    fn from(variant: NISTC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NISTC` reader - NISTC"]
pub type NISTC_R = crate::BitReader<NISTC>;
impl NISTC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NISTC {
        match self.bits {
            false => NISTC::Default,
            true => NISTC::Custom,
        }
    }
    #[doc = "Hardware default values for NIST compliant RNG. In this configuration per 128-bit output two conditioning loops are performed and 256 bits of noise source are used"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == NISTC::Default
    }
    #[doc = "Custom values for NIST compliant RNG"]
    #[inline(always)]
    pub fn is_custom(&self) -> bool {
        *self == NISTC::Custom
    }
}
#[doc = "Field `NISTC` writer - NISTC"]
pub type NISTC_W<'a, REG> = crate::BitWriter<'a, REG, NISTC>;
impl<'a, REG> NISTC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware default values for NIST compliant RNG. In this configuration per 128-bit output two conditioning loops are performed and 256 bits of noise source are used"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(NISTC::Default)
    }
    #[doc = "Custom values for NIST compliant RNG"]
    #[inline(always)]
    pub fn custom(self) -> &'a mut crate::W<REG> {
        self.variant(NISTC::Custom)
    }
}
#[doc = "RNG_CONFIG2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RNG_CONFIG2 {
    #[doc = "0: Recommended value for config A and B"]
    ConfigAB = 0,
}
impl From<RNG_CONFIG2> for u8 {
    #[inline(always)]
    fn from(variant: RNG_CONFIG2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RNG_CONFIG2 {
    type Ux = u8;
}
#[doc = "Field `RNG_CONFIG2` reader - RNG_CONFIG2"]
pub type RNG_CONFIG2_R = crate::FieldReader<RNG_CONFIG2>;
impl RNG_CONFIG2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RNG_CONFIG2> {
        match self.bits {
            0 => Some(RNG_CONFIG2::ConfigAB),
            _ => None,
        }
    }
    #[doc = "Recommended value for config A and B"]
    #[inline(always)]
    pub fn is_config_a_b(&self) -> bool {
        *self == RNG_CONFIG2::ConfigAB
    }
}
#[doc = "Field `RNG_CONFIG2` writer - RNG_CONFIG2"]
pub type RNG_CONFIG2_W<'a, REG> = crate::FieldWriter<'a, REG, 3, RNG_CONFIG2>;
impl<'a, REG> RNG_CONFIG2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Recommended value for config A and B"]
    #[inline(always)]
    pub fn config_a_b(self) -> &'a mut crate::W<REG> {
        self.variant(RNG_CONFIG2::ConfigAB)
    }
}
#[doc = "CLKDIV\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKDIV {
    #[doc = "0: Internal RNG clock after divider is similar to incoming RNG clock"]
    NoDiv = 0,
    #[doc = "1: Divide RNG clock by 2^1"]
    Div2_1 = 1,
    #[doc = "2: Divide RNG clock by 2^2"]
    Div2_2 = 2,
    #[doc = "3: Divide RNG clock by 2^3"]
    Div2_3 = 3,
    #[doc = "4: Divide RNG clock by 2^4"]
    Div2_4 = 4,
    #[doc = "5: Divide RNG clock by 2^5"]
    Div2_5 = 5,
    #[doc = "6: Divide RNG clock by 2^6"]
    Div2_6 = 6,
    #[doc = "7: Divide RNG clock by 2^7"]
    Div2_7 = 7,
    #[doc = "8: Divide RNG clock by 2^8"]
    Div2_8 = 8,
    #[doc = "9: Divide RNG clock by 2^9"]
    Div2_9 = 9,
    #[doc = "10: Divide RNG clock by 2^10"]
    Div2_10 = 10,
    #[doc = "11: Divide RNG clock by 2^11"]
    Div2_11 = 11,
    #[doc = "12: Divide RNG clock by 2^12"]
    Div2_12 = 12,
    #[doc = "13: Divide RNG clock by 2^13"]
    Div2_13 = 13,
    #[doc = "14: Divide RNG clock by 2^14"]
    Div2_14 = 14,
    #[doc = "15: Divide RNG clock by 2^15"]
    Div2_15 = 15,
}
impl From<CLKDIV> for u8 {
    #[inline(always)]
    fn from(variant: CLKDIV) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLKDIV {
    type Ux = u8;
}
#[doc = "Field `CLKDIV` reader - CLKDIV"]
pub type CLKDIV_R = crate::FieldReader<CLKDIV>;
impl CLKDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLKDIV {
        match self.bits {
            0 => CLKDIV::NoDiv,
            1 => CLKDIV::Div2_1,
            2 => CLKDIV::Div2_2,
            3 => CLKDIV::Div2_3,
            4 => CLKDIV::Div2_4,
            5 => CLKDIV::Div2_5,
            6 => CLKDIV::Div2_6,
            7 => CLKDIV::Div2_7,
            8 => CLKDIV::Div2_8,
            9 => CLKDIV::Div2_9,
            10 => CLKDIV::Div2_10,
            11 => CLKDIV::Div2_11,
            12 => CLKDIV::Div2_12,
            13 => CLKDIV::Div2_13,
            14 => CLKDIV::Div2_14,
            15 => CLKDIV::Div2_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Internal RNG clock after divider is similar to incoming RNG clock"]
    #[inline(always)]
    pub fn is_no_div(&self) -> bool {
        *self == CLKDIV::NoDiv
    }
    #[doc = "Divide RNG clock by 2^1"]
    #[inline(always)]
    pub fn is_div_2_1(&self) -> bool {
        *self == CLKDIV::Div2_1
    }
    #[doc = "Divide RNG clock by 2^2"]
    #[inline(always)]
    pub fn is_div_2_2(&self) -> bool {
        *self == CLKDIV::Div2_2
    }
    #[doc = "Divide RNG clock by 2^3"]
    #[inline(always)]
    pub fn is_div_2_3(&self) -> bool {
        *self == CLKDIV::Div2_3
    }
    #[doc = "Divide RNG clock by 2^4"]
    #[inline(always)]
    pub fn is_div_2_4(&self) -> bool {
        *self == CLKDIV::Div2_4
    }
    #[doc = "Divide RNG clock by 2^5"]
    #[inline(always)]
    pub fn is_div_2_5(&self) -> bool {
        *self == CLKDIV::Div2_5
    }
    #[doc = "Divide RNG clock by 2^6"]
    #[inline(always)]
    pub fn is_div_2_6(&self) -> bool {
        *self == CLKDIV::Div2_6
    }
    #[doc = "Divide RNG clock by 2^7"]
    #[inline(always)]
    pub fn is_div_2_7(&self) -> bool {
        *self == CLKDIV::Div2_7
    }
    #[doc = "Divide RNG clock by 2^8"]
    #[inline(always)]
    pub fn is_div_2_8(&self) -> bool {
        *self == CLKDIV::Div2_8
    }
    #[doc = "Divide RNG clock by 2^9"]
    #[inline(always)]
    pub fn is_div_2_9(&self) -> bool {
        *self == CLKDIV::Div2_9
    }
    #[doc = "Divide RNG clock by 2^10"]
    #[inline(always)]
    pub fn is_div_2_10(&self) -> bool {
        *self == CLKDIV::Div2_10
    }
    #[doc = "Divide RNG clock by 2^11"]
    #[inline(always)]
    pub fn is_div_2_11(&self) -> bool {
        *self == CLKDIV::Div2_11
    }
    #[doc = "Divide RNG clock by 2^12"]
    #[inline(always)]
    pub fn is_div_2_12(&self) -> bool {
        *self == CLKDIV::Div2_12
    }
    #[doc = "Divide RNG clock by 2^13"]
    #[inline(always)]
    pub fn is_div_2_13(&self) -> bool {
        *self == CLKDIV::Div2_13
    }
    #[doc = "Divide RNG clock by 2^14"]
    #[inline(always)]
    pub fn is_div_2_14(&self) -> bool {
        *self == CLKDIV::Div2_14
    }
    #[doc = "Divide RNG clock by 2^15"]
    #[inline(always)]
    pub fn is_div_2_15(&self) -> bool {
        *self == CLKDIV::Div2_15
    }
}
#[doc = "Field `CLKDIV` writer - CLKDIV"]
pub type CLKDIV_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, CLKDIV>;
impl<'a, REG> CLKDIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal RNG clock after divider is similar to incoming RNG clock"]
    #[inline(always)]
    pub fn no_div(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV::NoDiv)
    }
    #[doc = "Divide RNG clock by 2^1"]
    #[inline(always)]
    pub fn div_2_1(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV::Div2_1)
    }
    #[doc = "Divide RNG clock by 2^2"]
    #[inline(always)]
    pub fn div_2_2(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV::Div2_2)
    }
    #[doc = "Divide RNG clock by 2^3"]
    #[inline(always)]
    pub fn div_2_3(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV::Div2_3)
    }
    #[doc = "Divide RNG clock by 2^4"]
    #[inline(always)]
    pub fn div_2_4(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV::Div2_4)
    }
    #[doc = "Divide RNG clock by 2^5"]
    #[inline(always)]
    pub fn div_2_5(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV::Div2_5)
    }
    #[doc = "Divide RNG clock by 2^6"]
    #[inline(always)]
    pub fn div_2_6(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV::Div2_6)
    }
    #[doc = "Divide RNG clock by 2^7"]
    #[inline(always)]
    pub fn div_2_7(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV::Div2_7)
    }
    #[doc = "Divide RNG clock by 2^8"]
    #[inline(always)]
    pub fn div_2_8(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV::Div2_8)
    }
    #[doc = "Divide RNG clock by 2^9"]
    #[inline(always)]
    pub fn div_2_9(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV::Div2_9)
    }
    #[doc = "Divide RNG clock by 2^10"]
    #[inline(always)]
    pub fn div_2_10(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV::Div2_10)
    }
    #[doc = "Divide RNG clock by 2^11"]
    #[inline(always)]
    pub fn div_2_11(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV::Div2_11)
    }
    #[doc = "Divide RNG clock by 2^12"]
    #[inline(always)]
    pub fn div_2_12(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV::Div2_12)
    }
    #[doc = "Divide RNG clock by 2^13"]
    #[inline(always)]
    pub fn div_2_13(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV::Div2_13)
    }
    #[doc = "Divide RNG clock by 2^14"]
    #[inline(always)]
    pub fn div_2_14(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV::Div2_14)
    }
    #[doc = "Divide RNG clock by 2^15"]
    #[inline(always)]
    pub fn div_2_15(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV::Div2_15)
    }
}
#[doc = "RNG_CONFIG1\n\nValue on reset: 8"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RNG_CONFIG1 {
    #[doc = "15: Recommended value for config A (NIST certifiable)"]
    ConfigA = 15,
    #[doc = "24: Recommended value for config B (not NIST certifiable)"]
    ConfigB = 24,
}
impl From<RNG_CONFIG1> for u8 {
    #[inline(always)]
    fn from(variant: RNG_CONFIG1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RNG_CONFIG1 {
    type Ux = u8;
}
#[doc = "Field `RNG_CONFIG1` reader - RNG_CONFIG1"]
pub type RNG_CONFIG1_R = crate::FieldReader<RNG_CONFIG1>;
impl RNG_CONFIG1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RNG_CONFIG1> {
        match self.bits {
            15 => Some(RNG_CONFIG1::ConfigA),
            24 => Some(RNG_CONFIG1::ConfigB),
            _ => None,
        }
    }
    #[doc = "Recommended value for config A (NIST certifiable)"]
    #[inline(always)]
    pub fn is_config_a(&self) -> bool {
        *self == RNG_CONFIG1::ConfigA
    }
    #[doc = "Recommended value for config B (not NIST certifiable)"]
    #[inline(always)]
    pub fn is_config_b(&self) -> bool {
        *self == RNG_CONFIG1::ConfigB
    }
}
#[doc = "Field `RNG_CONFIG1` writer - RNG_CONFIG1"]
pub type RNG_CONFIG1_W<'a, REG> = crate::FieldWriter<'a, REG, 6, RNG_CONFIG1>;
impl<'a, REG> RNG_CONFIG1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Recommended value for config A (NIST certifiable)"]
    #[inline(always)]
    pub fn config_a(self) -> &'a mut crate::W<REG> {
        self.variant(RNG_CONFIG1::ConfigA)
    }
    #[doc = "Recommended value for config B (not NIST certifiable)"]
    #[inline(always)]
    pub fn config_b(self) -> &'a mut crate::W<REG> {
        self.variant(RNG_CONFIG1::ConfigB)
    }
}
#[doc = "Field `CONDRST` reader - Conditioning soft reset"]
pub type CONDRST_R = crate::BitReader;
#[doc = "Field `CONDRST` writer - Conditioning soft reset"]
pub type CONDRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "CONFIGLOCK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CONFIGLOCK {
    #[doc = "0: Writes to the RNG_CR configuration bits \\[29:4\\]
are allowed"]
    Enabled = 0,
    #[doc = "1: Writes to the RNG_CR configuration bits \\[29:4\\]
are ignored until the next RNG reset"]
    Disabled = 1,
}
impl From<CONFIGLOCK> for bool {
    #[inline(always)]
    fn from(variant: CONFIGLOCK) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONFIGLOCK` reader - CONFIGLOCK"]
pub type CONFIGLOCK_R = crate::BitReader<CONFIGLOCK>;
impl CONFIGLOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CONFIGLOCK {
        match self.bits {
            false => CONFIGLOCK::Enabled,
            true => CONFIGLOCK::Disabled,
        }
    }
    #[doc = "Writes to the RNG_CR configuration bits \\[29:4\\]
are allowed"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CONFIGLOCK::Enabled
    }
    #[doc = "Writes to the RNG_CR configuration bits \\[29:4\\]
are ignored until the next RNG reset"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CONFIGLOCK::Disabled
    }
}
#[doc = "Field `CONFIGLOCK` writer - CONFIGLOCK"]
pub type CONFIGLOCK_W<'a, REG> = crate::BitWriter<'a, REG, CONFIGLOCK>;
impl<'a, REG> CONFIGLOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writes to the RNG_CR configuration bits \\[29:4\\]
are allowed"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CONFIGLOCK::Enabled)
    }
    #[doc = "Writes to the RNG_CR configuration bits \\[29:4\\]
are ignored until the next RNG reset"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CONFIGLOCK::Disabled)
    }
}
impl R {
    #[doc = "Bit 2 - True random number generator enable"]
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Enable"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt Enable"]
    #[inline(always)]
    pub fn ced(&self) -> CED_R {
        CED_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11 - RNG_CONFIG3"]
    #[inline(always)]
    pub fn rng_config3(&self) -> RNG_CONFIG3_R {
        RNG_CONFIG3_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - NISTC"]
    #[inline(always)]
    pub fn nistc(&self) -> NISTC_R {
        NISTC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - RNG_CONFIG2"]
    #[inline(always)]
    pub fn rng_config2(&self) -> RNG_CONFIG2_R {
        RNG_CONFIG2_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:19 - CLKDIV"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:25 - RNG_CONFIG1"]
    #[inline(always)]
    pub fn rng_config1(&self) -> RNG_CONFIG1_R {
        RNG_CONFIG1_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - Conditioning soft reset"]
    #[inline(always)]
    pub fn condrst(&self) -> CONDRST_R {
        CONDRST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - CONFIGLOCK"]
    #[inline(always)]
    pub fn configlock(&self) -> CONFIGLOCK_R {
        CONFIGLOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - True random number generator enable"]
    #[inline(always)]
    #[must_use]
    pub fn rngen(&mut self) -> RNGEN_W<CRrs> {
        RNGEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ie(&mut self) -> IE_W<CRrs> {
        IE_W::new(self, 3)
    }
    #[doc = "Bit 5 - Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ced(&mut self) -> CED_W<CRrs> {
        CED_W::new(self, 5)
    }
    #[doc = "Bits 8:11 - RNG_CONFIG3"]
    #[inline(always)]
    #[must_use]
    pub fn rng_config3(&mut self) -> RNG_CONFIG3_W<CRrs> {
        RNG_CONFIG3_W::new(self, 8)
    }
    #[doc = "Bit 12 - NISTC"]
    #[inline(always)]
    #[must_use]
    pub fn nistc(&mut self) -> NISTC_W<CRrs> {
        NISTC_W::new(self, 12)
    }
    #[doc = "Bits 13:15 - RNG_CONFIG2"]
    #[inline(always)]
    #[must_use]
    pub fn rng_config2(&mut self) -> RNG_CONFIG2_W<CRrs> {
        RNG_CONFIG2_W::new(self, 13)
    }
    #[doc = "Bits 16:19 - CLKDIV"]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> CLKDIV_W<CRrs> {
        CLKDIV_W::new(self, 16)
    }
    #[doc = "Bits 20:25 - RNG_CONFIG1"]
    #[inline(always)]
    #[must_use]
    pub fn rng_config1(&mut self) -> RNG_CONFIG1_W<CRrs> {
        RNG_CONFIG1_W::new(self, 20)
    }
    #[doc = "Bit 30 - Conditioning soft reset"]
    #[inline(always)]
    #[must_use]
    pub fn condrst(&mut self) -> CONDRST_W<CRrs> {
        CONDRST_W::new(self, 30)
    }
    #[doc = "Bit 31 - CONFIGLOCK"]
    #[inline(always)]
    #[must_use]
    pub fn configlock(&mut self) -> CONFIGLOCK_W<CRrs> {
        CONFIGLOCK_W::new(self, 31)
    }
}
#[doc = "control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CR to value 0x0080_0000"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0x0080_0000;
}

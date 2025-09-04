///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
/**True random number generator enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RNGEN {
    ///0: Random number generator is disabled
    Disabled = 0,
    ///1: Random number generator is enabled
    Enabled = 1,
}
impl From<RNGEN> for bool {
    #[inline(always)]
    fn from(variant: RNGEN) -> Self {
        variant as u8 != 0
    }
}
///Field `RNGEN` reader - True random number generator enable
pub type RNGEN_R = crate::BitReader<RNGEN>;
impl RNGEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RNGEN {
        match self.bits {
            false => RNGEN::Disabled,
            true => RNGEN::Enabled,
        }
    }
    ///Random number generator is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RNGEN::Disabled
    }
    ///Random number generator is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RNGEN::Enabled
    }
}
///Field `RNGEN` writer - True random number generator enable
pub type RNGEN_W<'a, REG> = crate::BitWriter<'a, REG, RNGEN>;
impl<'a, REG> RNGEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Random number generator is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RNGEN::Disabled)
    }
    ///Random number generator is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RNGEN::Enabled)
    }
}
/**Interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IE {
    ///0: RNG interrupt is disabled
    Disabled = 0,
    ///1: RNG interrupt is enabled
    Enabled = 1,
}
impl From<IE> for bool {
    #[inline(always)]
    fn from(variant: IE) -> Self {
        variant as u8 != 0
    }
}
///Field `IE` reader - Interrupt enable
pub type IE_R = crate::BitReader<IE>;
impl IE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IE {
        match self.bits {
            false => IE::Disabled,
            true => IE::Enabled,
        }
    }
    ///RNG interrupt is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IE::Disabled
    }
    ///RNG interrupt is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IE::Enabled
    }
}
///Field `IE` writer - Interrupt enable
pub type IE_W<'a, REG> = crate::BitWriter<'a, REG, IE>;
impl<'a, REG> IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///RNG interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(IE::Disabled)
    }
    ///RNG interrupt is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(IE::Enabled)
    }
}
/**Clock error detection The clock error detection cannot be enabled nor disabled on-the-fly when the RNG is enabled, that is to enable or disable CED, the RNG must be disabled. Writing this bit is taken into account only if the CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CED {
    ///0: Clock error detection is enabled
    Enabled = 0,
    ///1: Clock error detection is disabled
    Disabled = 1,
}
impl From<CED> for bool {
    #[inline(always)]
    fn from(variant: CED) -> Self {
        variant as u8 != 0
    }
}
///Field `CED` reader - Clock error detection The clock error detection cannot be enabled nor disabled on-the-fly when the RNG is enabled, that is to enable or disable CED, the RNG must be disabled. Writing this bit is taken into account only if the CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11.
pub type CED_R = crate::BitReader<CED>;
impl CED_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CED {
        match self.bits {
            false => CED::Enabled,
            true => CED::Disabled,
        }
    }
    ///Clock error detection is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CED::Enabled
    }
    ///Clock error detection is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CED::Disabled
    }
}
///Field `CED` writer - Clock error detection The clock error detection cannot be enabled nor disabled on-the-fly when the RNG is enabled, that is to enable or disable CED, the RNG must be disabled. Writing this bit is taken into account only if the CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11.
pub type CED_W<'a, REG> = crate::BitWriter<'a, REG, CED>;
impl<'a, REG> CED_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clock error detection is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CED::Enabled)
    }
    ///Clock error detection is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CED::Disabled)
    }
}
///Field `ARDIS` reader - Auto reset disable When auto-reset is enabled the application still need to clear the SEIS bit after a noise source error. Writing this bit is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11.
pub type ARDIS_R = crate::BitReader;
///Field `ARDIS` writer - Auto reset disable When auto-reset is enabled the application still need to clear the SEIS bit after a noise source error. Writing this bit is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11.
pub type ARDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
/**RNG configuration 3 Reserved to the RNG configuration (bitfield 3). Refer to RNG_CONFIG1 bitfield for details. If the NISTC bit is cleared in this register RNG_CONFIG3 bitfield values are ignored by RNG.

Value on reset: 13*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RNG_CONFIG3 {
    ///0: Recommended value for config B (not NIST certifiable)
    ConfigB = 0,
    ///13: Recommended value for config A (NIST certifiable)
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
impl crate::IsEnum for RNG_CONFIG3 {}
///Field `RNG_CONFIG3` reader - RNG configuration 3 Reserved to the RNG configuration (bitfield 3). Refer to RNG_CONFIG1 bitfield for details. If the NISTC bit is cleared in this register RNG_CONFIG3 bitfield values are ignored by RNG.
pub type RNG_CONFIG3_R = crate::FieldReader<RNG_CONFIG3>;
impl RNG_CONFIG3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<RNG_CONFIG3> {
        match self.bits {
            0 => Some(RNG_CONFIG3::ConfigB),
            13 => Some(RNG_CONFIG3::ConfigA),
            _ => None,
        }
    }
    ///Recommended value for config B (not NIST certifiable)
    #[inline(always)]
    pub fn is_config_b(&self) -> bool {
        *self == RNG_CONFIG3::ConfigB
    }
    ///Recommended value for config A (NIST certifiable)
    #[inline(always)]
    pub fn is_config_a(&self) -> bool {
        *self == RNG_CONFIG3::ConfigA
    }
}
///Field `RNG_CONFIG3` writer - RNG configuration 3 Reserved to the RNG configuration (bitfield 3). Refer to RNG_CONFIG1 bitfield for details. If the NISTC bit is cleared in this register RNG_CONFIG3 bitfield values are ignored by RNG.
pub type RNG_CONFIG3_W<'a, REG> = crate::FieldWriter<'a, REG, 4, RNG_CONFIG3>;
impl<'a, REG> RNG_CONFIG3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Recommended value for config B (not NIST certifiable)
    #[inline(always)]
    pub fn config_b(self) -> &'a mut crate::W<REG> {
        self.variant(RNG_CONFIG3::ConfigB)
    }
    ///Recommended value for config A (NIST certifiable)
    #[inline(always)]
    pub fn config_a(self) -> &'a mut crate::W<REG> {
        self.variant(RNG_CONFIG3::ConfigA)
    }
}
/**NIST custom two conditioning loops are performed and 256 bits of noise source are used. Writing this bit is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NISTC {
    ///0: Hardware default values for NIST compliant RNG. In this configuration per 128-bit output two conditioning loops are performed and 256 bits of noise source are used
    Default = 0,
    ///1: Custom values for NIST compliant RNG
    Custom = 1,
}
impl From<NISTC> for bool {
    #[inline(always)]
    fn from(variant: NISTC) -> Self {
        variant as u8 != 0
    }
}
///Field `NISTC` reader - NIST custom two conditioning loops are performed and 256 bits of noise source are used. Writing this bit is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11.
pub type NISTC_R = crate::BitReader<NISTC>;
impl NISTC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> NISTC {
        match self.bits {
            false => NISTC::Default,
            true => NISTC::Custom,
        }
    }
    ///Hardware default values for NIST compliant RNG. In this configuration per 128-bit output two conditioning loops are performed and 256 bits of noise source are used
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == NISTC::Default
    }
    ///Custom values for NIST compliant RNG
    #[inline(always)]
    pub fn is_custom(&self) -> bool {
        *self == NISTC::Custom
    }
}
///Field `NISTC` writer - NIST custom two conditioning loops are performed and 256 bits of noise source are used. Writing this bit is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11.
pub type NISTC_W<'a, REG> = crate::BitWriter<'a, REG, NISTC>;
impl<'a, REG> NISTC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Hardware default values for NIST compliant RNG. In this configuration per 128-bit output two conditioning loops are performed and 256 bits of noise source are used
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(NISTC::Default)
    }
    ///Custom values for NIST compliant RNG
    #[inline(always)]
    pub fn custom(self) -> &'a mut crate::W<REG> {
        self.variant(NISTC::Custom)
    }
}
/**RNG configuration 2 Reserved to the RNG configuration (bitfield 2). Bit 13 can be set when RNG power consumption is critical. See Section120.3.8: RNG low-power use. Refer to the RNG_CONFIG1 bitfield for details.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RNG_CONFIG2 {
    ///0: Recommended value for config A and B
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
impl crate::IsEnum for RNG_CONFIG2 {}
///Field `RNG_CONFIG2` reader - RNG configuration 2 Reserved to the RNG configuration (bitfield 2). Bit 13 can be set when RNG power consumption is critical. See Section120.3.8: RNG low-power use. Refer to the RNG_CONFIG1 bitfield for details.
pub type RNG_CONFIG2_R = crate::FieldReader<RNG_CONFIG2>;
impl RNG_CONFIG2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<RNG_CONFIG2> {
        match self.bits {
            0 => Some(RNG_CONFIG2::ConfigAB),
            _ => None,
        }
    }
    ///Recommended value for config A and B
    #[inline(always)]
    pub fn is_config_a_b(&self) -> bool {
        *self == RNG_CONFIG2::ConfigAB
    }
}
///Field `RNG_CONFIG2` writer - RNG configuration 2 Reserved to the RNG configuration (bitfield 2). Bit 13 can be set when RNG power consumption is critical. See Section120.3.8: RNG low-power use. Refer to the RNG_CONFIG1 bitfield for details.
pub type RNG_CONFIG2_W<'a, REG> = crate::FieldWriter<'a, REG, 3, RNG_CONFIG2>;
impl<'a, REG> RNG_CONFIG2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Recommended value for config A and B
    #[inline(always)]
    pub fn config_a_b(self) -> &'a mut crate::W<REG> {
        self.variant(RNG_CONFIG2::ConfigAB)
    }
}
/**Clock divider factor This value used to configure an internal programmable divider (from 1 to 16) acting on the incoming RNG clock. These bits can be written only when the core is disabled (RNGEN1=10). ... Writing these bits is taken into account only if the CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKDIV {
    ///0: Internal RNG clock after divider is similar to incoming RNG clock
    Div1 = 0,
    ///1: Divide RNG clock by 2^1
    Div2 = 1,
    ///2: Divide RNG clock by 2^2
    Div4 = 2,
    ///3: Divide RNG clock by 2^3
    Div8 = 3,
    ///4: Divide RNG clock by 2^4
    Div16 = 4,
    ///5: Divide RNG clock by 2^5
    Div32 = 5,
    ///6: Divide RNG clock by 2^6
    Div64 = 6,
    ///7: Divide RNG clock by 2^7
    Div128 = 7,
    ///8: Divide RNG clock by 2^8
    Div256 = 8,
    ///9: Divide RNG clock by 2^9
    Div512 = 9,
    ///10: Divide RNG clock by 2^10
    Div1024 = 10,
    ///11: Divide RNG clock by 2^11
    Div2048 = 11,
    ///12: Divide RNG clock by 2^12
    Div4096 = 12,
    ///13: Divide RNG clock by 2^13
    Div8192 = 13,
    ///14: Divide RNG clock by 2^14
    Div16384 = 14,
    ///15: Divide RNG clock by 2^15
    Div32768 = 15,
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
impl crate::IsEnum for CLKDIV {}
///Field `CLKDIV` reader - Clock divider factor This value used to configure an internal programmable divider (from 1 to 16) acting on the incoming RNG clock. These bits can be written only when the core is disabled (RNGEN1=10). ... Writing these bits is taken into account only if the CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11.
pub type CLKDIV_R = crate::FieldReader<CLKDIV>;
impl CLKDIV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CLKDIV {
        match self.bits {
            0 => CLKDIV::Div1,
            1 => CLKDIV::Div2,
            2 => CLKDIV::Div4,
            3 => CLKDIV::Div8,
            4 => CLKDIV::Div16,
            5 => CLKDIV::Div32,
            6 => CLKDIV::Div64,
            7 => CLKDIV::Div128,
            8 => CLKDIV::Div256,
            9 => CLKDIV::Div512,
            10 => CLKDIV::Div1024,
            11 => CLKDIV::Div2048,
            12 => CLKDIV::Div4096,
            13 => CLKDIV::Div8192,
            14 => CLKDIV::Div16384,
            15 => CLKDIV::Div32768,
            _ => unreachable!(),
        }
    }
    ///Internal RNG clock after divider is similar to incoming RNG clock
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == CLKDIV::Div1
    }
    ///Divide RNG clock by 2^1
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == CLKDIV::Div2
    }
    ///Divide RNG clock by 2^2
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == CLKDIV::Div4
    }
    ///Divide RNG clock by 2^3
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == CLKDIV::Div8
    }
    ///Divide RNG clock by 2^4
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == CLKDIV::Div16
    }
    ///Divide RNG clock by 2^5
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == CLKDIV::Div32
    }
    ///Divide RNG clock by 2^6
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == CLKDIV::Div64
    }
    ///Divide RNG clock by 2^7
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == CLKDIV::Div128
    }
    ///Divide RNG clock by 2^8
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == CLKDIV::Div256
    }
    ///Divide RNG clock by 2^9
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == CLKDIV::Div512
    }
    ///Divide RNG clock by 2^10
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == CLKDIV::Div1024
    }
    ///Divide RNG clock by 2^11
    #[inline(always)]
    pub fn is_div2048(&self) -> bool {
        *self == CLKDIV::Div2048
    }
    ///Divide RNG clock by 2^12
    #[inline(always)]
    pub fn is_div4096(&self) -> bool {
        *self == CLKDIV::Div4096
    }
    ///Divide RNG clock by 2^13
    #[inline(always)]
    pub fn is_div8192(&self) -> bool {
        *self == CLKDIV::Div8192
    }
    ///Divide RNG clock by 2^14
    #[inline(always)]
    pub fn is_div16384(&self) -> bool {
        *self == CLKDIV::Div16384
    }
    ///Divide RNG clock by 2^15
    #[inline(always)]
    pub fn is_div32768(&self) -> bool {
        *self == CLKDIV::Div32768
    }
}
///Field `CLKDIV` writer - Clock divider factor This value used to configure an internal programmable divider (from 1 to 16) acting on the incoming RNG clock. These bits can be written only when the core is disabled (RNGEN1=10). ... Writing these bits is taken into account only if the CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11.
pub type CLKDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4, CLKDIV, crate::Safe>;
impl<'a, REG> CLKDIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Internal RNG clock after divider is similar to incoming RNG clock
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV::Div1)
    }
    ///Divide RNG clock by 2^1
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV::Div2)
    }
    ///Divide RNG clock by 2^2
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV::Div4)
    }
    ///Divide RNG clock by 2^3
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV::Div8)
    }
    ///Divide RNG clock by 2^4
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV::Div16)
    }
    ///Divide RNG clock by 2^5
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV::Div32)
    }
    ///Divide RNG clock by 2^6
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV::Div64)
    }
    ///Divide RNG clock by 2^7
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV::Div128)
    }
    ///Divide RNG clock by 2^8
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV::Div256)
    }
    ///Divide RNG clock by 2^9
    #[inline(always)]
    pub fn div512(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV::Div512)
    }
    ///Divide RNG clock by 2^10
    #[inline(always)]
    pub fn div1024(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV::Div1024)
    }
    ///Divide RNG clock by 2^11
    #[inline(always)]
    pub fn div2048(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV::Div2048)
    }
    ///Divide RNG clock by 2^12
    #[inline(always)]
    pub fn div4096(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV::Div4096)
    }
    ///Divide RNG clock by 2^13
    #[inline(always)]
    pub fn div8192(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV::Div8192)
    }
    ///Divide RNG clock by 2^14
    #[inline(always)]
    pub fn div16384(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV::Div16384)
    }
    ///Divide RNG clock by 2^15
    #[inline(always)]
    pub fn div32768(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV::Div32768)
    }
}
/**RNG configuration 1 Reserved to the RNG configuration (bitfield 1). Must be initialized using the recommended value documented in Section120.6: RNG entropy source validation. Writing any bit of RNG_CONFIG1 is taken into account only if the CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11.

Value on reset: 8*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RNG_CONFIG1 {
    ///15: Recommended value for config A (NIST certifiable)
    ConfigA = 15,
    ///24: Recommended value for config B (not NIST certifiable)
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
impl crate::IsEnum for RNG_CONFIG1 {}
///Field `RNG_CONFIG1` reader - RNG configuration 1 Reserved to the RNG configuration (bitfield 1). Must be initialized using the recommended value documented in Section120.6: RNG entropy source validation. Writing any bit of RNG_CONFIG1 is taken into account only if the CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11.
pub type RNG_CONFIG1_R = crate::FieldReader<RNG_CONFIG1>;
impl RNG_CONFIG1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<RNG_CONFIG1> {
        match self.bits {
            15 => Some(RNG_CONFIG1::ConfigA),
            24 => Some(RNG_CONFIG1::ConfigB),
            _ => None,
        }
    }
    ///Recommended value for config A (NIST certifiable)
    #[inline(always)]
    pub fn is_config_a(&self) -> bool {
        *self == RNG_CONFIG1::ConfigA
    }
    ///Recommended value for config B (not NIST certifiable)
    #[inline(always)]
    pub fn is_config_b(&self) -> bool {
        *self == RNG_CONFIG1::ConfigB
    }
}
///Field `RNG_CONFIG1` writer - RNG configuration 1 Reserved to the RNG configuration (bitfield 1). Must be initialized using the recommended value documented in Section120.6: RNG entropy source validation. Writing any bit of RNG_CONFIG1 is taken into account only if the CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11.
pub type RNG_CONFIG1_W<'a, REG> = crate::FieldWriter<'a, REG, 6, RNG_CONFIG1>;
impl<'a, REG> RNG_CONFIG1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Recommended value for config A (NIST certifiable)
    #[inline(always)]
    pub fn config_a(self) -> &'a mut crate::W<REG> {
        self.variant(RNG_CONFIG1::ConfigA)
    }
    ///Recommended value for config B (not NIST certifiable)
    #[inline(always)]
    pub fn config_b(self) -> &'a mut crate::W<REG> {
        self.variant(RNG_CONFIG1::ConfigB)
    }
}
///Field `CONDRST` reader - Conditioning soft reset Write 1 and then write 0 to reset the conditioning logic, clear all the FIFOs and start a new RNG initialization process, with RNG_SR cleared. Registers RNG_CR and RNG_HTCR are not changed by CONDRST. This bit must be set to 1 in the same access that set any configuration bits \[29:4\]. In other words, when CONDRST bit is set to 1 correct configuration in bits \[29:4\] must also be written. When CONDRST is set to 0 by the software, its value goes to 0 when the reset process is done. It takes about 2 AHB clock cycles + 2 RNG clock cycles.
pub type CONDRST_R = crate::BitReader;
///Field `CONDRST` writer - Conditioning soft reset Write 1 and then write 0 to reset the conditioning logic, clear all the FIFOs and start a new RNG initialization process, with RNG_SR cleared. Registers RNG_CR and RNG_HTCR are not changed by CONDRST. This bit must be set to 1 in the same access that set any configuration bits \[29:4\]. In other words, when CONDRST bit is set to 1 correct configuration in bits \[29:4\] must also be written. When CONDRST is set to 0 by the software, its value goes to 0 when the reset process is done. It takes about 2 AHB clock cycles + 2 RNG clock cycles.
pub type CONDRST_W<'a, REG> = crate::BitWriter<'a, REG>;
/**RNG Config lock This bitfield is set once: if this bit is set it can only be reset to 0 if RNG is reset.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CONFIGLOCK {
    ///0: Writes to the RNG_CR configuration bits \[29:4\] are allowed
    Enabled = 0,
    ///1: Writes to the RNG_CR configuration bits \[29:4\] are ignored until the next RNG reset
    Disabled = 1,
}
impl From<CONFIGLOCK> for bool {
    #[inline(always)]
    fn from(variant: CONFIGLOCK) -> Self {
        variant as u8 != 0
    }
}
///Field `CONFIGLOCK` reader - RNG Config lock This bitfield is set once: if this bit is set it can only be reset to 0 if RNG is reset.
pub type CONFIGLOCK_R = crate::BitReader<CONFIGLOCK>;
impl CONFIGLOCK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CONFIGLOCK {
        match self.bits {
            false => CONFIGLOCK::Enabled,
            true => CONFIGLOCK::Disabled,
        }
    }
    ///Writes to the RNG_CR configuration bits \[29:4\] are allowed
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CONFIGLOCK::Enabled
    }
    ///Writes to the RNG_CR configuration bits \[29:4\] are ignored until the next RNG reset
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CONFIGLOCK::Disabled
    }
}
///Field `CONFIGLOCK` writer - RNG Config lock This bitfield is set once: if this bit is set it can only be reset to 0 if RNG is reset.
pub type CONFIGLOCK_W<'a, REG> = crate::BitWriter<'a, REG, CONFIGLOCK>;
impl<'a, REG> CONFIGLOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Writes to the RNG_CR configuration bits \[29:4\] are allowed
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CONFIGLOCK::Enabled)
    }
    ///Writes to the RNG_CR configuration bits \[29:4\] are ignored until the next RNG reset
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CONFIGLOCK::Disabled)
    }
}
impl R {
    ///Bit 2 - True random number generator enable
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Interrupt enable
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - Clock error detection The clock error detection cannot be enabled nor disabled on-the-fly when the RNG is enabled, that is to enable or disable CED, the RNG must be disabled. Writing this bit is taken into account only if the CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11.
    #[inline(always)]
    pub fn ced(&self) -> CED_R {
        CED_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - Auto reset disable When auto-reset is enabled the application still need to clear the SEIS bit after a noise source error. Writing this bit is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11.
    #[inline(always)]
    pub fn ardis(&self) -> ARDIS_R {
        ARDIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:11 - RNG configuration 3 Reserved to the RNG configuration (bitfield 3). Refer to RNG_CONFIG1 bitfield for details. If the NISTC bit is cleared in this register RNG_CONFIG3 bitfield values are ignored by RNG.
    #[inline(always)]
    pub fn rng_config3(&self) -> RNG_CONFIG3_R {
        RNG_CONFIG3_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 12 - NIST custom two conditioning loops are performed and 256 bits of noise source are used. Writing this bit is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11.
    #[inline(always)]
    pub fn nistc(&self) -> NISTC_R {
        NISTC_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:15 - RNG configuration 2 Reserved to the RNG configuration (bitfield 2). Bit 13 can be set when RNG power consumption is critical. See Section120.3.8: RNG low-power use. Refer to the RNG_CONFIG1 bitfield for details.
    #[inline(always)]
    pub fn rng_config2(&self) -> RNG_CONFIG2_R {
        RNG_CONFIG2_R::new(((self.bits >> 13) & 7) as u8)
    }
    ///Bits 16:19 - Clock divider factor This value used to configure an internal programmable divider (from 1 to 16) acting on the incoming RNG clock. These bits can be written only when the core is disabled (RNGEN1=10). ... Writing these bits is taken into account only if the CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11.
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:25 - RNG configuration 1 Reserved to the RNG configuration (bitfield 1). Must be initialized using the recommended value documented in Section120.6: RNG entropy source validation. Writing any bit of RNG_CONFIG1 is taken into account only if the CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11.
    #[inline(always)]
    pub fn rng_config1(&self) -> RNG_CONFIG1_R {
        RNG_CONFIG1_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
    ///Bit 30 - Conditioning soft reset Write 1 and then write 0 to reset the conditioning logic, clear all the FIFOs and start a new RNG initialization process, with RNG_SR cleared. Registers RNG_CR and RNG_HTCR are not changed by CONDRST. This bit must be set to 1 in the same access that set any configuration bits \[29:4\]. In other words, when CONDRST bit is set to 1 correct configuration in bits \[29:4\] must also be written. When CONDRST is set to 0 by the software, its value goes to 0 when the reset process is done. It takes about 2 AHB clock cycles + 2 RNG clock cycles.
    #[inline(always)]
    pub fn condrst(&self) -> CONDRST_R {
        CONDRST_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - RNG Config lock This bitfield is set once: if this bit is set it can only be reset to 0 if RNG is reset.
    #[inline(always)]
    pub fn configlock(&self) -> CONFIGLOCK_R {
        CONFIGLOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("rngen", &self.rngen())
            .field("ie", &self.ie())
            .field("ced", &self.ced())
            .field("ardis", &self.ardis())
            .field("rng_config3", &self.rng_config3())
            .field("nistc", &self.nistc())
            .field("rng_config2", &self.rng_config2())
            .field("clkdiv", &self.clkdiv())
            .field("rng_config1", &self.rng_config1())
            .field("condrst", &self.condrst())
            .field("configlock", &self.configlock())
            .finish()
    }
}
impl W {
    ///Bit 2 - True random number generator enable
    #[inline(always)]
    pub fn rngen(&mut self) -> RNGEN_W<CRrs> {
        RNGEN_W::new(self, 2)
    }
    ///Bit 3 - Interrupt enable
    #[inline(always)]
    pub fn ie(&mut self) -> IE_W<CRrs> {
        IE_W::new(self, 3)
    }
    ///Bit 5 - Clock error detection The clock error detection cannot be enabled nor disabled on-the-fly when the RNG is enabled, that is to enable or disable CED, the RNG must be disabled. Writing this bit is taken into account only if the CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11.
    #[inline(always)]
    pub fn ced(&mut self) -> CED_W<CRrs> {
        CED_W::new(self, 5)
    }
    ///Bit 7 - Auto reset disable When auto-reset is enabled the application still need to clear the SEIS bit after a noise source error. Writing this bit is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11.
    #[inline(always)]
    pub fn ardis(&mut self) -> ARDIS_W<CRrs> {
        ARDIS_W::new(self, 7)
    }
    ///Bits 8:11 - RNG configuration 3 Reserved to the RNG configuration (bitfield 3). Refer to RNG_CONFIG1 bitfield for details. If the NISTC bit is cleared in this register RNG_CONFIG3 bitfield values are ignored by RNG.
    #[inline(always)]
    pub fn rng_config3(&mut self) -> RNG_CONFIG3_W<CRrs> {
        RNG_CONFIG3_W::new(self, 8)
    }
    ///Bit 12 - NIST custom two conditioning loops are performed and 256 bits of noise source are used. Writing this bit is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11.
    #[inline(always)]
    pub fn nistc(&mut self) -> NISTC_W<CRrs> {
        NISTC_W::new(self, 12)
    }
    ///Bits 13:15 - RNG configuration 2 Reserved to the RNG configuration (bitfield 2). Bit 13 can be set when RNG power consumption is critical. See Section120.3.8: RNG low-power use. Refer to the RNG_CONFIG1 bitfield for details.
    #[inline(always)]
    pub fn rng_config2(&mut self) -> RNG_CONFIG2_W<CRrs> {
        RNG_CONFIG2_W::new(self, 13)
    }
    ///Bits 16:19 - Clock divider factor This value used to configure an internal programmable divider (from 1 to 16) acting on the incoming RNG clock. These bits can be written only when the core is disabled (RNGEN1=10). ... Writing these bits is taken into account only if the CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11.
    #[inline(always)]
    pub fn clkdiv(&mut self) -> CLKDIV_W<CRrs> {
        CLKDIV_W::new(self, 16)
    }
    ///Bits 20:25 - RNG configuration 1 Reserved to the RNG configuration (bitfield 1). Must be initialized using the recommended value documented in Section120.6: RNG entropy source validation. Writing any bit of RNG_CONFIG1 is taken into account only if the CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11.
    #[inline(always)]
    pub fn rng_config1(&mut self) -> RNG_CONFIG1_W<CRrs> {
        RNG_CONFIG1_W::new(self, 20)
    }
    ///Bit 30 - Conditioning soft reset Write 1 and then write 0 to reset the conditioning logic, clear all the FIFOs and start a new RNG initialization process, with RNG_SR cleared. Registers RNG_CR and RNG_HTCR are not changed by CONDRST. This bit must be set to 1 in the same access that set any configuration bits \[29:4\]. In other words, when CONDRST bit is set to 1 correct configuration in bits \[29:4\] must also be written. When CONDRST is set to 0 by the software, its value goes to 0 when the reset process is done. It takes about 2 AHB clock cycles + 2 RNG clock cycles.
    #[inline(always)]
    pub fn condrst(&mut self) -> CONDRST_W<CRrs> {
        CONDRST_W::new(self, 30)
    }
    ///Bit 31 - RNG Config lock This bitfield is set once: if this bit is set it can only be reset to 0 if RNG is reset.
    #[inline(always)]
    pub fn configlock(&mut self) -> CONFIGLOCK_W<CRrs> {
        CONFIGLOCK_W::new(self, 31)
    }
}
/**RNG control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#RNG:CR)*/
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
///`reset()` method sets CR to value 0x0080_0d00
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0x0080_0d00;
}

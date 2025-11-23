///Register `CFGR1` reader
pub type R = crate::R<CFGR1rs>;
///Register `CFGR1` writer
pub type W = crate::W<CFGR1rs>;
/**Memory mapping selection bits

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MEM_MODE {
    ///0: Main flash memory mapped at zero address
    MainFlash = 0,
    ///2: System flash memory mapped at zero address
    SystemFlash = 2,
    ///3: Embedded SRAM mapped at zero address
    Sram = 3,
}
impl From<MEM_MODE> for u8 {
    #[inline(always)]
    fn from(variant: MEM_MODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MEM_MODE {
    type Ux = u8;
}
impl crate::IsEnum for MEM_MODE {}
///Field `MEM_MODE` reader - Memory mapping selection bits
pub type MEM_MODE_R = crate::FieldReader<MEM_MODE>;
impl MEM_MODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<MEM_MODE> {
        match self.bits {
            0 => Some(MEM_MODE::MainFlash),
            2 => Some(MEM_MODE::SystemFlash),
            3 => Some(MEM_MODE::Sram),
            _ => None,
        }
    }
    ///Main flash memory mapped at zero address
    #[inline(always)]
    pub fn is_main_flash(&self) -> bool {
        *self == MEM_MODE::MainFlash
    }
    ///System flash memory mapped at zero address
    #[inline(always)]
    pub fn is_system_flash(&self) -> bool {
        *self == MEM_MODE::SystemFlash
    }
    ///Embedded SRAM mapped at zero address
    #[inline(always)]
    pub fn is_sram(&self) -> bool {
        *self == MEM_MODE::Sram
    }
}
///Field `MEM_MODE` writer - Memory mapping selection bits
pub type MEM_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MEM_MODE>;
impl<'a, REG> MEM_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Main flash memory mapped at zero address
    #[inline(always)]
    pub fn main_flash(self) -> &'a mut crate::W<REG> {
        self.variant(MEM_MODE::MainFlash)
    }
    ///System flash memory mapped at zero address
    #[inline(always)]
    pub fn system_flash(self) -> &'a mut crate::W<REG> {
        self.variant(MEM_MODE::SystemFlash)
    }
    ///Embedded SRAM mapped at zero address
    #[inline(always)]
    pub fn sram(self) -> &'a mut crate::W<REG> {
        self.variant(MEM_MODE::Sram)
    }
}
/**PA11_RMP

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PA11_RMP {
    ///0: PA11 pin connected to PA11 GPIO
    Normal = 0,
    ///1: PA11 pin connected to PA9 GPIO
    Remap = 1,
}
impl From<PA11_RMP> for bool {
    #[inline(always)]
    fn from(variant: PA11_RMP) -> Self {
        variant as u8 != 0
    }
}
///Field `PA11_RMP` reader - PA11_RMP
pub type PA11_RMP_R = crate::BitReader<PA11_RMP>;
impl PA11_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PA11_RMP {
        match self.bits {
            false => PA11_RMP::Normal,
            true => PA11_RMP::Remap,
        }
    }
    ///PA11 pin connected to PA11 GPIO
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PA11_RMP::Normal
    }
    ///PA11 pin connected to PA9 GPIO
    #[inline(always)]
    pub fn is_remap(&self) -> bool {
        *self == PA11_RMP::Remap
    }
}
///Field `PA11_RMP` writer - PA11_RMP
pub type PA11_RMP_W<'a, REG> = crate::BitWriter<'a, REG, PA11_RMP>;
impl<'a, REG> PA11_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PA11 pin connected to PA11 GPIO
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PA11_RMP::Normal)
    }
    ///PA11 pin connected to PA9 GPIO
    #[inline(always)]
    pub fn remap(self) -> &'a mut crate::W<REG> {
        self.variant(PA11_RMP::Remap)
    }
}
/**PA11 and PA12 remapping bit.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PA12_RMP {
    ///0: PA12 pin connected to PA12 GPIO
    Normal = 0,
    ///1: PA12 pin connected to PA10 GPIO
    Remap = 1,
}
impl From<PA12_RMP> for bool {
    #[inline(always)]
    fn from(variant: PA12_RMP) -> Self {
        variant as u8 != 0
    }
}
///Field `PA12_RMP` reader - PA11 and PA12 remapping bit.
pub type PA12_RMP_R = crate::BitReader<PA12_RMP>;
impl PA12_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PA12_RMP {
        match self.bits {
            false => PA12_RMP::Normal,
            true => PA12_RMP::Remap,
        }
    }
    ///PA12 pin connected to PA12 GPIO
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PA12_RMP::Normal
    }
    ///PA12 pin connected to PA10 GPIO
    #[inline(always)]
    pub fn is_remap(&self) -> bool {
        *self == PA12_RMP::Remap
    }
}
///Field `PA12_RMP` writer - PA11 and PA12 remapping bit.
pub type PA12_RMP_W<'a, REG> = crate::BitWriter<'a, REG, PA12_RMP>;
impl<'a, REG> PA12_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PA12 pin connected to PA12 GPIO
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PA12_RMP::Normal)
    }
    ///PA12 pin connected to PA10 GPIO
    #[inline(always)]
    pub fn remap(self) -> &'a mut crate::W<REG> {
        self.variant(PA12_RMP::Remap)
    }
}
/**IR output polarity selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IR_POL {
    ///0: Output of IRTIM is not inverted
    Normal = 0,
    ///1: Output of IRTIM is inverted
    Inverted = 1,
}
impl From<IR_POL> for bool {
    #[inline(always)]
    fn from(variant: IR_POL) -> Self {
        variant as u8 != 0
    }
}
///Field `IR_POL` reader - IR output polarity selection
pub type IR_POL_R = crate::BitReader<IR_POL>;
impl IR_POL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IR_POL {
        match self.bits {
            false => IR_POL::Normal,
            true => IR_POL::Inverted,
        }
    }
    ///Output of IRTIM is not inverted
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == IR_POL::Normal
    }
    ///Output of IRTIM is inverted
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == IR_POL::Inverted
    }
}
///Field `IR_POL` writer - IR output polarity selection
pub type IR_POL_W<'a, REG> = crate::BitWriter<'a, REG, IR_POL>;
impl<'a, REG> IR_POL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output of IRTIM is not inverted
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(IR_POL::Normal)
    }
    ///Output of IRTIM is inverted
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(IR_POL::Inverted)
    }
}
/**IR Modulation Envelope signal selection.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IR_MOD {
    ///0: IR modulation envelope from TIM16
    Tim16 = 0,
    ///1: IR modulation envelope from USART1
    Usart1 = 1,
    ///2: IR modulation envelope from USART4
    Usart4 = 2,
}
impl From<IR_MOD> for u8 {
    #[inline(always)]
    fn from(variant: IR_MOD) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IR_MOD {
    type Ux = u8;
}
impl crate::IsEnum for IR_MOD {}
///Field `IR_MOD` reader - IR Modulation Envelope signal selection.
pub type IR_MOD_R = crate::FieldReader<IR_MOD>;
impl IR_MOD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<IR_MOD> {
        match self.bits {
            0 => Some(IR_MOD::Tim16),
            1 => Some(IR_MOD::Usart1),
            2 => Some(IR_MOD::Usart4),
            _ => None,
        }
    }
    ///IR modulation envelope from TIM16
    #[inline(always)]
    pub fn is_tim16(&self) -> bool {
        *self == IR_MOD::Tim16
    }
    ///IR modulation envelope from USART1
    #[inline(always)]
    pub fn is_usart1(&self) -> bool {
        *self == IR_MOD::Usart1
    }
    ///IR modulation envelope from USART4
    #[inline(always)]
    pub fn is_usart4(&self) -> bool {
        *self == IR_MOD::Usart4
    }
}
///Field `IR_MOD` writer - IR Modulation Envelope signal selection.
pub type IR_MOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2, IR_MOD>;
impl<'a, REG> IR_MOD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///IR modulation envelope from TIM16
    #[inline(always)]
    pub fn tim16(self) -> &'a mut crate::W<REG> {
        self.variant(IR_MOD::Tim16)
    }
    ///IR modulation envelope from USART1
    #[inline(always)]
    pub fn usart1(self) -> &'a mut crate::W<REG> {
        self.variant(IR_MOD::Usart1)
    }
    ///IR modulation envelope from USART4
    #[inline(always)]
    pub fn usart4(self) -> &'a mut crate::W<REG> {
        self.variant(IR_MOD::Usart4)
    }
}
/**I/O analog switch voltage booster enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOOSTEN {
    ///0: supply analog switches from VDD
    Vdd = 0,
    ///1: supply analog switches from dedicated voltage booster
    Boost = 1,
}
impl From<BOOSTEN> for bool {
    #[inline(always)]
    fn from(variant: BOOSTEN) -> Self {
        variant as u8 != 0
    }
}
///Field `BOOSTEN` reader - I/O analog switch voltage booster enable
pub type BOOSTEN_R = crate::BitReader<BOOSTEN>;
impl BOOSTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BOOSTEN {
        match self.bits {
            false => BOOSTEN::Vdd,
            true => BOOSTEN::Boost,
        }
    }
    ///supply analog switches from VDD
    #[inline(always)]
    pub fn is_vdd(&self) -> bool {
        *self == BOOSTEN::Vdd
    }
    ///supply analog switches from dedicated voltage booster
    #[inline(always)]
    pub fn is_boost(&self) -> bool {
        *self == BOOSTEN::Boost
    }
}
///Field `BOOSTEN` writer - I/O analog switch voltage booster enable
pub type BOOSTEN_W<'a, REG> = crate::BitWriter<'a, REG, BOOSTEN>;
impl<'a, REG> BOOSTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///supply analog switches from VDD
    #[inline(always)]
    pub fn vdd(self) -> &'a mut crate::W<REG> {
        self.variant(BOOSTEN::Vdd)
    }
    ///supply analog switches from dedicated voltage booster
    #[inline(always)]
    pub fn boost(self) -> &'a mut crate::W<REG> {
        self.variant(BOOSTEN::Boost)
    }
}
/**Strobe signal bit for UCPD1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCPD1_STROBE {
    ///1: Disconnect the UCPD pull-down resistors
    Disconnect = 1,
}
impl From<UCPD1_STROBE> for bool {
    #[inline(always)]
    fn from(variant: UCPD1_STROBE) -> Self {
        variant as u8 != 0
    }
}
///Field `UCPD1_STROBE` reader - Strobe signal bit for UCPD1
pub type UCPD1_STROBE_R = crate::BitReader<UCPD1_STROBE>;
impl UCPD1_STROBE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<UCPD1_STROBE> {
        match self.bits {
            true => Some(UCPD1_STROBE::Disconnect),
            _ => None,
        }
    }
    ///Disconnect the UCPD pull-down resistors
    #[inline(always)]
    pub fn is_disconnect(&self) -> bool {
        *self == UCPD1_STROBE::Disconnect
    }
}
///Field `UCPD1_STROBE` writer - Strobe signal bit for UCPD1
pub type UCPD1_STROBE_W<'a, REG> = crate::BitWriter<'a, REG, UCPD1_STROBE>;
impl<'a, REG> UCPD1_STROBE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disconnect the UCPD pull-down resistors
    #[inline(always)]
    pub fn disconnect(self) -> &'a mut crate::W<REG> {
        self.variant(UCPD1_STROBE::Disconnect)
    }
}
///Field `UCPD2_STROBE` reader - Strobe signal bit for UCPD2
pub use UCPD1_STROBE_R as UCPD2_STROBE_R;
///Field `UCPD2_STROBE` writer - Strobe signal bit for UCPD2
pub use UCPD1_STROBE_W as UCPD2_STROBE_W;
/**Fast Mode Plus (FM+) driving capability activation bits

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C_PBX_FMP {
    ///0: Uses normal GPIO drive
    Disabled = 0,
    ///1: Uses I2C FastMode+ drive
    Enabled = 1,
}
impl From<I2C_PBX_FMP> for bool {
    #[inline(always)]
    fn from(variant: I2C_PBX_FMP) -> Self {
        variant as u8 != 0
    }
}
///Field `I2C_PBx_FMP` reader - Fast Mode Plus (FM+) driving capability activation bits
pub type I2C_PBX_FMP_R = crate::BitReader<I2C_PBX_FMP>;
impl I2C_PBX_FMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2C_PBX_FMP {
        match self.bits {
            false => I2C_PBX_FMP::Disabled,
            true => I2C_PBX_FMP::Enabled,
        }
    }
    ///Uses normal GPIO drive
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == I2C_PBX_FMP::Disabled
    }
    ///Uses I2C FastMode+ drive
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == I2C_PBX_FMP::Enabled
    }
}
///Field `I2C_PBx_FMP` writer - Fast Mode Plus (FM+) driving capability activation bits
pub type I2C_PBX_FMP_W<'a, REG> = crate::BitWriter<'a, REG, I2C_PBX_FMP>;
impl<'a, REG> I2C_PBX_FMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Uses normal GPIO drive
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_PBX_FMP::Disabled)
    }
    ///Uses I2C FastMode+ drive
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_PBX_FMP::Enabled)
    }
}
///Field `I2C_PB7_FMP` reader - I2C_PB7_FMP
pub use I2C_PBX_FMP_R as I2C_PB7_FMP_R;
///Field `I2C_PB8_FMP` reader - I2C_PB8_FMP
pub use I2C_PBX_FMP_R as I2C_PB8_FMP_R;
///Field `I2C_PB9_FMP` reader - I2C_PB9_FMP
pub use I2C_PBX_FMP_R as I2C_PB9_FMP_R;
///Field `I2C1_FMP` reader - FM+ driving capability activation for I2C1
pub use I2C_PBX_FMP_R as I2C1_FMP_R;
///Field `I2C2_FMP` reader - FM+ driving capability activation for I2C2
pub use I2C_PBX_FMP_R as I2C2_FMP_R;
///Field `I2C_PA9_FMP` reader - Fast Mode Plus (FM+) driving capability activation bits
pub use I2C_PBX_FMP_R as I2C_PA9_FMP_R;
///Field `I2C_PA10_FMP` reader - Fast Mode Plus (FM+) driving capability activation bits
pub use I2C_PBX_FMP_R as I2C_PA10_FMP_R;
///Field `I2C3_FMP` reader - I2C3_FMP
pub use I2C_PBX_FMP_R as I2C3_FMP_R;
///Field `I2C_PB7_FMP` writer - I2C_PB7_FMP
pub use I2C_PBX_FMP_W as I2C_PB7_FMP_W;
///Field `I2C_PB8_FMP` writer - I2C_PB8_FMP
pub use I2C_PBX_FMP_W as I2C_PB8_FMP_W;
///Field `I2C_PB9_FMP` writer - I2C_PB9_FMP
pub use I2C_PBX_FMP_W as I2C_PB9_FMP_W;
///Field `I2C1_FMP` writer - FM+ driving capability activation for I2C1
pub use I2C_PBX_FMP_W as I2C1_FMP_W;
///Field `I2C2_FMP` writer - FM+ driving capability activation for I2C2
pub use I2C_PBX_FMP_W as I2C2_FMP_W;
///Field `I2C_PA9_FMP` writer - Fast Mode Plus (FM+) driving capability activation bits
pub use I2C_PBX_FMP_W as I2C_PA9_FMP_W;
///Field `I2C_PA10_FMP` writer - Fast Mode Plus (FM+) driving capability activation bits
pub use I2C_PBX_FMP_W as I2C_PA10_FMP_W;
///Field `I2C3_FMP` writer - I2C3_FMP
pub use I2C_PBX_FMP_W as I2C3_FMP_W;
impl R {
    ///Bits 0:1 - Memory mapping selection bits
    #[inline(always)]
    pub fn mem_mode(&self) -> MEM_MODE_R {
        MEM_MODE_R::new((self.bits & 3) as u8)
    }
    ///Bit 3 - PA11_RMP
    #[inline(always)]
    pub fn pa11_rmp(&self) -> PA11_RMP_R {
        PA11_RMP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - PA11 and PA12 remapping bit.
    #[inline(always)]
    pub fn pa12_rmp(&self) -> PA12_RMP_R {
        PA12_RMP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - IR output polarity selection
    #[inline(always)]
    pub fn ir_pol(&self) -> IR_POL_R {
        IR_POL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:7 - IR Modulation Envelope signal selection.
    #[inline(always)]
    pub fn ir_mod(&self) -> IR_MOD_R {
        IR_MOD_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bit 8 - I/O analog switch voltage booster enable
    #[inline(always)]
    pub fn boosten(&self) -> BOOSTEN_R {
        BOOSTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Strobe signal bit for UCPD1
    #[inline(always)]
    pub fn ucpd1_strobe(&self) -> UCPD1_STROBE_R {
        UCPD1_STROBE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Strobe signal bit for UCPD2
    #[inline(always)]
    pub fn ucpd2_strobe(&self) -> UCPD2_STROBE_R {
        UCPD2_STROBE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 16 - Fast Mode Plus (FM+) driving capability activation bits
    #[inline(always)]
    pub fn i2c_pbx_fmp(&self) -> I2C_PBX_FMP_R {
        I2C_PBX_FMP_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - I2C_PB7_FMP
    #[inline(always)]
    pub fn i2c_pb7_fmp(&self) -> I2C_PB7_FMP_R {
        I2C_PB7_FMP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - I2C_PB8_FMP
    #[inline(always)]
    pub fn i2c_pb8_fmp(&self) -> I2C_PB8_FMP_R {
        I2C_PB8_FMP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - I2C_PB9_FMP
    #[inline(always)]
    pub fn i2c_pb9_fmp(&self) -> I2C_PB9_FMP_R {
        I2C_PB9_FMP_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - FM+ driving capability activation for I2C1
    #[inline(always)]
    pub fn i2c1_fmp(&self) -> I2C1_FMP_R {
        I2C1_FMP_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - FM+ driving capability activation for I2C2
    #[inline(always)]
    pub fn i2c2_fmp(&self) -> I2C2_FMP_R {
        I2C2_FMP_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Fast Mode Plus (FM+) driving capability activation bits
    #[inline(always)]
    pub fn i2c_pa9_fmp(&self) -> I2C_PA9_FMP_R {
        I2C_PA9_FMP_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Fast Mode Plus (FM+) driving capability activation bits
    #[inline(always)]
    pub fn i2c_pa10_fmp(&self) -> I2C_PA10_FMP_R {
        I2C_PA10_FMP_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - I2C3_FMP
    #[inline(always)]
    pub fn i2c3_fmp(&self) -> I2C3_FMP_R {
        I2C3_FMP_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR1")
            .field("i2c_pbx_fmp", &self.i2c_pbx_fmp())
            .field("i2c3_fmp", &self.i2c3_fmp())
            .field("i2c_pa10_fmp", &self.i2c_pa10_fmp())
            .field("i2c_pa9_fmp", &self.i2c_pa9_fmp())
            .field("i2c2_fmp", &self.i2c2_fmp())
            .field("i2c1_fmp", &self.i2c1_fmp())
            .field("i2c_pb9_fmp", &self.i2c_pb9_fmp())
            .field("i2c_pb8_fmp", &self.i2c_pb8_fmp())
            .field("i2c_pb7_fmp", &self.i2c_pb7_fmp())
            .field("ucpd1_strobe", &self.ucpd1_strobe())
            .field("ucpd2_strobe", &self.ucpd2_strobe())
            .field("boosten", &self.boosten())
            .field("ir_mod", &self.ir_mod())
            .field("ir_pol", &self.ir_pol())
            .field("pa12_rmp", &self.pa12_rmp())
            .field("pa11_rmp", &self.pa11_rmp())
            .field("mem_mode", &self.mem_mode())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Memory mapping selection bits
    #[inline(always)]
    pub fn mem_mode(&mut self) -> MEM_MODE_W<'_, CFGR1rs> {
        MEM_MODE_W::new(self, 0)
    }
    ///Bit 3 - PA11_RMP
    #[inline(always)]
    pub fn pa11_rmp(&mut self) -> PA11_RMP_W<'_, CFGR1rs> {
        PA11_RMP_W::new(self, 3)
    }
    ///Bit 4 - PA11 and PA12 remapping bit.
    #[inline(always)]
    pub fn pa12_rmp(&mut self) -> PA12_RMP_W<'_, CFGR1rs> {
        PA12_RMP_W::new(self, 4)
    }
    ///Bit 5 - IR output polarity selection
    #[inline(always)]
    pub fn ir_pol(&mut self) -> IR_POL_W<'_, CFGR1rs> {
        IR_POL_W::new(self, 5)
    }
    ///Bits 6:7 - IR Modulation Envelope signal selection.
    #[inline(always)]
    pub fn ir_mod(&mut self) -> IR_MOD_W<'_, CFGR1rs> {
        IR_MOD_W::new(self, 6)
    }
    ///Bit 8 - I/O analog switch voltage booster enable
    #[inline(always)]
    pub fn boosten(&mut self) -> BOOSTEN_W<'_, CFGR1rs> {
        BOOSTEN_W::new(self, 8)
    }
    ///Bit 9 - Strobe signal bit for UCPD1
    #[inline(always)]
    pub fn ucpd1_strobe(&mut self) -> UCPD1_STROBE_W<'_, CFGR1rs> {
        UCPD1_STROBE_W::new(self, 9)
    }
    ///Bit 10 - Strobe signal bit for UCPD2
    #[inline(always)]
    pub fn ucpd2_strobe(&mut self) -> UCPD2_STROBE_W<'_, CFGR1rs> {
        UCPD2_STROBE_W::new(self, 10)
    }
    ///Bit 16 - Fast Mode Plus (FM+) driving capability activation bits
    #[inline(always)]
    pub fn i2c_pbx_fmp(&mut self) -> I2C_PBX_FMP_W<'_, CFGR1rs> {
        I2C_PBX_FMP_W::new(self, 16)
    }
    ///Bit 17 - I2C_PB7_FMP
    #[inline(always)]
    pub fn i2c_pb7_fmp(&mut self) -> I2C_PB7_FMP_W<'_, CFGR1rs> {
        I2C_PB7_FMP_W::new(self, 17)
    }
    ///Bit 18 - I2C_PB8_FMP
    #[inline(always)]
    pub fn i2c_pb8_fmp(&mut self) -> I2C_PB8_FMP_W<'_, CFGR1rs> {
        I2C_PB8_FMP_W::new(self, 18)
    }
    ///Bit 19 - I2C_PB9_FMP
    #[inline(always)]
    pub fn i2c_pb9_fmp(&mut self) -> I2C_PB9_FMP_W<'_, CFGR1rs> {
        I2C_PB9_FMP_W::new(self, 19)
    }
    ///Bit 20 - FM+ driving capability activation for I2C1
    #[inline(always)]
    pub fn i2c1_fmp(&mut self) -> I2C1_FMP_W<'_, CFGR1rs> {
        I2C1_FMP_W::new(self, 20)
    }
    ///Bit 21 - FM+ driving capability activation for I2C2
    #[inline(always)]
    pub fn i2c2_fmp(&mut self) -> I2C2_FMP_W<'_, CFGR1rs> {
        I2C2_FMP_W::new(self, 21)
    }
    ///Bit 22 - Fast Mode Plus (FM+) driving capability activation bits
    #[inline(always)]
    pub fn i2c_pa9_fmp(&mut self) -> I2C_PA9_FMP_W<'_, CFGR1rs> {
        I2C_PA9_FMP_W::new(self, 22)
    }
    ///Bit 23 - Fast Mode Plus (FM+) driving capability activation bits
    #[inline(always)]
    pub fn i2c_pa10_fmp(&mut self) -> I2C_PA10_FMP_W<'_, CFGR1rs> {
        I2C_PA10_FMP_W::new(self, 23)
    }
    ///Bit 24 - I2C3_FMP
    #[inline(always)]
    pub fn i2c3_fmp(&mut self) -> I2C3_FMP_W<'_, CFGR1rs> {
        I2C3_FMP_W::new(self, 24)
    }
}
/**SYSCFG configuration register 1

You can [`read`](crate::Reg::read) this register and get [`cfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0B0.html#SYSCFG:CFGR1)*/
pub struct CFGR1rs;
impl crate::RegisterSpec for CFGR1rs {
    type Ux = u32;
}
///`read()` method returns [`cfgr1::R`](R) reader structure
impl crate::Readable for CFGR1rs {}
///`write(|w| ..)` method takes [`cfgr1::W`](W) writer structure
impl crate::Writable for CFGR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR1 to value 0
impl crate::Resettable for CFGR1rs {}

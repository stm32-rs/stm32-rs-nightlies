///Register `CFGR1` reader
pub type R = crate::R<CFGR1rs>;
///Register `CFGR1` writer
pub type W = crate::W<CFGR1rs>;
/**Firewall disable

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FWDIS {
    ///0: Firewall protection enabled
    Enabled = 0,
    ///1: Firewall protection disabled
    Disabled = 1,
}
impl From<FWDIS> for bool {
    #[inline(always)]
    fn from(variant: FWDIS) -> Self {
        variant as u8 != 0
    }
}
///Field `FWDIS` reader - Firewall disable
pub type FWDIS_R = crate::BitReader<FWDIS>;
impl FWDIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FWDIS {
        match self.bits {
            false => FWDIS::Enabled,
            true => FWDIS::Disabled,
        }
    }
    ///Firewall protection enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FWDIS::Enabled
    }
    ///Firewall protection disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FWDIS::Disabled
    }
}
///Field `FWDIS` writer - Firewall disable
pub type FWDIS_W<'a, REG> = crate::BitWriter<'a, REG, FWDIS>;
impl<'a, REG> FWDIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Firewall protection enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FWDIS::Enabled)
    }
    ///Firewall protection disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FWDIS::Disabled)
    }
}
/**I/O analog switch voltage booster enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOOSTEN {
    ///0: I/O analog switches are supplied by VDDA voltage. This is the recommended configuration when using the ADC in high VDDA voltage operation
    Disabled = 0,
    ///1: I/O analog switches are supplied by a dedicated voltage booster (supplied by VDD). This is the recommended configuration when using the ADC in low VDDA voltage operation
    Enabled = 1,
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
            false => BOOSTEN::Disabled,
            true => BOOSTEN::Enabled,
        }
    }
    ///I/O analog switches are supplied by VDDA voltage. This is the recommended configuration when using the ADC in high VDDA voltage operation
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BOOSTEN::Disabled
    }
    ///I/O analog switches are supplied by a dedicated voltage booster (supplied by VDD). This is the recommended configuration when using the ADC in low VDDA voltage operation
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BOOSTEN::Enabled
    }
}
///Field `BOOSTEN` writer - I/O analog switch voltage booster enable
pub type BOOSTEN_W<'a, REG> = crate::BitWriter<'a, REG, BOOSTEN>;
impl<'a, REG> BOOSTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///I/O analog switches are supplied by VDDA voltage. This is the recommended configuration when using the ADC in high VDDA voltage operation
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BOOSTEN::Disabled)
    }
    ///I/O analog switches are supplied by a dedicated voltage booster (supplied by VDD). This is the recommended configuration when using the ADC in low VDDA voltage operation
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BOOSTEN::Enabled)
    }
}
/**GPIO analog switch control voltage selection when at least one analog peripheral supplied by VDDA is enabled (COMP, OPAMP, VREFBUF, ADC,...)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANASWVDD {
    ///0: I/O analog switches supplied by VDDA or booster when booster is ON
    Vdda = 0,
    ///1: I/O analog switches supplied by VDD
    Vdd = 1,
}
impl From<ANASWVDD> for bool {
    #[inline(always)]
    fn from(variant: ANASWVDD) -> Self {
        variant as u8 != 0
    }
}
///Field `ANASWVDD` reader - GPIO analog switch control voltage selection when at least one analog peripheral supplied by VDDA is enabled (COMP, OPAMP, VREFBUF, ADC,...)
pub type ANASWVDD_R = crate::BitReader<ANASWVDD>;
impl ANASWVDD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANASWVDD {
        match self.bits {
            false => ANASWVDD::Vdda,
            true => ANASWVDD::Vdd,
        }
    }
    ///I/O analog switches supplied by VDDA or booster when booster is ON
    #[inline(always)]
    pub fn is_vdda(&self) -> bool {
        *self == ANASWVDD::Vdda
    }
    ///I/O analog switches supplied by VDD
    #[inline(always)]
    pub fn is_vdd(&self) -> bool {
        *self == ANASWVDD::Vdd
    }
}
///Field `ANASWVDD` writer - GPIO analog switch control voltage selection when at least one analog peripheral supplied by VDDA is enabled (COMP, OPAMP, VREFBUF, ADC,...)
pub type ANASWVDD_W<'a, REG> = crate::BitWriter<'a, REG, ANASWVDD>;
impl<'a, REG> ANASWVDD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///I/O analog switches supplied by VDDA or booster when booster is ON
    #[inline(always)]
    pub fn vdda(self) -> &'a mut crate::W<REG> {
        self.variant(ANASWVDD::Vdda)
    }
    ///I/O analog switches supplied by VDD
    #[inline(always)]
    pub fn vdd(self) -> &'a mut crate::W<REG> {
        self.variant(ANASWVDD::Vdd)
    }
}
/**Fast-mode Plus (Fm+) driving capability activation on PB6

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C_PB6_FMP {
    ///0: PBx pin operates in standard mode
    Disabled = 0,
    ///1: Fm+ mode enabled on PB7 pin, and the Speed control is bypassed
    Enabled = 1,
}
impl From<I2C_PB6_FMP> for bool {
    #[inline(always)]
    fn from(variant: I2C_PB6_FMP) -> Self {
        variant as u8 != 0
    }
}
///Field `I2C_PB6_FMP` reader - Fast-mode Plus (Fm+) driving capability activation on PB6
pub type I2C_PB6_FMP_R = crate::BitReader<I2C_PB6_FMP>;
impl I2C_PB6_FMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2C_PB6_FMP {
        match self.bits {
            false => I2C_PB6_FMP::Disabled,
            true => I2C_PB6_FMP::Enabled,
        }
    }
    ///PBx pin operates in standard mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == I2C_PB6_FMP::Disabled
    }
    ///Fm+ mode enabled on PB7 pin, and the Speed control is bypassed
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == I2C_PB6_FMP::Enabled
    }
}
///Field `I2C_PB6_FMP` writer - Fast-mode Plus (Fm+) driving capability activation on PB6
pub type I2C_PB6_FMP_W<'a, REG> = crate::BitWriter<'a, REG, I2C_PB6_FMP>;
impl<'a, REG> I2C_PB6_FMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PBx pin operates in standard mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_PB6_FMP::Disabled)
    }
    ///Fm+ mode enabled on PB7 pin, and the Speed control is bypassed
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_PB6_FMP::Enabled)
    }
}
///Field `I2C_PB7_FMP` reader - Fast-mode Plus (Fm+) driving capability activation on PB7
pub use I2C_PB6_FMP_R as I2C_PB7_FMP_R;
///Field `I2C_PB8_FMP` reader - Fast-mode Plus (Fm+) driving capability activation on PB8
pub use I2C_PB6_FMP_R as I2C_PB8_FMP_R;
///Field `I2C_PB9_FMP` reader - Fast-mode Plus (Fm+) driving capability activation on PB9
pub use I2C_PB6_FMP_R as I2C_PB9_FMP_R;
///Field `I2C_PB7_FMP` writer - Fast-mode Plus (Fm+) driving capability activation on PB7
pub use I2C_PB6_FMP_W as I2C_PB7_FMP_W;
///Field `I2C_PB8_FMP` writer - Fast-mode Plus (Fm+) driving capability activation on PB8
pub use I2C_PB6_FMP_W as I2C_PB8_FMP_W;
///Field `I2C_PB9_FMP` writer - Fast-mode Plus (Fm+) driving capability activation on PB9
pub use I2C_PB6_FMP_W as I2C_PB9_FMP_W;
/**I2C1 Fast-mode Plus driving capability activation

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C1_FMP {
    ///0: Fm+ mode is not enabled on I2Cx pins selected through AF selection bits
    Disabled = 0,
    ///1: Fm+ mode is enabled on I2Cx pins selected through AF selection bits
    Enabled = 1,
}
impl From<I2C1_FMP> for bool {
    #[inline(always)]
    fn from(variant: I2C1_FMP) -> Self {
        variant as u8 != 0
    }
}
///Field `I2C1_FMP` reader - I2C1 Fast-mode Plus driving capability activation
pub type I2C1_FMP_R = crate::BitReader<I2C1_FMP>;
impl I2C1_FMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2C1_FMP {
        match self.bits {
            false => I2C1_FMP::Disabled,
            true => I2C1_FMP::Enabled,
        }
    }
    ///Fm+ mode is not enabled on I2Cx pins selected through AF selection bits
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == I2C1_FMP::Disabled
    }
    ///Fm+ mode is enabled on I2Cx pins selected through AF selection bits
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == I2C1_FMP::Enabled
    }
}
///Field `I2C1_FMP` writer - I2C1 Fast-mode Plus driving capability activation
pub type I2C1_FMP_W<'a, REG> = crate::BitWriter<'a, REG, I2C1_FMP>;
impl<'a, REG> I2C1_FMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Fm+ mode is not enabled on I2Cx pins selected through AF selection bits
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1_FMP::Disabled)
    }
    ///Fm+ mode is enabled on I2Cx pins selected through AF selection bits
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1_FMP::Enabled)
    }
}
///Field `I2C2_FMP` reader - I2C2 Fast-mode Plus driving capability activation
pub use I2C1_FMP_R as I2C2_FMP_R;
///Field `I2C3_FMP` reader - I2C3 Fast-mode Plus driving capability activation
pub use I2C1_FMP_R as I2C3_FMP_R;
///Field `I2C4_FMP` reader - I2C3 Fast-mode Plus driving capability activation
pub use I2C1_FMP_R as I2C4_FMP_R;
///Field `I2C2_FMP` writer - I2C2 Fast-mode Plus driving capability activation
pub use I2C1_FMP_W as I2C2_FMP_W;
///Field `I2C3_FMP` writer - I2C3 Fast-mode Plus driving capability activation
pub use I2C1_FMP_W as I2C3_FMP_W;
///Field `I2C4_FMP` writer - I2C3 Fast-mode Plus driving capability activation
pub use I2C1_FMP_W as I2C4_FMP_W;
/**Invalid operation interrupt enable

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPU_IE0 {
    ///0: Interrupt disabled
    Disabled = 0,
    ///1: Interrupt enabled
    Enabled = 1,
}
impl From<FPU_IE0> for bool {
    #[inline(always)]
    fn from(variant: FPU_IE0) -> Self {
        variant as u8 != 0
    }
}
///Field `FPU_IE0` reader - Invalid operation interrupt enable
pub type FPU_IE0_R = crate::BitReader<FPU_IE0>;
impl FPU_IE0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FPU_IE0 {
        match self.bits {
            false => FPU_IE0::Disabled,
            true => FPU_IE0::Enabled,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FPU_IE0::Disabled
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FPU_IE0::Enabled
    }
}
///Field `FPU_IE0` writer - Invalid operation interrupt enable
pub type FPU_IE0_W<'a, REG> = crate::BitWriter<'a, REG, FPU_IE0>;
impl<'a, REG> FPU_IE0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FPU_IE0::Disabled)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FPU_IE0::Enabled)
    }
}
///Field `FPU_IE1` reader - Divide-by-zero interrupt enable
pub use FPU_IE0_R as FPU_IE1_R;
///Field `FPU_IE2` reader - Underflow interrupt enable
pub use FPU_IE0_R as FPU_IE2_R;
///Field `FPU_IE3` reader - Overflow interrupt enable
pub use FPU_IE0_R as FPU_IE3_R;
///Field `FPU_IE4` reader - Input denormal interrupt enable
pub use FPU_IE0_R as FPU_IE4_R;
///Field `FPU_IE5` reader - Inexact interrupt enable
pub use FPU_IE0_R as FPU_IE5_R;
///Field `FPU_IE1` writer - Divide-by-zero interrupt enable
pub use FPU_IE0_W as FPU_IE1_W;
///Field `FPU_IE2` writer - Underflow interrupt enable
pub use FPU_IE0_W as FPU_IE2_W;
///Field `FPU_IE3` writer - Overflow interrupt enable
pub use FPU_IE0_W as FPU_IE3_W;
///Field `FPU_IE4` writer - Input denormal interrupt enable
pub use FPU_IE0_W as FPU_IE4_W;
///Field `FPU_IE5` writer - Inexact interrupt enable
pub use FPU_IE0_W as FPU_IE5_W;
impl R {
    ///Bit 0 - Firewall disable
    #[inline(always)]
    pub fn fwdis(&self) -> FWDIS_R {
        FWDIS_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - I/O analog switch voltage booster enable
    #[inline(always)]
    pub fn boosten(&self) -> BOOSTEN_R {
        BOOSTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - GPIO analog switch control voltage selection when at least one analog peripheral supplied by VDDA is enabled (COMP, OPAMP, VREFBUF, ADC,...)
    #[inline(always)]
    pub fn anaswvdd(&self) -> ANASWVDD_R {
        ANASWVDD_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 16 - Fast-mode Plus (Fm+) driving capability activation on PB6
    #[inline(always)]
    pub fn i2c_pb6_fmp(&self) -> I2C_PB6_FMP_R {
        I2C_PB6_FMP_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Fast-mode Plus (Fm+) driving capability activation on PB7
    #[inline(always)]
    pub fn i2c_pb7_fmp(&self) -> I2C_PB7_FMP_R {
        I2C_PB7_FMP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Fast-mode Plus (Fm+) driving capability activation on PB8
    #[inline(always)]
    pub fn i2c_pb8_fmp(&self) -> I2C_PB8_FMP_R {
        I2C_PB8_FMP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Fast-mode Plus (Fm+) driving capability activation on PB9
    #[inline(always)]
    pub fn i2c_pb9_fmp(&self) -> I2C_PB9_FMP_R {
        I2C_PB9_FMP_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - I2C1 Fast-mode Plus driving capability activation
    #[inline(always)]
    pub fn i2c1_fmp(&self) -> I2C1_FMP_R {
        I2C1_FMP_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - I2C2 Fast-mode Plus driving capability activation
    #[inline(always)]
    pub fn i2c2_fmp(&self) -> I2C2_FMP_R {
        I2C2_FMP_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I2C3 Fast-mode Plus driving capability activation
    #[inline(always)]
    pub fn i2c3_fmp(&self) -> I2C3_FMP_R {
        I2C3_FMP_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - I2C3 Fast-mode Plus driving capability activation
    #[inline(always)]
    pub fn i2c4_fmp(&self) -> I2C4_FMP_R {
        I2C4_FMP_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 26 - Invalid operation interrupt enable
    #[inline(always)]
    pub fn fpu_ie0(&self) -> FPU_IE0_R {
        FPU_IE0_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Divide-by-zero interrupt enable
    #[inline(always)]
    pub fn fpu_ie1(&self) -> FPU_IE1_R {
        FPU_IE1_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Underflow interrupt enable
    #[inline(always)]
    pub fn fpu_ie2(&self) -> FPU_IE2_R {
        FPU_IE2_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Overflow interrupt enable
    #[inline(always)]
    pub fn fpu_ie3(&self) -> FPU_IE3_R {
        FPU_IE3_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Input denormal interrupt enable
    #[inline(always)]
    pub fn fpu_ie4(&self) -> FPU_IE4_R {
        FPU_IE4_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Inexact interrupt enable
    #[inline(always)]
    pub fn fpu_ie5(&self) -> FPU_IE5_R {
        FPU_IE5_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR1")
            .field("i2c1_fmp", &self.i2c1_fmp())
            .field("i2c3_fmp", &self.i2c3_fmp())
            .field("i2c2_fmp", &self.i2c2_fmp())
            .field("i2c_pb6_fmp", &self.i2c_pb6_fmp())
            .field("i2c_pb9_fmp", &self.i2c_pb9_fmp())
            .field("i2c_pb8_fmp", &self.i2c_pb8_fmp())
            .field("i2c_pb7_fmp", &self.i2c_pb7_fmp())
            .field("boosten", &self.boosten())
            .field("fwdis", &self.fwdis())
            .field("fpu_ie0", &self.fpu_ie0())
            .field("fpu_ie5", &self.fpu_ie5())
            .field("fpu_ie4", &self.fpu_ie4())
            .field("fpu_ie3", &self.fpu_ie3())
            .field("fpu_ie2", &self.fpu_ie2())
            .field("fpu_ie1", &self.fpu_ie1())
            .field("anaswvdd", &self.anaswvdd())
            .field("i2c4_fmp", &self.i2c4_fmp())
            .finish()
    }
}
impl W {
    ///Bit 0 - Firewall disable
    #[inline(always)]
    pub fn fwdis(&mut self) -> FWDIS_W<'_, CFGR1rs> {
        FWDIS_W::new(self, 0)
    }
    ///Bit 8 - I/O analog switch voltage booster enable
    #[inline(always)]
    pub fn boosten(&mut self) -> BOOSTEN_W<'_, CFGR1rs> {
        BOOSTEN_W::new(self, 8)
    }
    ///Bit 9 - GPIO analog switch control voltage selection when at least one analog peripheral supplied by VDDA is enabled (COMP, OPAMP, VREFBUF, ADC,...)
    #[inline(always)]
    pub fn anaswvdd(&mut self) -> ANASWVDD_W<'_, CFGR1rs> {
        ANASWVDD_W::new(self, 9)
    }
    ///Bit 16 - Fast-mode Plus (Fm+) driving capability activation on PB6
    #[inline(always)]
    pub fn i2c_pb6_fmp(&mut self) -> I2C_PB6_FMP_W<'_, CFGR1rs> {
        I2C_PB6_FMP_W::new(self, 16)
    }
    ///Bit 17 - Fast-mode Plus (Fm+) driving capability activation on PB7
    #[inline(always)]
    pub fn i2c_pb7_fmp(&mut self) -> I2C_PB7_FMP_W<'_, CFGR1rs> {
        I2C_PB7_FMP_W::new(self, 17)
    }
    ///Bit 18 - Fast-mode Plus (Fm+) driving capability activation on PB8
    #[inline(always)]
    pub fn i2c_pb8_fmp(&mut self) -> I2C_PB8_FMP_W<'_, CFGR1rs> {
        I2C_PB8_FMP_W::new(self, 18)
    }
    ///Bit 19 - Fast-mode Plus (Fm+) driving capability activation on PB9
    #[inline(always)]
    pub fn i2c_pb9_fmp(&mut self) -> I2C_PB9_FMP_W<'_, CFGR1rs> {
        I2C_PB9_FMP_W::new(self, 19)
    }
    ///Bit 20 - I2C1 Fast-mode Plus driving capability activation
    #[inline(always)]
    pub fn i2c1_fmp(&mut self) -> I2C1_FMP_W<'_, CFGR1rs> {
        I2C1_FMP_W::new(self, 20)
    }
    ///Bit 21 - I2C2 Fast-mode Plus driving capability activation
    #[inline(always)]
    pub fn i2c2_fmp(&mut self) -> I2C2_FMP_W<'_, CFGR1rs> {
        I2C2_FMP_W::new(self, 21)
    }
    ///Bit 22 - I2C3 Fast-mode Plus driving capability activation
    #[inline(always)]
    pub fn i2c3_fmp(&mut self) -> I2C3_FMP_W<'_, CFGR1rs> {
        I2C3_FMP_W::new(self, 22)
    }
    ///Bit 23 - I2C3 Fast-mode Plus driving capability activation
    #[inline(always)]
    pub fn i2c4_fmp(&mut self) -> I2C4_FMP_W<'_, CFGR1rs> {
        I2C4_FMP_W::new(self, 23)
    }
    ///Bit 26 - Invalid operation interrupt enable
    #[inline(always)]
    pub fn fpu_ie0(&mut self) -> FPU_IE0_W<'_, CFGR1rs> {
        FPU_IE0_W::new(self, 26)
    }
    ///Bit 27 - Divide-by-zero interrupt enable
    #[inline(always)]
    pub fn fpu_ie1(&mut self) -> FPU_IE1_W<'_, CFGR1rs> {
        FPU_IE1_W::new(self, 27)
    }
    ///Bit 28 - Underflow interrupt enable
    #[inline(always)]
    pub fn fpu_ie2(&mut self) -> FPU_IE2_W<'_, CFGR1rs> {
        FPU_IE2_W::new(self, 28)
    }
    ///Bit 29 - Overflow interrupt enable
    #[inline(always)]
    pub fn fpu_ie3(&mut self) -> FPU_IE3_W<'_, CFGR1rs> {
        FPU_IE3_W::new(self, 29)
    }
    ///Bit 30 - Input denormal interrupt enable
    #[inline(always)]
    pub fn fpu_ie4(&mut self) -> FPU_IE4_W<'_, CFGR1rs> {
        FPU_IE4_W::new(self, 30)
    }
    ///Bit 31 - Inexact interrupt enable
    #[inline(always)]
    pub fn fpu_ie5(&mut self) -> FPU_IE5_W<'_, CFGR1rs> {
        FPU_IE5_W::new(self, 31)
    }
}
/**configuration register 1

You can [`read`](crate::Reg::read) this register and get [`cfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#SYSCFG:CFGR1)*/
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
///`reset()` method sets CFGR1 to value 0x7c00_0001
impl crate::Resettable for CFGR1rs {
    const RESET_VALUE: u32 = 0x7c00_0001;
}

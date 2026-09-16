///Register `MCHDLYCR` reader
pub type R = crate::R<MCHDLYCRrs>;
///Register `MCHDLYCR` writer
pub type W = crate::W<MCHDLYCRrs>;
///Field `BSCKSEL` reader - Bitstream clock source selection
pub type BSCKSEL_R = crate::BitReader;
///Field `BSCKSEL` writer - Bitstream clock source selection
pub type BSCKSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
/**MCHDLY clock enable for DFSDM1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCHDLYEN1 {
    ///0: Delay clock for DFSDM1 is disabled (G\[2:1\] = 0)
    Disabled = 0,
    ///1: Delay clock for DFSDM1 is enabled (G\[2:1\] = 1)
    Enabled = 1,
}
impl From<MCHDLYEN1> for bool {
    #[inline(always)]
    fn from(variant: MCHDLYEN1) -> Self {
        variant as u8 != 0
    }
}
///Field `MCHDLYEN1` reader - MCHDLY clock enable for DFSDM1
pub type MCHDLYEN1_R = crate::BitReader<MCHDLYEN1>;
impl MCHDLYEN1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MCHDLYEN1 {
        match self.bits {
            false => MCHDLYEN1::Disabled,
            true => MCHDLYEN1::Enabled,
        }
    }
    ///Delay clock for DFSDM1 is disabled (G\[2:1\] = 0)
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MCHDLYEN1::Disabled
    }
    ///Delay clock for DFSDM1 is enabled (G\[2:1\] = 1)
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MCHDLYEN1::Enabled
    }
}
///Field `MCHDLYEN1` writer - MCHDLY clock enable for DFSDM1
pub type MCHDLYEN1_W<'a, REG> = crate::BitWriter<'a, REG, MCHDLYEN1>;
impl<'a, REG> MCHDLYEN1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Delay clock for DFSDM1 is disabled (G\[2:1\] = 0)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MCHDLYEN1::Disabled)
    }
    ///Delay clock for DFSDM1 is enabled (G\[2:1\] = 1)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MCHDLYEN1::Enabled)
    }
}
///Field `DFSDM1_D0SEL` reader - Source selection for DatIn0 of DFSDM1
pub type DFSDM1_D0SEL_R = crate::BitReader;
///Field `DFSDM1_D0SEL` writer - Source selection for DatIn0 of DFSDM1
pub type DFSDM1_D0SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DFSDM1_D2SEL` reader - Source selection for DatIn2 of DFSDM1
pub type DFSDM1_D2SEL_R = crate::BitReader;
///Field `DFSDM1_D2SEL` writer - Source selection for DatIn2 of DFSDM1
pub type DFSDM1_D2SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Distribution of the DFSDM1 bitstream clock gated by TIM4 OC2

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFSDM1_CK02SEL {
    ///0: The gated clock is distributed to CkIn0 (DM2 = 0)
    CkIn0 = 0,
    ///1: The gated clock is distributed to CkIn2 (DM2 = 1)
    CkIn2 = 1,
}
impl From<DFSDM1_CK02SEL> for bool {
    #[inline(always)]
    fn from(variant: DFSDM1_CK02SEL) -> Self {
        variant as u8 != 0
    }
}
///Field `DFSDM1_CK02SEL` reader - Distribution of the DFSDM1 bitstream clock gated by TIM4 OC2
pub type DFSDM1_CK02SEL_R = crate::BitReader<DFSDM1_CK02SEL>;
impl DFSDM1_CK02SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DFSDM1_CK02SEL {
        match self.bits {
            false => DFSDM1_CK02SEL::CkIn0,
            true => DFSDM1_CK02SEL::CkIn2,
        }
    }
    ///The gated clock is distributed to CkIn0 (DM2 = 0)
    #[inline(always)]
    pub fn is_ck_in0(&self) -> bool {
        *self == DFSDM1_CK02SEL::CkIn0
    }
    ///The gated clock is distributed to CkIn2 (DM2 = 1)
    #[inline(always)]
    pub fn is_ck_in2(&self) -> bool {
        *self == DFSDM1_CK02SEL::CkIn2
    }
}
///Field `DFSDM1_CK02SEL` writer - Distribution of the DFSDM1 bitstream clock gated by TIM4 OC2
pub type DFSDM1_CK02SEL_W<'a, REG> = crate::BitWriter<'a, REG, DFSDM1_CK02SEL>;
impl<'a, REG> DFSDM1_CK02SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The gated clock is distributed to CkIn0 (DM2 = 0)
    #[inline(always)]
    pub fn ck_in0(self) -> &'a mut crate::W<REG> {
        self.variant(DFSDM1_CK02SEL::CkIn0)
    }
    ///The gated clock is distributed to CkIn2 (DM2 = 1)
    #[inline(always)]
    pub fn ck_in2(self) -> &'a mut crate::W<REG> {
        self.variant(DFSDM1_CK02SEL::CkIn2)
    }
}
/**Distribution of the DFSDM1 bitstream clock gated by TIM4 OC1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFSDM1_CK13SEL {
    ///0: The gated clock is distributed to CkIn1 (DM1 = 0)
    CkIn1 = 0,
    ///1: The gated clock is distributed to CkIn3 (DM1 = 1)
    CkIn3 = 1,
}
impl From<DFSDM1_CK13SEL> for bool {
    #[inline(always)]
    fn from(variant: DFSDM1_CK13SEL) -> Self {
        variant as u8 != 0
    }
}
///Field `DFSDM1_CK13SEL` reader - Distribution of the DFSDM1 bitstream clock gated by TIM4 OC1
pub type DFSDM1_CK13SEL_R = crate::BitReader<DFSDM1_CK13SEL>;
impl DFSDM1_CK13SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DFSDM1_CK13SEL {
        match self.bits {
            false => DFSDM1_CK13SEL::CkIn1,
            true => DFSDM1_CK13SEL::CkIn3,
        }
    }
    ///The gated clock is distributed to CkIn1 (DM1 = 0)
    #[inline(always)]
    pub fn is_ck_in1(&self) -> bool {
        *self == DFSDM1_CK13SEL::CkIn1
    }
    ///The gated clock is distributed to CkIn3 (DM1 = 1)
    #[inline(always)]
    pub fn is_ck_in3(&self) -> bool {
        *self == DFSDM1_CK13SEL::CkIn3
    }
}
///Field `DFSDM1_CK13SEL` writer - Distribution of the DFSDM1 bitstream clock gated by TIM4 OC1
pub type DFSDM1_CK13SEL_W<'a, REG> = crate::BitWriter<'a, REG, DFSDM1_CK13SEL>;
impl<'a, REG> DFSDM1_CK13SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The gated clock is distributed to CkIn1 (DM1 = 0)
    #[inline(always)]
    pub fn ck_in1(self) -> &'a mut crate::W<REG> {
        self.variant(DFSDM1_CK13SEL::CkIn1)
    }
    ///The gated clock is distributed to CkIn3 (DM1 = 1)
    #[inline(always)]
    pub fn ck_in3(self) -> &'a mut crate::W<REG> {
        self.variant(DFSDM1_CK13SEL::CkIn3)
    }
}
///Field `DFSDM1_CFG` reader - CkIn source selection for DFSDM1
pub type DFSDM1_CFG_R = crate::BitReader;
///Field `DFSDM1_CFG` writer - CkIn source selection for DFSDM1
pub type DFSDM1_CFG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DFSDM1_CKOSEL` reader - Source selection for DFSDM1_CKOUT
pub type DFSDM1_CKOSEL_R = crate::BitReader;
///Field `DFSDM1_CKOSEL` writer - Source selection for DFSDM1_CKOUT
pub type DFSDM1_CKOSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
/**MCHDLY clock enable for DFSDM2

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCHDLYEN2 {
    ///0: Delay clock for DFSDM2 is disabled (G\[6:3\] = 0)
    Disabled = 0,
    ///1: Delay clock for DFSDM2 is enabled (G\[6:3\] = 1)
    Enabled = 1,
}
impl From<MCHDLYEN2> for bool {
    #[inline(always)]
    fn from(variant: MCHDLYEN2) -> Self {
        variant as u8 != 0
    }
}
///Field `MCHDLYEN2` reader - MCHDLY clock enable for DFSDM2
pub type MCHDLYEN2_R = crate::BitReader<MCHDLYEN2>;
impl MCHDLYEN2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MCHDLYEN2 {
        match self.bits {
            false => MCHDLYEN2::Disabled,
            true => MCHDLYEN2::Enabled,
        }
    }
    ///Delay clock for DFSDM2 is disabled (G\[6:3\] = 0)
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MCHDLYEN2::Disabled
    }
    ///Delay clock for DFSDM2 is enabled (G\[6:3\] = 1)
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MCHDLYEN2::Enabled
    }
}
///Field `MCHDLYEN2` writer - MCHDLY clock enable for DFSDM2
pub type MCHDLYEN2_W<'a, REG> = crate::BitWriter<'a, REG, MCHDLYEN2>;
impl<'a, REG> MCHDLYEN2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Delay clock for DFSDM2 is disabled (G\[6:3\] = 0)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MCHDLYEN2::Disabled)
    }
    ///Delay clock for DFSDM2 is enabled (G\[6:3\] = 1)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MCHDLYEN2::Enabled)
    }
}
///Field `DFSDM2_D0SEL` reader - Source selection for DatIn0 of DFSDM2
pub type DFSDM2_D0SEL_R = crate::BitReader;
///Field `DFSDM2_D0SEL` writer - Source selection for DatIn0 of DFSDM2
pub type DFSDM2_D0SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DFSDM2_D2SEL` reader - Source selection for DatIn2 of DFSDM2
pub type DFSDM2_D2SEL_R = crate::BitReader;
///Field `DFSDM2_D2SEL` writer - Source selection for DatIn2 of DFSDM2
pub type DFSDM2_D2SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DFSDM2_D4SEL` reader - Source selection for DatIn4 of DFSDM2
pub type DFSDM2_D4SEL_R = crate::BitReader;
///Field `DFSDM2_D4SEL` writer - Source selection for DatIn4 of DFSDM2
pub type DFSDM2_D4SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DFSDM2_D6SEL` reader - Source selection for DatIn6 of DFSDM2
pub type DFSDM2_D6SEL_R = crate::BitReader;
///Field `DFSDM2_D6SEL` writer - Source selection for DatIn6 of DFSDM2
pub type DFSDM2_D6SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Distribution of the DFSDM2 bitstream clock gated by TIM3 OC4

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFSDM2_CK04SEL {
    ///0: The gated clock is distributed to CkIn0 (DM6 = 0)
    CkIn0 = 0,
    ///1: The gated clock is distributed to CkIn4 (DM6 = 1)
    CkIn4 = 1,
}
impl From<DFSDM2_CK04SEL> for bool {
    #[inline(always)]
    fn from(variant: DFSDM2_CK04SEL) -> Self {
        variant as u8 != 0
    }
}
///Field `DFSDM2_CK04SEL` reader - Distribution of the DFSDM2 bitstream clock gated by TIM3 OC4
pub type DFSDM2_CK04SEL_R = crate::BitReader<DFSDM2_CK04SEL>;
impl DFSDM2_CK04SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DFSDM2_CK04SEL {
        match self.bits {
            false => DFSDM2_CK04SEL::CkIn0,
            true => DFSDM2_CK04SEL::CkIn4,
        }
    }
    ///The gated clock is distributed to CkIn0 (DM6 = 0)
    #[inline(always)]
    pub fn is_ck_in0(&self) -> bool {
        *self == DFSDM2_CK04SEL::CkIn0
    }
    ///The gated clock is distributed to CkIn4 (DM6 = 1)
    #[inline(always)]
    pub fn is_ck_in4(&self) -> bool {
        *self == DFSDM2_CK04SEL::CkIn4
    }
}
///Field `DFSDM2_CK04SEL` writer - Distribution of the DFSDM2 bitstream clock gated by TIM3 OC4
pub type DFSDM2_CK04SEL_W<'a, REG> = crate::BitWriter<'a, REG, DFSDM2_CK04SEL>;
impl<'a, REG> DFSDM2_CK04SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The gated clock is distributed to CkIn0 (DM6 = 0)
    #[inline(always)]
    pub fn ck_in0(self) -> &'a mut crate::W<REG> {
        self.variant(DFSDM2_CK04SEL::CkIn0)
    }
    ///The gated clock is distributed to CkIn4 (DM6 = 1)
    #[inline(always)]
    pub fn ck_in4(self) -> &'a mut crate::W<REG> {
        self.variant(DFSDM2_CK04SEL::CkIn4)
    }
}
/**Distribution of the DFSDM2 bitstream clock gated by TIM3 OC3

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFSDM2_CK15SEL {
    ///0: The gated clock is distributed to CkIn1 (DM5 = 0)
    CkIn1 = 0,
    ///1: The gated clock is distributed to CkIn5 (DM5 = 1)
    CkIn5 = 1,
}
impl From<DFSDM2_CK15SEL> for bool {
    #[inline(always)]
    fn from(variant: DFSDM2_CK15SEL) -> Self {
        variant as u8 != 0
    }
}
///Field `DFSDM2_CK15SEL` reader - Distribution of the DFSDM2 bitstream clock gated by TIM3 OC3
pub type DFSDM2_CK15SEL_R = crate::BitReader<DFSDM2_CK15SEL>;
impl DFSDM2_CK15SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DFSDM2_CK15SEL {
        match self.bits {
            false => DFSDM2_CK15SEL::CkIn1,
            true => DFSDM2_CK15SEL::CkIn5,
        }
    }
    ///The gated clock is distributed to CkIn1 (DM5 = 0)
    #[inline(always)]
    pub fn is_ck_in1(&self) -> bool {
        *self == DFSDM2_CK15SEL::CkIn1
    }
    ///The gated clock is distributed to CkIn5 (DM5 = 1)
    #[inline(always)]
    pub fn is_ck_in5(&self) -> bool {
        *self == DFSDM2_CK15SEL::CkIn5
    }
}
///Field `DFSDM2_CK15SEL` writer - Distribution of the DFSDM2 bitstream clock gated by TIM3 OC3
pub type DFSDM2_CK15SEL_W<'a, REG> = crate::BitWriter<'a, REG, DFSDM2_CK15SEL>;
impl<'a, REG> DFSDM2_CK15SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The gated clock is distributed to CkIn1 (DM5 = 0)
    #[inline(always)]
    pub fn ck_in1(self) -> &'a mut crate::W<REG> {
        self.variant(DFSDM2_CK15SEL::CkIn1)
    }
    ///The gated clock is distributed to CkIn5 (DM5 = 1)
    #[inline(always)]
    pub fn ck_in5(self) -> &'a mut crate::W<REG> {
        self.variant(DFSDM2_CK15SEL::CkIn5)
    }
}
/**Distribution of the DFSDM2 bitstream clock gated by TIM3 OC2

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFSDM2_CK26SEL {
    ///0: The gated clock is distributed to CkIn2 (DM4 = 0)
    CkIn2 = 0,
    ///1: The gated clock is distributed to CkIn6 (DM4 = 1)
    CkIn6 = 1,
}
impl From<DFSDM2_CK26SEL> for bool {
    #[inline(always)]
    fn from(variant: DFSDM2_CK26SEL) -> Self {
        variant as u8 != 0
    }
}
///Field `DFSDM2_CK26SEL` reader - Distribution of the DFSDM2 bitstream clock gated by TIM3 OC2
pub type DFSDM2_CK26SEL_R = crate::BitReader<DFSDM2_CK26SEL>;
impl DFSDM2_CK26SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DFSDM2_CK26SEL {
        match self.bits {
            false => DFSDM2_CK26SEL::CkIn2,
            true => DFSDM2_CK26SEL::CkIn6,
        }
    }
    ///The gated clock is distributed to CkIn2 (DM4 = 0)
    #[inline(always)]
    pub fn is_ck_in2(&self) -> bool {
        *self == DFSDM2_CK26SEL::CkIn2
    }
    ///The gated clock is distributed to CkIn6 (DM4 = 1)
    #[inline(always)]
    pub fn is_ck_in6(&self) -> bool {
        *self == DFSDM2_CK26SEL::CkIn6
    }
}
///Field `DFSDM2_CK26SEL` writer - Distribution of the DFSDM2 bitstream clock gated by TIM3 OC2
pub type DFSDM2_CK26SEL_W<'a, REG> = crate::BitWriter<'a, REG, DFSDM2_CK26SEL>;
impl<'a, REG> DFSDM2_CK26SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The gated clock is distributed to CkIn2 (DM4 = 0)
    #[inline(always)]
    pub fn ck_in2(self) -> &'a mut crate::W<REG> {
        self.variant(DFSDM2_CK26SEL::CkIn2)
    }
    ///The gated clock is distributed to CkIn6 (DM4 = 1)
    #[inline(always)]
    pub fn ck_in6(self) -> &'a mut crate::W<REG> {
        self.variant(DFSDM2_CK26SEL::CkIn6)
    }
}
/**Distribution of the DFSDM2 bitstream clock gated by TIM3 OC1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFSDM2_CK37SEL {
    ///0: The gated clock is distributed to CkIn3 (DM3 = 0)
    CkIn3 = 0,
    ///1: The gated clock is distributed to CkIn7 (DM3 = 1)
    CkIn7 = 1,
}
impl From<DFSDM2_CK37SEL> for bool {
    #[inline(always)]
    fn from(variant: DFSDM2_CK37SEL) -> Self {
        variant as u8 != 0
    }
}
///Field `DFSDM2_CK37SEL` reader - Distribution of the DFSDM2 bitstream clock gated by TIM3 OC1
pub type DFSDM2_CK37SEL_R = crate::BitReader<DFSDM2_CK37SEL>;
impl DFSDM2_CK37SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DFSDM2_CK37SEL {
        match self.bits {
            false => DFSDM2_CK37SEL::CkIn3,
            true => DFSDM2_CK37SEL::CkIn7,
        }
    }
    ///The gated clock is distributed to CkIn3 (DM3 = 0)
    #[inline(always)]
    pub fn is_ck_in3(&self) -> bool {
        *self == DFSDM2_CK37SEL::CkIn3
    }
    ///The gated clock is distributed to CkIn7 (DM3 = 1)
    #[inline(always)]
    pub fn is_ck_in7(&self) -> bool {
        *self == DFSDM2_CK37SEL::CkIn7
    }
}
///Field `DFSDM2_CK37SEL` writer - Distribution of the DFSDM2 bitstream clock gated by TIM3 OC1
pub type DFSDM2_CK37SEL_W<'a, REG> = crate::BitWriter<'a, REG, DFSDM2_CK37SEL>;
impl<'a, REG> DFSDM2_CK37SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The gated clock is distributed to CkIn3 (DM3 = 0)
    #[inline(always)]
    pub fn ck_in3(self) -> &'a mut crate::W<REG> {
        self.variant(DFSDM2_CK37SEL::CkIn3)
    }
    ///The gated clock is distributed to CkIn7 (DM3 = 1)
    #[inline(always)]
    pub fn ck_in7(self) -> &'a mut crate::W<REG> {
        self.variant(DFSDM2_CK37SEL::CkIn7)
    }
}
///Field `DFSDM2_CFG` reader - CkIn source selection for DFSDM2
pub type DFSDM2_CFG_R = crate::BitReader;
///Field `DFSDM2_CFG` writer - CkIn source selection for DFSDM2
pub type DFSDM2_CFG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DFSDM2_CKOSEL` reader - Source selection for DFSDM2_CKOUT
pub type DFSDM2_CKOSEL_R = crate::BitReader;
///Field `DFSDM2_CKOSEL` writer - Source selection for DFSDM2_CKOUT
pub type DFSDM2_CKOSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Bitstream clock source selection
    #[inline(always)]
    pub fn bscksel(&self) -> BSCKSEL_R {
        BSCKSEL_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - MCHDLY clock enable for DFSDM1
    #[inline(always)]
    pub fn mchdlyen1(&self) -> MCHDLYEN1_R {
        MCHDLYEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Source selection for DatIn0 of DFSDM1
    #[inline(always)]
    pub fn dfsdm1_d0sel(&self) -> DFSDM1_D0SEL_R {
        DFSDM1_D0SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Source selection for DatIn2 of DFSDM1
    #[inline(always)]
    pub fn dfsdm1_d2sel(&self) -> DFSDM1_D2SEL_R {
        DFSDM1_D2SEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Distribution of the DFSDM1 bitstream clock gated by TIM4 OC2
    #[inline(always)]
    pub fn dfsdm1_ck02sel(&self) -> DFSDM1_CK02SEL_R {
        DFSDM1_CK02SEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Distribution of the DFSDM1 bitstream clock gated by TIM4 OC1
    #[inline(always)]
    pub fn dfsdm1_ck13sel(&self) -> DFSDM1_CK13SEL_R {
        DFSDM1_CK13SEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CkIn source selection for DFSDM1
    #[inline(always)]
    pub fn dfsdm1_cfg(&self) -> DFSDM1_CFG_R {
        DFSDM1_CFG_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Source selection for DFSDM1_CKOUT
    #[inline(always)]
    pub fn dfsdm1_ckosel(&self) -> DFSDM1_CKOSEL_R {
        DFSDM1_CKOSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - MCHDLY clock enable for DFSDM2
    #[inline(always)]
    pub fn mchdlyen2(&self) -> MCHDLYEN2_R {
        MCHDLYEN2_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Source selection for DatIn0 of DFSDM2
    #[inline(always)]
    pub fn dfsdm2_d0sel(&self) -> DFSDM2_D0SEL_R {
        DFSDM2_D0SEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Source selection for DatIn2 of DFSDM2
    #[inline(always)]
    pub fn dfsdm2_d2sel(&self) -> DFSDM2_D2SEL_R {
        DFSDM2_D2SEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Source selection for DatIn4 of DFSDM2
    #[inline(always)]
    pub fn dfsdm2_d4sel(&self) -> DFSDM2_D4SEL_R {
        DFSDM2_D4SEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Source selection for DatIn6 of DFSDM2
    #[inline(always)]
    pub fn dfsdm2_d6sel(&self) -> DFSDM2_D6SEL_R {
        DFSDM2_D6SEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Distribution of the DFSDM2 bitstream clock gated by TIM3 OC4
    #[inline(always)]
    pub fn dfsdm2_ck04sel(&self) -> DFSDM2_CK04SEL_R {
        DFSDM2_CK04SEL_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Distribution of the DFSDM2 bitstream clock gated by TIM3 OC3
    #[inline(always)]
    pub fn dfsdm2_ck15sel(&self) -> DFSDM2_CK15SEL_R {
        DFSDM2_CK15SEL_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Distribution of the DFSDM2 bitstream clock gated by TIM3 OC2
    #[inline(always)]
    pub fn dfsdm2_ck26sel(&self) -> DFSDM2_CK26SEL_R {
        DFSDM2_CK26SEL_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Distribution of the DFSDM2 bitstream clock gated by TIM3 OC1
    #[inline(always)]
    pub fn dfsdm2_ck37sel(&self) -> DFSDM2_CK37SEL_R {
        DFSDM2_CK37SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - CkIn source selection for DFSDM2
    #[inline(always)]
    pub fn dfsdm2_cfg(&self) -> DFSDM2_CFG_R {
        DFSDM2_CFG_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Source selection for DFSDM2_CKOUT
    #[inline(always)]
    pub fn dfsdm2_ckosel(&self) -> DFSDM2_CKOSEL_R {
        DFSDM2_CKOSEL_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MCHDLYCR")
            .field("dfsdm2_ckosel", &self.dfsdm2_ckosel())
            .field("dfsdm2_cfg", &self.dfsdm2_cfg())
            .field("dfsdm2_ck37sel", &self.dfsdm2_ck37sel())
            .field("dfsdm2_ck26sel", &self.dfsdm2_ck26sel())
            .field("dfsdm2_ck15sel", &self.dfsdm2_ck15sel())
            .field("dfsdm2_ck04sel", &self.dfsdm2_ck04sel())
            .field("dfsdm2_d6sel", &self.dfsdm2_d6sel())
            .field("dfsdm2_d4sel", &self.dfsdm2_d4sel())
            .field("dfsdm2_d2sel", &self.dfsdm2_d2sel())
            .field("dfsdm2_d0sel", &self.dfsdm2_d0sel())
            .field("mchdlyen2", &self.mchdlyen2())
            .field("dfsdm1_ckosel", &self.dfsdm1_ckosel())
            .field("dfsdm1_cfg", &self.dfsdm1_cfg())
            .field("dfsdm1_ck13sel", &self.dfsdm1_ck13sel())
            .field("dfsdm1_ck02sel", &self.dfsdm1_ck02sel())
            .field("dfsdm1_d2sel", &self.dfsdm1_d2sel())
            .field("dfsdm1_d0sel", &self.dfsdm1_d0sel())
            .field("mchdlyen1", &self.mchdlyen1())
            .field("bscksel", &self.bscksel())
            .finish()
    }
}
impl W {
    ///Bit 0 - Bitstream clock source selection
    #[inline(always)]
    pub fn bscksel(&mut self) -> BSCKSEL_W<'_, MCHDLYCRrs> {
        BSCKSEL_W::new(self, 0)
    }
    ///Bit 1 - MCHDLY clock enable for DFSDM1
    #[inline(always)]
    pub fn mchdlyen1(&mut self) -> MCHDLYEN1_W<'_, MCHDLYCRrs> {
        MCHDLYEN1_W::new(self, 1)
    }
    ///Bit 2 - Source selection for DatIn0 of DFSDM1
    #[inline(always)]
    pub fn dfsdm1_d0sel(&mut self) -> DFSDM1_D0SEL_W<'_, MCHDLYCRrs> {
        DFSDM1_D0SEL_W::new(self, 2)
    }
    ///Bit 3 - Source selection for DatIn2 of DFSDM1
    #[inline(always)]
    pub fn dfsdm1_d2sel(&mut self) -> DFSDM1_D2SEL_W<'_, MCHDLYCRrs> {
        DFSDM1_D2SEL_W::new(self, 3)
    }
    ///Bit 4 - Distribution of the DFSDM1 bitstream clock gated by TIM4 OC2
    #[inline(always)]
    pub fn dfsdm1_ck02sel(&mut self) -> DFSDM1_CK02SEL_W<'_, MCHDLYCRrs> {
        DFSDM1_CK02SEL_W::new(self, 4)
    }
    ///Bit 5 - Distribution of the DFSDM1 bitstream clock gated by TIM4 OC1
    #[inline(always)]
    pub fn dfsdm1_ck13sel(&mut self) -> DFSDM1_CK13SEL_W<'_, MCHDLYCRrs> {
        DFSDM1_CK13SEL_W::new(self, 5)
    }
    ///Bit 6 - CkIn source selection for DFSDM1
    #[inline(always)]
    pub fn dfsdm1_cfg(&mut self) -> DFSDM1_CFG_W<'_, MCHDLYCRrs> {
        DFSDM1_CFG_W::new(self, 6)
    }
    ///Bit 7 - Source selection for DFSDM1_CKOUT
    #[inline(always)]
    pub fn dfsdm1_ckosel(&mut self) -> DFSDM1_CKOSEL_W<'_, MCHDLYCRrs> {
        DFSDM1_CKOSEL_W::new(self, 7)
    }
    ///Bit 8 - MCHDLY clock enable for DFSDM2
    #[inline(always)]
    pub fn mchdlyen2(&mut self) -> MCHDLYEN2_W<'_, MCHDLYCRrs> {
        MCHDLYEN2_W::new(self, 8)
    }
    ///Bit 9 - Source selection for DatIn0 of DFSDM2
    #[inline(always)]
    pub fn dfsdm2_d0sel(&mut self) -> DFSDM2_D0SEL_W<'_, MCHDLYCRrs> {
        DFSDM2_D0SEL_W::new(self, 9)
    }
    ///Bit 10 - Source selection for DatIn2 of DFSDM2
    #[inline(always)]
    pub fn dfsdm2_d2sel(&mut self) -> DFSDM2_D2SEL_W<'_, MCHDLYCRrs> {
        DFSDM2_D2SEL_W::new(self, 10)
    }
    ///Bit 11 - Source selection for DatIn4 of DFSDM2
    #[inline(always)]
    pub fn dfsdm2_d4sel(&mut self) -> DFSDM2_D4SEL_W<'_, MCHDLYCRrs> {
        DFSDM2_D4SEL_W::new(self, 11)
    }
    ///Bit 12 - Source selection for DatIn6 of DFSDM2
    #[inline(always)]
    pub fn dfsdm2_d6sel(&mut self) -> DFSDM2_D6SEL_W<'_, MCHDLYCRrs> {
        DFSDM2_D6SEL_W::new(self, 12)
    }
    ///Bit 13 - Distribution of the DFSDM2 bitstream clock gated by TIM3 OC4
    #[inline(always)]
    pub fn dfsdm2_ck04sel(&mut self) -> DFSDM2_CK04SEL_W<'_, MCHDLYCRrs> {
        DFSDM2_CK04SEL_W::new(self, 13)
    }
    ///Bit 14 - Distribution of the DFSDM2 bitstream clock gated by TIM3 OC3
    #[inline(always)]
    pub fn dfsdm2_ck15sel(&mut self) -> DFSDM2_CK15SEL_W<'_, MCHDLYCRrs> {
        DFSDM2_CK15SEL_W::new(self, 14)
    }
    ///Bit 15 - Distribution of the DFSDM2 bitstream clock gated by TIM3 OC2
    #[inline(always)]
    pub fn dfsdm2_ck26sel(&mut self) -> DFSDM2_CK26SEL_W<'_, MCHDLYCRrs> {
        DFSDM2_CK26SEL_W::new(self, 15)
    }
    ///Bit 16 - Distribution of the DFSDM2 bitstream clock gated by TIM3 OC1
    #[inline(always)]
    pub fn dfsdm2_ck37sel(&mut self) -> DFSDM2_CK37SEL_W<'_, MCHDLYCRrs> {
        DFSDM2_CK37SEL_W::new(self, 16)
    }
    ///Bit 17 - CkIn source selection for DFSDM2
    #[inline(always)]
    pub fn dfsdm2_cfg(&mut self) -> DFSDM2_CFG_W<'_, MCHDLYCRrs> {
        DFSDM2_CFG_W::new(self, 17)
    }
    ///Bit 18 - Source selection for DFSDM2_CKOUT
    #[inline(always)]
    pub fn dfsdm2_ckosel(&mut self) -> DFSDM2_CKOSEL_W<'_, MCHDLYCRrs> {
        DFSDM2_CKOSEL_W::new(self, 18)
    }
}
/**DFSDM Multi-channel delay control register

You can [`read`](crate::Reg::read) this register and get [`mchdlycr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mchdlycr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F413.html#SYSCFG:MCHDLYCR)*/
pub struct MCHDLYCRrs;
impl crate::RegisterSpec for MCHDLYCRrs {
    type Ux = u32;
}
///`read()` method returns [`mchdlycr::R`](R) reader structure
impl crate::Readable for MCHDLYCRrs {}
///`write(|w| ..)` method takes [`mchdlycr::W`](W) writer structure
impl crate::Writable for MCHDLYCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MCHDLYCR to value 0
impl crate::Resettable for MCHDLYCRrs {}

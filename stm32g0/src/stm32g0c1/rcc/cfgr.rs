///Register `CFGR` reader
pub type R = crate::R<CFGRrs>;
///Register `CFGR` writer
pub type W = crate::W<CFGRrs>;
/**System clock switch

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SW {
    ///0: HSISYS clock selected
    Hsisys = 0,
    ///1: HSE clock selected
    Hse = 1,
    ///2: PLLRCLK clock selected
    Pllr = 2,
    ///3: LSI clock selected
    Lsi = 3,
    ///4: LSE clock selected
    Lse = 4,
}
impl From<SW> for u8 {
    #[inline(always)]
    fn from(variant: SW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SW {
    type Ux = u8;
}
impl crate::IsEnum for SW {}
///Field `SW` reader - System clock switch
pub type SW_R = crate::FieldReader<SW>;
impl SW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SW> {
        match self.bits {
            0 => Some(SW::Hsisys),
            1 => Some(SW::Hse),
            2 => Some(SW::Pllr),
            3 => Some(SW::Lsi),
            4 => Some(SW::Lse),
            _ => None,
        }
    }
    ///HSISYS clock selected
    #[inline(always)]
    pub fn is_hsisys(&self) -> bool {
        *self == SW::Hsisys
    }
    ///HSE clock selected
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == SW::Hse
    }
    ///PLLRCLK clock selected
    #[inline(always)]
    pub fn is_pllr(&self) -> bool {
        *self == SW::Pllr
    }
    ///LSI clock selected
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == SW::Lsi
    }
    ///LSE clock selected
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == SW::Lse
    }
}
///Field `SW` writer - System clock switch
pub type SW_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SW>;
impl<'a, REG> SW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///HSISYS clock selected
    #[inline(always)]
    pub fn hsisys(self) -> &'a mut crate::W<REG> {
        self.variant(SW::Hsisys)
    }
    ///HSE clock selected
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(SW::Hse)
    }
    ///PLLRCLK clock selected
    #[inline(always)]
    pub fn pllr(self) -> &'a mut crate::W<REG> {
        self.variant(SW::Pllr)
    }
    ///LSI clock selected
    #[inline(always)]
    pub fn lsi(self) -> &'a mut crate::W<REG> {
        self.variant(SW::Lsi)
    }
    ///LSE clock selected
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(SW::Lse)
    }
}
/**System clock switch status

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SWS {
    ///0: HSISYS clock selected
    Hsisys = 0,
    ///1: HSE clock selected
    Hse = 1,
    ///2: PLLRCLK clock selected
    Pllr = 2,
    ///3: LSI clock selected
    Lsi = 3,
    ///4: LSE clock selected
    Lse = 4,
}
impl From<SWS> for u8 {
    #[inline(always)]
    fn from(variant: SWS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SWS {
    type Ux = u8;
}
impl crate::IsEnum for SWS {}
///Field `SWS` reader - System clock switch status
pub type SWS_R = crate::FieldReader<SWS>;
impl SWS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SWS> {
        match self.bits {
            0 => Some(SWS::Hsisys),
            1 => Some(SWS::Hse),
            2 => Some(SWS::Pllr),
            3 => Some(SWS::Lsi),
            4 => Some(SWS::Lse),
            _ => None,
        }
    }
    ///HSISYS clock selected
    #[inline(always)]
    pub fn is_hsisys(&self) -> bool {
        *self == SWS::Hsisys
    }
    ///HSE clock selected
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == SWS::Hse
    }
    ///PLLRCLK clock selected
    #[inline(always)]
    pub fn is_pllr(&self) -> bool {
        *self == SWS::Pllr
    }
    ///LSI clock selected
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == SWS::Lsi
    }
    ///LSE clock selected
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == SWS::Lse
    }
}
/**AHB prescaler

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HPRE {
    ///8: Divide by 2
    Div2 = 8,
    ///9: Divide by 4
    Div4 = 9,
    ///10: Divide by 8
    Div8 = 10,
    ///11: Divide by 16
    Div16 = 11,
    ///12: Divide by 64
    Div64 = 12,
    ///13: Divide by 128
    Div128 = 13,
    ///14: Divide by 256
    Div256 = 14,
    ///15: Divide by 512
    Div512 = 15,
    ///0: Divide by 1
    Div1 = 0,
}
impl From<HPRE> for u8 {
    #[inline(always)]
    fn from(variant: HPRE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HPRE {
    type Ux = u8;
}
impl crate::IsEnum for HPRE {}
///Field `HPRE` reader - AHB prescaler
pub type HPRE_R = crate::FieldReader<HPRE>;
impl HPRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HPRE {
        match self.bits {
            8 => HPRE::Div2,
            9 => HPRE::Div4,
            10 => HPRE::Div8,
            11 => HPRE::Div16,
            12 => HPRE::Div64,
            13 => HPRE::Div128,
            14 => HPRE::Div256,
            15 => HPRE::Div512,
            _ => HPRE::Div1,
        }
    }
    ///Divide by 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == HPRE::Div2
    }
    ///Divide by 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == HPRE::Div4
    }
    ///Divide by 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == HPRE::Div8
    }
    ///Divide by 16
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == HPRE::Div16
    }
    ///Divide by 64
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == HPRE::Div64
    }
    ///Divide by 128
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == HPRE::Div128
    }
    ///Divide by 256
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == HPRE::Div256
    }
    ///Divide by 512
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == HPRE::Div512
    }
    ///Divide by 1
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        matches!(self.variant(), HPRE::Div1)
    }
}
///Field `HPRE` writer - AHB prescaler
pub type HPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, HPRE, crate::Safe>;
impl<'a, REG> HPRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Divide by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div2)
    }
    ///Divide by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div4)
    }
    ///Divide by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div8)
    }
    ///Divide by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div16)
    }
    ///Divide by 64
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div64)
    }
    ///Divide by 128
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div128)
    }
    ///Divide by 256
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div256)
    }
    ///Divide by 512
    #[inline(always)]
    pub fn div512(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div512)
    }
    ///Divide by 1
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div1)
    }
}
/**APB prescaler

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PPRE {
    ///4: Divide by 2
    Div2 = 4,
    ///5: Divide by 4
    Div4 = 5,
    ///6: Divide by 8
    Div8 = 6,
    ///7: Divide by 16
    Div16 = 7,
    ///0: Divide by 1
    Div1 = 0,
}
impl From<PPRE> for u8 {
    #[inline(always)]
    fn from(variant: PPRE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PPRE {
    type Ux = u8;
}
impl crate::IsEnum for PPRE {}
///Field `PPRE` reader - APB prescaler
pub type PPRE_R = crate::FieldReader<PPRE>;
impl PPRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PPRE {
        match self.bits {
            4 => PPRE::Div2,
            5 => PPRE::Div4,
            6 => PPRE::Div8,
            7 => PPRE::Div16,
            _ => PPRE::Div1,
        }
    }
    ///Divide by 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PPRE::Div2
    }
    ///Divide by 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PPRE::Div4
    }
    ///Divide by 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PPRE::Div8
    }
    ///Divide by 16
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PPRE::Div16
    }
    ///Divide by 1
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        matches!(self.variant(), PPRE::Div1)
    }
}
///Field `PPRE` writer - APB prescaler
pub type PPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PPRE, crate::Safe>;
impl<'a, REG> PPRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Divide by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PPRE::Div2)
    }
    ///Divide by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PPRE::Div4)
    }
    ///Divide by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PPRE::Div8)
    }
    ///Divide by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PPRE::Div16)
    }
    ///Divide by 1
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(PPRE::Div1)
    }
}
/**MCO2SEL

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCO2SEL {
    ///0: No clock
    NoClock = 0,
    ///1: SYSCLK clock selected
    Sysclk = 1,
    ///3: HSI16 oscillator clock selected
    Hsi16 = 3,
    ///4: HSE oscillator clock selected
    Hse = 4,
    ///5: PLLRCLK clock selected
    Pllr = 5,
    ///6: LSI oscillator clock selected
    Lsi = 6,
    ///7: LSE oscillator clock selected
    Lse = 7,
    ///8: PLLPCLK clock selected
    Pllp = 8,
    ///9: PLLQCLK clock selected
    Pllq = 9,
    ///10: RTC clock selected
    Rtc = 10,
    ///11: RTC wakeup output selected
    Rtcwake = 11,
}
impl From<MCO2SEL> for u8 {
    #[inline(always)]
    fn from(variant: MCO2SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MCO2SEL {
    type Ux = u8;
}
impl crate::IsEnum for MCO2SEL {}
///Field `MCO2SEL` reader - MCO2SEL
pub type MCO2SEL_R = crate::FieldReader<MCO2SEL>;
impl MCO2SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<MCO2SEL> {
        match self.bits {
            0 => Some(MCO2SEL::NoClock),
            1 => Some(MCO2SEL::Sysclk),
            3 => Some(MCO2SEL::Hsi16),
            4 => Some(MCO2SEL::Hse),
            5 => Some(MCO2SEL::Pllr),
            6 => Some(MCO2SEL::Lsi),
            7 => Some(MCO2SEL::Lse),
            8 => Some(MCO2SEL::Pllp),
            9 => Some(MCO2SEL::Pllq),
            10 => Some(MCO2SEL::Rtc),
            11 => Some(MCO2SEL::Rtcwake),
            _ => None,
        }
    }
    ///No clock
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == MCO2SEL::NoClock
    }
    ///SYSCLK clock selected
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == MCO2SEL::Sysclk
    }
    ///HSI16 oscillator clock selected
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == MCO2SEL::Hsi16
    }
    ///HSE oscillator clock selected
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == MCO2SEL::Hse
    }
    ///PLLRCLK clock selected
    #[inline(always)]
    pub fn is_pllr(&self) -> bool {
        *self == MCO2SEL::Pllr
    }
    ///LSI oscillator clock selected
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == MCO2SEL::Lsi
    }
    ///LSE oscillator clock selected
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == MCO2SEL::Lse
    }
    ///PLLPCLK clock selected
    #[inline(always)]
    pub fn is_pllp(&self) -> bool {
        *self == MCO2SEL::Pllp
    }
    ///PLLQCLK clock selected
    #[inline(always)]
    pub fn is_pllq(&self) -> bool {
        *self == MCO2SEL::Pllq
    }
    ///RTC clock selected
    #[inline(always)]
    pub fn is_rtc(&self) -> bool {
        *self == MCO2SEL::Rtc
    }
    ///RTC wakeup output selected
    #[inline(always)]
    pub fn is_rtcwake(&self) -> bool {
        *self == MCO2SEL::Rtcwake
    }
}
///Field `MCO2SEL` writer - MCO2SEL
pub type MCO2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, MCO2SEL>;
impl<'a, REG> MCO2SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No clock
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2SEL::NoClock)
    }
    ///SYSCLK clock selected
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2SEL::Sysclk)
    }
    ///HSI16 oscillator clock selected
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2SEL::Hsi16)
    }
    ///HSE oscillator clock selected
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2SEL::Hse)
    }
    ///PLLRCLK clock selected
    #[inline(always)]
    pub fn pllr(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2SEL::Pllr)
    }
    ///LSI oscillator clock selected
    #[inline(always)]
    pub fn lsi(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2SEL::Lsi)
    }
    ///LSE oscillator clock selected
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2SEL::Lse)
    }
    ///PLLPCLK clock selected
    #[inline(always)]
    pub fn pllp(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2SEL::Pllp)
    }
    ///PLLQCLK clock selected
    #[inline(always)]
    pub fn pllq(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2SEL::Pllq)
    }
    ///RTC clock selected
    #[inline(always)]
    pub fn rtc(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2SEL::Rtc)
    }
    ///RTC wakeup output selected
    #[inline(always)]
    pub fn rtcwake(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2SEL::Rtcwake)
    }
}
/**MCO2PRE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCO2PRE {
    ///0: Divide by 1
    Div1 = 0,
    ///1: Divide by 2
    Div2 = 1,
    ///2: Divide by 4
    Div3 = 2,
    ///3: Divide by 8
    Div8 = 3,
    ///4: Divide by 16
    Div16 = 4,
    ///5: Divide by 32
    Div32 = 5,
    ///6: Divide by 64
    Div64 = 6,
    ///7: Divide by 128
    Div128 = 7,
    ///8: Divide by 256
    Div256 = 8,
    ///9: Divide by 512
    Div512 = 9,
    ///10: Divide by 1024
    Div1024 = 10,
}
impl From<MCO2PRE> for u8 {
    #[inline(always)]
    fn from(variant: MCO2PRE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MCO2PRE {
    type Ux = u8;
}
impl crate::IsEnum for MCO2PRE {}
///Field `MCO2PRE` reader - MCO2PRE
pub type MCO2PRE_R = crate::FieldReader<MCO2PRE>;
impl MCO2PRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<MCO2PRE> {
        match self.bits {
            0 => Some(MCO2PRE::Div1),
            1 => Some(MCO2PRE::Div2),
            2 => Some(MCO2PRE::Div3),
            3 => Some(MCO2PRE::Div8),
            4 => Some(MCO2PRE::Div16),
            5 => Some(MCO2PRE::Div32),
            6 => Some(MCO2PRE::Div64),
            7 => Some(MCO2PRE::Div128),
            8 => Some(MCO2PRE::Div256),
            9 => Some(MCO2PRE::Div512),
            10 => Some(MCO2PRE::Div1024),
            _ => None,
        }
    }
    ///Divide by 1
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == MCO2PRE::Div1
    }
    ///Divide by 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == MCO2PRE::Div2
    }
    ///Divide by 4
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == MCO2PRE::Div3
    }
    ///Divide by 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == MCO2PRE::Div8
    }
    ///Divide by 16
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == MCO2PRE::Div16
    }
    ///Divide by 32
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == MCO2PRE::Div32
    }
    ///Divide by 64
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == MCO2PRE::Div64
    }
    ///Divide by 128
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == MCO2PRE::Div128
    }
    ///Divide by 256
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == MCO2PRE::Div256
    }
    ///Divide by 512
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == MCO2PRE::Div512
    }
    ///Divide by 1024
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == MCO2PRE::Div1024
    }
}
///Field `MCO2PRE` writer - MCO2PRE
pub type MCO2PRE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, MCO2PRE>;
impl<'a, REG> MCO2PRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Divide by 1
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2PRE::Div1)
    }
    ///Divide by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2PRE::Div2)
    }
    ///Divide by 4
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2PRE::Div3)
    }
    ///Divide by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2PRE::Div8)
    }
    ///Divide by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2PRE::Div16)
    }
    ///Divide by 32
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2PRE::Div32)
    }
    ///Divide by 64
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2PRE::Div64)
    }
    ///Divide by 128
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2PRE::Div128)
    }
    ///Divide by 256
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2PRE::Div256)
    }
    ///Divide by 512
    #[inline(always)]
    pub fn div512(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2PRE::Div512)
    }
    ///Divide by 1024
    #[inline(always)]
    pub fn div1024(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2PRE::Div1024)
    }
}
///Field `MCOPRE` reader - Microcontroller clock output prescaler
pub use MCO2PRE_R as MCOPRE_R;
///Field `MCOPRE` writer - Microcontroller clock output prescaler
pub use MCO2PRE_W as MCOPRE_W;
///Field `MCOSEL` reader - Microcontroller clock output
pub use MCO2SEL_R as MCOSEL_R;
///Field `MCOSEL` writer - Microcontroller clock output
pub use MCO2SEL_W as MCOSEL_W;
impl R {
    ///Bits 0:2 - System clock switch
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:5 - System clock switch status
    #[inline(always)]
    pub fn sws(&self) -> SWS_R {
        SWS_R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bits 8:11 - AHB prescaler
    #[inline(always)]
    pub fn hpre(&self) -> HPRE_R {
        HPRE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:14 - APB prescaler
    #[inline(always)]
    pub fn ppre(&self) -> PPRE_R {
        PPRE_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 16:19 - MCO2SEL
    #[inline(always)]
    pub fn mco2sel(&self) -> MCO2SEL_R {
        MCO2SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - MCO2PRE
    #[inline(always)]
    pub fn mco2pre(&self) -> MCO2PRE_R {
        MCO2PRE_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - Microcontroller clock output
    #[inline(always)]
    pub fn mcosel(&self) -> MCOSEL_R {
        MCOSEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - Microcontroller clock output prescaler
    #[inline(always)]
    pub fn mcopre(&self) -> MCOPRE_R {
        MCOPRE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR")
            .field("mco2pre", &self.mco2pre())
            .field("mcopre", &self.mcopre())
            .field("mco2sel", &self.mco2sel())
            .field("mcosel", &self.mcosel())
            .field("ppre", &self.ppre())
            .field("hpre", &self.hpre())
            .field("sws", &self.sws())
            .field("sw", &self.sw())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - System clock switch
    #[inline(always)]
    pub fn sw(&mut self) -> SW_W<CFGRrs> {
        SW_W::new(self, 0)
    }
    ///Bits 8:11 - AHB prescaler
    #[inline(always)]
    pub fn hpre(&mut self) -> HPRE_W<CFGRrs> {
        HPRE_W::new(self, 8)
    }
    ///Bits 12:14 - APB prescaler
    #[inline(always)]
    pub fn ppre(&mut self) -> PPRE_W<CFGRrs> {
        PPRE_W::new(self, 12)
    }
    ///Bits 16:19 - MCO2SEL
    #[inline(always)]
    pub fn mco2sel(&mut self) -> MCO2SEL_W<CFGRrs> {
        MCO2SEL_W::new(self, 16)
    }
    ///Bits 20:23 - MCO2PRE
    #[inline(always)]
    pub fn mco2pre(&mut self) -> MCO2PRE_W<CFGRrs> {
        MCO2PRE_W::new(self, 20)
    }
    ///Bits 24:27 - Microcontroller clock output
    #[inline(always)]
    pub fn mcosel(&mut self) -> MCOSEL_W<CFGRrs> {
        MCOSEL_W::new(self, 24)
    }
    ///Bits 28:31 - Microcontroller clock output prescaler
    #[inline(always)]
    pub fn mcopre(&mut self) -> MCOPRE_W<CFGRrs> {
        MCOPRE_W::new(self, 28)
    }
}
/**Clock configuration register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0C1.html#RCC:CFGR)*/
pub struct CFGRrs;
impl crate::RegisterSpec for CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`cfgr::R`](R) reader structure
impl crate::Readable for CFGRrs {}
///`write(|w| ..)` method takes [`cfgr::W`](W) writer structure
impl crate::Writable for CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR to value 0
impl crate::Resettable for CFGRrs {}

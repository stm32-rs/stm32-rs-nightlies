#[doc = "Register `CFGR2` reader"]
pub type R = crate::R<CFGR2rs>;
#[doc = "Register `CFGR2` writer"]
pub type W = crate::W<CFGR2rs>;
#[doc = "AHB prescaler Set and reset by software to control the division factor of rcc_hclk. Changing this division ratio has an impact on the frequency of all bus matrix clocks 0xxx: rcc_hclk = sys_ck (default after reset)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HPRE {
    #[doc = "0: SYSCLK not divided"]
    Div1 = 0,
    #[doc = "8: SYSCLK divided by 2"]
    Div2 = 8,
    #[doc = "9: SYSCLK divided by 4"]
    Div4 = 9,
    #[doc = "10: SYSCLK divided by 8"]
    Div8 = 10,
    #[doc = "11: SYSCLK divided by 16"]
    Div16 = 11,
    #[doc = "12: SYSCLK divided by 64"]
    Div64 = 12,
    #[doc = "13: SYSCLK divided by 128"]
    Div128 = 13,
    #[doc = "14: SYSCLK divided by 256"]
    Div256 = 14,
    #[doc = "15: SYSCLK divided by 512"]
    Div512 = 15,
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
#[doc = "Field `HPRE` reader - AHB prescaler Set and reset by software to control the division factor of rcc_hclk. Changing this division ratio has an impact on the frequency of all bus matrix clocks 0xxx: rcc_hclk = sys_ck (default after reset)"]
pub type HPRE_R = crate::FieldReader<HPRE>;
impl HPRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<HPRE> {
        match self.bits {
            0 => Some(HPRE::Div1),
            8 => Some(HPRE::Div2),
            9 => Some(HPRE::Div4),
            10 => Some(HPRE::Div8),
            11 => Some(HPRE::Div16),
            12 => Some(HPRE::Div64),
            13 => Some(HPRE::Div128),
            14 => Some(HPRE::Div256),
            15 => Some(HPRE::Div512),
            _ => None,
        }
    }
    #[doc = "SYSCLK not divided"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == HPRE::Div1
    }
    #[doc = "SYSCLK divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == HPRE::Div2
    }
    #[doc = "SYSCLK divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == HPRE::Div4
    }
    #[doc = "SYSCLK divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == HPRE::Div8
    }
    #[doc = "SYSCLK divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == HPRE::Div16
    }
    #[doc = "SYSCLK divided by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == HPRE::Div64
    }
    #[doc = "SYSCLK divided by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == HPRE::Div128
    }
    #[doc = "SYSCLK divided by 256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == HPRE::Div256
    }
    #[doc = "SYSCLK divided by 512"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == HPRE::Div512
    }
}
#[doc = "Field `HPRE` writer - AHB prescaler Set and reset by software to control the division factor of rcc_hclk. Changing this division ratio has an impact on the frequency of all bus matrix clocks 0xxx: rcc_hclk = sys_ck (default after reset)"]
pub type HPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, HPRE>;
impl<'a, REG> HPRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SYSCLK not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div1)
    }
    #[doc = "SYSCLK divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div2)
    }
    #[doc = "SYSCLK divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div4)
    }
    #[doc = "SYSCLK divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div8)
    }
    #[doc = "SYSCLK divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div16)
    }
    #[doc = "SYSCLK divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div64)
    }
    #[doc = "SYSCLK divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div128)
    }
    #[doc = "SYSCLK divided by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div256)
    }
    #[doc = "SYSCLK divided by 512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div512)
    }
}
#[doc = "APB low-speed prescaler (APB1) Set and reset by software to control the division factor of rcc_pclk1. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk after PPRE write. 0xx: rcc_pclk1 = rcc_hclk1 (default after reset)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PPRE1 {
    #[doc = "0: HCLK not divided"]
    Div1 = 0,
    #[doc = "4: HCLK divided by 2"]
    Div2 = 4,
    #[doc = "5: HCLK divided by 4"]
    Div4 = 5,
    #[doc = "6: HCLK divided by 8"]
    Div8 = 6,
    #[doc = "7: HCLK divided by 16"]
    Div16 = 7,
}
impl From<PPRE1> for u8 {
    #[inline(always)]
    fn from(variant: PPRE1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PPRE1 {
    type Ux = u8;
}
#[doc = "Field `PPRE1` reader - APB low-speed prescaler (APB1) Set and reset by software to control the division factor of rcc_pclk1. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk after PPRE write. 0xx: rcc_pclk1 = rcc_hclk1 (default after reset)"]
pub type PPRE1_R = crate::FieldReader<PPRE1>;
impl PPRE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PPRE1> {
        match self.bits {
            0 => Some(PPRE1::Div1),
            4 => Some(PPRE1::Div2),
            5 => Some(PPRE1::Div4),
            6 => Some(PPRE1::Div8),
            7 => Some(PPRE1::Div16),
            _ => None,
        }
    }
    #[doc = "HCLK not divided"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PPRE1::Div1
    }
    #[doc = "HCLK divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PPRE1::Div2
    }
    #[doc = "HCLK divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PPRE1::Div4
    }
    #[doc = "HCLK divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PPRE1::Div8
    }
    #[doc = "HCLK divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PPRE1::Div16
    }
}
#[doc = "Field `PPRE1` writer - APB low-speed prescaler (APB1) Set and reset by software to control the division factor of rcc_pclk1. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk after PPRE write. 0xx: rcc_pclk1 = rcc_hclk1 (default after reset)"]
pub type PPRE1_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PPRE1>;
impl<'a, REG> PPRE1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HCLK not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(PPRE1::Div1)
    }
    #[doc = "HCLK divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PPRE1::Div2)
    }
    #[doc = "HCLK divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PPRE1::Div4)
    }
    #[doc = "HCLK divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PPRE1::Div8)
    }
    #[doc = "HCLK divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PPRE1::Div16)
    }
}
#[doc = "Field `PPRE2` reader - APB high-speed prescaler (APB2) Set and reset by software to control APB high-speed clocks division factor. The clocks are divided with the new prescaler factor from 1 to 16 APB cycles after PPRE2 write. 0xx: rcc_pclk2 = rcc_hclk1"]
pub use PPRE1_R as PPRE2_R;
#[doc = "Field `PPRE3` reader - APB low-speed prescaler (APB3) Set and reset by software to control APB low-speed clocks division factor. The clocks are divided with the new prescaler factor from 1 to 16 APB cycles after PPRE3 write. 0xx: rcc_pclk3 = rcc_hclk1"]
pub use PPRE1_R as PPRE3_R;
#[doc = "Field `PPRE2` writer - APB high-speed prescaler (APB2) Set and reset by software to control APB high-speed clocks division factor. The clocks are divided with the new prescaler factor from 1 to 16 APB cycles after PPRE2 write. 0xx: rcc_pclk2 = rcc_hclk1"]
pub use PPRE1_W as PPRE2_W;
#[doc = "Field `PPRE3` writer - APB low-speed prescaler (APB3) Set and reset by software to control APB low-speed clocks division factor. The clocks are divided with the new prescaler factor from 1 to 16 APB cycles after PPRE3 write. 0xx: rcc_pclk3 = rcc_hclk1"]
pub use PPRE1_W as PPRE3_W;
#[doc = "AHB1 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB1 peripherals from RCC_AHB1ENR are used and when their clocks are disabled in RCC_AHB1ENR. When this bit is set, all the AHB1 peripherals clocks from RCC_AHB1ENR are off. enable control bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AHB1DIS {
    #[doc = "0: The selected clock is enabled"]
    Enabled = 0,
    #[doc = "1: The selected clock is disabled"]
    Disabled = 1,
}
impl From<AHB1DIS> for bool {
    #[inline(always)]
    fn from(variant: AHB1DIS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHB1DIS` reader - AHB1 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB1 peripherals from RCC_AHB1ENR are used and when their clocks are disabled in RCC_AHB1ENR. When this bit is set, all the AHB1 peripherals clocks from RCC_AHB1ENR are off. enable control bits"]
pub type AHB1DIS_R = crate::BitReader<AHB1DIS>;
impl AHB1DIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AHB1DIS {
        match self.bits {
            false => AHB1DIS::Enabled,
            true => AHB1DIS::Disabled,
        }
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AHB1DIS::Enabled
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AHB1DIS::Disabled
    }
}
#[doc = "Field `AHB1DIS` writer - AHB1 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB1 peripherals from RCC_AHB1ENR are used and when their clocks are disabled in RCC_AHB1ENR. When this bit is set, all the AHB1 peripherals clocks from RCC_AHB1ENR are off. enable control bits"]
pub type AHB1DIS_W<'a, REG> = crate::BitWriter<'a, REG, AHB1DIS>;
impl<'a, REG> AHB1DIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AHB1DIS::Enabled)
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AHB1DIS::Disabled)
    }
}
#[doc = "Field `AHB2DIS` reader - AHB2 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB2 peripherals from RCC_AHB2ENR are used and when their clocks are disabled in RCC_AHB2ENR. When this bit is set, all the AHB2 peripherals clocks from RCC_AHB2ENR are off. enable control bits"]
pub use AHB1DIS_R as AHB2DIS_R;
#[doc = "Field `AHB4DIS` reader - AHB4 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB4 peripherals from RCC_AHB4ENR are used and when their clocks are disabled in RCC_AHB4ENR. When this bit is set, all the AHB4 peripherals clocks from RCC_AHB4ENR are off. enable control bits"]
pub use AHB1DIS_R as AHB4DIS_R;
#[doc = "Field `APB1DIS` reader - APB1 clock disable value This bit can be set in order to further reduce power consumption, when none of the APB1 peripherals (except IWDG) are used and when their clocks are disabled in RCC_APB1ENR. When this bit is set, all the APB1 peripherals clocks are off, except for IWDG. control bits"]
pub use AHB1DIS_R as APB1DIS_R;
#[doc = "Field `APB2DIS` reader - APB2 clock disable value This bit can be set in order to further reduce power consumption, when none of the APB2 peripherals are used and when their clocks are disabled in RCC_APB2ENR. When this bit is set, all the APB2 peripherals clocks are off. control bits"]
pub use AHB1DIS_R as APB2DIS_R;
#[doc = "Field `APB3DIS` reader - APB3 clock disable value.Set and cleared by software This bit can be set in order to further reduce power consumption, when none of the APB3 peripherals are used and when their clocks are disabled in RCC_APB3ENR. When this bit is set, all the APB3 peripherals clocks are off. control bits"]
pub use AHB1DIS_R as APB3DIS_R;
#[doc = "Field `AHB2DIS` writer - AHB2 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB2 peripherals from RCC_AHB2ENR are used and when their clocks are disabled in RCC_AHB2ENR. When this bit is set, all the AHB2 peripherals clocks from RCC_AHB2ENR are off. enable control bits"]
pub use AHB1DIS_W as AHB2DIS_W;
#[doc = "Field `AHB4DIS` writer - AHB4 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB4 peripherals from RCC_AHB4ENR are used and when their clocks are disabled in RCC_AHB4ENR. When this bit is set, all the AHB4 peripherals clocks from RCC_AHB4ENR are off. enable control bits"]
pub use AHB1DIS_W as AHB4DIS_W;
#[doc = "Field `APB1DIS` writer - APB1 clock disable value This bit can be set in order to further reduce power consumption, when none of the APB1 peripherals (except IWDG) are used and when their clocks are disabled in RCC_APB1ENR. When this bit is set, all the APB1 peripherals clocks are off, except for IWDG. control bits"]
pub use AHB1DIS_W as APB1DIS_W;
#[doc = "Field `APB2DIS` writer - APB2 clock disable value This bit can be set in order to further reduce power consumption, when none of the APB2 peripherals are used and when their clocks are disabled in RCC_APB2ENR. When this bit is set, all the APB2 peripherals clocks are off. control bits"]
pub use AHB1DIS_W as APB2DIS_W;
#[doc = "Field `APB3DIS` writer - APB3 clock disable value.Set and cleared by software This bit can be set in order to further reduce power consumption, when none of the APB3 peripherals are used and when their clocks are disabled in RCC_APB3ENR. When this bit is set, all the APB3 peripherals clocks are off. control bits"]
pub use AHB1DIS_W as APB3DIS_W;
impl R {
    #[doc = "Bits 0:3 - AHB prescaler Set and reset by software to control the division factor of rcc_hclk. Changing this division ratio has an impact on the frequency of all bus matrix clocks 0xxx: rcc_hclk = sys_ck (default after reset)"]
    #[inline(always)]
    pub fn hpre(&self) -> HPRE_R {
        HPRE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - APB low-speed prescaler (APB1) Set and reset by software to control the division factor of rcc_pclk1. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk after PPRE write. 0xx: rcc_pclk1 = rcc_hclk1 (default after reset)"]
    #[inline(always)]
    pub fn ppre1(&self) -> PPRE1_R {
        PPRE1_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - APB high-speed prescaler (APB2) Set and reset by software to control APB high-speed clocks division factor. The clocks are divided with the new prescaler factor from 1 to 16 APB cycles after PPRE2 write. 0xx: rcc_pclk2 = rcc_hclk1"]
    #[inline(always)]
    pub fn ppre2(&self) -> PPRE2_R {
        PPRE2_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - APB low-speed prescaler (APB3) Set and reset by software to control APB low-speed clocks division factor. The clocks are divided with the new prescaler factor from 1 to 16 APB cycles after PPRE3 write. 0xx: rcc_pclk3 = rcc_hclk1"]
    #[inline(always)]
    pub fn ppre3(&self) -> PPRE3_R {
        PPRE3_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 16 - AHB1 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB1 peripherals from RCC_AHB1ENR are used and when their clocks are disabled in RCC_AHB1ENR. When this bit is set, all the AHB1 peripherals clocks from RCC_AHB1ENR are off. enable control bits"]
    #[inline(always)]
    pub fn ahb1dis(&self) -> AHB1DIS_R {
        AHB1DIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - AHB2 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB2 peripherals from RCC_AHB2ENR are used and when their clocks are disabled in RCC_AHB2ENR. When this bit is set, all the AHB2 peripherals clocks from RCC_AHB2ENR are off. enable control bits"]
    #[inline(always)]
    pub fn ahb2dis(&self) -> AHB2DIS_R {
        AHB2DIS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - AHB4 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB4 peripherals from RCC_AHB4ENR are used and when their clocks are disabled in RCC_AHB4ENR. When this bit is set, all the AHB4 peripherals clocks from RCC_AHB4ENR are off. enable control bits"]
    #[inline(always)]
    pub fn ahb4dis(&self) -> AHB4DIS_R {
        AHB4DIS_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - APB1 clock disable value This bit can be set in order to further reduce power consumption, when none of the APB1 peripherals (except IWDG) are used and when their clocks are disabled in RCC_APB1ENR. When this bit is set, all the APB1 peripherals clocks are off, except for IWDG. control bits"]
    #[inline(always)]
    pub fn apb1dis(&self) -> APB1DIS_R {
        APB1DIS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - APB2 clock disable value This bit can be set in order to further reduce power consumption, when none of the APB2 peripherals are used and when their clocks are disabled in RCC_APB2ENR. When this bit is set, all the APB2 peripherals clocks are off. control bits"]
    #[inline(always)]
    pub fn apb2dis(&self) -> APB2DIS_R {
        APB2DIS_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - APB3 clock disable value.Set and cleared by software This bit can be set in order to further reduce power consumption, when none of the APB3 peripherals are used and when their clocks are disabled in RCC_APB3ENR. When this bit is set, all the APB3 peripherals clocks are off. control bits"]
    #[inline(always)]
    pub fn apb3dis(&self) -> APB3DIS_R {
        APB3DIS_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - AHB prescaler Set and reset by software to control the division factor of rcc_hclk. Changing this division ratio has an impact on the frequency of all bus matrix clocks 0xxx: rcc_hclk = sys_ck (default after reset)"]
    #[inline(always)]
    #[must_use]
    pub fn hpre(&mut self) -> HPRE_W<CFGR2rs> {
        HPRE_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - APB low-speed prescaler (APB1) Set and reset by software to control the division factor of rcc_pclk1. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk after PPRE write. 0xx: rcc_pclk1 = rcc_hclk1 (default after reset)"]
    #[inline(always)]
    #[must_use]
    pub fn ppre1(&mut self) -> PPRE1_W<CFGR2rs> {
        PPRE1_W::new(self, 4)
    }
    #[doc = "Bits 8:10 - APB high-speed prescaler (APB2) Set and reset by software to control APB high-speed clocks division factor. The clocks are divided with the new prescaler factor from 1 to 16 APB cycles after PPRE2 write. 0xx: rcc_pclk2 = rcc_hclk1"]
    #[inline(always)]
    #[must_use]
    pub fn ppre2(&mut self) -> PPRE2_W<CFGR2rs> {
        PPRE2_W::new(self, 8)
    }
    #[doc = "Bits 12:14 - APB low-speed prescaler (APB3) Set and reset by software to control APB low-speed clocks division factor. The clocks are divided with the new prescaler factor from 1 to 16 APB cycles after PPRE3 write. 0xx: rcc_pclk3 = rcc_hclk1"]
    #[inline(always)]
    #[must_use]
    pub fn ppre3(&mut self) -> PPRE3_W<CFGR2rs> {
        PPRE3_W::new(self, 12)
    }
    #[doc = "Bit 16 - AHB1 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB1 peripherals from RCC_AHB1ENR are used and when their clocks are disabled in RCC_AHB1ENR. When this bit is set, all the AHB1 peripherals clocks from RCC_AHB1ENR are off. enable control bits"]
    #[inline(always)]
    #[must_use]
    pub fn ahb1dis(&mut self) -> AHB1DIS_W<CFGR2rs> {
        AHB1DIS_W::new(self, 16)
    }
    #[doc = "Bit 17 - AHB2 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB2 peripherals from RCC_AHB2ENR are used and when their clocks are disabled in RCC_AHB2ENR. When this bit is set, all the AHB2 peripherals clocks from RCC_AHB2ENR are off. enable control bits"]
    #[inline(always)]
    #[must_use]
    pub fn ahb2dis(&mut self) -> AHB2DIS_W<CFGR2rs> {
        AHB2DIS_W::new(self, 17)
    }
    #[doc = "Bit 19 - AHB4 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB4 peripherals from RCC_AHB4ENR are used and when their clocks are disabled in RCC_AHB4ENR. When this bit is set, all the AHB4 peripherals clocks from RCC_AHB4ENR are off. enable control bits"]
    #[inline(always)]
    #[must_use]
    pub fn ahb4dis(&mut self) -> AHB4DIS_W<CFGR2rs> {
        AHB4DIS_W::new(self, 19)
    }
    #[doc = "Bit 20 - APB1 clock disable value This bit can be set in order to further reduce power consumption, when none of the APB1 peripherals (except IWDG) are used and when their clocks are disabled in RCC_APB1ENR. When this bit is set, all the APB1 peripherals clocks are off, except for IWDG. control bits"]
    #[inline(always)]
    #[must_use]
    pub fn apb1dis(&mut self) -> APB1DIS_W<CFGR2rs> {
        APB1DIS_W::new(self, 20)
    }
    #[doc = "Bit 21 - APB2 clock disable value This bit can be set in order to further reduce power consumption, when none of the APB2 peripherals are used and when their clocks are disabled in RCC_APB2ENR. When this bit is set, all the APB2 peripherals clocks are off. control bits"]
    #[inline(always)]
    #[must_use]
    pub fn apb2dis(&mut self) -> APB2DIS_W<CFGR2rs> {
        APB2DIS_W::new(self, 21)
    }
    #[doc = "Bit 22 - APB3 clock disable value.Set and cleared by software This bit can be set in order to further reduce power consumption, when none of the APB3 peripherals are used and when their clocks are disabled in RCC_APB3ENR. When this bit is set, all the APB3 peripherals clocks are off. control bits"]
    #[inline(always)]
    #[must_use]
    pub fn apb3dis(&mut self) -> APB3DIS_W<CFGR2rs> {
        APB3DIS_W::new(self, 22)
    }
}
#[doc = "RCC CPU domain clock configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGR2rs;
impl crate::RegisterSpec for CFGR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr2::R`](R) reader structure"]
impl crate::Readable for CFGR2rs {}
#[doc = "`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure"]
impl crate::Writable for CFGR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR2 to value 0"]
impl crate::Resettable for CFGR2rs {
    const RESET_VALUE: u32 = 0;
}

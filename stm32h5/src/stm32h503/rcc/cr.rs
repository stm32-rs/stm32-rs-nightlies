#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "HSI clock enable Set and cleared by software. Set by hardware to force the HSI to ON when the product leaves Stop mode, if STOPWUCK = 1 or STOPKERWUCK = 1. Set by hardware to force the HSI to ON when the product leaves Standby mode or in case of a failure of the HSE which is used as the system clock source. This bit cannot be cleared if the HSI is used directly (via SW mux) as system clock, or if the HSI is selected as reference clock for PLL1 with PLL1 enabled (PLL1ON bit set to 1).\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSION {
    #[doc = "0: Clock Off"]
    Off = 0,
    #[doc = "1: Clock On"]
    On = 1,
}
impl From<HSION> for bool {
    #[inline(always)]
    fn from(variant: HSION) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSION` reader - HSI clock enable Set and cleared by software. Set by hardware to force the HSI to ON when the product leaves Stop mode, if STOPWUCK = 1 or STOPKERWUCK = 1. Set by hardware to force the HSI to ON when the product leaves Standby mode or in case of a failure of the HSE which is used as the system clock source. This bit cannot be cleared if the HSI is used directly (via SW mux) as system clock, or if the HSI is selected as reference clock for PLL1 with PLL1 enabled (PLL1ON bit set to 1)."]
pub type HSION_R = crate::BitReader<HSION>;
impl HSION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSION {
        match self.bits {
            false => HSION::Off,
            true => HSION::On,
        }
    }
    #[doc = "Clock Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == HSION::Off
    }
    #[doc = "Clock On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == HSION::On
    }
}
#[doc = "Field `HSION` writer - HSI clock enable Set and cleared by software. Set by hardware to force the HSI to ON when the product leaves Stop mode, if STOPWUCK = 1 or STOPKERWUCK = 1. Set by hardware to force the HSI to ON when the product leaves Standby mode or in case of a failure of the HSE which is used as the system clock source. This bit cannot be cleared if the HSI is used directly (via SW mux) as system clock, or if the HSI is selected as reference clock for PLL1 with PLL1 enabled (PLL1ON bit set to 1)."]
pub type HSION_W<'a, REG> = crate::BitWriter<'a, REG, HSION>;
impl<'a, REG> HSION_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(HSION::Off)
    }
    #[doc = "Clock On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(HSION::On)
    }
}
#[doc = "HSI clock ready flag Set by hardware to indicate that the HSI oscillator is stable.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIRDYR {
    #[doc = "0: Clock not ready"]
    NotReady = 0,
    #[doc = "1: Clock ready"]
    Ready = 1,
}
impl From<HSIRDYR> for bool {
    #[inline(always)]
    fn from(variant: HSIRDYR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSIRDY` reader - HSI clock ready flag Set by hardware to indicate that the HSI oscillator is stable."]
pub type HSIRDY_R = crate::BitReader<HSIRDYR>;
impl HSIRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSIRDYR {
        match self.bits {
            false => HSIRDYR::NotReady,
            true => HSIRDYR::Ready,
        }
    }
    #[doc = "Clock not ready"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == HSIRDYR::NotReady
    }
    #[doc = "Clock ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == HSIRDYR::Ready
    }
}
#[doc = "Field `HSIKERON` reader - HSI clock enable in Stop mode Set and reset by software to force the HSI to ON, even in Stop mode, in order to be quickly available as kernel clock for peripherals. This bit has no effect on the value of HSION."]
pub use HSION_R as HSIKERON_R;
#[doc = "Field `HSIKERON` writer - HSI clock enable in Stop mode Set and reset by software to force the HSI to ON, even in Stop mode, in order to be quickly available as kernel clock for peripherals. This bit has no effect on the value of HSION."]
pub use HSION_W as HSIKERON_W;
#[doc = "HSI clock divider Set and reset by software. These bits allow selecting a division ratio in order to configure the wanted HSI clock frequency. The HSIDIV cannot be changed if the HSI is selected as reference clock for at least one enabled PLL (PLLxON bit set to 1). In that case, the new HSIDIV value is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HSIDIV {
    #[doc = "0: No division"]
    Div1 = 0,
    #[doc = "1: Division by 2"]
    Div2 = 1,
    #[doc = "2: Division by 4"]
    Div4 = 2,
    #[doc = "3: Division by 8"]
    Div8 = 3,
}
impl From<HSIDIV> for u8 {
    #[inline(always)]
    fn from(variant: HSIDIV) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HSIDIV {
    type Ux = u8;
}
#[doc = "Field `HSIDIV` reader - HSI clock divider Set and reset by software. These bits allow selecting a division ratio in order to configure the wanted HSI clock frequency. The HSIDIV cannot be changed if the HSI is selected as reference clock for at least one enabled PLL (PLLxON bit set to 1). In that case, the new HSIDIV value is ignored."]
pub type HSIDIV_R = crate::FieldReader<HSIDIV>;
impl HSIDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSIDIV {
        match self.bits {
            0 => HSIDIV::Div1,
            1 => HSIDIV::Div2,
            2 => HSIDIV::Div4,
            3 => HSIDIV::Div8,
            _ => unreachable!(),
        }
    }
    #[doc = "No division"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == HSIDIV::Div1
    }
    #[doc = "Division by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == HSIDIV::Div2
    }
    #[doc = "Division by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == HSIDIV::Div4
    }
    #[doc = "Division by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == HSIDIV::Div8
    }
}
#[doc = "Field `HSIDIV` writer - HSI clock divider Set and reset by software. These bits allow selecting a division ratio in order to configure the wanted HSI clock frequency. The HSIDIV cannot be changed if the HSI is selected as reference clock for at least one enabled PLL (PLLxON bit set to 1). In that case, the new HSIDIV value is ignored."]
pub type HSIDIV_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, HSIDIV>;
impl<'a, REG> HSIDIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No division"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(HSIDIV::Div1)
    }
    #[doc = "Division by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(HSIDIV::Div2)
    }
    #[doc = "Division by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(HSIDIV::Div4)
    }
    #[doc = "Division by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(HSIDIV::Div8)
    }
}
#[doc = "HSI divider flag Set and reset by hardware. As a write operation to HSIDIV has not an immediate effect on the frequency, this flag indicates the current status of the HSI divider. HSIDIVF goes immediately to 0 when HSIDIV value is changed, and is set back to 1 when the output frequency matches the value programmed into HSIDIV.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIDIVFR {
    #[doc = "0: New HSIDIV ratio has not yet propagated to hsi_ck"]
    NotPropagated = 0,
    #[doc = "1: HSIDIV ratio has propagated to hsi_ck"]
    Propagated = 1,
}
impl From<HSIDIVFR> for bool {
    #[inline(always)]
    fn from(variant: HSIDIVFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSIDIVF` reader - HSI divider flag Set and reset by hardware. As a write operation to HSIDIV has not an immediate effect on the frequency, this flag indicates the current status of the HSI divider. HSIDIVF goes immediately to 0 when HSIDIV value is changed, and is set back to 1 when the output frequency matches the value programmed into HSIDIV."]
pub type HSIDIVF_R = crate::BitReader<HSIDIVFR>;
impl HSIDIVF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSIDIVFR {
        match self.bits {
            false => HSIDIVFR::NotPropagated,
            true => HSIDIVFR::Propagated,
        }
    }
    #[doc = "New HSIDIV ratio has not yet propagated to hsi_ck"]
    #[inline(always)]
    pub fn is_not_propagated(&self) -> bool {
        *self == HSIDIVFR::NotPropagated
    }
    #[doc = "HSIDIV ratio has propagated to hsi_ck"]
    #[inline(always)]
    pub fn is_propagated(&self) -> bool {
        *self == HSIDIVFR::Propagated
    }
}
#[doc = "Field `CSION` reader - CSI clock enable Set and reset by software to enable/disable CSI clock for system and/or peripheral. Set by hardware to force the CSI to ON when the system leaves Stop mode, if STOPWUCK = 1 or STOPKERWUCK = 1. This bit cannot be cleared if the CSI is used directly (via SW mux) as system clock, or if the CSI is selected as reference clock for PLL1 with PLL1 enabled (PLL1ON bit set to 1)."]
pub use HSION_R as CSION_R;
#[doc = "Field `CSIKERON` reader - CSI clock enable in Stop mode Set and reset by software to force the CSI to ON, even in Stop mode, in order to be quickly available as kernel clock for some peripherals. This bit has no effect on the value of CSION."]
pub use HSION_R as CSIKERON_R;
#[doc = "Field `HSI48ON` reader - HSI48 clock enable Set by software and cleared by software or by the hardware when the system enters to Stop or Standby mode."]
pub use HSION_R as HSI48ON_R;
#[doc = "Field `HSEON` reader - HSE clock enable Set and cleared by software. Cleared by hardware to stop the HSE when entering Stop or Standby mode. This bit cannot be cleared if the HSE is used directly (via SW mux) as system clock, or if the HSE is selected as reference clock for PLL1 with PLL1 enabled (PLL1ON bit set to 1)."]
pub use HSION_R as HSEON_R;
#[doc = "Field `CSION` writer - CSI clock enable Set and reset by software to enable/disable CSI clock for system and/or peripheral. Set by hardware to force the CSI to ON when the system leaves Stop mode, if STOPWUCK = 1 or STOPKERWUCK = 1. This bit cannot be cleared if the CSI is used directly (via SW mux) as system clock, or if the CSI is selected as reference clock for PLL1 with PLL1 enabled (PLL1ON bit set to 1)."]
pub use HSION_W as CSION_W;
#[doc = "Field `CSIKERON` writer - CSI clock enable in Stop mode Set and reset by software to force the CSI to ON, even in Stop mode, in order to be quickly available as kernel clock for some peripherals. This bit has no effect on the value of CSION."]
pub use HSION_W as CSIKERON_W;
#[doc = "Field `HSI48ON` writer - HSI48 clock enable Set by software and cleared by software or by the hardware when the system enters to Stop or Standby mode."]
pub use HSION_W as HSI48ON_W;
#[doc = "Field `HSEON` writer - HSE clock enable Set and cleared by software. Cleared by hardware to stop the HSE when entering Stop or Standby mode. This bit cannot be cleared if the HSE is used directly (via SW mux) as system clock, or if the HSE is selected as reference clock for PLL1 with PLL1 enabled (PLL1ON bit set to 1)."]
pub use HSION_W as HSEON_W;
#[doc = "Field `CSIRDY` reader - CSI clock ready flag Set by hardware to indicate that the CSI oscillator is stable. This bit is activated only if the RC is enabled by CSION (it is not activated if the CSI is enabled by CSIKERON or by a peripheral request)."]
pub use HSIRDY_R as CSIRDY_R;
#[doc = "Field `HSI48RDY` reader - HSI48 clock ready flag Set by hardware to indicate that the HSI48 oscillator is stable."]
pub use HSIRDY_R as HSI48RDY_R;
#[doc = "Field `HSERDY` reader - HSE clock ready flag Set by hardware to indicate that the HSE oscillator is stable."]
pub use HSIRDY_R as HSERDY_R;
#[doc = "HSE clock bypass Set and cleared by software to bypass the oscillator with an external clock. The external clock must be enabled with the HSEON bit to be used by the device. The HSEBYP bit can be written only if the HSE oscillator is disabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSEBYP {
    #[doc = "0: HSE crystal oscillator not bypassed"]
    NotBypassed = 0,
    #[doc = "1: HSE crystal oscillator bypassed with external clock"]
    Bypassed = 1,
}
impl From<HSEBYP> for bool {
    #[inline(always)]
    fn from(variant: HSEBYP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSEBYP` reader - HSE clock bypass Set and cleared by software to bypass the oscillator with an external clock. The external clock must be enabled with the HSEON bit to be used by the device. The HSEBYP bit can be written only if the HSE oscillator is disabled."]
pub type HSEBYP_R = crate::BitReader<HSEBYP>;
impl HSEBYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSEBYP {
        match self.bits {
            false => HSEBYP::NotBypassed,
            true => HSEBYP::Bypassed,
        }
    }
    #[doc = "HSE crystal oscillator not bypassed"]
    #[inline(always)]
    pub fn is_not_bypassed(&self) -> bool {
        *self == HSEBYP::NotBypassed
    }
    #[doc = "HSE crystal oscillator bypassed with external clock"]
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        *self == HSEBYP::Bypassed
    }
}
#[doc = "Field `HSEBYP` writer - HSE clock bypass Set and cleared by software to bypass the oscillator with an external clock. The external clock must be enabled with the HSEON bit to be used by the device. The HSEBYP bit can be written only if the HSE oscillator is disabled."]
pub type HSEBYP_W<'a, REG> = crate::BitWriter<'a, REG, HSEBYP>;
impl<'a, REG> HSEBYP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSE crystal oscillator not bypassed"]
    #[inline(always)]
    pub fn not_bypassed(self) -> &'a mut crate::W<REG> {
        self.variant(HSEBYP::NotBypassed)
    }
    #[doc = "HSE crystal oscillator bypassed with external clock"]
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut crate::W<REG> {
        self.variant(HSEBYP::Bypassed)
    }
}
#[doc = "Field `HSECSSON` reader - HSE clock security system enable Set by software to enable clock security system on HSE. This bit is “set only” (disabled by a system reset or when the system enters in Standby mode). When HSECSSON is set, the clock detector is enabled by hardware when the HSE is ready and disabled by hardware if an oscillator failure is detected."]
pub use HSION_R as HSECSSON_R;
#[doc = "Field `HSECSSON` writer - HSE clock security system enable Set by software to enable clock security system on HSE. This bit is “set only” (disabled by a system reset or when the system enters in Standby mode). When HSECSSON is set, the clock detector is enabled by hardware when the HSE is ready and disabled by hardware if an oscillator failure is detected."]
pub use HSION_W as HSECSSON_W;
#[doc = "external high speed clock type in Bypass mode Set and reset by software to select the external clock type (analog or digital). The external clock must be enabled with the HSEON bit to be used by the device. The HSEEXT bit can be written only if the HSE oscillator is disabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSEEXT {
    #[doc = "0: HSE in analog mode"]
    Analog = 0,
    #[doc = "1: HSE in digital mode"]
    Digital = 1,
}
impl From<HSEEXT> for bool {
    #[inline(always)]
    fn from(variant: HSEEXT) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSEEXT` reader - external high speed clock type in Bypass mode Set and reset by software to select the external clock type (analog or digital). The external clock must be enabled with the HSEON bit to be used by the device. The HSEEXT bit can be written only if the HSE oscillator is disabled."]
pub type HSEEXT_R = crate::BitReader<HSEEXT>;
impl HSEEXT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSEEXT {
        match self.bits {
            false => HSEEXT::Analog,
            true => HSEEXT::Digital,
        }
    }
    #[doc = "HSE in analog mode"]
    #[inline(always)]
    pub fn is_analog(&self) -> bool {
        *self == HSEEXT::Analog
    }
    #[doc = "HSE in digital mode"]
    #[inline(always)]
    pub fn is_digital(&self) -> bool {
        *self == HSEEXT::Digital
    }
}
#[doc = "Field `HSEEXT` writer - external high speed clock type in Bypass mode Set and reset by software to select the external clock type (analog or digital). The external clock must be enabled with the HSEON bit to be used by the device. The HSEEXT bit can be written only if the HSE oscillator is disabled."]
pub type HSEEXT_W<'a, REG> = crate::BitWriter<'a, REG, HSEEXT>;
impl<'a, REG> HSEEXT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSE in analog mode"]
    #[inline(always)]
    pub fn analog(self) -> &'a mut crate::W<REG> {
        self.variant(HSEEXT::Analog)
    }
    #[doc = "HSE in digital mode"]
    #[inline(always)]
    pub fn digital(self) -> &'a mut crate::W<REG> {
        self.variant(HSEEXT::Digital)
    }
}
#[doc = "Field `PLL1ON` reader - PLL1 enable Set and cleared by software to enable PLL1. Cleared by hardware when entering Stop or Standby mode. Note that the hardware prevents writing this bit to 0, if the PLL1 output is used as the system clock."]
pub use HSION_R as PLL1ON_R;
#[doc = "Field `PLL2ON` reader - PLL2 enable Set and cleared by software to enable PLL2. Cleared by hardware when entering Stop or Standby mode."]
pub use HSION_R as PLL2ON_R;
#[doc = "Field `PLL1ON` writer - PLL1 enable Set and cleared by software to enable PLL1. Cleared by hardware when entering Stop or Standby mode. Note that the hardware prevents writing this bit to 0, if the PLL1 output is used as the system clock."]
pub use HSION_W as PLL1ON_W;
#[doc = "Field `PLL2ON` writer - PLL2 enable Set and cleared by software to enable PLL2. Cleared by hardware when entering Stop or Standby mode."]
pub use HSION_W as PLL2ON_W;
#[doc = "Field `PLL1RDY` reader - PLL1 clock ready flag Set by hardware to indicate that the PLL1 is locked."]
pub use HSIRDY_R as PLL1RDY_R;
#[doc = "Field `PLL2RDY` reader - PLL2 clock ready flag Set by hardware to indicate that the PLL is locked."]
pub use HSIRDY_R as PLL2RDY_R;
impl R {
    #[doc = "Bit 0 - HSI clock enable Set and cleared by software. Set by hardware to force the HSI to ON when the product leaves Stop mode, if STOPWUCK = 1 or STOPKERWUCK = 1. Set by hardware to force the HSI to ON when the product leaves Standby mode or in case of a failure of the HSE which is used as the system clock source. This bit cannot be cleared if the HSI is used directly (via SW mux) as system clock, or if the HSI is selected as reference clock for PLL1 with PLL1 enabled (PLL1ON bit set to 1)."]
    #[inline(always)]
    pub fn hsion(&self) -> HSION_R {
        HSION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HSI clock ready flag Set by hardware to indicate that the HSI oscillator is stable."]
    #[inline(always)]
    pub fn hsirdy(&self) -> HSIRDY_R {
        HSIRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HSI clock enable in Stop mode Set and reset by software to force the HSI to ON, even in Stop mode, in order to be quickly available as kernel clock for peripherals. This bit has no effect on the value of HSION."]
    #[inline(always)]
    pub fn hsikeron(&self) -> HSIKERON_R {
        HSIKERON_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - HSI clock divider Set and reset by software. These bits allow selecting a division ratio in order to configure the wanted HSI clock frequency. The HSIDIV cannot be changed if the HSI is selected as reference clock for at least one enabled PLL (PLLxON bit set to 1). In that case, the new HSIDIV value is ignored."]
    #[inline(always)]
    pub fn hsidiv(&self) -> HSIDIV_R {
        HSIDIV_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - HSI divider flag Set and reset by hardware. As a write operation to HSIDIV has not an immediate effect on the frequency, this flag indicates the current status of the HSI divider. HSIDIVF goes immediately to 0 when HSIDIV value is changed, and is set back to 1 when the output frequency matches the value programmed into HSIDIV."]
    #[inline(always)]
    pub fn hsidivf(&self) -> HSIDIVF_R {
        HSIDIVF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - CSI clock enable Set and reset by software to enable/disable CSI clock for system and/or peripheral. Set by hardware to force the CSI to ON when the system leaves Stop mode, if STOPWUCK = 1 or STOPKERWUCK = 1. This bit cannot be cleared if the CSI is used directly (via SW mux) as system clock, or if the CSI is selected as reference clock for PLL1 with PLL1 enabled (PLL1ON bit set to 1)."]
    #[inline(always)]
    pub fn csion(&self) -> CSION_R {
        CSION_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CSI clock ready flag Set by hardware to indicate that the CSI oscillator is stable. This bit is activated only if the RC is enabled by CSION (it is not activated if the CSI is enabled by CSIKERON or by a peripheral request)."]
    #[inline(always)]
    pub fn csirdy(&self) -> CSIRDY_R {
        CSIRDY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CSI clock enable in Stop mode Set and reset by software to force the CSI to ON, even in Stop mode, in order to be quickly available as kernel clock for some peripherals. This bit has no effect on the value of CSION."]
    #[inline(always)]
    pub fn csikeron(&self) -> CSIKERON_R {
        CSIKERON_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - HSI48 clock enable Set by software and cleared by software or by the hardware when the system enters to Stop or Standby mode."]
    #[inline(always)]
    pub fn hsi48on(&self) -> HSI48ON_R {
        HSI48ON_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - HSI48 clock ready flag Set by hardware to indicate that the HSI48 oscillator is stable."]
    #[inline(always)]
    pub fn hsi48rdy(&self) -> HSI48RDY_R {
        HSI48RDY_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - HSE clock enable Set and cleared by software. Cleared by hardware to stop the HSE when entering Stop or Standby mode. This bit cannot be cleared if the HSE is used directly (via SW mux) as system clock, or if the HSE is selected as reference clock for PLL1 with PLL1 enabled (PLL1ON bit set to 1)."]
    #[inline(always)]
    pub fn hseon(&self) -> HSEON_R {
        HSEON_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HSE clock ready flag Set by hardware to indicate that the HSE oscillator is stable."]
    #[inline(always)]
    pub fn hserdy(&self) -> HSERDY_R {
        HSERDY_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - HSE clock bypass Set and cleared by software to bypass the oscillator with an external clock. The external clock must be enabled with the HSEON bit to be used by the device. The HSEBYP bit can be written only if the HSE oscillator is disabled."]
    #[inline(always)]
    pub fn hsebyp(&self) -> HSEBYP_R {
        HSEBYP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - HSE clock security system enable Set by software to enable clock security system on HSE. This bit is “set only” (disabled by a system reset or when the system enters in Standby mode). When HSECSSON is set, the clock detector is enabled by hardware when the HSE is ready and disabled by hardware if an oscillator failure is detected."]
    #[inline(always)]
    pub fn hsecsson(&self) -> HSECSSON_R {
        HSECSSON_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - external high speed clock type in Bypass mode Set and reset by software to select the external clock type (analog or digital). The external clock must be enabled with the HSEON bit to be used by the device. The HSEEXT bit can be written only if the HSE oscillator is disabled."]
    #[inline(always)]
    pub fn hseext(&self) -> HSEEXT_R {
        HSEEXT_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - PLL1 enable Set and cleared by software to enable PLL1. Cleared by hardware when entering Stop or Standby mode. Note that the hardware prevents writing this bit to 0, if the PLL1 output is used as the system clock."]
    #[inline(always)]
    pub fn pll1on(&self) -> PLL1ON_R {
        PLL1ON_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - PLL1 clock ready flag Set by hardware to indicate that the PLL1 is locked."]
    #[inline(always)]
    pub fn pll1rdy(&self) -> PLL1RDY_R {
        PLL1RDY_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - PLL2 enable Set and cleared by software to enable PLL2. Cleared by hardware when entering Stop or Standby mode."]
    #[inline(always)]
    pub fn pll2on(&self) -> PLL2ON_R {
        PLL2ON_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PLL2 clock ready flag Set by hardware to indicate that the PLL is locked."]
    #[inline(always)]
    pub fn pll2rdy(&self) -> PLL2RDY_R {
        PLL2RDY_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HSI clock enable Set and cleared by software. Set by hardware to force the HSI to ON when the product leaves Stop mode, if STOPWUCK = 1 or STOPKERWUCK = 1. Set by hardware to force the HSI to ON when the product leaves Standby mode or in case of a failure of the HSE which is used as the system clock source. This bit cannot be cleared if the HSI is used directly (via SW mux) as system clock, or if the HSI is selected as reference clock for PLL1 with PLL1 enabled (PLL1ON bit set to 1)."]
    #[inline(always)]
    #[must_use]
    pub fn hsion(&mut self) -> HSION_W<CRrs> {
        HSION_W::new(self, 0)
    }
    #[doc = "Bit 2 - HSI clock enable in Stop mode Set and reset by software to force the HSI to ON, even in Stop mode, in order to be quickly available as kernel clock for peripherals. This bit has no effect on the value of HSION."]
    #[inline(always)]
    #[must_use]
    pub fn hsikeron(&mut self) -> HSIKERON_W<CRrs> {
        HSIKERON_W::new(self, 2)
    }
    #[doc = "Bits 3:4 - HSI clock divider Set and reset by software. These bits allow selecting a division ratio in order to configure the wanted HSI clock frequency. The HSIDIV cannot be changed if the HSI is selected as reference clock for at least one enabled PLL (PLLxON bit set to 1). In that case, the new HSIDIV value is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn hsidiv(&mut self) -> HSIDIV_W<CRrs> {
        HSIDIV_W::new(self, 3)
    }
    #[doc = "Bit 8 - CSI clock enable Set and reset by software to enable/disable CSI clock for system and/or peripheral. Set by hardware to force the CSI to ON when the system leaves Stop mode, if STOPWUCK = 1 or STOPKERWUCK = 1. This bit cannot be cleared if the CSI is used directly (via SW mux) as system clock, or if the CSI is selected as reference clock for PLL1 with PLL1 enabled (PLL1ON bit set to 1)."]
    #[inline(always)]
    #[must_use]
    pub fn csion(&mut self) -> CSION_W<CRrs> {
        CSION_W::new(self, 8)
    }
    #[doc = "Bit 10 - CSI clock enable in Stop mode Set and reset by software to force the CSI to ON, even in Stop mode, in order to be quickly available as kernel clock for some peripherals. This bit has no effect on the value of CSION."]
    #[inline(always)]
    #[must_use]
    pub fn csikeron(&mut self) -> CSIKERON_W<CRrs> {
        CSIKERON_W::new(self, 10)
    }
    #[doc = "Bit 12 - HSI48 clock enable Set by software and cleared by software or by the hardware when the system enters to Stop or Standby mode."]
    #[inline(always)]
    #[must_use]
    pub fn hsi48on(&mut self) -> HSI48ON_W<CRrs> {
        HSI48ON_W::new(self, 12)
    }
    #[doc = "Bit 16 - HSE clock enable Set and cleared by software. Cleared by hardware to stop the HSE when entering Stop or Standby mode. This bit cannot be cleared if the HSE is used directly (via SW mux) as system clock, or if the HSE is selected as reference clock for PLL1 with PLL1 enabled (PLL1ON bit set to 1)."]
    #[inline(always)]
    #[must_use]
    pub fn hseon(&mut self) -> HSEON_W<CRrs> {
        HSEON_W::new(self, 16)
    }
    #[doc = "Bit 18 - HSE clock bypass Set and cleared by software to bypass the oscillator with an external clock. The external clock must be enabled with the HSEON bit to be used by the device. The HSEBYP bit can be written only if the HSE oscillator is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn hsebyp(&mut self) -> HSEBYP_W<CRrs> {
        HSEBYP_W::new(self, 18)
    }
    #[doc = "Bit 19 - HSE clock security system enable Set by software to enable clock security system on HSE. This bit is “set only” (disabled by a system reset or when the system enters in Standby mode). When HSECSSON is set, the clock detector is enabled by hardware when the HSE is ready and disabled by hardware if an oscillator failure is detected."]
    #[inline(always)]
    #[must_use]
    pub fn hsecsson(&mut self) -> HSECSSON_W<CRrs> {
        HSECSSON_W::new(self, 19)
    }
    #[doc = "Bit 20 - external high speed clock type in Bypass mode Set and reset by software to select the external clock type (analog or digital). The external clock must be enabled with the HSEON bit to be used by the device. The HSEEXT bit can be written only if the HSE oscillator is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn hseext(&mut self) -> HSEEXT_W<CRrs> {
        HSEEXT_W::new(self, 20)
    }
    #[doc = "Bit 24 - PLL1 enable Set and cleared by software to enable PLL1. Cleared by hardware when entering Stop or Standby mode. Note that the hardware prevents writing this bit to 0, if the PLL1 output is used as the system clock."]
    #[inline(always)]
    #[must_use]
    pub fn pll1on(&mut self) -> PLL1ON_W<CRrs> {
        PLL1ON_W::new(self, 24)
    }
    #[doc = "Bit 26 - PLL2 enable Set and cleared by software to enable PLL2. Cleared by hardware when entering Stop or Standby mode."]
    #[inline(always)]
    #[must_use]
    pub fn pll2on(&mut self) -> PLL2ON_W<CRrs> {
        PLL2ON_W::new(self, 26)
    }
}
#[doc = "RCC clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CR to value 0x23"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0x23;
}

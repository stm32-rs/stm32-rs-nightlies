#[doc = "Register `CCR` reader"]
pub type R = crate::R<CCRrs>;
#[doc = "Register `CCR` writer"]
pub type W = crate::W<CCRrs>;
#[doc = "Dual ADC mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DUAL {
    #[doc = "0: Independent mode"]
    Independent = 0,
    #[doc = "1: Dual, combined regular simultaneous + injected simultaneous mode"]
    DualRj = 1,
    #[doc = "2: Dual, combined regular simultaneous + alternate trigger mode"]
    DualRa = 2,
    #[doc = "3: Dual, combined interleaved mode + injected simultaneous mode"]
    DualIj = 3,
    #[doc = "5: Dual, injected simultaneous mode only"]
    DualJ = 5,
    #[doc = "6: Dual, regular simultaneous mode only"]
    DualR = 6,
    #[doc = "7: Dual, interleaved mode only"]
    DualI = 7,
    #[doc = "9: Dual, alternate trigger mode only"]
    DualA = 9,
}
impl From<DUAL> for u8 {
    #[inline(always)]
    fn from(variant: DUAL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DUAL {
    type Ux = u8;
}
#[doc = "Field `DUAL` reader - Dual ADC mode selection"]
pub type DUAL_R = crate::FieldReader<DUAL>;
impl DUAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DUAL> {
        match self.bits {
            0 => Some(DUAL::Independent),
            1 => Some(DUAL::DualRj),
            2 => Some(DUAL::DualRa),
            3 => Some(DUAL::DualIj),
            5 => Some(DUAL::DualJ),
            6 => Some(DUAL::DualR),
            7 => Some(DUAL::DualI),
            9 => Some(DUAL::DualA),
            _ => None,
        }
    }
    #[doc = "Independent mode"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == DUAL::Independent
    }
    #[doc = "Dual, combined regular simultaneous + injected simultaneous mode"]
    #[inline(always)]
    pub fn is_dual_rj(&self) -> bool {
        *self == DUAL::DualRj
    }
    #[doc = "Dual, combined regular simultaneous + alternate trigger mode"]
    #[inline(always)]
    pub fn is_dual_ra(&self) -> bool {
        *self == DUAL::DualRa
    }
    #[doc = "Dual, combined interleaved mode + injected simultaneous mode"]
    #[inline(always)]
    pub fn is_dual_ij(&self) -> bool {
        *self == DUAL::DualIj
    }
    #[doc = "Dual, injected simultaneous mode only"]
    #[inline(always)]
    pub fn is_dual_j(&self) -> bool {
        *self == DUAL::DualJ
    }
    #[doc = "Dual, regular simultaneous mode only"]
    #[inline(always)]
    pub fn is_dual_r(&self) -> bool {
        *self == DUAL::DualR
    }
    #[doc = "Dual, interleaved mode only"]
    #[inline(always)]
    pub fn is_dual_i(&self) -> bool {
        *self == DUAL::DualI
    }
    #[doc = "Dual, alternate trigger mode only"]
    #[inline(always)]
    pub fn is_dual_a(&self) -> bool {
        *self == DUAL::DualA
    }
}
#[doc = "Field `DUAL` writer - Dual ADC mode selection"]
pub type DUAL_W<'a, REG> = crate::FieldWriter<'a, REG, 5, DUAL>;
impl<'a, REG> DUAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Independent mode"]
    #[inline(always)]
    pub fn independent(self) -> &'a mut crate::W<REG> {
        self.variant(DUAL::Independent)
    }
    #[doc = "Dual, combined regular simultaneous + injected simultaneous mode"]
    #[inline(always)]
    pub fn dual_rj(self) -> &'a mut crate::W<REG> {
        self.variant(DUAL::DualRj)
    }
    #[doc = "Dual, combined regular simultaneous + alternate trigger mode"]
    #[inline(always)]
    pub fn dual_ra(self) -> &'a mut crate::W<REG> {
        self.variant(DUAL::DualRa)
    }
    #[doc = "Dual, combined interleaved mode + injected simultaneous mode"]
    #[inline(always)]
    pub fn dual_ij(self) -> &'a mut crate::W<REG> {
        self.variant(DUAL::DualIj)
    }
    #[doc = "Dual, injected simultaneous mode only"]
    #[inline(always)]
    pub fn dual_j(self) -> &'a mut crate::W<REG> {
        self.variant(DUAL::DualJ)
    }
    #[doc = "Dual, regular simultaneous mode only"]
    #[inline(always)]
    pub fn dual_r(self) -> &'a mut crate::W<REG> {
        self.variant(DUAL::DualR)
    }
    #[doc = "Dual, interleaved mode only"]
    #[inline(always)]
    pub fn dual_i(self) -> &'a mut crate::W<REG> {
        self.variant(DUAL::DualI)
    }
    #[doc = "Dual, alternate trigger mode only"]
    #[inline(always)]
    pub fn dual_a(self) -> &'a mut crate::W<REG> {
        self.variant(DUAL::DualA)
    }
}
#[doc = "Field `DELAY` reader - Delay between 2 sampling phases"]
pub type DELAY_R = crate::FieldReader;
#[doc = "Field `DELAY` writer - Delay between 2 sampling phases"]
pub type DELAY_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
#[doc = "DMA configuration (for multi-ADC mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMACFG {
    #[doc = "0: DMA One Shot mode selected"]
    OneShotMode = 0,
    #[doc = "1: DMA Circular mode selected"]
    CircularMode = 1,
}
impl From<DMACFG> for bool {
    #[inline(always)]
    fn from(variant: DMACFG) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMACFG` reader - DMA configuration (for multi-ADC mode)"]
pub type DMACFG_R = crate::BitReader<DMACFG>;
impl DMACFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMACFG {
        match self.bits {
            false => DMACFG::OneShotMode,
            true => DMACFG::CircularMode,
        }
    }
    #[doc = "DMA One Shot mode selected"]
    #[inline(always)]
    pub fn is_one_shot_mode(&self) -> bool {
        *self == DMACFG::OneShotMode
    }
    #[doc = "DMA Circular mode selected"]
    #[inline(always)]
    pub fn is_circular_mode(&self) -> bool {
        *self == DMACFG::CircularMode
    }
}
#[doc = "Field `DMACFG` writer - DMA configuration (for multi-ADC mode)"]
pub type DMACFG_W<'a, REG> = crate::BitWriter<'a, REG, DMACFG>;
impl<'a, REG> DMACFG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA One Shot mode selected"]
    #[inline(always)]
    pub fn one_shot_mode(self) -> &'a mut crate::W<REG> {
        self.variant(DMACFG::OneShotMode)
    }
    #[doc = "DMA Circular mode selected"]
    #[inline(always)]
    pub fn circular_mode(self) -> &'a mut crate::W<REG> {
        self.variant(DMACFG::CircularMode)
    }
}
#[doc = "Direct memory access mode for multi ADC mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MDMA {
    #[doc = "0: MDMA mode disabled"]
    Disabled = 0,
    #[doc = "1: Enable dual interleaved mode to output to the master channel of DFSDM interface both Master and the Slave result (16-bit data width)"]
    Interleaved = 1,
    #[doc = "2: MDMA mode enabled for 12 and 10-bit resolution"]
    Bits12_10 = 2,
    #[doc = "3: MDMA mode enabled for 8 and 6-bit resolution"]
    Bits8_6 = 3,
}
impl From<MDMA> for u8 {
    #[inline(always)]
    fn from(variant: MDMA) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MDMA {
    type Ux = u8;
}
#[doc = "Field `MDMA` reader - Direct memory access mode for multi ADC mode"]
pub type MDMA_R = crate::FieldReader<MDMA>;
impl MDMA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MDMA {
        match self.bits {
            0 => MDMA::Disabled,
            1 => MDMA::Interleaved,
            2 => MDMA::Bits12_10,
            3 => MDMA::Bits8_6,
            _ => unreachable!(),
        }
    }
    #[doc = "MDMA mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MDMA::Disabled
    }
    #[doc = "Enable dual interleaved mode to output to the master channel of DFSDM interface both Master and the Slave result (16-bit data width)"]
    #[inline(always)]
    pub fn is_interleaved(&self) -> bool {
        *self == MDMA::Interleaved
    }
    #[doc = "MDMA mode enabled for 12 and 10-bit resolution"]
    #[inline(always)]
    pub fn is_bits12_10(&self) -> bool {
        *self == MDMA::Bits12_10
    }
    #[doc = "MDMA mode enabled for 8 and 6-bit resolution"]
    #[inline(always)]
    pub fn is_bits8_6(&self) -> bool {
        *self == MDMA::Bits8_6
    }
}
#[doc = "Field `MDMA` writer - Direct memory access mode for multi ADC mode"]
pub type MDMA_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, MDMA>;
impl<'a, REG> MDMA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MDMA mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MDMA::Disabled)
    }
    #[doc = "Enable dual interleaved mode to output to the master channel of DFSDM interface both Master and the Slave result (16-bit data width)"]
    #[inline(always)]
    pub fn interleaved(self) -> &'a mut crate::W<REG> {
        self.variant(MDMA::Interleaved)
    }
    #[doc = "MDMA mode enabled for 12 and 10-bit resolution"]
    #[inline(always)]
    pub fn bits12_10(self) -> &'a mut crate::W<REG> {
        self.variant(MDMA::Bits12_10)
    }
    #[doc = "MDMA mode enabled for 8 and 6-bit resolution"]
    #[inline(always)]
    pub fn bits8_6(self) -> &'a mut crate::W<REG> {
        self.variant(MDMA::Bits8_6)
    }
}
#[doc = "ADC clock mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKMODE {
    #[doc = "0: Use Kernel Clock adc_ker_ck_input divided by PRESC. Asynchronous to AHB clock"]
    Asynchronous = 0,
    #[doc = "1: Use AHB clock rcc_hclk3. In this case rcc_hclk must equal sys_d1cpre_ck"]
    SyncDiv1 = 1,
    #[doc = "2: Use AHB clock rcc_hclk3 divided by 2"]
    SyncDiv2 = 2,
    #[doc = "3: Use AHB clock rcc_hclk3 divided by 4"]
    SyncDiv4 = 3,
}
impl From<CKMODE> for u8 {
    #[inline(always)]
    fn from(variant: CKMODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CKMODE {
    type Ux = u8;
}
#[doc = "Field `CKMODE` reader - ADC clock mode"]
pub type CKMODE_R = crate::FieldReader<CKMODE>;
impl CKMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CKMODE {
        match self.bits {
            0 => CKMODE::Asynchronous,
            1 => CKMODE::SyncDiv1,
            2 => CKMODE::SyncDiv2,
            3 => CKMODE::SyncDiv4,
            _ => unreachable!(),
        }
    }
    #[doc = "Use Kernel Clock adc_ker_ck_input divided by PRESC. Asynchronous to AHB clock"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == CKMODE::Asynchronous
    }
    #[doc = "Use AHB clock rcc_hclk3. In this case rcc_hclk must equal sys_d1cpre_ck"]
    #[inline(always)]
    pub fn is_sync_div1(&self) -> bool {
        *self == CKMODE::SyncDiv1
    }
    #[doc = "Use AHB clock rcc_hclk3 divided by 2"]
    #[inline(always)]
    pub fn is_sync_div2(&self) -> bool {
        *self == CKMODE::SyncDiv2
    }
    #[doc = "Use AHB clock rcc_hclk3 divided by 4"]
    #[inline(always)]
    pub fn is_sync_div4(&self) -> bool {
        *self == CKMODE::SyncDiv4
    }
}
#[doc = "Field `CKMODE` writer - ADC clock mode"]
pub type CKMODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CKMODE>;
impl<'a, REG> CKMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Use Kernel Clock adc_ker_ck_input divided by PRESC. Asynchronous to AHB clock"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut crate::W<REG> {
        self.variant(CKMODE::Asynchronous)
    }
    #[doc = "Use AHB clock rcc_hclk3. In this case rcc_hclk must equal sys_d1cpre_ck"]
    #[inline(always)]
    pub fn sync_div1(self) -> &'a mut crate::W<REG> {
        self.variant(CKMODE::SyncDiv1)
    }
    #[doc = "Use AHB clock rcc_hclk3 divided by 2"]
    #[inline(always)]
    pub fn sync_div2(self) -> &'a mut crate::W<REG> {
        self.variant(CKMODE::SyncDiv2)
    }
    #[doc = "Use AHB clock rcc_hclk3 divided by 4"]
    #[inline(always)]
    pub fn sync_div4(self) -> &'a mut crate::W<REG> {
        self.variant(CKMODE::SyncDiv4)
    }
}
#[doc = "ADC prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESC {
    #[doc = "0: Input ADC clock not divided"]
    Div1 = 0,
    #[doc = "1: Input ADC clock divided by 2"]
    Div2 = 1,
    #[doc = "2: Input ADC clock divided by 4"]
    Div4 = 2,
    #[doc = "3: Input ADC clock divided by 6"]
    Div6 = 3,
    #[doc = "4: Input ADC clock divided by 8"]
    Div8 = 4,
    #[doc = "5: Input ADC clock divided by 10"]
    Div10 = 5,
    #[doc = "6: Input ADC clock divided by 12"]
    Div12 = 6,
    #[doc = "7: Input ADC clock divided by 16"]
    Div16 = 7,
    #[doc = "8: Input ADC clock divided by 32"]
    Div32 = 8,
    #[doc = "9: Input ADC clock divided by 64"]
    Div64 = 9,
    #[doc = "10: Input ADC clock divided by 128"]
    Div128 = 10,
    #[doc = "11: Input ADC clock divided by 256"]
    Div256 = 11,
}
impl From<PRESC> for u8 {
    #[inline(always)]
    fn from(variant: PRESC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRESC {
    type Ux = u8;
}
#[doc = "Field `PRESC` reader - ADC prescaler"]
pub type PRESC_R = crate::FieldReader<PRESC>;
impl PRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PRESC> {
        match self.bits {
            0 => Some(PRESC::Div1),
            1 => Some(PRESC::Div2),
            2 => Some(PRESC::Div4),
            3 => Some(PRESC::Div6),
            4 => Some(PRESC::Div8),
            5 => Some(PRESC::Div10),
            6 => Some(PRESC::Div12),
            7 => Some(PRESC::Div16),
            8 => Some(PRESC::Div32),
            9 => Some(PRESC::Div64),
            10 => Some(PRESC::Div128),
            11 => Some(PRESC::Div256),
            _ => None,
        }
    }
    #[doc = "Input ADC clock not divided"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PRESC::Div1
    }
    #[doc = "Input ADC clock divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PRESC::Div2
    }
    #[doc = "Input ADC clock divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PRESC::Div4
    }
    #[doc = "Input ADC clock divided by 6"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PRESC::Div6
    }
    #[doc = "Input ADC clock divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PRESC::Div8
    }
    #[doc = "Input ADC clock divided by 10"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PRESC::Div10
    }
    #[doc = "Input ADC clock divided by 12"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PRESC::Div12
    }
    #[doc = "Input ADC clock divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PRESC::Div16
    }
    #[doc = "Input ADC clock divided by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PRESC::Div32
    }
    #[doc = "Input ADC clock divided by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PRESC::Div64
    }
    #[doc = "Input ADC clock divided by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PRESC::Div128
    }
    #[doc = "Input ADC clock divided by 256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == PRESC::Div256
    }
}
#[doc = "Field `PRESC` writer - ADC prescaler"]
pub type PRESC_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PRESC>;
impl<'a, REG> PRESC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input ADC clock not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div1)
    }
    #[doc = "Input ADC clock divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div2)
    }
    #[doc = "Input ADC clock divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div4)
    }
    #[doc = "Input ADC clock divided by 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div6)
    }
    #[doc = "Input ADC clock divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div8)
    }
    #[doc = "Input ADC clock divided by 10"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div10)
    }
    #[doc = "Input ADC clock divided by 12"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div12)
    }
    #[doc = "Input ADC clock divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div16)
    }
    #[doc = "Input ADC clock divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div32)
    }
    #[doc = "Input ADC clock divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div64)
    }
    #[doc = "Input ADC clock divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div128)
    }
    #[doc = "Input ADC clock divided by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div256)
    }
}
#[doc = "VREFINT enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VREFEN {
    #[doc = "0: V_REFINT channel disabled"]
    Disabled = 0,
    #[doc = "1: V_REFINT channel enabled"]
    Enabled = 1,
}
impl From<VREFEN> for bool {
    #[inline(always)]
    fn from(variant: VREFEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VREFEN` reader - VREFINT enable"]
pub type VREFEN_R = crate::BitReader<VREFEN>;
impl VREFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VREFEN {
        match self.bits {
            false => VREFEN::Disabled,
            true => VREFEN::Enabled,
        }
    }
    #[doc = "V_REFINT channel disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VREFEN::Disabled
    }
    #[doc = "V_REFINT channel enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VREFEN::Enabled
    }
}
#[doc = "Field `VREFEN` writer - VREFINT enable"]
pub type VREFEN_W<'a, REG> = crate::BitWriter<'a, REG, VREFEN>;
impl<'a, REG> VREFEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "V_REFINT channel disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(VREFEN::Disabled)
    }
    #[doc = "V_REFINT channel enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(VREFEN::Enabled)
    }
}
#[doc = "Temperature sensor selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VSENSEEN {
    #[doc = "0: Temperature sensor channel disabled"]
    Disabled = 0,
    #[doc = "1: Temperature sensor channel enabled"]
    Enabled = 1,
}
impl From<VSENSEEN> for bool {
    #[inline(always)]
    fn from(variant: VSENSEEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VSENSEEN` reader - Temperature sensor selection"]
pub type VSENSEEN_R = crate::BitReader<VSENSEEN>;
impl VSENSEEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VSENSEEN {
        match self.bits {
            false => VSENSEEN::Disabled,
            true => VSENSEEN::Enabled,
        }
    }
    #[doc = "Temperature sensor channel disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VSENSEEN::Disabled
    }
    #[doc = "Temperature sensor channel enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VSENSEEN::Enabled
    }
}
#[doc = "Field `VSENSEEN` writer - Temperature sensor selection"]
pub type VSENSEEN_W<'a, REG> = crate::BitWriter<'a, REG, VSENSEEN>;
impl<'a, REG> VSENSEEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Temperature sensor channel disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(VSENSEEN::Disabled)
    }
    #[doc = "Temperature sensor channel enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(VSENSEEN::Enabled)
    }
}
#[doc = "VBAT selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBATEN {
    #[doc = "0: VBAT channel disabled"]
    Disabled = 0,
    #[doc = "1: VBAT channel enabled"]
    Enabled = 1,
}
impl From<VBATEN> for bool {
    #[inline(always)]
    fn from(variant: VBATEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBATEN` reader - VBAT selection"]
pub type VBATEN_R = crate::BitReader<VBATEN>;
impl VBATEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VBATEN {
        match self.bits {
            false => VBATEN::Disabled,
            true => VBATEN::Enabled,
        }
    }
    #[doc = "VBAT channel disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VBATEN::Disabled
    }
    #[doc = "VBAT channel enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VBATEN::Enabled
    }
}
#[doc = "Field `VBATEN` writer - VBAT selection"]
pub type VBATEN_W<'a, REG> = crate::BitWriter<'a, REG, VBATEN>;
impl<'a, REG> VBATEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBAT channel disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(VBATEN::Disabled)
    }
    #[doc = "VBAT channel enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(VBATEN::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:4 - Dual ADC mode selection"]
    #[inline(always)]
    pub fn dual(&self) -> DUAL_R {
        DUAL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - Delay between 2 sampling phases"]
    #[inline(always)]
    pub fn delay(&self) -> DELAY_R {
        DELAY_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - DMA configuration (for multi-ADC mode)"]
    #[inline(always)]
    pub fn dmacfg(&self) -> DMACFG_R {
        DMACFG_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Direct memory access mode for multi ADC mode"]
    #[inline(always)]
    pub fn mdma(&self) -> MDMA_R {
        MDMA_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - ADC clock mode"]
    #[inline(always)]
    pub fn ckmode(&self) -> CKMODE_R {
        CKMODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:21 - ADC prescaler"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - VREFINT enable"]
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Temperature sensor selection"]
    #[inline(always)]
    pub fn vsenseen(&self) -> VSENSEEN_R {
        VSENSEEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - VBAT selection"]
    #[inline(always)]
    pub fn vbaten(&self) -> VBATEN_R {
        VBATEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Dual ADC mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn dual(&mut self) -> DUAL_W<CCRrs> {
        DUAL_W::new(self, 0)
    }
    #[doc = "Bits 8:11 - Delay between 2 sampling phases"]
    #[inline(always)]
    #[must_use]
    pub fn delay(&mut self) -> DELAY_W<CCRrs> {
        DELAY_W::new(self, 8)
    }
    #[doc = "Bit 13 - DMA configuration (for multi-ADC mode)"]
    #[inline(always)]
    #[must_use]
    pub fn dmacfg(&mut self) -> DMACFG_W<CCRrs> {
        DMACFG_W::new(self, 13)
    }
    #[doc = "Bits 14:15 - Direct memory access mode for multi ADC mode"]
    #[inline(always)]
    #[must_use]
    pub fn mdma(&mut self) -> MDMA_W<CCRrs> {
        MDMA_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - ADC clock mode"]
    #[inline(always)]
    #[must_use]
    pub fn ckmode(&mut self) -> CKMODE_W<CCRrs> {
        CKMODE_W::new(self, 16)
    }
    #[doc = "Bits 18:21 - ADC prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PRESC_W<CCRrs> {
        PRESC_W::new(self, 18)
    }
    #[doc = "Bit 22 - VREFINT enable"]
    #[inline(always)]
    #[must_use]
    pub fn vrefen(&mut self) -> VREFEN_W<CCRrs> {
        VREFEN_W::new(self, 22)
    }
    #[doc = "Bit 23 - Temperature sensor selection"]
    #[inline(always)]
    #[must_use]
    pub fn vsenseen(&mut self) -> VSENSEEN_W<CCRrs> {
        VSENSEEN_W::new(self, 23)
    }
    #[doc = "Bit 24 - VBAT selection"]
    #[inline(always)]
    #[must_use]
    pub fn vbaten(&mut self) -> VBATEN_W<CCRrs> {
        VBATEN_W::new(self, 24)
    }
}
#[doc = "ADC common control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCRrs;
impl crate::RegisterSpec for CCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr::R`](R) reader structure"]
impl crate::Readable for CCRrs {}
#[doc = "`write(|w| ..)` method takes [`ccr::W`](W) writer structure"]
impl crate::Writable for CCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCR to value 0"]
impl crate::Resettable for CCRrs {
    const RESET_VALUE: u32 = 0;
}

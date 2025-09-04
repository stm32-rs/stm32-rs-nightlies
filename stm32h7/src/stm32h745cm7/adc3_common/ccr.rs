///Register `CCR` reader
pub type R = crate::R<CCRrs>;
///Register `CCR` writer
pub type W = crate::W<CCRrs>;
/**Dual ADC mode selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DUAL {
    ///0: Independent mode
    Independent = 0,
    ///1: Dual, combined regular simultaneous + injected simultaneous mode
    DualRj = 1,
    ///2: Dual, combined regular simultaneous + alternate trigger mode
    DualRa = 2,
    ///3: Dual, combined interleaved mode + injected simultaneous mode
    DualIj = 3,
    ///5: Dual, injected simultaneous mode only
    DualJ = 5,
    ///6: Dual, regular simultaneous mode only
    DualR = 6,
    ///7: Dual, interleaved mode only
    DualI = 7,
    ///9: Dual, alternate trigger mode only
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
impl crate::IsEnum for DUAL {}
///Field `DUAL` reader - Dual ADC mode selection
pub type DUAL_R = crate::FieldReader<DUAL>;
impl DUAL_R {
    ///Get enumerated values variant
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
    ///Independent mode
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == DUAL::Independent
    }
    ///Dual, combined regular simultaneous + injected simultaneous mode
    #[inline(always)]
    pub fn is_dual_rj(&self) -> bool {
        *self == DUAL::DualRj
    }
    ///Dual, combined regular simultaneous + alternate trigger mode
    #[inline(always)]
    pub fn is_dual_ra(&self) -> bool {
        *self == DUAL::DualRa
    }
    ///Dual, combined interleaved mode + injected simultaneous mode
    #[inline(always)]
    pub fn is_dual_ij(&self) -> bool {
        *self == DUAL::DualIj
    }
    ///Dual, injected simultaneous mode only
    #[inline(always)]
    pub fn is_dual_j(&self) -> bool {
        *self == DUAL::DualJ
    }
    ///Dual, regular simultaneous mode only
    #[inline(always)]
    pub fn is_dual_r(&self) -> bool {
        *self == DUAL::DualR
    }
    ///Dual, interleaved mode only
    #[inline(always)]
    pub fn is_dual_i(&self) -> bool {
        *self == DUAL::DualI
    }
    ///Dual, alternate trigger mode only
    #[inline(always)]
    pub fn is_dual_a(&self) -> bool {
        *self == DUAL::DualA
    }
}
///Field `DUAL` writer - Dual ADC mode selection
pub type DUAL_W<'a, REG> = crate::FieldWriter<'a, REG, 5, DUAL>;
impl<'a, REG> DUAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Independent mode
    #[inline(always)]
    pub fn independent(self) -> &'a mut crate::W<REG> {
        self.variant(DUAL::Independent)
    }
    ///Dual, combined regular simultaneous + injected simultaneous mode
    #[inline(always)]
    pub fn dual_rj(self) -> &'a mut crate::W<REG> {
        self.variant(DUAL::DualRj)
    }
    ///Dual, combined regular simultaneous + alternate trigger mode
    #[inline(always)]
    pub fn dual_ra(self) -> &'a mut crate::W<REG> {
        self.variant(DUAL::DualRa)
    }
    ///Dual, combined interleaved mode + injected simultaneous mode
    #[inline(always)]
    pub fn dual_ij(self) -> &'a mut crate::W<REG> {
        self.variant(DUAL::DualIj)
    }
    ///Dual, injected simultaneous mode only
    #[inline(always)]
    pub fn dual_j(self) -> &'a mut crate::W<REG> {
        self.variant(DUAL::DualJ)
    }
    ///Dual, regular simultaneous mode only
    #[inline(always)]
    pub fn dual_r(self) -> &'a mut crate::W<REG> {
        self.variant(DUAL::DualR)
    }
    ///Dual, interleaved mode only
    #[inline(always)]
    pub fn dual_i(self) -> &'a mut crate::W<REG> {
        self.variant(DUAL::DualI)
    }
    ///Dual, alternate trigger mode only
    #[inline(always)]
    pub fn dual_a(self) -> &'a mut crate::W<REG> {
        self.variant(DUAL::DualA)
    }
}
///Field `DELAY` reader - Delay between 2 sampling phases
pub type DELAY_R = crate::FieldReader;
///Field `DELAY` writer - Delay between 2 sampling phases
pub type DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
/**Dual ADC Mode Data Format

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DAMDF {
    ///0: Without data packing, CDR/CDR2 not used
    NoPack = 0,
    ///2: CDR formatted for 32-bit down to 10-bit resolution
    Format32to10 = 2,
    ///3: CDR formatted for 8-bit resolution
    Format8 = 3,
}
impl From<DAMDF> for u8 {
    #[inline(always)]
    fn from(variant: DAMDF) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DAMDF {
    type Ux = u8;
}
impl crate::IsEnum for DAMDF {}
///Field `DAMDF` reader - Dual ADC Mode Data Format
pub type DAMDF_R = crate::FieldReader<DAMDF>;
impl DAMDF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<DAMDF> {
        match self.bits {
            0 => Some(DAMDF::NoPack),
            2 => Some(DAMDF::Format32to10),
            3 => Some(DAMDF::Format8),
            _ => None,
        }
    }
    ///Without data packing, CDR/CDR2 not used
    #[inline(always)]
    pub fn is_no_pack(&self) -> bool {
        *self == DAMDF::NoPack
    }
    ///CDR formatted for 32-bit down to 10-bit resolution
    #[inline(always)]
    pub fn is_format32to10(&self) -> bool {
        *self == DAMDF::Format32to10
    }
    ///CDR formatted for 8-bit resolution
    #[inline(always)]
    pub fn is_format8(&self) -> bool {
        *self == DAMDF::Format8
    }
}
///Field `DAMDF` writer - Dual ADC Mode Data Format
pub type DAMDF_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DAMDF>;
impl<'a, REG> DAMDF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Without data packing, CDR/CDR2 not used
    #[inline(always)]
    pub fn no_pack(self) -> &'a mut crate::W<REG> {
        self.variant(DAMDF::NoPack)
    }
    ///CDR formatted for 32-bit down to 10-bit resolution
    #[inline(always)]
    pub fn format32to10(self) -> &'a mut crate::W<REG> {
        self.variant(DAMDF::Format32to10)
    }
    ///CDR formatted for 8-bit resolution
    #[inline(always)]
    pub fn format8(self) -> &'a mut crate::W<REG> {
        self.variant(DAMDF::Format8)
    }
}
/**ADC clock mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKMODE {
    ///0: Use Kernel Clock adc_ker_ck_input divided by PRESC. Asynchronous to AHB clock
    Asynchronous = 0,
    ///1: Use AHB clock rcc_hclk3. In this case rcc_hclk must equal sys_d1cpre_ck
    SyncDiv1 = 1,
    ///2: Use AHB clock rcc_hclk3 divided by 2
    SyncDiv2 = 2,
    ///3: Use AHB clock rcc_hclk3 divided by 4
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
impl crate::IsEnum for CKMODE {}
///Field `CKMODE` reader - ADC clock mode
pub type CKMODE_R = crate::FieldReader<CKMODE>;
impl CKMODE_R {
    ///Get enumerated values variant
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
    ///Use Kernel Clock adc_ker_ck_input divided by PRESC. Asynchronous to AHB clock
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == CKMODE::Asynchronous
    }
    ///Use AHB clock rcc_hclk3. In this case rcc_hclk must equal sys_d1cpre_ck
    #[inline(always)]
    pub fn is_sync_div1(&self) -> bool {
        *self == CKMODE::SyncDiv1
    }
    ///Use AHB clock rcc_hclk3 divided by 2
    #[inline(always)]
    pub fn is_sync_div2(&self) -> bool {
        *self == CKMODE::SyncDiv2
    }
    ///Use AHB clock rcc_hclk3 divided by 4
    #[inline(always)]
    pub fn is_sync_div4(&self) -> bool {
        *self == CKMODE::SyncDiv4
    }
}
///Field `CKMODE` writer - ADC clock mode
pub type CKMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CKMODE, crate::Safe>;
impl<'a, REG> CKMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Use Kernel Clock adc_ker_ck_input divided by PRESC. Asynchronous to AHB clock
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut crate::W<REG> {
        self.variant(CKMODE::Asynchronous)
    }
    ///Use AHB clock rcc_hclk3. In this case rcc_hclk must equal sys_d1cpre_ck
    #[inline(always)]
    pub fn sync_div1(self) -> &'a mut crate::W<REG> {
        self.variant(CKMODE::SyncDiv1)
    }
    ///Use AHB clock rcc_hclk3 divided by 2
    #[inline(always)]
    pub fn sync_div2(self) -> &'a mut crate::W<REG> {
        self.variant(CKMODE::SyncDiv2)
    }
    ///Use AHB clock rcc_hclk3 divided by 4
    #[inline(always)]
    pub fn sync_div4(self) -> &'a mut crate::W<REG> {
        self.variant(CKMODE::SyncDiv4)
    }
}
/**ADC prescaler

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESC {
    ///0: adc_ker_ck_input not divided
    Div1 = 0,
    ///1: adc_ker_ck_input divided by 2
    Div2 = 1,
    ///2: adc_ker_ck_input divided by 4
    Div4 = 2,
    ///3: adc_ker_ck_input divided by 6
    Div6 = 3,
    ///4: adc_ker_ck_input divided by 8
    Div8 = 4,
    ///5: adc_ker_ck_input divided by 10
    Div10 = 5,
    ///6: adc_ker_ck_input divided by 12
    Div12 = 6,
    ///7: adc_ker_ck_input divided by 16
    Div16 = 7,
    ///8: adc_ker_ck_input divided by 32
    Div32 = 8,
    ///9: adc_ker_ck_input divided by 64
    Div64 = 9,
    ///10: adc_ker_ck_input divided by 128
    Div128 = 10,
    ///11: adc_ker_ck_input divided by 256
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
impl crate::IsEnum for PRESC {}
///Field `PRESC` reader - ADC prescaler
pub type PRESC_R = crate::FieldReader<PRESC>;
impl PRESC_R {
    ///Get enumerated values variant
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
    ///adc_ker_ck_input not divided
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PRESC::Div1
    }
    ///adc_ker_ck_input divided by 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PRESC::Div2
    }
    ///adc_ker_ck_input divided by 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PRESC::Div4
    }
    ///adc_ker_ck_input divided by 6
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PRESC::Div6
    }
    ///adc_ker_ck_input divided by 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PRESC::Div8
    }
    ///adc_ker_ck_input divided by 10
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PRESC::Div10
    }
    ///adc_ker_ck_input divided by 12
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PRESC::Div12
    }
    ///adc_ker_ck_input divided by 16
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PRESC::Div16
    }
    ///adc_ker_ck_input divided by 32
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PRESC::Div32
    }
    ///adc_ker_ck_input divided by 64
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PRESC::Div64
    }
    ///adc_ker_ck_input divided by 128
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PRESC::Div128
    }
    ///adc_ker_ck_input divided by 256
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == PRESC::Div256
    }
}
///Field `PRESC` writer - ADC prescaler
pub type PRESC_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PRESC>;
impl<'a, REG> PRESC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///adc_ker_ck_input not divided
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div1)
    }
    ///adc_ker_ck_input divided by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div2)
    }
    ///adc_ker_ck_input divided by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div4)
    }
    ///adc_ker_ck_input divided by 6
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div6)
    }
    ///adc_ker_ck_input divided by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div8)
    }
    ///adc_ker_ck_input divided by 10
    #[inline(always)]
    pub fn div10(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div10)
    }
    ///adc_ker_ck_input divided by 12
    #[inline(always)]
    pub fn div12(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div12)
    }
    ///adc_ker_ck_input divided by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div16)
    }
    ///adc_ker_ck_input divided by 32
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div32)
    }
    ///adc_ker_ck_input divided by 64
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div64)
    }
    ///adc_ker_ck_input divided by 128
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div128)
    }
    ///adc_ker_ck_input divided by 256
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div256)
    }
}
/**VREFINT enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VREFEN {
    ///0: V_REFINT channel disabled
    Disabled = 0,
    ///1: V_REFINT channel enabled
    Enabled = 1,
}
impl From<VREFEN> for bool {
    #[inline(always)]
    fn from(variant: VREFEN) -> Self {
        variant as u8 != 0
    }
}
///Field `VREFEN` reader - VREFINT enable
pub type VREFEN_R = crate::BitReader<VREFEN>;
impl VREFEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VREFEN {
        match self.bits {
            false => VREFEN::Disabled,
            true => VREFEN::Enabled,
        }
    }
    ///V_REFINT channel disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VREFEN::Disabled
    }
    ///V_REFINT channel enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VREFEN::Enabled
    }
}
///Field `VREFEN` writer - VREFINT enable
pub type VREFEN_W<'a, REG> = crate::BitWriter<'a, REG, VREFEN>;
impl<'a, REG> VREFEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///V_REFINT channel disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(VREFEN::Disabled)
    }
    ///V_REFINT channel enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(VREFEN::Enabled)
    }
}
/**Temperature sensor enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VSENSEEN {
    ///0: The selected ADC channel disabled
    Disabled = 0,
    ///1: The selected ADC channel enabled
    Enabled = 1,
}
impl From<VSENSEEN> for bool {
    #[inline(always)]
    fn from(variant: VSENSEEN) -> Self {
        variant as u8 != 0
    }
}
///Field `VSENSEEN` reader - Temperature sensor enable
pub type VSENSEEN_R = crate::BitReader<VSENSEEN>;
impl VSENSEEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VSENSEEN {
        match self.bits {
            false => VSENSEEN::Disabled,
            true => VSENSEEN::Enabled,
        }
    }
    ///The selected ADC channel disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VSENSEEN::Disabled
    }
    ///The selected ADC channel enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VSENSEEN::Enabled
    }
}
///Field `VSENSEEN` writer - Temperature sensor enable
pub type VSENSEEN_W<'a, REG> = crate::BitWriter<'a, REG, VSENSEEN>;
impl<'a, REG> VSENSEEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected ADC channel disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(VSENSEEN::Disabled)
    }
    ///The selected ADC channel enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(VSENSEEN::Enabled)
    }
}
///Field `VBATEN` reader - VBAT enable
pub use VSENSEEN_R as VBATEN_R;
///Field `VBATEN` writer - VBAT enable
pub use VSENSEEN_W as VBATEN_W;
impl R {
    ///Bits 0:4 - Dual ADC mode selection
    #[inline(always)]
    pub fn dual(&self) -> DUAL_R {
        DUAL_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 8:11 - Delay between 2 sampling phases
    #[inline(always)]
    pub fn delay(&self) -> DELAY_R {
        DELAY_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 14:15 - Dual ADC Mode Data Format
    #[inline(always)]
    pub fn damdf(&self) -> DAMDF_R {
        DAMDF_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:17 - ADC clock mode
    #[inline(always)]
    pub fn ckmode(&self) -> CKMODE_R {
        CKMODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:21 - ADC prescaler
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    ///Bit 22 - VREFINT enable
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Temperature sensor enable
    #[inline(always)]
    pub fn vsenseen(&self) -> VSENSEEN_R {
        VSENSEEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - VBAT enable
    #[inline(always)]
    pub fn vbaten(&self) -> VBATEN_R {
        VBATEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR")
            .field("dual", &self.dual())
            .field("delay", &self.delay())
            .field("damdf", &self.damdf())
            .field("ckmode", &self.ckmode())
            .field("presc", &self.presc())
            .field("vrefen", &self.vrefen())
            .field("vsenseen", &self.vsenseen())
            .field("vbaten", &self.vbaten())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - Dual ADC mode selection
    #[inline(always)]
    pub fn dual(&mut self) -> DUAL_W<CCRrs> {
        DUAL_W::new(self, 0)
    }
    ///Bits 8:11 - Delay between 2 sampling phases
    #[inline(always)]
    pub fn delay(&mut self) -> DELAY_W<CCRrs> {
        DELAY_W::new(self, 8)
    }
    ///Bits 14:15 - Dual ADC Mode Data Format
    #[inline(always)]
    pub fn damdf(&mut self) -> DAMDF_W<CCRrs> {
        DAMDF_W::new(self, 14)
    }
    ///Bits 16:17 - ADC clock mode
    #[inline(always)]
    pub fn ckmode(&mut self) -> CKMODE_W<CCRrs> {
        CKMODE_W::new(self, 16)
    }
    ///Bits 18:21 - ADC prescaler
    #[inline(always)]
    pub fn presc(&mut self) -> PRESC_W<CCRrs> {
        PRESC_W::new(self, 18)
    }
    ///Bit 22 - VREFINT enable
    #[inline(always)]
    pub fn vrefen(&mut self) -> VREFEN_W<CCRrs> {
        VREFEN_W::new(self, 22)
    }
    ///Bit 23 - Temperature sensor enable
    #[inline(always)]
    pub fn vsenseen(&mut self) -> VSENSEEN_W<CCRrs> {
        VSENSEEN_W::new(self, 23)
    }
    ///Bit 24 - VBAT enable
    #[inline(always)]
    pub fn vbaten(&mut self) -> VBATEN_W<CCRrs> {
        VBATEN_W::new(self, 24)
    }
}
/**ADC common control register

You can [`read`](crate::Reg::read) this register and get [`ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM7.html#ADC3_Common:CCR)*/
pub struct CCRrs;
impl crate::RegisterSpec for CCRrs {
    type Ux = u32;
}
///`read()` method returns [`ccr::R`](R) reader structure
impl crate::Readable for CCRrs {}
///`write(|w| ..)` method takes [`ccr::W`](W) writer structure
impl crate::Writable for CCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCR to value 0
impl crate::Resettable for CCRrs {}

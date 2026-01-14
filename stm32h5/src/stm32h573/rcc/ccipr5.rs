///Register `CCIPR5` reader
pub type R = crate::R<CCIPR5rs>;
///Register `CCIPR5` writer
pub type W = crate::W<CCIPR5rs>;
/**ADC and DAC kernel clock source selection others: reserved, the kernel clock is disabled

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADCDACSEL {
    ///0: HLCK clock selected as clock source (rcc_hclk)
    Hclk = 0,
    ///1: System clock selected as pclock source (sys_ck)
    Sys = 1,
    ///2: PLL2 R clock selected as clock source (pll2_r_ck)
    Pll2R = 2,
    ///3: HSE clock selected as clock source (hse_ck)
    Hse = 3,
    ///4: HSI kernel clock selected as clock source (hsi_ker_ck)
    HsiKer = 4,
    ///5: CSI kernel clock selected as clock source (csi_ker_ck)
    CsiKer = 5,
}
impl From<ADCDACSEL> for u8 {
    #[inline(always)]
    fn from(variant: ADCDACSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADCDACSEL {
    type Ux = u8;
}
impl crate::IsEnum for ADCDACSEL {}
///Field `ADCDACSEL` reader - ADC and DAC kernel clock source selection others: reserved, the kernel clock is disabled
pub type ADCDACSEL_R = crate::FieldReader<ADCDACSEL>;
impl ADCDACSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<ADCDACSEL> {
        match self.bits {
            0 => Some(ADCDACSEL::Hclk),
            1 => Some(ADCDACSEL::Sys),
            2 => Some(ADCDACSEL::Pll2R),
            3 => Some(ADCDACSEL::Hse),
            4 => Some(ADCDACSEL::HsiKer),
            5 => Some(ADCDACSEL::CsiKer),
            _ => None,
        }
    }
    ///HLCK clock selected as clock source (rcc_hclk)
    #[inline(always)]
    pub fn is_hclk(&self) -> bool {
        *self == ADCDACSEL::Hclk
    }
    ///System clock selected as pclock source (sys_ck)
    #[inline(always)]
    pub fn is_sys(&self) -> bool {
        *self == ADCDACSEL::Sys
    }
    ///PLL2 R clock selected as clock source (pll2_r_ck)
    #[inline(always)]
    pub fn is_pll2_r(&self) -> bool {
        *self == ADCDACSEL::Pll2R
    }
    ///HSE clock selected as clock source (hse_ck)
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == ADCDACSEL::Hse
    }
    ///HSI kernel clock selected as clock source (hsi_ker_ck)
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == ADCDACSEL::HsiKer
    }
    ///CSI kernel clock selected as clock source (csi_ker_ck)
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        *self == ADCDACSEL::CsiKer
    }
}
///Field `ADCDACSEL` writer - ADC and DAC kernel clock source selection others: reserved, the kernel clock is disabled
pub type ADCDACSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, ADCDACSEL>;
impl<'a, REG> ADCDACSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///HLCK clock selected as clock source (rcc_hclk)
    #[inline(always)]
    pub fn hclk(self) -> &'a mut crate::W<REG> {
        self.variant(ADCDACSEL::Hclk)
    }
    ///System clock selected as pclock source (sys_ck)
    #[inline(always)]
    pub fn sys(self) -> &'a mut crate::W<REG> {
        self.variant(ADCDACSEL::Sys)
    }
    ///PLL2 R clock selected as clock source (pll2_r_ck)
    #[inline(always)]
    pub fn pll2_r(self) -> &'a mut crate::W<REG> {
        self.variant(ADCDACSEL::Pll2R)
    }
    ///HSE clock selected as clock source (hse_ck)
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(ADCDACSEL::Hse)
    }
    ///HSI kernel clock selected as clock source (hsi_ker_ck)
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(ADCDACSEL::HsiKer)
    }
    ///CSI kernel clock selected as clock source (csi_ker_ck)
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(ADCDACSEL::CsiKer)
    }
}
/**DAC hold clock

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DACSEL {
    ///0: LSE selected as clock source (lse_ck)
    Lse = 0,
    ///1: LSI kernel selected as clock source (lsi_ker_ck)
    LsiKer = 1,
}
impl From<DACSEL> for bool {
    #[inline(always)]
    fn from(variant: DACSEL) -> Self {
        variant as u8 != 0
    }
}
///Field `DACSEL` reader - DAC hold clock
pub type DACSEL_R = crate::BitReader<DACSEL>;
impl DACSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DACSEL {
        match self.bits {
            false => DACSEL::Lse,
            true => DACSEL::LsiKer,
        }
    }
    ///LSE selected as clock source (lse_ck)
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == DACSEL::Lse
    }
    ///LSI kernel selected as clock source (lsi_ker_ck)
    #[inline(always)]
    pub fn is_lsi_ker(&self) -> bool {
        *self == DACSEL::LsiKer
    }
}
///Field `DACSEL` writer - DAC hold clock
pub type DACSEL_W<'a, REG> = crate::BitWriter<'a, REG, DACSEL>;
impl<'a, REG> DACSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LSE selected as clock source (lse_ck)
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(DACSEL::Lse)
    }
    ///LSI kernel selected as clock source (lsi_ker_ck)
    #[inline(always)]
    pub fn lsi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(DACSEL::LsiKer)
    }
}
/**RNG kernel clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RNGSEL {
    ///0: HSI48 kernel clock selected as clock source (hsi48_ker_ck)
    Hsi48Ker = 0,
    ///1: PLL1 Q clock selected as clock source (pll1_q_ck)
    Pll1Q = 1,
    ///2: LSE clock selected as clock source (lse_ck)
    Lse = 2,
    ///3: LSI kernel clock selected as clock source (lsi_ker_ck)
    Lsi = 3,
}
impl From<RNGSEL> for u8 {
    #[inline(always)]
    fn from(variant: RNGSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RNGSEL {
    type Ux = u8;
}
impl crate::IsEnum for RNGSEL {}
///Field `RNGSEL` reader - RNG kernel clock source selection
pub type RNGSEL_R = crate::FieldReader<RNGSEL>;
impl RNGSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RNGSEL {
        match self.bits {
            0 => RNGSEL::Hsi48Ker,
            1 => RNGSEL::Pll1Q,
            2 => RNGSEL::Lse,
            3 => RNGSEL::Lsi,
            _ => unreachable!(),
        }
    }
    ///HSI48 kernel clock selected as clock source (hsi48_ker_ck)
    #[inline(always)]
    pub fn is_hsi48_ker(&self) -> bool {
        *self == RNGSEL::Hsi48Ker
    }
    ///PLL1 Q clock selected as clock source (pll1_q_ck)
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == RNGSEL::Pll1Q
    }
    ///LSE clock selected as clock source (lse_ck)
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == RNGSEL::Lse
    }
    ///LSI kernel clock selected as clock source (lsi_ker_ck)
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == RNGSEL::Lsi
    }
}
///Field `RNGSEL` writer - RNG kernel clock source selection
pub type RNGSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, RNGSEL, crate::Safe>;
impl<'a, REG> RNGSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///HSI48 kernel clock selected as clock source (hsi48_ker_ck)
    #[inline(always)]
    pub fn hsi48_ker(self) -> &'a mut crate::W<REG> {
        self.variant(RNGSEL::Hsi48Ker)
    }
    ///PLL1 Q clock selected as clock source (pll1_q_ck)
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut crate::W<REG> {
        self.variant(RNGSEL::Pll1Q)
    }
    ///LSE clock selected as clock source (lse_ck)
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(RNGSEL::Lse)
    }
    ///LSI kernel clock selected as clock source (lsi_ker_ck)
    #[inline(always)]
    pub fn lsi(self) -> &'a mut crate::W<REG> {
        self.variant(RNGSEL::Lsi)
    }
}
/**HSMI-CEC kernel clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CECSEL {
    ///0: LSE selected as clock source (lse_ck)
    Lse = 0,
    ///1: LSI kernel selected as clock source (lsi_ker_ck)
    LsiKer = 1,
    ///2: CSI kernel clock divided by 122 selected as clock source (csi_ker_ck/122)
    CsiKer = 2,
}
impl From<CECSEL> for u8 {
    #[inline(always)]
    fn from(variant: CECSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CECSEL {
    type Ux = u8;
}
impl crate::IsEnum for CECSEL {}
///Field `CECSEL` reader - HSMI-CEC kernel clock source selection
pub type CECSEL_R = crate::FieldReader<CECSEL>;
impl CECSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CECSEL> {
        match self.bits {
            0 => Some(CECSEL::Lse),
            1 => Some(CECSEL::LsiKer),
            2 => Some(CECSEL::CsiKer),
            _ => None,
        }
    }
    ///LSE selected as clock source (lse_ck)
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == CECSEL::Lse
    }
    ///LSI kernel selected as clock source (lsi_ker_ck)
    #[inline(always)]
    pub fn is_lsi_ker(&self) -> bool {
        *self == CECSEL::LsiKer
    }
    ///CSI kernel clock divided by 122 selected as clock source (csi_ker_ck/122)
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        *self == CECSEL::CsiKer
    }
}
///Field `CECSEL` writer - HSMI-CEC kernel clock source selection
pub type CECSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CECSEL>;
impl<'a, REG> CECSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///LSE selected as clock source (lse_ck)
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(CECSEL::Lse)
    }
    ///LSI kernel selected as clock source (lsi_ker_ck)
    #[inline(always)]
    pub fn lsi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(CECSEL::LsiKer)
    }
    ///CSI kernel clock divided by 122 selected as clock source (csi_ker_ck/122)
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(CECSEL::CsiKer)
    }
}
/**FDCAN kernel clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FDCANSEL {
    ///0: HSE clock selected as clock source (hse_ck)
    Hse = 0,
    ///1: PLL1 Q clock selected as clock source (pll1_q_ck)
    Pll1Q = 1,
    ///2: PLL2 Q clock selected as clock source (pll2_q_ck)
    Pll2Q = 2,
}
impl From<FDCANSEL> for u8 {
    #[inline(always)]
    fn from(variant: FDCANSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FDCANSEL {
    type Ux = u8;
}
impl crate::IsEnum for FDCANSEL {}
///Field `FDCANSEL` reader - FDCAN kernel clock source selection
pub type FDCANSEL_R = crate::FieldReader<FDCANSEL>;
impl FDCANSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<FDCANSEL> {
        match self.bits {
            0 => Some(FDCANSEL::Hse),
            1 => Some(FDCANSEL::Pll1Q),
            2 => Some(FDCANSEL::Pll2Q),
            _ => None,
        }
    }
    ///HSE clock selected as clock source (hse_ck)
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == FDCANSEL::Hse
    }
    ///PLL1 Q clock selected as clock source (pll1_q_ck)
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == FDCANSEL::Pll1Q
    }
    ///PLL2 Q clock selected as clock source (pll2_q_ck)
    #[inline(always)]
    pub fn is_pll2_q(&self) -> bool {
        *self == FDCANSEL::Pll2Q
    }
}
///Field `FDCANSEL` writer - FDCAN kernel clock source selection
pub type FDCANSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, FDCANSEL>;
impl<'a, REG> FDCANSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///HSE clock selected as clock source (hse_ck)
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(FDCANSEL::Hse)
    }
    ///PLL1 Q clock selected as clock source (pll1_q_ck)
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut crate::W<REG> {
        self.variant(FDCANSEL::Pll1Q)
    }
    ///PLL2 Q clock selected as clock source (pll2_q_ck)
    #[inline(always)]
    pub fn pll2_q(self) -> &'a mut crate::W<REG> {
        self.variant(FDCANSEL::Pll2Q)
    }
}
/**SAI1 kernel clock source selection others: reserved, the kernel clock is disabled

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SAISEL {
    ///0: PLL1 Q clock selected as clock source (pll1_q_ck)
    Pll1Q = 0,
    ///1: PLL2 P clock selected as clock source (pll2_p_ck)
    Pll2P = 1,
    ///2: PLL3 P clock selected as clock source (pll3_p_ck)
    Pll3P = 2,
    ///3: AUDIOCLK clock selected as clock source
    Audioclk = 3,
    ///4: per_ck clock selected as clock source
    PerCk = 4,
}
impl From<SAISEL> for u8 {
    #[inline(always)]
    fn from(variant: SAISEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SAISEL {
    type Ux = u8;
}
impl crate::IsEnum for SAISEL {}
///Field `SAI1SEL` reader - SAI1 kernel clock source selection others: reserved, the kernel clock is disabled
pub type SAI1SEL_R = crate::FieldReader<SAISEL>;
impl SAI1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SAISEL> {
        match self.bits {
            0 => Some(SAISEL::Pll1Q),
            1 => Some(SAISEL::Pll2P),
            2 => Some(SAISEL::Pll3P),
            3 => Some(SAISEL::Audioclk),
            4 => Some(SAISEL::PerCk),
            _ => None,
        }
    }
    ///PLL1 Q clock selected as clock source (pll1_q_ck)
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == SAISEL::Pll1Q
    }
    ///PLL2 P clock selected as clock source (pll2_p_ck)
    #[inline(always)]
    pub fn is_pll2_p(&self) -> bool {
        *self == SAISEL::Pll2P
    }
    ///PLL3 P clock selected as clock source (pll3_p_ck)
    #[inline(always)]
    pub fn is_pll3_p(&self) -> bool {
        *self == SAISEL::Pll3P
    }
    ///AUDIOCLK clock selected as clock source
    #[inline(always)]
    pub fn is_audioclk(&self) -> bool {
        *self == SAISEL::Audioclk
    }
    ///per_ck clock selected as clock source
    #[inline(always)]
    pub fn is_per_ck(&self) -> bool {
        *self == SAISEL::PerCk
    }
}
///Field `SAI1SEL` writer - SAI1 kernel clock source selection others: reserved, the kernel clock is disabled
pub type SAI1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SAISEL>;
impl<'a, REG> SAI1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PLL1 Q clock selected as clock source (pll1_q_ck)
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut crate::W<REG> {
        self.variant(SAISEL::Pll1Q)
    }
    ///PLL2 P clock selected as clock source (pll2_p_ck)
    #[inline(always)]
    pub fn pll2_p(self) -> &'a mut crate::W<REG> {
        self.variant(SAISEL::Pll2P)
    }
    ///PLL3 P clock selected as clock source (pll3_p_ck)
    #[inline(always)]
    pub fn pll3_p(self) -> &'a mut crate::W<REG> {
        self.variant(SAISEL::Pll3P)
    }
    ///AUDIOCLK clock selected as clock source
    #[inline(always)]
    pub fn audioclk(self) -> &'a mut crate::W<REG> {
        self.variant(SAISEL::Audioclk)
    }
    ///per_ck clock selected as clock source
    #[inline(always)]
    pub fn per_ck(self) -> &'a mut crate::W<REG> {
        self.variant(SAISEL::PerCk)
    }
}
///Field `SAI2SEL` reader - SAI2 kernel clock source selection others: reserved, the kernel clock is disabled
pub use SAI1SEL_R as SAI2SEL_R;
///Field `SAI2SEL` writer - SAI2 kernel clock source selection others: reserved, the kernel clock is disabled
pub use SAI1SEL_W as SAI2SEL_W;
/**per_ck clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKPERSEL {
    ///0: HSI kernel clock selected as clock source (hsi_ker_ck)
    HsiKer = 0,
    ///1: CSI kernel clock selected as clock source (csi_ker_ck)
    CsiKer = 1,
    ///2: HSE clock selected as clock source (hse_ck)
    Hse = 2,
}
impl From<CKPERSEL> for u8 {
    #[inline(always)]
    fn from(variant: CKPERSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CKPERSEL {
    type Ux = u8;
}
impl crate::IsEnum for CKPERSEL {}
///Field `CKPERSEL` reader - per_ck clock source selection
pub type CKPERSEL_R = crate::FieldReader<CKPERSEL>;
impl CKPERSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CKPERSEL> {
        match self.bits {
            0 => Some(CKPERSEL::HsiKer),
            1 => Some(CKPERSEL::CsiKer),
            2 => Some(CKPERSEL::Hse),
            _ => None,
        }
    }
    ///HSI kernel clock selected as clock source (hsi_ker_ck)
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == CKPERSEL::HsiKer
    }
    ///CSI kernel clock selected as clock source (csi_ker_ck)
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        *self == CKPERSEL::CsiKer
    }
    ///HSE clock selected as clock source (hse_ck)
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == CKPERSEL::Hse
    }
}
///Field `CKPERSEL` writer - per_ck clock source selection
pub type CKPERSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CKPERSEL>;
impl<'a, REG> CKPERSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///HSI kernel clock selected as clock source (hsi_ker_ck)
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(CKPERSEL::HsiKer)
    }
    ///CSI kernel clock selected as clock source (csi_ker_ck)
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(CKPERSEL::CsiKer)
    }
    ///HSE clock selected as clock source (hse_ck)
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(CKPERSEL::Hse)
    }
}
impl R {
    ///Bits 0:2 - ADC and DAC kernel clock source selection others: reserved, the kernel clock is disabled
    #[inline(always)]
    pub fn adcdacsel(&self) -> ADCDACSEL_R {
        ADCDACSEL_R::new((self.bits & 7) as u8)
    }
    ///Bit 3 - DAC hold clock
    #[inline(always)]
    pub fn dacsel(&self) -> DACSEL_R {
        DACSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - RNG kernel clock source selection
    #[inline(always)]
    pub fn rngsel(&self) -> RNGSEL_R {
        RNGSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - HSMI-CEC kernel clock source selection
    #[inline(always)]
    pub fn cecsel(&self) -> CECSEL_R {
        CECSEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - FDCAN kernel clock source selection
    #[inline(always)]
    pub fn fdcansel(&self) -> FDCANSEL_R {
        FDCANSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 16:18 - SAI1 kernel clock source selection others: reserved, the kernel clock is disabled
    #[inline(always)]
    pub fn sai1sel(&self) -> SAI1SEL_R {
        SAI1SEL_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 19:21 - SAI2 kernel clock source selection others: reserved, the kernel clock is disabled
    #[inline(always)]
    pub fn sai2sel(&self) -> SAI2SEL_R {
        SAI2SEL_R::new(((self.bits >> 19) & 7) as u8)
    }
    ///Bits 30:31 - per_ck clock source selection
    #[inline(always)]
    pub fn ckpersel(&self) -> CKPERSEL_R {
        CKPERSEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCIPR5")
            .field("adcdacsel", &self.adcdacsel())
            .field("dacsel", &self.dacsel())
            .field("rngsel", &self.rngsel())
            .field("cecsel", &self.cecsel())
            .field("fdcansel", &self.fdcansel())
            .field("sai1sel", &self.sai1sel())
            .field("sai2sel", &self.sai2sel())
            .field("ckpersel", &self.ckpersel())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - ADC and DAC kernel clock source selection others: reserved, the kernel clock is disabled
    #[inline(always)]
    pub fn adcdacsel(&mut self) -> ADCDACSEL_W<'_, CCIPR5rs> {
        ADCDACSEL_W::new(self, 0)
    }
    ///Bit 3 - DAC hold clock
    #[inline(always)]
    pub fn dacsel(&mut self) -> DACSEL_W<'_, CCIPR5rs> {
        DACSEL_W::new(self, 3)
    }
    ///Bits 4:5 - RNG kernel clock source selection
    #[inline(always)]
    pub fn rngsel(&mut self) -> RNGSEL_W<'_, CCIPR5rs> {
        RNGSEL_W::new(self, 4)
    }
    ///Bits 6:7 - HSMI-CEC kernel clock source selection
    #[inline(always)]
    pub fn cecsel(&mut self) -> CECSEL_W<'_, CCIPR5rs> {
        CECSEL_W::new(self, 6)
    }
    ///Bits 8:9 - FDCAN kernel clock source selection
    #[inline(always)]
    pub fn fdcansel(&mut self) -> FDCANSEL_W<'_, CCIPR5rs> {
        FDCANSEL_W::new(self, 8)
    }
    ///Bits 16:18 - SAI1 kernel clock source selection others: reserved, the kernel clock is disabled
    #[inline(always)]
    pub fn sai1sel(&mut self) -> SAI1SEL_W<'_, CCIPR5rs> {
        SAI1SEL_W::new(self, 16)
    }
    ///Bits 19:21 - SAI2 kernel clock source selection others: reserved, the kernel clock is disabled
    #[inline(always)]
    pub fn sai2sel(&mut self) -> SAI2SEL_W<'_, CCIPR5rs> {
        SAI2SEL_W::new(self, 19)
    }
    ///Bits 30:31 - per_ck clock source selection
    #[inline(always)]
    pub fn ckpersel(&mut self) -> CKPERSEL_W<'_, CCIPR5rs> {
        CKPERSEL_W::new(self, 30)
    }
}
/**RCC kernel clock configuration register

You can [`read`](crate::Reg::read) this register and get [`ccipr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RCC:CCIPR5)*/
pub struct CCIPR5rs;
impl crate::RegisterSpec for CCIPR5rs {
    type Ux = u32;
}
///`read()` method returns [`ccipr5::R`](R) reader structure
impl crate::Readable for CCIPR5rs {}
///`write(|w| ..)` method takes [`ccipr5::W`](W) writer structure
impl crate::Writable for CCIPR5rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCIPR5 to value 0
impl crate::Resettable for CCIPR5rs {}

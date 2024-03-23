#[doc = "Register `CCIPR5` reader"]
pub type R = crate::R<CCIPR5rs>;
#[doc = "Register `CCIPR5` writer"]
pub type W = crate::W<CCIPR5rs>;
#[doc = "ADC and DAC kernel clock source selection others: reserved, the kernel clock is disabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADCDACSEL {
    #[doc = "0: HLCK clock selected as clock source (rcc_hclk)"]
    Hclk = 0,
    #[doc = "1: System clock selected as pclock source (sys_ck)"]
    Sys = 1,
    #[doc = "2: PLL2 R clock selected as clock source (pll2_r_ck)"]
    Pll2R = 2,
    #[doc = "3: HSE clock selected as clock source (hse_ck)"]
    Hse = 3,
    #[doc = "4: HSI kernel clock selected as clock source (hsi_ker_ck)"]
    HsiKer = 4,
    #[doc = "5: CSI kernel clock selected as clock source (csi_ker_ck)"]
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
#[doc = "Field `ADCDACSEL` reader - ADC and DAC kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type ADCDACSEL_R = crate::FieldReader<ADCDACSEL>;
impl ADCDACSEL_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "HLCK clock selected as clock source (rcc_hclk)"]
    #[inline(always)]
    pub fn is_hclk(&self) -> bool {
        *self == ADCDACSEL::Hclk
    }
    #[doc = "System clock selected as pclock source (sys_ck)"]
    #[inline(always)]
    pub fn is_sys(&self) -> bool {
        *self == ADCDACSEL::Sys
    }
    #[doc = "PLL2 R clock selected as clock source (pll2_r_ck)"]
    #[inline(always)]
    pub fn is_pll2_r(&self) -> bool {
        *self == ADCDACSEL::Pll2R
    }
    #[doc = "HSE clock selected as clock source (hse_ck)"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == ADCDACSEL::Hse
    }
    #[doc = "HSI kernel clock selected as clock source (hsi_ker_ck)"]
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == ADCDACSEL::HsiKer
    }
    #[doc = "CSI kernel clock selected as clock source (csi_ker_ck)"]
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        *self == ADCDACSEL::CsiKer
    }
}
#[doc = "Field `ADCDACSEL` writer - ADC and DAC kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type ADCDACSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, ADCDACSEL>;
impl<'a, REG> ADCDACSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HLCK clock selected as clock source (rcc_hclk)"]
    #[inline(always)]
    pub fn hclk(self) -> &'a mut crate::W<REG> {
        self.variant(ADCDACSEL::Hclk)
    }
    #[doc = "System clock selected as pclock source (sys_ck)"]
    #[inline(always)]
    pub fn sys(self) -> &'a mut crate::W<REG> {
        self.variant(ADCDACSEL::Sys)
    }
    #[doc = "PLL2 R clock selected as clock source (pll2_r_ck)"]
    #[inline(always)]
    pub fn pll2_r(self) -> &'a mut crate::W<REG> {
        self.variant(ADCDACSEL::Pll2R)
    }
    #[doc = "HSE clock selected as clock source (hse_ck)"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(ADCDACSEL::Hse)
    }
    #[doc = "HSI kernel clock selected as clock source (hsi_ker_ck)"]
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(ADCDACSEL::HsiKer)
    }
    #[doc = "CSI kernel clock selected as clock source (csi_ker_ck)"]
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(ADCDACSEL::CsiKer)
    }
}
#[doc = "DAC hold clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAC1SEL {
    #[doc = "0: LSE selected as clock source (lse_ck)"]
    Lse = 0,
    #[doc = "1: LSI kernel selected as clock source (lsi_ker_ck)"]
    LsiKer = 1,
}
impl From<DAC1SEL> for bool {
    #[inline(always)]
    fn from(variant: DAC1SEL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAC1SEL` reader - DAC hold clock"]
pub type DAC1SEL_R = crate::BitReader<DAC1SEL>;
impl DAC1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DAC1SEL {
        match self.bits {
            false => DAC1SEL::Lse,
            true => DAC1SEL::LsiKer,
        }
    }
    #[doc = "LSE selected as clock source (lse_ck)"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == DAC1SEL::Lse
    }
    #[doc = "LSI kernel selected as clock source (lsi_ker_ck)"]
    #[inline(always)]
    pub fn is_lsi_ker(&self) -> bool {
        *self == DAC1SEL::LsiKer
    }
}
#[doc = "Field `DAC1SEL` writer - DAC hold clock"]
pub type DAC1SEL_W<'a, REG> = crate::BitWriter<'a, REG, DAC1SEL>;
impl<'a, REG> DAC1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LSE selected as clock source (lse_ck)"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(DAC1SEL::Lse)
    }
    #[doc = "LSI kernel selected as clock source (lsi_ker_ck)"]
    #[inline(always)]
    pub fn lsi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(DAC1SEL::LsiKer)
    }
}
#[doc = "RNG kernel clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RNGSEL {
    #[doc = "0: HSI48 kernel clock selected as clock source (hsi48_ker_ck)"]
    Hsi48Ker = 0,
    #[doc = "1: PLL1 Q clock selected as clock source (pll1_q_ck)"]
    Pll1Q = 1,
    #[doc = "2: LSE clock selected as clock source (lse_ck)"]
    Lse = 2,
    #[doc = "3: LSI kernel clock selected as clock source (lsi_ker_ck)"]
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
#[doc = "Field `RNGSEL` reader - RNG kernel clock source selection"]
pub type RNGSEL_R = crate::FieldReader<RNGSEL>;
impl RNGSEL_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "HSI48 kernel clock selected as clock source (hsi48_ker_ck)"]
    #[inline(always)]
    pub fn is_hsi48_ker(&self) -> bool {
        *self == RNGSEL::Hsi48Ker
    }
    #[doc = "PLL1 Q clock selected as clock source (pll1_q_ck)"]
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == RNGSEL::Pll1Q
    }
    #[doc = "LSE clock selected as clock source (lse_ck)"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == RNGSEL::Lse
    }
    #[doc = "LSI kernel clock selected as clock source (lsi_ker_ck)"]
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == RNGSEL::Lsi
    }
}
#[doc = "Field `RNGSEL` writer - RNG kernel clock source selection"]
pub type RNGSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, RNGSEL>;
impl<'a, REG> RNGSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HSI48 kernel clock selected as clock source (hsi48_ker_ck)"]
    #[inline(always)]
    pub fn hsi48_ker(self) -> &'a mut crate::W<REG> {
        self.variant(RNGSEL::Hsi48Ker)
    }
    #[doc = "PLL1 Q clock selected as clock source (pll1_q_ck)"]
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut crate::W<REG> {
        self.variant(RNGSEL::Pll1Q)
    }
    #[doc = "LSE clock selected as clock source (lse_ck)"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(RNGSEL::Lse)
    }
    #[doc = "LSI kernel clock selected as clock source (lsi_ker_ck)"]
    #[inline(always)]
    pub fn lsi(self) -> &'a mut crate::W<REG> {
        self.variant(RNGSEL::Lsi)
    }
}
#[doc = "FDCAN1 kernel clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FDCANSEL {
    #[doc = "0: HSE clock selected as clock source (hse_ck)"]
    Hse = 0,
    #[doc = "1: PLL1 Q clock selected as clock source (pll1_q_ck)"]
    Pll1Q = 1,
    #[doc = "2: PLL2 Q clock selected as clock source (pll2_q_ck)"]
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
#[doc = "Field `FDCANSEL` reader - FDCAN1 kernel clock source selection"]
pub type FDCANSEL_R = crate::FieldReader<FDCANSEL>;
impl FDCANSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<FDCANSEL> {
        match self.bits {
            0 => Some(FDCANSEL::Hse),
            1 => Some(FDCANSEL::Pll1Q),
            2 => Some(FDCANSEL::Pll2Q),
            _ => None,
        }
    }
    #[doc = "HSE clock selected as clock source (hse_ck)"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == FDCANSEL::Hse
    }
    #[doc = "PLL1 Q clock selected as clock source (pll1_q_ck)"]
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == FDCANSEL::Pll1Q
    }
    #[doc = "PLL2 Q clock selected as clock source (pll2_q_ck)"]
    #[inline(always)]
    pub fn is_pll2_q(&self) -> bool {
        *self == FDCANSEL::Pll2Q
    }
}
#[doc = "Field `FDCANSEL` writer - FDCAN1 kernel clock source selection"]
pub type FDCANSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, FDCANSEL>;
impl<'a, REG> FDCANSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HSE clock selected as clock source (hse_ck)"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(FDCANSEL::Hse)
    }
    #[doc = "PLL1 Q clock selected as clock source (pll1_q_ck)"]
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut crate::W<REG> {
        self.variant(FDCANSEL::Pll1Q)
    }
    #[doc = "PLL2 Q clock selected as clock source (pll2_q_ck)"]
    #[inline(always)]
    pub fn pll2_q(self) -> &'a mut crate::W<REG> {
        self.variant(FDCANSEL::Pll2Q)
    }
}
#[doc = "per_ck clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKPERSEL {
    #[doc = "0: HSI kernel clock selected as clock source (hsi_ker_ck)"]
    HsiKer = 0,
    #[doc = "1: CSI kernel clock selected as clock source (csi_ker_ck)"]
    CsiKer = 1,
    #[doc = "2: HSE clock selected as clock source (hse_ck)"]
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
#[doc = "Field `CKPERSEL` reader - per_ck clock source selection"]
pub type CKPERSEL_R = crate::FieldReader<CKPERSEL>;
impl CKPERSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CKPERSEL> {
        match self.bits {
            0 => Some(CKPERSEL::HsiKer),
            1 => Some(CKPERSEL::CsiKer),
            2 => Some(CKPERSEL::Hse),
            _ => None,
        }
    }
    #[doc = "HSI kernel clock selected as clock source (hsi_ker_ck)"]
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == CKPERSEL::HsiKer
    }
    #[doc = "CSI kernel clock selected as clock source (csi_ker_ck)"]
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        *self == CKPERSEL::CsiKer
    }
    #[doc = "HSE clock selected as clock source (hse_ck)"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == CKPERSEL::Hse
    }
}
#[doc = "Field `CKPERSEL` writer - per_ck clock source selection"]
pub type CKPERSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CKPERSEL>;
impl<'a, REG> CKPERSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HSI kernel clock selected as clock source (hsi_ker_ck)"]
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(CKPERSEL::HsiKer)
    }
    #[doc = "CSI kernel clock selected as clock source (csi_ker_ck)"]
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(CKPERSEL::CsiKer)
    }
    #[doc = "HSE clock selected as clock source (hse_ck)"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(CKPERSEL::Hse)
    }
}
impl R {
    #[doc = "Bits 0:2 - ADC and DAC kernel clock source selection others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn adcdacsel(&self) -> ADCDACSEL_R {
        ADCDACSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - DAC hold clock"]
    #[inline(always)]
    pub fn dac1sel(&self) -> DAC1SEL_R {
        DAC1SEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - RNG kernel clock source selection"]
    #[inline(always)]
    pub fn rngsel(&self) -> RNGSEL_R {
        RNGSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - FDCAN1 kernel clock source selection"]
    #[inline(always)]
    pub fn fdcansel(&self) -> FDCANSEL_R {
        FDCANSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 30:31 - per_ck clock source selection"]
    #[inline(always)]
    pub fn ckpersel(&self) -> CKPERSEL_R {
        CKPERSEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - ADC and DAC kernel clock source selection others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn adcdacsel(&mut self) -> ADCDACSEL_W<CCIPR5rs> {
        ADCDACSEL_W::new(self, 0)
    }
    #[doc = "Bit 3 - DAC hold clock"]
    #[inline(always)]
    #[must_use]
    pub fn dac1sel(&mut self) -> DAC1SEL_W<CCIPR5rs> {
        DAC1SEL_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - RNG kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn rngsel(&mut self) -> RNGSEL_W<CCIPR5rs> {
        RNGSEL_W::new(self, 4)
    }
    #[doc = "Bits 8:9 - FDCAN1 kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn fdcansel(&mut self) -> FDCANSEL_W<CCIPR5rs> {
        FDCANSEL_W::new(self, 8)
    }
    #[doc = "Bits 30:31 - per_ck clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn ckpersel(&mut self) -> CKPERSEL_W<CCIPR5rs> {
        CKPERSEL_W::new(self, 30)
    }
}
#[doc = "RCC kernel clock configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccipr5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccipr5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCIPR5rs;
impl crate::RegisterSpec for CCIPR5rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccipr5::R`](R) reader structure"]
impl crate::Readable for CCIPR5rs {}
#[doc = "`write(|w| ..)` method takes [`ccipr5::W`](W) writer structure"]
impl crate::Writable for CCIPR5rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCIPR5 to value 0"]
impl crate::Resettable for CCIPR5rs {
    const RESET_VALUE: u32 = 0;
}

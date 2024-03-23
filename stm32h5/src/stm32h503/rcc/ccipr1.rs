#[doc = "Register `CCIPR1` reader"]
pub type R = crate::R<CCIPR1rs>;
#[doc = "Register `CCIPR1` writer"]
pub type W = crate::W<CCIPR1rs>;
#[doc = "USART1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USART1SEL {
    #[doc = "0: PCLK2 selected as clock source (rcc_pclk2)"]
    RccPclk2 = 0,
    #[doc = "1: PLL2 Q clock selected as clock source (pll2_q_ck)"]
    Pll2Q = 1,
    #[doc = "3: HSI kernel clock selected as clock source (hsi_ker_ck)"]
    HsiKer = 3,
    #[doc = "4: CSI kernel clock selected as clock source (csi_ker_ck)"]
    CsiKer = 4,
    #[doc = "5: LSE clock selected as clock source (lse_ck)"]
    Lse = 5,
}
impl From<USART1SEL> for u8 {
    #[inline(always)]
    fn from(variant: USART1SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USART1SEL {
    type Ux = u8;
}
#[doc = "Field `USART1SEL` reader - USART1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type USART1SEL_R = crate::FieldReader<USART1SEL>;
impl USART1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<USART1SEL> {
        match self.bits {
            0 => Some(USART1SEL::RccPclk2),
            1 => Some(USART1SEL::Pll2Q),
            3 => Some(USART1SEL::HsiKer),
            4 => Some(USART1SEL::CsiKer),
            5 => Some(USART1SEL::Lse),
            _ => None,
        }
    }
    #[doc = "PCLK2 selected as clock source (rcc_pclk2)"]
    #[inline(always)]
    pub fn is_rcc_pclk2(&self) -> bool {
        *self == USART1SEL::RccPclk2
    }
    #[doc = "PLL2 Q clock selected as clock source (pll2_q_ck)"]
    #[inline(always)]
    pub fn is_pll2_q(&self) -> bool {
        *self == USART1SEL::Pll2Q
    }
    #[doc = "HSI kernel clock selected as clock source (hsi_ker_ck)"]
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == USART1SEL::HsiKer
    }
    #[doc = "CSI kernel clock selected as clock source (csi_ker_ck)"]
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        *self == USART1SEL::CsiKer
    }
    #[doc = "LSE clock selected as clock source (lse_ck)"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == USART1SEL::Lse
    }
}
#[doc = "Field `USART1SEL` writer - USART1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type USART1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, USART1SEL>;
impl<'a, REG> USART1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCLK2 selected as clock source (rcc_pclk2)"]
    #[inline(always)]
    pub fn rcc_pclk2(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SEL::RccPclk2)
    }
    #[doc = "PLL2 Q clock selected as clock source (pll2_q_ck)"]
    #[inline(always)]
    pub fn pll2_q(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SEL::Pll2Q)
    }
    #[doc = "HSI kernel clock selected as clock source (hsi_ker_ck)"]
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SEL::HsiKer)
    }
    #[doc = "CSI kernel clock selected as clock source (csi_ker_ck)"]
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SEL::CsiKer)
    }
    #[doc = "LSE clock selected as clock source (lse_ck)"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SEL::Lse)
    }
}
#[doc = "USART2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USART2SEL {
    #[doc = "0: PCLK1 selected as clock source (rcc_pclk1)"]
    RccPclk1 = 0,
    #[doc = "1: PLL2 Q clock selected as clock source (pll2_q_ck)"]
    Pll2Q = 1,
    #[doc = "3: HSI kernel clock selected as clock source (hsi_ker_ck)"]
    HsiKer = 3,
    #[doc = "4: CSI kernel clock selected as clock source (csi_ker_ck)"]
    CsiKer = 4,
    #[doc = "5: LSE clock selected as clock source (lse_ck)"]
    Lse = 5,
}
impl From<USART2SEL> for u8 {
    #[inline(always)]
    fn from(variant: USART2SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USART2SEL {
    type Ux = u8;
}
#[doc = "Field `USART2SEL` reader - USART2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type USART2SEL_R = crate::FieldReader<USART2SEL>;
impl USART2SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<USART2SEL> {
        match self.bits {
            0 => Some(USART2SEL::RccPclk1),
            1 => Some(USART2SEL::Pll2Q),
            3 => Some(USART2SEL::HsiKer),
            4 => Some(USART2SEL::CsiKer),
            5 => Some(USART2SEL::Lse),
            _ => None,
        }
    }
    #[doc = "PCLK1 selected as clock source (rcc_pclk1)"]
    #[inline(always)]
    pub fn is_rcc_pclk1(&self) -> bool {
        *self == USART2SEL::RccPclk1
    }
    #[doc = "PLL2 Q clock selected as clock source (pll2_q_ck)"]
    #[inline(always)]
    pub fn is_pll2_q(&self) -> bool {
        *self == USART2SEL::Pll2Q
    }
    #[doc = "HSI kernel clock selected as clock source (hsi_ker_ck)"]
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == USART2SEL::HsiKer
    }
    #[doc = "CSI kernel clock selected as clock source (csi_ker_ck)"]
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        *self == USART2SEL::CsiKer
    }
    #[doc = "LSE clock selected as clock source (lse_ck)"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == USART2SEL::Lse
    }
}
#[doc = "Field `USART2SEL` writer - USART2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type USART2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, USART2SEL>;
impl<'a, REG> USART2SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCLK1 selected as clock source (rcc_pclk1)"]
    #[inline(always)]
    pub fn rcc_pclk1(self) -> &'a mut crate::W<REG> {
        self.variant(USART2SEL::RccPclk1)
    }
    #[doc = "PLL2 Q clock selected as clock source (pll2_q_ck)"]
    #[inline(always)]
    pub fn pll2_q(self) -> &'a mut crate::W<REG> {
        self.variant(USART2SEL::Pll2Q)
    }
    #[doc = "HSI kernel clock selected as clock source (hsi_ker_ck)"]
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(USART2SEL::HsiKer)
    }
    #[doc = "CSI kernel clock selected as clock source (csi_ker_ck)"]
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(USART2SEL::CsiKer)
    }
    #[doc = "LSE clock selected as clock source (lse_ck)"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(USART2SEL::Lse)
    }
}
#[doc = "USART3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USART3SEL {
    #[doc = "0: PCLK1 selected as clock source (rcc_pclk1)"]
    RccPclk1 = 0,
    #[doc = "1: PLL2 Q clock selected as clock source (pll2_q_ck)"]
    Pll2Q = 1,
    #[doc = "3: HSI kernel clock selected as clock source (hsi_ker_ck)"]
    HsiKer = 3,
    #[doc = "4: CSI kernel clock selected as clock source (csi_ker_ck)"]
    CsiKer = 4,
    #[doc = "5: LSE clock selected as clock source (lse_ck)"]
    Lse = 5,
}
impl From<USART3SEL> for u8 {
    #[inline(always)]
    fn from(variant: USART3SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USART3SEL {
    type Ux = u8;
}
#[doc = "Field `USART3SEL` reader - USART3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type USART3SEL_R = crate::FieldReader<USART3SEL>;
impl USART3SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<USART3SEL> {
        match self.bits {
            0 => Some(USART3SEL::RccPclk1),
            1 => Some(USART3SEL::Pll2Q),
            3 => Some(USART3SEL::HsiKer),
            4 => Some(USART3SEL::CsiKer),
            5 => Some(USART3SEL::Lse),
            _ => None,
        }
    }
    #[doc = "PCLK1 selected as clock source (rcc_pclk1)"]
    #[inline(always)]
    pub fn is_rcc_pclk1(&self) -> bool {
        *self == USART3SEL::RccPclk1
    }
    #[doc = "PLL2 Q clock selected as clock source (pll2_q_ck)"]
    #[inline(always)]
    pub fn is_pll2_q(&self) -> bool {
        *self == USART3SEL::Pll2Q
    }
    #[doc = "HSI kernel clock selected as clock source (hsi_ker_ck)"]
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == USART3SEL::HsiKer
    }
    #[doc = "CSI kernel clock selected as clock source (csi_ker_ck)"]
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        *self == USART3SEL::CsiKer
    }
    #[doc = "LSE clock selected as clock source (lse_ck)"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == USART3SEL::Lse
    }
}
#[doc = "Field `USART3SEL` writer - USART3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type USART3SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, USART3SEL>;
impl<'a, REG> USART3SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCLK1 selected as clock source (rcc_pclk1)"]
    #[inline(always)]
    pub fn rcc_pclk1(self) -> &'a mut crate::W<REG> {
        self.variant(USART3SEL::RccPclk1)
    }
    #[doc = "PLL2 Q clock selected as clock source (pll2_q_ck)"]
    #[inline(always)]
    pub fn pll2_q(self) -> &'a mut crate::W<REG> {
        self.variant(USART3SEL::Pll2Q)
    }
    #[doc = "HSI kernel clock selected as clock source (hsi_ker_ck)"]
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(USART3SEL::HsiKer)
    }
    #[doc = "CSI kernel clock selected as clock source (csi_ker_ck)"]
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(USART3SEL::CsiKer)
    }
    #[doc = "LSE clock selected as clock source (lse_ck)"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(USART3SEL::Lse)
    }
}
#[doc = "TIM2, TIM3 and LPTIM2 input capture source selection Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMICSEL {
    #[doc = "0: No internal clock available for timers input capture"]
    Disabled = 0,
    #[doc = "1: hsi_ker_ck/1024, hsi_ker_ck/8 and csi_ker_ck/128 selected for timers input capture"]
    Enabled = 1,
}
impl From<TIMICSEL> for bool {
    #[inline(always)]
    fn from(variant: TIMICSEL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMICSEL` reader - TIM2, TIM3 and LPTIM2 input capture source selection Set and reset by software."]
pub type TIMICSEL_R = crate::BitReader<TIMICSEL>;
impl TIMICSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIMICSEL {
        match self.bits {
            false => TIMICSEL::Disabled,
            true => TIMICSEL::Enabled,
        }
    }
    #[doc = "No internal clock available for timers input capture"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIMICSEL::Disabled
    }
    #[doc = "hsi_ker_ck/1024, hsi_ker_ck/8 and csi_ker_ck/128 selected for timers input capture"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIMICSEL::Enabled
    }
}
#[doc = "Field `TIMICSEL` writer - TIM2, TIM3 and LPTIM2 input capture source selection Set and reset by software."]
pub type TIMICSEL_W<'a, REG> = crate::BitWriter<'a, REG, TIMICSEL>;
impl<'a, REG> TIMICSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No internal clock available for timers input capture"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIMICSEL::Disabled)
    }
    #[doc = "hsi_ker_ck/1024, hsi_ker_ck/8 and csi_ker_ck/128 selected for timers input capture"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIMICSEL::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:2 - USART1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn usart1sel(&self) -> USART1SEL_R {
        USART1SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - USART2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn usart2sel(&self) -> USART2SEL_R {
        USART2SEL_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - USART3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn usart3sel(&self) -> USART3SEL_R {
        USART3SEL_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bit 31 - TIM2, TIM3 and LPTIM2 input capture source selection Set and reset by software."]
    #[inline(always)]
    pub fn timicsel(&self) -> TIMICSEL_R {
        TIMICSEL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - USART1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn usart1sel(&mut self) -> USART1SEL_W<CCIPR1rs> {
        USART1SEL_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - USART2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn usart2sel(&mut self) -> USART2SEL_W<CCIPR1rs> {
        USART2SEL_W::new(self, 3)
    }
    #[doc = "Bits 6:8 - USART3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn usart3sel(&mut self) -> USART3SEL_W<CCIPR1rs> {
        USART3SEL_W::new(self, 6)
    }
    #[doc = "Bit 31 - TIM2, TIM3 and LPTIM2 input capture source selection Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn timicsel(&mut self) -> TIMICSEL_W<CCIPR1rs> {
        TIMICSEL_W::new(self, 31)
    }
}
#[doc = "RCC kernel clock configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccipr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccipr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCIPR1rs;
impl crate::RegisterSpec for CCIPR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccipr1::R`](R) reader structure"]
impl crate::Readable for CCIPR1rs {}
#[doc = "`write(|w| ..)` method takes [`ccipr1::W`](W) writer structure"]
impl crate::Writable for CCIPR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCIPR1 to value 0"]
impl crate::Resettable for CCIPR1rs {
    const RESET_VALUE: u32 = 0;
}

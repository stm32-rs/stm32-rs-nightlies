///Register `CCIPR2` reader
pub type R = crate::R<CCIPR2rs>;
///Register `CCIPR2` writer
pub type W = crate::W<CCIPR2rs>;
/**LPTIM1 kernel clock source selection others: reserved, the kernel clock is disabled

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPTIM1SEL {
    ///0: PCLK3 selected as clock source (rcc_pclk3)
    RccPclk3 = 0,
    ///1: PLL2 P clock selected as clock source (pll2_p_ck)
    Pll2P = 1,
    ///3: LSE kernel selected as clock source (lse_ck)
    LseKer = 3,
    ///4: LSI kernel selected as clock source (lsi_ker_ck)
    LsiKer = 4,
    ///5: per_ck clock selected as clock source
    PerCk = 5,
}
impl From<LPTIM1SEL> for u8 {
    #[inline(always)]
    fn from(variant: LPTIM1SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LPTIM1SEL {
    type Ux = u8;
}
impl crate::IsEnum for LPTIM1SEL {}
///Field `LPTIM1SEL` reader - LPTIM1 kernel clock source selection others: reserved, the kernel clock is disabled
pub type LPTIM1SEL_R = crate::FieldReader<LPTIM1SEL>;
impl LPTIM1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<LPTIM1SEL> {
        match self.bits {
            0 => Some(LPTIM1SEL::RccPclk3),
            1 => Some(LPTIM1SEL::Pll2P),
            3 => Some(LPTIM1SEL::LseKer),
            4 => Some(LPTIM1SEL::LsiKer),
            5 => Some(LPTIM1SEL::PerCk),
            _ => None,
        }
    }
    ///PCLK3 selected as clock source (rcc_pclk3)
    #[inline(always)]
    pub fn is_rcc_pclk3(&self) -> bool {
        *self == LPTIM1SEL::RccPclk3
    }
    ///PLL2 P clock selected as clock source (pll2_p_ck)
    #[inline(always)]
    pub fn is_pll2_p(&self) -> bool {
        *self == LPTIM1SEL::Pll2P
    }
    ///LSE kernel selected as clock source (lse_ck)
    #[inline(always)]
    pub fn is_lse_ker(&self) -> bool {
        *self == LPTIM1SEL::LseKer
    }
    ///LSI kernel selected as clock source (lsi_ker_ck)
    #[inline(always)]
    pub fn is_lsi_ker(&self) -> bool {
        *self == LPTIM1SEL::LsiKer
    }
    ///per_ck clock selected as clock source
    #[inline(always)]
    pub fn is_per_ck(&self) -> bool {
        *self == LPTIM1SEL::PerCk
    }
}
///Field `LPTIM1SEL` writer - LPTIM1 kernel clock source selection others: reserved, the kernel clock is disabled
pub type LPTIM1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, LPTIM1SEL>;
impl<'a, REG> LPTIM1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PCLK3 selected as clock source (rcc_pclk3)
    #[inline(always)]
    pub fn rcc_pclk3(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::RccPclk3)
    }
    ///PLL2 P clock selected as clock source (pll2_p_ck)
    #[inline(always)]
    pub fn pll2_p(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::Pll2P)
    }
    ///LSE kernel selected as clock source (lse_ck)
    #[inline(always)]
    pub fn lse_ker(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::LseKer)
    }
    ///LSI kernel selected as clock source (lsi_ker_ck)
    #[inline(always)]
    pub fn lsi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::LsiKer)
    }
    ///per_ck clock selected as clock source
    #[inline(always)]
    pub fn per_ck(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::PerCk)
    }
}
/**LPTIM2 kernel clock source selection others: reserved, the kernel clock is disabled

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPTIM2SEL {
    ///0: PCLK1 selected as clock source (rcc_pclk1)
    RccPclk1 = 0,
    ///1: PLL2 P clock selected as clock source (pll2_p_ck)
    Pll2P = 1,
    ///3: LSE kernel selected as clock source (lse_ck)
    LseKer = 3,
    ///4: LSI kernel selected as clock source (lsi_ker_ck)
    LsiKer = 4,
    ///5: per_ck clock selected as clock source
    PerCk = 5,
}
impl From<LPTIM2SEL> for u8 {
    #[inline(always)]
    fn from(variant: LPTIM2SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LPTIM2SEL {
    type Ux = u8;
}
impl crate::IsEnum for LPTIM2SEL {}
///Field `LPTIM2SEL` reader - LPTIM2 kernel clock source selection others: reserved, the kernel clock is disabled
pub type LPTIM2SEL_R = crate::FieldReader<LPTIM2SEL>;
impl LPTIM2SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<LPTIM2SEL> {
        match self.bits {
            0 => Some(LPTIM2SEL::RccPclk1),
            1 => Some(LPTIM2SEL::Pll2P),
            3 => Some(LPTIM2SEL::LseKer),
            4 => Some(LPTIM2SEL::LsiKer),
            5 => Some(LPTIM2SEL::PerCk),
            _ => None,
        }
    }
    ///PCLK1 selected as clock source (rcc_pclk1)
    #[inline(always)]
    pub fn is_rcc_pclk1(&self) -> bool {
        *self == LPTIM2SEL::RccPclk1
    }
    ///PLL2 P clock selected as clock source (pll2_p_ck)
    #[inline(always)]
    pub fn is_pll2_p(&self) -> bool {
        *self == LPTIM2SEL::Pll2P
    }
    ///LSE kernel selected as clock source (lse_ck)
    #[inline(always)]
    pub fn is_lse_ker(&self) -> bool {
        *self == LPTIM2SEL::LseKer
    }
    ///LSI kernel selected as clock source (lsi_ker_ck)
    #[inline(always)]
    pub fn is_lsi_ker(&self) -> bool {
        *self == LPTIM2SEL::LsiKer
    }
    ///per_ck clock selected as clock source
    #[inline(always)]
    pub fn is_per_ck(&self) -> bool {
        *self == LPTIM2SEL::PerCk
    }
}
///Field `LPTIM2SEL` writer - LPTIM2 kernel clock source selection others: reserved, the kernel clock is disabled
pub type LPTIM2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, LPTIM2SEL>;
impl<'a, REG> LPTIM2SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PCLK1 selected as clock source (rcc_pclk1)
    #[inline(always)]
    pub fn rcc_pclk1(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM2SEL::RccPclk1)
    }
    ///PLL2 P clock selected as clock source (pll2_p_ck)
    #[inline(always)]
    pub fn pll2_p(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM2SEL::Pll2P)
    }
    ///LSE kernel selected as clock source (lse_ck)
    #[inline(always)]
    pub fn lse_ker(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM2SEL::LseKer)
    }
    ///LSI kernel selected as clock source (lsi_ker_ck)
    #[inline(always)]
    pub fn lsi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM2SEL::LsiKer)
    }
    ///per_ck clock selected as clock source
    #[inline(always)]
    pub fn per_ck(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM2SEL::PerCk)
    }
}
impl R {
    ///Bits 8:10 - LPTIM1 kernel clock source selection others: reserved, the kernel clock is disabled
    #[inline(always)]
    pub fn lptim1sel(&self) -> LPTIM1SEL_R {
        LPTIM1SEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 12:14 - LPTIM2 kernel clock source selection others: reserved, the kernel clock is disabled
    #[inline(always)]
    pub fn lptim2sel(&self) -> LPTIM2SEL_R {
        LPTIM2SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCIPR2")
            .field("lptim1sel", &self.lptim1sel())
            .field("lptim2sel", &self.lptim2sel())
            .finish()
    }
}
impl W {
    ///Bits 8:10 - LPTIM1 kernel clock source selection others: reserved, the kernel clock is disabled
    #[inline(always)]
    #[must_use]
    pub fn lptim1sel(&mut self) -> LPTIM1SEL_W<CCIPR2rs> {
        LPTIM1SEL_W::new(self, 8)
    }
    ///Bits 12:14 - LPTIM2 kernel clock source selection others: reserved, the kernel clock is disabled
    #[inline(always)]
    #[must_use]
    pub fn lptim2sel(&mut self) -> LPTIM2SEL_W<CCIPR2rs> {
        LPTIM2SEL_W::new(self, 12)
    }
}
/**RCC kernel clock configuration register

You can [`read`](crate::Reg::read) this register and get [`ccipr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#RCC:CCIPR2)*/
pub struct CCIPR2rs;
impl crate::RegisterSpec for CCIPR2rs {
    type Ux = u32;
}
///`read()` method returns [`ccipr2::R`](R) reader structure
impl crate::Readable for CCIPR2rs {}
///`write(|w| ..)` method takes [`ccipr2::W`](W) writer structure
impl crate::Writable for CCIPR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CCIPR2 to value 0
impl crate::Resettable for CCIPR2rs {
    const RESET_VALUE: u32 = 0;
}

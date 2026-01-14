///Register `CCIPR4` reader
pub type R = crate::R<CCIPR4rs>;
///Register `CCIPR4` writer
pub type W = crate::W<CCIPR4rs>;
/**LPUART1 kernel clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPUART1SEL {
    ///0: pclk4 selected as peripheral clock
    Pclk4 = 0,
    ///1: pll2_q selected as peripheral clock
    Pll2Q = 1,
    ///2: pll3_q selected as peripheral clock
    Pll3Q = 2,
    ///3: hsi_ker selected as peripheral clock
    HsiKer = 3,
    ///4: csi_ker selected as peripheral clock
    CsiKer = 4,
    ///5: lse selected as peripheral clock
    Lse = 5,
}
impl From<LPUART1SEL> for u8 {
    #[inline(always)]
    fn from(variant: LPUART1SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LPUART1SEL {
    type Ux = u8;
}
impl crate::IsEnum for LPUART1SEL {}
///Field `LPUART1SEL` reader - LPUART1 kernel clock source selection
pub type LPUART1SEL_R = crate::FieldReader<LPUART1SEL>;
impl LPUART1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<LPUART1SEL> {
        match self.bits {
            0 => Some(LPUART1SEL::Pclk4),
            1 => Some(LPUART1SEL::Pll2Q),
            2 => Some(LPUART1SEL::Pll3Q),
            3 => Some(LPUART1SEL::HsiKer),
            4 => Some(LPUART1SEL::CsiKer),
            5 => Some(LPUART1SEL::Lse),
            _ => None,
        }
    }
    ///pclk4 selected as peripheral clock
    #[inline(always)]
    pub fn is_pclk4(&self) -> bool {
        *self == LPUART1SEL::Pclk4
    }
    ///pll2_q selected as peripheral clock
    #[inline(always)]
    pub fn is_pll2_q(&self) -> bool {
        *self == LPUART1SEL::Pll2Q
    }
    ///pll3_q selected as peripheral clock
    #[inline(always)]
    pub fn is_pll3_q(&self) -> bool {
        *self == LPUART1SEL::Pll3Q
    }
    ///hsi_ker selected as peripheral clock
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == LPUART1SEL::HsiKer
    }
    ///csi_ker selected as peripheral clock
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        *self == LPUART1SEL::CsiKer
    }
    ///lse selected as peripheral clock
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == LPUART1SEL::Lse
    }
}
///Field `LPUART1SEL` writer - LPUART1 kernel clock source selection
pub type LPUART1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, LPUART1SEL>;
impl<'a, REG> LPUART1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///pclk4 selected as peripheral clock
    #[inline(always)]
    pub fn pclk4(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1SEL::Pclk4)
    }
    ///pll2_q selected as peripheral clock
    #[inline(always)]
    pub fn pll2_q(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1SEL::Pll2Q)
    }
    ///pll3_q selected as peripheral clock
    #[inline(always)]
    pub fn pll3_q(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1SEL::Pll3Q)
    }
    ///hsi_ker selected as peripheral clock
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1SEL::HsiKer)
    }
    ///csi_ker selected as peripheral clock
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1SEL::CsiKer)
    }
    ///lse selected as peripheral clock
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1SEL::Lse)
    }
}
/**SPI/I2S6 kernel clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPI6SEL {
    ///0: pclk4 selected as peripheral clock
    Pclk4 = 0,
    ///1: pll2_q selected as peripheral clock
    Pll2Q = 1,
    ///2: pll3_q selected as peripheral clock
    Pll3Q = 2,
    ///3: hsi_ker selected as peripheral clock
    HsiKer = 3,
    ///4: csi_ker selected as peripheral clock
    CsiKer = 4,
    ///5: hse_ker selected as peripheral clock
    HseKer = 5,
}
impl From<SPI6SEL> for u8 {
    #[inline(always)]
    fn from(variant: SPI6SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SPI6SEL {
    type Ux = u8;
}
impl crate::IsEnum for SPI6SEL {}
///Field `SPI6SEL` reader - SPI/I2S6 kernel clock source selection
pub type SPI6SEL_R = crate::FieldReader<SPI6SEL>;
impl SPI6SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SPI6SEL> {
        match self.bits {
            0 => Some(SPI6SEL::Pclk4),
            1 => Some(SPI6SEL::Pll2Q),
            2 => Some(SPI6SEL::Pll3Q),
            3 => Some(SPI6SEL::HsiKer),
            4 => Some(SPI6SEL::CsiKer),
            5 => Some(SPI6SEL::HseKer),
            _ => None,
        }
    }
    ///pclk4 selected as peripheral clock
    #[inline(always)]
    pub fn is_pclk4(&self) -> bool {
        *self == SPI6SEL::Pclk4
    }
    ///pll2_q selected as peripheral clock
    #[inline(always)]
    pub fn is_pll2_q(&self) -> bool {
        *self == SPI6SEL::Pll2Q
    }
    ///pll3_q selected as peripheral clock
    #[inline(always)]
    pub fn is_pll3_q(&self) -> bool {
        *self == SPI6SEL::Pll3Q
    }
    ///hsi_ker selected as peripheral clock
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == SPI6SEL::HsiKer
    }
    ///csi_ker selected as peripheral clock
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        *self == SPI6SEL::CsiKer
    }
    ///hse_ker selected as peripheral clock
    #[inline(always)]
    pub fn is_hse_ker(&self) -> bool {
        *self == SPI6SEL::HseKer
    }
}
///Field `SPI6SEL` writer - SPI/I2S6 kernel clock source selection
pub type SPI6SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SPI6SEL>;
impl<'a, REG> SPI6SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///pclk4 selected as peripheral clock
    #[inline(always)]
    pub fn pclk4(self) -> &'a mut crate::W<REG> {
        self.variant(SPI6SEL::Pclk4)
    }
    ///pll2_q selected as peripheral clock
    #[inline(always)]
    pub fn pll2_q(self) -> &'a mut crate::W<REG> {
        self.variant(SPI6SEL::Pll2Q)
    }
    ///pll3_q selected as peripheral clock
    #[inline(always)]
    pub fn pll3_q(self) -> &'a mut crate::W<REG> {
        self.variant(SPI6SEL::Pll3Q)
    }
    ///hsi_ker selected as peripheral clock
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(SPI6SEL::HsiKer)
    }
    ///csi_ker selected as peripheral clock
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(SPI6SEL::CsiKer)
    }
    ///hse_ker selected as peripheral clock
    #[inline(always)]
    pub fn hse_ker(self) -> &'a mut crate::W<REG> {
        self.variant(SPI6SEL::HseKer)
    }
}
/**LPTIM2 and LPTIM3 kernel clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPTIM23SEL {
    ///0: pclk4 selected as peripheral clock
    Pclk4 = 0,
    ///1: pll2_p selected as peripheral clock
    Pll2P = 1,
    ///2: pll3_r selected as peripheral clock
    Pll3R = 2,
    ///3: lse selected as peripheral clock
    Lse = 3,
    ///4: lsi selected as peripheral clock
    Lsi = 4,
    ///5: per selected as peripheral clock
    Per = 5,
}
impl From<LPTIM23SEL> for u8 {
    #[inline(always)]
    fn from(variant: LPTIM23SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LPTIM23SEL {
    type Ux = u8;
}
impl crate::IsEnum for LPTIM23SEL {}
///Field `LPTIM23SEL` reader - LPTIM2 and LPTIM3 kernel clock source selection
pub type LPTIM23SEL_R = crate::FieldReader<LPTIM23SEL>;
impl LPTIM23SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<LPTIM23SEL> {
        match self.bits {
            0 => Some(LPTIM23SEL::Pclk4),
            1 => Some(LPTIM23SEL::Pll2P),
            2 => Some(LPTIM23SEL::Pll3R),
            3 => Some(LPTIM23SEL::Lse),
            4 => Some(LPTIM23SEL::Lsi),
            5 => Some(LPTIM23SEL::Per),
            _ => None,
        }
    }
    ///pclk4 selected as peripheral clock
    #[inline(always)]
    pub fn is_pclk4(&self) -> bool {
        *self == LPTIM23SEL::Pclk4
    }
    ///pll2_p selected as peripheral clock
    #[inline(always)]
    pub fn is_pll2_p(&self) -> bool {
        *self == LPTIM23SEL::Pll2P
    }
    ///pll3_r selected as peripheral clock
    #[inline(always)]
    pub fn is_pll3_r(&self) -> bool {
        *self == LPTIM23SEL::Pll3R
    }
    ///lse selected as peripheral clock
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == LPTIM23SEL::Lse
    }
    ///lsi selected as peripheral clock
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == LPTIM23SEL::Lsi
    }
    ///per selected as peripheral clock
    #[inline(always)]
    pub fn is_per(&self) -> bool {
        *self == LPTIM23SEL::Per
    }
}
///Field `LPTIM23SEL` writer - LPTIM2 and LPTIM3 kernel clock source selection
pub type LPTIM23SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, LPTIM23SEL>;
impl<'a, REG> LPTIM23SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///pclk4 selected as peripheral clock
    #[inline(always)]
    pub fn pclk4(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM23SEL::Pclk4)
    }
    ///pll2_p selected as peripheral clock
    #[inline(always)]
    pub fn pll2_p(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM23SEL::Pll2P)
    }
    ///pll3_r selected as peripheral clock
    #[inline(always)]
    pub fn pll3_r(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM23SEL::Pll3R)
    }
    ///lse selected as peripheral clock
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM23SEL::Lse)
    }
    ///lsi selected as peripheral clock
    #[inline(always)]
    pub fn lsi(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM23SEL::Lsi)
    }
    ///per selected as peripheral clock
    #[inline(always)]
    pub fn per(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM23SEL::Per)
    }
}
/**LPTIM4, and LPTIM5 kernel clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPTIM45SEL {
    ///0: pclk4 selected as peripheral clock
    Pclk4 = 0,
    ///1: pll2_p selected as peripheral clock
    Pll2P = 1,
    ///2: pll3_r selected as peripheral clock
    Pll3R = 2,
    ///3: lse selected as peripheral clock
    Lse = 3,
    ///4: lsi selected as peripheral clock
    Lsi = 4,
    ///5: per selected as peripheral clock
    Per = 5,
}
impl From<LPTIM45SEL> for u8 {
    #[inline(always)]
    fn from(variant: LPTIM45SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LPTIM45SEL {
    type Ux = u8;
}
impl crate::IsEnum for LPTIM45SEL {}
///Field `LPTIM45SEL` reader - LPTIM4, and LPTIM5 kernel clock source selection
pub type LPTIM45SEL_R = crate::FieldReader<LPTIM45SEL>;
impl LPTIM45SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<LPTIM45SEL> {
        match self.bits {
            0 => Some(LPTIM45SEL::Pclk4),
            1 => Some(LPTIM45SEL::Pll2P),
            2 => Some(LPTIM45SEL::Pll3R),
            3 => Some(LPTIM45SEL::Lse),
            4 => Some(LPTIM45SEL::Lsi),
            5 => Some(LPTIM45SEL::Per),
            _ => None,
        }
    }
    ///pclk4 selected as peripheral clock
    #[inline(always)]
    pub fn is_pclk4(&self) -> bool {
        *self == LPTIM45SEL::Pclk4
    }
    ///pll2_p selected as peripheral clock
    #[inline(always)]
    pub fn is_pll2_p(&self) -> bool {
        *self == LPTIM45SEL::Pll2P
    }
    ///pll3_r selected as peripheral clock
    #[inline(always)]
    pub fn is_pll3_r(&self) -> bool {
        *self == LPTIM45SEL::Pll3R
    }
    ///lse selected as peripheral clock
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == LPTIM45SEL::Lse
    }
    ///lsi selected as peripheral clock
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == LPTIM45SEL::Lsi
    }
    ///per selected as peripheral clock
    #[inline(always)]
    pub fn is_per(&self) -> bool {
        *self == LPTIM45SEL::Per
    }
}
///Field `LPTIM45SEL` writer - LPTIM4, and LPTIM5 kernel clock source selection
pub type LPTIM45SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, LPTIM45SEL>;
impl<'a, REG> LPTIM45SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///pclk4 selected as peripheral clock
    #[inline(always)]
    pub fn pclk4(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM45SEL::Pclk4)
    }
    ///pll2_p selected as peripheral clock
    #[inline(always)]
    pub fn pll2_p(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM45SEL::Pll2P)
    }
    ///pll3_r selected as peripheral clock
    #[inline(always)]
    pub fn pll3_r(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM45SEL::Pll3R)
    }
    ///lse selected as peripheral clock
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM45SEL::Lse)
    }
    ///lsi selected as peripheral clock
    #[inline(always)]
    pub fn lsi(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM45SEL::Lsi)
    }
    ///per selected as peripheral clock
    #[inline(always)]
    pub fn per(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM45SEL::Per)
    }
}
impl R {
    ///Bits 0:2 - LPUART1 kernel clock source selection
    #[inline(always)]
    pub fn lpuart1sel(&self) -> LPUART1SEL_R {
        LPUART1SEL_R::new((self.bits & 7) as u8)
    }
    ///Bits 4:6 - SPI/I2S6 kernel clock source selection
    #[inline(always)]
    pub fn spi6sel(&self) -> SPI6SEL_R {
        SPI6SEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:10 - LPTIM2 and LPTIM3 kernel clock source selection
    #[inline(always)]
    pub fn lptim23sel(&self) -> LPTIM23SEL_R {
        LPTIM23SEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 12:14 - LPTIM4, and LPTIM5 kernel clock source selection
    #[inline(always)]
    pub fn lptim45sel(&self) -> LPTIM45SEL_R {
        LPTIM45SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCIPR4")
            .field("lpuart1sel", &self.lpuart1sel())
            .field("spi6sel", &self.spi6sel())
            .field("lptim23sel", &self.lptim23sel())
            .field("lptim45sel", &self.lptim45sel())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - LPUART1 kernel clock source selection
    #[inline(always)]
    pub fn lpuart1sel(&mut self) -> LPUART1SEL_W<'_, CCIPR4rs> {
        LPUART1SEL_W::new(self, 0)
    }
    ///Bits 4:6 - SPI/I2S6 kernel clock source selection
    #[inline(always)]
    pub fn spi6sel(&mut self) -> SPI6SEL_W<'_, CCIPR4rs> {
        SPI6SEL_W::new(self, 4)
    }
    ///Bits 8:10 - LPTIM2 and LPTIM3 kernel clock source selection
    #[inline(always)]
    pub fn lptim23sel(&mut self) -> LPTIM23SEL_W<'_, CCIPR4rs> {
        LPTIM23SEL_W::new(self, 8)
    }
    ///Bits 12:14 - LPTIM4, and LPTIM5 kernel clock source selection
    #[inline(always)]
    pub fn lptim45sel(&mut self) -> LPTIM45SEL_W<'_, CCIPR4rs> {
        LPTIM45SEL_W::new(self, 12)
    }
}
/**RCC APB4,5 peripherals kernel clock selection register

You can [`read`](crate::Reg::read) this register and get [`ccipr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:CCIPR4)*/
pub struct CCIPR4rs;
impl crate::RegisterSpec for CCIPR4rs {
    type Ux = u32;
}
///`read()` method returns [`ccipr4::R`](R) reader structure
impl crate::Readable for CCIPR4rs {}
///`write(|w| ..)` method takes [`ccipr4::W`](W) writer structure
impl crate::Writable for CCIPR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCIPR4 to value 0
impl crate::Resettable for CCIPR4rs {}

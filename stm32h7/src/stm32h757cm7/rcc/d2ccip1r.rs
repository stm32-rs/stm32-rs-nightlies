///Register `D2CCIP1R` reader
pub type R = crate::R<D2CCIP1Rrs>;
///Register `D2CCIP1R` writer
pub type W = crate::W<D2CCIP1Rrs>;
/**SAI1 and DFSDM1 kernel Aclk clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SAI1SEL {
    ///0: pll1_q selected as peripheral clock
    Pll1Q = 0,
    ///1: pll2_p selected as peripheral clock
    Pll2P = 1,
    ///2: pll3_p selected as peripheral clock
    Pll3P = 2,
    ///3: I2S_CKIN selected as peripheral clock
    I2sCkin = 3,
    ///4: PER selected as peripheral clock
    Per = 4,
}
impl From<SAI1SEL> for u8 {
    #[inline(always)]
    fn from(variant: SAI1SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SAI1SEL {
    type Ux = u8;
}
impl crate::IsEnum for SAI1SEL {}
///Field `SAI1SEL` reader - SAI1 and DFSDM1 kernel Aclk clock source selection
pub type SAI1SEL_R = crate::FieldReader<SAI1SEL>;
impl SAI1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SAI1SEL> {
        match self.bits {
            0 => Some(SAI1SEL::Pll1Q),
            1 => Some(SAI1SEL::Pll2P),
            2 => Some(SAI1SEL::Pll3P),
            3 => Some(SAI1SEL::I2sCkin),
            4 => Some(SAI1SEL::Per),
            _ => None,
        }
    }
    ///pll1_q selected as peripheral clock
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == SAI1SEL::Pll1Q
    }
    ///pll2_p selected as peripheral clock
    #[inline(always)]
    pub fn is_pll2_p(&self) -> bool {
        *self == SAI1SEL::Pll2P
    }
    ///pll3_p selected as peripheral clock
    #[inline(always)]
    pub fn is_pll3_p(&self) -> bool {
        *self == SAI1SEL::Pll3P
    }
    ///I2S_CKIN selected as peripheral clock
    #[inline(always)]
    pub fn is_i2s_ckin(&self) -> bool {
        *self == SAI1SEL::I2sCkin
    }
    ///PER selected as peripheral clock
    #[inline(always)]
    pub fn is_per(&self) -> bool {
        *self == SAI1SEL::Per
    }
}
///Field `SAI1SEL` writer - SAI1 and DFSDM1 kernel Aclk clock source selection
pub type SAI1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SAI1SEL>;
impl<'a, REG> SAI1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///pll1_q selected as peripheral clock
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut crate::W<REG> {
        self.variant(SAI1SEL::Pll1Q)
    }
    ///pll2_p selected as peripheral clock
    #[inline(always)]
    pub fn pll2_p(self) -> &'a mut crate::W<REG> {
        self.variant(SAI1SEL::Pll2P)
    }
    ///pll3_p selected as peripheral clock
    #[inline(always)]
    pub fn pll3_p(self) -> &'a mut crate::W<REG> {
        self.variant(SAI1SEL::Pll3P)
    }
    ///I2S_CKIN selected as peripheral clock
    #[inline(always)]
    pub fn i2s_ckin(self) -> &'a mut crate::W<REG> {
        self.variant(SAI1SEL::I2sCkin)
    }
    ///PER selected as peripheral clock
    #[inline(always)]
    pub fn per(self) -> &'a mut crate::W<REG> {
        self.variant(SAI1SEL::Per)
    }
}
///Field `SAI23SEL` reader - SAI2 and SAI3 kernel clock source selection
pub use SAI1SEL_R as SAI23SEL_R;
///Field `SPI123SEL` reader - SPI/I2S1,2 and 3 kernel clock source selection
pub use SAI1SEL_R as SPI123SEL_R;
///Field `SAI23SEL` writer - SAI2 and SAI3 kernel clock source selection
pub use SAI1SEL_W as SAI23SEL_W;
///Field `SPI123SEL` writer - SPI/I2S1,2 and 3 kernel clock source selection
pub use SAI1SEL_W as SPI123SEL_W;
/**SPI4 and 5 kernel clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPI45SEL {
    ///0: APB clock selected as peripheral clock
    Apb = 0,
    ///1: pll2_q selected as peripheral clock
    Pll2Q = 1,
    ///2: pll3_q selected as peripheral clock
    Pll3Q = 2,
    ///3: hsi_ker selected as peripheral clock
    HsiKer = 3,
    ///4: csi_ker selected as peripheral clock
    CsiKer = 4,
    ///5: HSE selected as peripheral clock
    Hse = 5,
}
impl From<SPI45SEL> for u8 {
    #[inline(always)]
    fn from(variant: SPI45SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SPI45SEL {
    type Ux = u8;
}
impl crate::IsEnum for SPI45SEL {}
///Field `SPI45SEL` reader - SPI4 and 5 kernel clock source selection
pub type SPI45SEL_R = crate::FieldReader<SPI45SEL>;
impl SPI45SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SPI45SEL> {
        match self.bits {
            0 => Some(SPI45SEL::Apb),
            1 => Some(SPI45SEL::Pll2Q),
            2 => Some(SPI45SEL::Pll3Q),
            3 => Some(SPI45SEL::HsiKer),
            4 => Some(SPI45SEL::CsiKer),
            5 => Some(SPI45SEL::Hse),
            _ => None,
        }
    }
    ///APB clock selected as peripheral clock
    #[inline(always)]
    pub fn is_apb(&self) -> bool {
        *self == SPI45SEL::Apb
    }
    ///pll2_q selected as peripheral clock
    #[inline(always)]
    pub fn is_pll2_q(&self) -> bool {
        *self == SPI45SEL::Pll2Q
    }
    ///pll3_q selected as peripheral clock
    #[inline(always)]
    pub fn is_pll3_q(&self) -> bool {
        *self == SPI45SEL::Pll3Q
    }
    ///hsi_ker selected as peripheral clock
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == SPI45SEL::HsiKer
    }
    ///csi_ker selected as peripheral clock
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        *self == SPI45SEL::CsiKer
    }
    ///HSE selected as peripheral clock
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == SPI45SEL::Hse
    }
}
///Field `SPI45SEL` writer - SPI4 and 5 kernel clock source selection
pub type SPI45SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SPI45SEL>;
impl<'a, REG> SPI45SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///APB clock selected as peripheral clock
    #[inline(always)]
    pub fn apb(self) -> &'a mut crate::W<REG> {
        self.variant(SPI45SEL::Apb)
    }
    ///pll2_q selected as peripheral clock
    #[inline(always)]
    pub fn pll2_q(self) -> &'a mut crate::W<REG> {
        self.variant(SPI45SEL::Pll2Q)
    }
    ///pll3_q selected as peripheral clock
    #[inline(always)]
    pub fn pll3_q(self) -> &'a mut crate::W<REG> {
        self.variant(SPI45SEL::Pll3Q)
    }
    ///hsi_ker selected as peripheral clock
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(SPI45SEL::HsiKer)
    }
    ///csi_ker selected as peripheral clock
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(SPI45SEL::CsiKer)
    }
    ///HSE selected as peripheral clock
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(SPI45SEL::Hse)
    }
}
/**SPDIFRX kernel clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPDIFSEL {
    ///0: pll1_q selected as peripheral clock
    Pll1Q = 0,
    ///1: pll2_r selected as peripheral clock
    Pll2R = 1,
    ///2: pll3_r selected as peripheral clock
    Pll3R = 2,
    ///3: hsi_ker selected as peripheral clock
    HsiKer = 3,
}
impl From<SPDIFSEL> for u8 {
    #[inline(always)]
    fn from(variant: SPDIFSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SPDIFSEL {
    type Ux = u8;
}
impl crate::IsEnum for SPDIFSEL {}
///Field `SPDIFSEL` reader - SPDIFRX kernel clock source selection
pub type SPDIFSEL_R = crate::FieldReader<SPDIFSEL>;
impl SPDIFSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPDIFSEL {
        match self.bits {
            0 => SPDIFSEL::Pll1Q,
            1 => SPDIFSEL::Pll2R,
            2 => SPDIFSEL::Pll3R,
            3 => SPDIFSEL::HsiKer,
            _ => unreachable!(),
        }
    }
    ///pll1_q selected as peripheral clock
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == SPDIFSEL::Pll1Q
    }
    ///pll2_r selected as peripheral clock
    #[inline(always)]
    pub fn is_pll2_r(&self) -> bool {
        *self == SPDIFSEL::Pll2R
    }
    ///pll3_r selected as peripheral clock
    #[inline(always)]
    pub fn is_pll3_r(&self) -> bool {
        *self == SPDIFSEL::Pll3R
    }
    ///hsi_ker selected as peripheral clock
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == SPDIFSEL::HsiKer
    }
}
///Field `SPDIFSEL` writer - SPDIFRX kernel clock source selection
pub type SPDIFSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SPDIFSEL, crate::Safe>;
impl<'a, REG> SPDIFSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///pll1_q selected as peripheral clock
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut crate::W<REG> {
        self.variant(SPDIFSEL::Pll1Q)
    }
    ///pll2_r selected as peripheral clock
    #[inline(always)]
    pub fn pll2_r(self) -> &'a mut crate::W<REG> {
        self.variant(SPDIFSEL::Pll2R)
    }
    ///pll3_r selected as peripheral clock
    #[inline(always)]
    pub fn pll3_r(self) -> &'a mut crate::W<REG> {
        self.variant(SPDIFSEL::Pll3R)
    }
    ///hsi_ker selected as peripheral clock
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(SPDIFSEL::HsiKer)
    }
}
/**DFSDM1 kernel Clk clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFSDM1SEL {
    ///0: rcc_pclk2 selected as peripheral clock
    RccPclk2 = 0,
    ///1: System clock selected as peripheral clock
    Sys = 1,
}
impl From<DFSDM1SEL> for bool {
    #[inline(always)]
    fn from(variant: DFSDM1SEL) -> Self {
        variant as u8 != 0
    }
}
///Field `DFSDM1SEL` reader - DFSDM1 kernel Clk clock source selection
pub type DFSDM1SEL_R = crate::BitReader<DFSDM1SEL>;
impl DFSDM1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DFSDM1SEL {
        match self.bits {
            false => DFSDM1SEL::RccPclk2,
            true => DFSDM1SEL::Sys,
        }
    }
    ///rcc_pclk2 selected as peripheral clock
    #[inline(always)]
    pub fn is_rcc_pclk2(&self) -> bool {
        *self == DFSDM1SEL::RccPclk2
    }
    ///System clock selected as peripheral clock
    #[inline(always)]
    pub fn is_sys(&self) -> bool {
        *self == DFSDM1SEL::Sys
    }
}
///Field `DFSDM1SEL` writer - DFSDM1 kernel Clk clock source selection
pub type DFSDM1SEL_W<'a, REG> = crate::BitWriter<'a, REG, DFSDM1SEL>;
impl<'a, REG> DFSDM1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///rcc_pclk2 selected as peripheral clock
    #[inline(always)]
    pub fn rcc_pclk2(self) -> &'a mut crate::W<REG> {
        self.variant(DFSDM1SEL::RccPclk2)
    }
    ///System clock selected as peripheral clock
    #[inline(always)]
    pub fn sys(self) -> &'a mut crate::W<REG> {
        self.variant(DFSDM1SEL::Sys)
    }
}
/**FDCAN kernel clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FDCANSEL {
    ///0: HSE selected as peripheral clock
    Hse = 0,
    ///1: pll1_q selected as peripheral clock
    Pll1Q = 1,
    ///2: pll2_q selected as peripheral clock
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
    ///HSE selected as peripheral clock
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == FDCANSEL::Hse
    }
    ///pll1_q selected as peripheral clock
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == FDCANSEL::Pll1Q
    }
    ///pll2_q selected as peripheral clock
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
    ///HSE selected as peripheral clock
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(FDCANSEL::Hse)
    }
    ///pll1_q selected as peripheral clock
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut crate::W<REG> {
        self.variant(FDCANSEL::Pll1Q)
    }
    ///pll2_q selected as peripheral clock
    #[inline(always)]
    pub fn pll2_q(self) -> &'a mut crate::W<REG> {
        self.variant(FDCANSEL::Pll2Q)
    }
}
/**SWPMI kernel clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWPSEL {
    ///0: pclk selected as peripheral clock
    Pclk = 0,
    ///1: hsi_ker selected as peripheral clock
    HsiKer = 1,
}
impl From<SWPSEL> for bool {
    #[inline(always)]
    fn from(variant: SWPSEL) -> Self {
        variant as u8 != 0
    }
}
///Field `SWPSEL` reader - SWPMI kernel clock source selection
pub type SWPSEL_R = crate::BitReader<SWPSEL>;
impl SWPSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SWPSEL {
        match self.bits {
            false => SWPSEL::Pclk,
            true => SWPSEL::HsiKer,
        }
    }
    ///pclk selected as peripheral clock
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == SWPSEL::Pclk
    }
    ///hsi_ker selected as peripheral clock
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == SWPSEL::HsiKer
    }
}
///Field `SWPSEL` writer - SWPMI kernel clock source selection
pub type SWPSEL_W<'a, REG> = crate::BitWriter<'a, REG, SWPSEL>;
impl<'a, REG> SWPSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///pclk selected as peripheral clock
    #[inline(always)]
    pub fn pclk(self) -> &'a mut crate::W<REG> {
        self.variant(SWPSEL::Pclk)
    }
    ///hsi_ker selected as peripheral clock
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(SWPSEL::HsiKer)
    }
}
impl R {
    ///Bits 0:2 - SAI1 and DFSDM1 kernel Aclk clock source selection
    #[inline(always)]
    pub fn sai1sel(&self) -> SAI1SEL_R {
        SAI1SEL_R::new((self.bits & 7) as u8)
    }
    ///Bits 6:8 - SAI2 and SAI3 kernel clock source selection
    #[inline(always)]
    pub fn sai23sel(&self) -> SAI23SEL_R {
        SAI23SEL_R::new(((self.bits >> 6) & 7) as u8)
    }
    ///Bits 12:14 - SPI/I2S1,2 and 3 kernel clock source selection
    #[inline(always)]
    pub fn spi123sel(&self) -> SPI123SEL_R {
        SPI123SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 16:18 - SPI4 and 5 kernel clock source selection
    #[inline(always)]
    pub fn spi45sel(&self) -> SPI45SEL_R {
        SPI45SEL_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 20:21 - SPDIFRX kernel clock source selection
    #[inline(always)]
    pub fn spdifsel(&self) -> SPDIFSEL_R {
        SPDIFSEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bit 24 - DFSDM1 kernel Clk clock source selection
    #[inline(always)]
    pub fn dfsdm1sel(&self) -> DFSDM1SEL_R {
        DFSDM1SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 28:29 - FDCAN kernel clock source selection
    #[inline(always)]
    pub fn fdcansel(&self) -> FDCANSEL_R {
        FDCANSEL_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bit 31 - SWPMI kernel clock source selection
    #[inline(always)]
    pub fn swpsel(&self) -> SWPSEL_R {
        SWPSEL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("D2CCIP1R")
            .field("sai1sel", &self.sai1sel())
            .field("sai23sel", &self.sai23sel())
            .field("spi123sel", &self.spi123sel())
            .field("spi45sel", &self.spi45sel())
            .field("spdifsel", &self.spdifsel())
            .field("dfsdm1sel", &self.dfsdm1sel())
            .field("fdcansel", &self.fdcansel())
            .field("swpsel", &self.swpsel())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - SAI1 and DFSDM1 kernel Aclk clock source selection
    #[inline(always)]
    pub fn sai1sel(&mut self) -> SAI1SEL_W<'_, D2CCIP1Rrs> {
        SAI1SEL_W::new(self, 0)
    }
    ///Bits 6:8 - SAI2 and SAI3 kernel clock source selection
    #[inline(always)]
    pub fn sai23sel(&mut self) -> SAI23SEL_W<'_, D2CCIP1Rrs> {
        SAI23SEL_W::new(self, 6)
    }
    ///Bits 12:14 - SPI/I2S1,2 and 3 kernel clock source selection
    #[inline(always)]
    pub fn spi123sel(&mut self) -> SPI123SEL_W<'_, D2CCIP1Rrs> {
        SPI123SEL_W::new(self, 12)
    }
    ///Bits 16:18 - SPI4 and 5 kernel clock source selection
    #[inline(always)]
    pub fn spi45sel(&mut self) -> SPI45SEL_W<'_, D2CCIP1Rrs> {
        SPI45SEL_W::new(self, 16)
    }
    ///Bits 20:21 - SPDIFRX kernel clock source selection
    #[inline(always)]
    pub fn spdifsel(&mut self) -> SPDIFSEL_W<'_, D2CCIP1Rrs> {
        SPDIFSEL_W::new(self, 20)
    }
    ///Bit 24 - DFSDM1 kernel Clk clock source selection
    #[inline(always)]
    pub fn dfsdm1sel(&mut self) -> DFSDM1SEL_W<'_, D2CCIP1Rrs> {
        DFSDM1SEL_W::new(self, 24)
    }
    ///Bits 28:29 - FDCAN kernel clock source selection
    #[inline(always)]
    pub fn fdcansel(&mut self) -> FDCANSEL_W<'_, D2CCIP1Rrs> {
        FDCANSEL_W::new(self, 28)
    }
    ///Bit 31 - SWPMI kernel clock source selection
    #[inline(always)]
    pub fn swpsel(&mut self) -> SWPSEL_W<'_, D2CCIP1Rrs> {
        SWPSEL_W::new(self, 31)
    }
}
/**RCC Domain 2 Kernel Clock Configuration Register

You can [`read`](crate::Reg::read) this register and get [`d2ccip1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d2ccip1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM7.html#RCC:D2CCIP1R)*/
pub struct D2CCIP1Rrs;
impl crate::RegisterSpec for D2CCIP1Rrs {
    type Ux = u32;
}
///`read()` method returns [`d2ccip1r::R`](R) reader structure
impl crate::Readable for D2CCIP1Rrs {}
///`write(|w| ..)` method takes [`d2ccip1r::W`](W) writer structure
impl crate::Writable for D2CCIP1Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets D2CCIP1R to value 0
impl crate::Resettable for D2CCIP1Rrs {}

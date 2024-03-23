#[doc = "Register `D2CCIP1R` reader"]
pub type R = crate::R<D2CCIP1Rrs>;
#[doc = "Register `D2CCIP1R` writer"]
pub type W = crate::W<D2CCIP1Rrs>;
#[doc = "SAI1 and DFSDM1 kernel Aclk clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SAI1SEL {
    #[doc = "0: pll1_q selected as peripheral clock"]
    Pll1Q = 0,
    #[doc = "1: pll2_p selected as peripheral clock"]
    Pll2P = 1,
    #[doc = "2: pll3_p selected as peripheral clock"]
    Pll3P = 2,
    #[doc = "3: I2S_CKIN selected as peripheral clock"]
    I2sCkin = 3,
    #[doc = "4: PER selected as peripheral clock"]
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
#[doc = "Field `SAI1SEL` reader - SAI1 and DFSDM1 kernel Aclk clock source selection"]
pub type SAI1SEL_R = crate::FieldReader<SAI1SEL>;
impl SAI1SEL_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "pll1_q selected as peripheral clock"]
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == SAI1SEL::Pll1Q
    }
    #[doc = "pll2_p selected as peripheral clock"]
    #[inline(always)]
    pub fn is_pll2_p(&self) -> bool {
        *self == SAI1SEL::Pll2P
    }
    #[doc = "pll3_p selected as peripheral clock"]
    #[inline(always)]
    pub fn is_pll3_p(&self) -> bool {
        *self == SAI1SEL::Pll3P
    }
    #[doc = "I2S_CKIN selected as peripheral clock"]
    #[inline(always)]
    pub fn is_i2s_ckin(&self) -> bool {
        *self == SAI1SEL::I2sCkin
    }
    #[doc = "PER selected as peripheral clock"]
    #[inline(always)]
    pub fn is_per(&self) -> bool {
        *self == SAI1SEL::Per
    }
}
#[doc = "Field `SAI1SEL` writer - SAI1 and DFSDM1 kernel Aclk clock source selection"]
pub type SAI1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SAI1SEL>;
impl<'a, REG> SAI1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "pll1_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut crate::W<REG> {
        self.variant(SAI1SEL::Pll1Q)
    }
    #[doc = "pll2_p selected as peripheral clock"]
    #[inline(always)]
    pub fn pll2_p(self) -> &'a mut crate::W<REG> {
        self.variant(SAI1SEL::Pll2P)
    }
    #[doc = "pll3_p selected as peripheral clock"]
    #[inline(always)]
    pub fn pll3_p(self) -> &'a mut crate::W<REG> {
        self.variant(SAI1SEL::Pll3P)
    }
    #[doc = "I2S_CKIN selected as peripheral clock"]
    #[inline(always)]
    pub fn i2s_ckin(self) -> &'a mut crate::W<REG> {
        self.variant(SAI1SEL::I2sCkin)
    }
    #[doc = "PER selected as peripheral clock"]
    #[inline(always)]
    pub fn per(self) -> &'a mut crate::W<REG> {
        self.variant(SAI1SEL::Per)
    }
}
#[doc = "Field `SAI23SEL` reader - SAI2 and SAI3 kernel clock source selection"]
pub use SAI1SEL_R as SAI23SEL_R;
#[doc = "Field `SPI123SEL` reader - SPI/I2S1,2 and 3 kernel clock source selection"]
pub use SAI1SEL_R as SPI123SEL_R;
#[doc = "Field `SAI23SEL` writer - SAI2 and SAI3 kernel clock source selection"]
pub use SAI1SEL_W as SAI23SEL_W;
#[doc = "Field `SPI123SEL` writer - SPI/I2S1,2 and 3 kernel clock source selection"]
pub use SAI1SEL_W as SPI123SEL_W;
#[doc = "SPI4 and 5 kernel clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPI45SEL {
    #[doc = "0: APB clock selected as peripheral clock"]
    Apb = 0,
    #[doc = "1: pll2_q selected as peripheral clock"]
    Pll2Q = 1,
    #[doc = "2: pll3_q selected as peripheral clock"]
    Pll3Q = 2,
    #[doc = "3: hsi_ker selected as peripheral clock"]
    HsiKer = 3,
    #[doc = "4: csi_ker selected as peripheral clock"]
    CsiKer = 4,
    #[doc = "5: HSE selected as peripheral clock"]
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
#[doc = "Field `SPI45SEL` reader - SPI4 and 5 kernel clock source selection"]
pub type SPI45SEL_R = crate::FieldReader<SPI45SEL>;
impl SPI45SEL_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "APB clock selected as peripheral clock"]
    #[inline(always)]
    pub fn is_apb(&self) -> bool {
        *self == SPI45SEL::Apb
    }
    #[doc = "pll2_q selected as peripheral clock"]
    #[inline(always)]
    pub fn is_pll2_q(&self) -> bool {
        *self == SPI45SEL::Pll2Q
    }
    #[doc = "pll3_q selected as peripheral clock"]
    #[inline(always)]
    pub fn is_pll3_q(&self) -> bool {
        *self == SPI45SEL::Pll3Q
    }
    #[doc = "hsi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == SPI45SEL::HsiKer
    }
    #[doc = "csi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        *self == SPI45SEL::CsiKer
    }
    #[doc = "HSE selected as peripheral clock"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == SPI45SEL::Hse
    }
}
#[doc = "Field `SPI45SEL` writer - SPI4 and 5 kernel clock source selection"]
pub type SPI45SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SPI45SEL>;
impl<'a, REG> SPI45SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "APB clock selected as peripheral clock"]
    #[inline(always)]
    pub fn apb(self) -> &'a mut crate::W<REG> {
        self.variant(SPI45SEL::Apb)
    }
    #[doc = "pll2_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll2_q(self) -> &'a mut crate::W<REG> {
        self.variant(SPI45SEL::Pll2Q)
    }
    #[doc = "pll3_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll3_q(self) -> &'a mut crate::W<REG> {
        self.variant(SPI45SEL::Pll3Q)
    }
    #[doc = "hsi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(SPI45SEL::HsiKer)
    }
    #[doc = "csi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(SPI45SEL::CsiKer)
    }
    #[doc = "HSE selected as peripheral clock"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(SPI45SEL::Hse)
    }
}
#[doc = "SPDIFRX kernel clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPDIFSEL {
    #[doc = "0: pll1_q selected as peripheral clock"]
    Pll1Q = 0,
    #[doc = "1: pll2_r selected as peripheral clock"]
    Pll2R = 1,
    #[doc = "2: pll3_r selected as peripheral clock"]
    Pll3R = 2,
    #[doc = "3: hsi_ker selected as peripheral clock"]
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
#[doc = "Field `SPDIFSEL` reader - SPDIFRX kernel clock source selection"]
pub type SPDIFSEL_R = crate::FieldReader<SPDIFSEL>;
impl SPDIFSEL_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "pll1_q selected as peripheral clock"]
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == SPDIFSEL::Pll1Q
    }
    #[doc = "pll2_r selected as peripheral clock"]
    #[inline(always)]
    pub fn is_pll2_r(&self) -> bool {
        *self == SPDIFSEL::Pll2R
    }
    #[doc = "pll3_r selected as peripheral clock"]
    #[inline(always)]
    pub fn is_pll3_r(&self) -> bool {
        *self == SPDIFSEL::Pll3R
    }
    #[doc = "hsi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == SPDIFSEL::HsiKer
    }
}
#[doc = "Field `SPDIFSEL` writer - SPDIFRX kernel clock source selection"]
pub type SPDIFSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, SPDIFSEL>;
impl<'a, REG> SPDIFSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "pll1_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut crate::W<REG> {
        self.variant(SPDIFSEL::Pll1Q)
    }
    #[doc = "pll2_r selected as peripheral clock"]
    #[inline(always)]
    pub fn pll2_r(self) -> &'a mut crate::W<REG> {
        self.variant(SPDIFSEL::Pll2R)
    }
    #[doc = "pll3_r selected as peripheral clock"]
    #[inline(always)]
    pub fn pll3_r(self) -> &'a mut crate::W<REG> {
        self.variant(SPDIFSEL::Pll3R)
    }
    #[doc = "hsi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(SPDIFSEL::HsiKer)
    }
}
#[doc = "DFSDM1 kernel Clk clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFSDM1SEL {
    #[doc = "0: rcc_pclk2 selected as peripheral clock"]
    RccPclk2 = 0,
    #[doc = "1: System clock selected as peripheral clock"]
    Sys = 1,
}
impl From<DFSDM1SEL> for bool {
    #[inline(always)]
    fn from(variant: DFSDM1SEL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DFSDM1SEL` reader - DFSDM1 kernel Clk clock source selection"]
pub type DFSDM1SEL_R = crate::BitReader<DFSDM1SEL>;
impl DFSDM1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DFSDM1SEL {
        match self.bits {
            false => DFSDM1SEL::RccPclk2,
            true => DFSDM1SEL::Sys,
        }
    }
    #[doc = "rcc_pclk2 selected as peripheral clock"]
    #[inline(always)]
    pub fn is_rcc_pclk2(&self) -> bool {
        *self == DFSDM1SEL::RccPclk2
    }
    #[doc = "System clock selected as peripheral clock"]
    #[inline(always)]
    pub fn is_sys(&self) -> bool {
        *self == DFSDM1SEL::Sys
    }
}
#[doc = "Field `DFSDM1SEL` writer - DFSDM1 kernel Clk clock source selection"]
pub type DFSDM1SEL_W<'a, REG> = crate::BitWriter<'a, REG, DFSDM1SEL>;
impl<'a, REG> DFSDM1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "rcc_pclk2 selected as peripheral clock"]
    #[inline(always)]
    pub fn rcc_pclk2(self) -> &'a mut crate::W<REG> {
        self.variant(DFSDM1SEL::RccPclk2)
    }
    #[doc = "System clock selected as peripheral clock"]
    #[inline(always)]
    pub fn sys(self) -> &'a mut crate::W<REG> {
        self.variant(DFSDM1SEL::Sys)
    }
}
#[doc = "FDCAN kernel clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FDCANSEL {
    #[doc = "0: HSE selected as peripheral clock"]
    Hse = 0,
    #[doc = "1: pll1_q selected as peripheral clock"]
    Pll1Q = 1,
    #[doc = "2: pll2_q selected as peripheral clock"]
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
#[doc = "Field `FDCANSEL` reader - FDCAN kernel clock source selection"]
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
    #[doc = "HSE selected as peripheral clock"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == FDCANSEL::Hse
    }
    #[doc = "pll1_q selected as peripheral clock"]
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == FDCANSEL::Pll1Q
    }
    #[doc = "pll2_q selected as peripheral clock"]
    #[inline(always)]
    pub fn is_pll2_q(&self) -> bool {
        *self == FDCANSEL::Pll2Q
    }
}
#[doc = "Field `FDCANSEL` writer - FDCAN kernel clock source selection"]
pub type FDCANSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, FDCANSEL>;
impl<'a, REG> FDCANSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HSE selected as peripheral clock"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(FDCANSEL::Hse)
    }
    #[doc = "pll1_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut crate::W<REG> {
        self.variant(FDCANSEL::Pll1Q)
    }
    #[doc = "pll2_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll2_q(self) -> &'a mut crate::W<REG> {
        self.variant(FDCANSEL::Pll2Q)
    }
}
#[doc = "SWPMI kernel clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWPSEL {
    #[doc = "0: pclk selected as peripheral clock"]
    Pclk = 0,
    #[doc = "1: hsi_ker selected as peripheral clock"]
    HsiKer = 1,
}
impl From<SWPSEL> for bool {
    #[inline(always)]
    fn from(variant: SWPSEL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWPSEL` reader - SWPMI kernel clock source selection"]
pub type SWPSEL_R = crate::BitReader<SWPSEL>;
impl SWPSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SWPSEL {
        match self.bits {
            false => SWPSEL::Pclk,
            true => SWPSEL::HsiKer,
        }
    }
    #[doc = "pclk selected as peripheral clock"]
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == SWPSEL::Pclk
    }
    #[doc = "hsi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == SWPSEL::HsiKer
    }
}
#[doc = "Field `SWPSEL` writer - SWPMI kernel clock source selection"]
pub type SWPSEL_W<'a, REG> = crate::BitWriter<'a, REG, SWPSEL>;
impl<'a, REG> SWPSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "pclk selected as peripheral clock"]
    #[inline(always)]
    pub fn pclk(self) -> &'a mut crate::W<REG> {
        self.variant(SWPSEL::Pclk)
    }
    #[doc = "hsi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(SWPSEL::HsiKer)
    }
}
impl R {
    #[doc = "Bits 0:2 - SAI1 and DFSDM1 kernel Aclk clock source selection"]
    #[inline(always)]
    pub fn sai1sel(&self) -> SAI1SEL_R {
        SAI1SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 6:8 - SAI2 and SAI3 kernel clock source selection"]
    #[inline(always)]
    pub fn sai23sel(&self) -> SAI23SEL_R {
        SAI23SEL_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 12:14 - SPI/I2S1,2 and 3 kernel clock source selection"]
    #[inline(always)]
    pub fn spi123sel(&self) -> SPI123SEL_R {
        SPI123SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - SPI4 and 5 kernel clock source selection"]
    #[inline(always)]
    pub fn spi45sel(&self) -> SPI45SEL_R {
        SPI45SEL_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:21 - SPDIFRX kernel clock source selection"]
    #[inline(always)]
    pub fn spdifsel(&self) -> SPDIFSEL_R {
        SPDIFSEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 24 - DFSDM1 kernel Clk clock source selection"]
    #[inline(always)]
    pub fn dfsdm1sel(&self) -> DFSDM1SEL_R {
        DFSDM1SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 28:29 - FDCAN kernel clock source selection"]
    #[inline(always)]
    pub fn fdcansel(&self) -> FDCANSEL_R {
        FDCANSEL_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 31 - SWPMI kernel clock source selection"]
    #[inline(always)]
    pub fn swpsel(&self) -> SWPSEL_R {
        SWPSEL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - SAI1 and DFSDM1 kernel Aclk clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn sai1sel(&mut self) -> SAI1SEL_W<D2CCIP1Rrs> {
        SAI1SEL_W::new(self, 0)
    }
    #[doc = "Bits 6:8 - SAI2 and SAI3 kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn sai23sel(&mut self) -> SAI23SEL_W<D2CCIP1Rrs> {
        SAI23SEL_W::new(self, 6)
    }
    #[doc = "Bits 12:14 - SPI/I2S1,2 and 3 kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn spi123sel(&mut self) -> SPI123SEL_W<D2CCIP1Rrs> {
        SPI123SEL_W::new(self, 12)
    }
    #[doc = "Bits 16:18 - SPI4 and 5 kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn spi45sel(&mut self) -> SPI45SEL_W<D2CCIP1Rrs> {
        SPI45SEL_W::new(self, 16)
    }
    #[doc = "Bits 20:21 - SPDIFRX kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn spdifsel(&mut self) -> SPDIFSEL_W<D2CCIP1Rrs> {
        SPDIFSEL_W::new(self, 20)
    }
    #[doc = "Bit 24 - DFSDM1 kernel Clk clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn dfsdm1sel(&mut self) -> DFSDM1SEL_W<D2CCIP1Rrs> {
        DFSDM1SEL_W::new(self, 24)
    }
    #[doc = "Bits 28:29 - FDCAN kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn fdcansel(&mut self) -> FDCANSEL_W<D2CCIP1Rrs> {
        FDCANSEL_W::new(self, 28)
    }
    #[doc = "Bit 31 - SWPMI kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn swpsel(&mut self) -> SWPSEL_W<D2CCIP1Rrs> {
        SWPSEL_W::new(self, 31)
    }
}
#[doc = "RCC Domain 2 Kernel Clock Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d2ccip1r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d2ccip1r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D2CCIP1Rrs;
impl crate::RegisterSpec for D2CCIP1Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d2ccip1r::R`](R) reader structure"]
impl crate::Readable for D2CCIP1Rrs {}
#[doc = "`write(|w| ..)` method takes [`d2ccip1r::W`](W) writer structure"]
impl crate::Writable for D2CCIP1Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets D2CCIP1R to value 0"]
impl crate::Resettable for D2CCIP1Rrs {
    const RESET_VALUE: u32 = 0;
}

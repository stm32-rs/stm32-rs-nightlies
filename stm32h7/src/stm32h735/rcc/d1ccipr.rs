#[doc = "Register `D1CCIPR` reader"]
pub type R = crate::R<D1CCIPRrs>;
#[doc = "Register `D1CCIPR` writer"]
pub type W = crate::W<D1CCIPRrs>;
#[doc = "FMC kernel clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FMCSEL {
    #[doc = "0: rcc_hclk3 selected as peripheral clock"]
    RccHclk3 = 0,
    #[doc = "1: pll1_q selected as peripheral clock"]
    Pll1Q = 1,
    #[doc = "2: pll2_r selected as peripheral clock"]
    Pll2R = 2,
    #[doc = "3: PER selected as peripheral clock"]
    Per = 3,
}
impl From<FMCSEL> for u8 {
    #[inline(always)]
    fn from(variant: FMCSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FMCSEL {
    type Ux = u8;
}
#[doc = "Field `FMCSEL` reader - FMC kernel clock source selection"]
pub type FMCSEL_R = crate::FieldReader<FMCSEL>;
impl FMCSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FMCSEL {
        match self.bits {
            0 => FMCSEL::RccHclk3,
            1 => FMCSEL::Pll1Q,
            2 => FMCSEL::Pll2R,
            3 => FMCSEL::Per,
            _ => unreachable!(),
        }
    }
    #[doc = "rcc_hclk3 selected as peripheral clock"]
    #[inline(always)]
    pub fn is_rcc_hclk3(&self) -> bool {
        *self == FMCSEL::RccHclk3
    }
    #[doc = "pll1_q selected as peripheral clock"]
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == FMCSEL::Pll1Q
    }
    #[doc = "pll2_r selected as peripheral clock"]
    #[inline(always)]
    pub fn is_pll2_r(&self) -> bool {
        *self == FMCSEL::Pll2R
    }
    #[doc = "PER selected as peripheral clock"]
    #[inline(always)]
    pub fn is_per(&self) -> bool {
        *self == FMCSEL::Per
    }
}
#[doc = "Field `FMCSEL` writer - FMC kernel clock source selection"]
pub type FMCSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, FMCSEL>;
impl<'a, REG> FMCSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "rcc_hclk3 selected as peripheral clock"]
    #[inline(always)]
    pub fn rcc_hclk3(self) -> &'a mut crate::W<REG> {
        self.variant(FMCSEL::RccHclk3)
    }
    #[doc = "pll1_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut crate::W<REG> {
        self.variant(FMCSEL::Pll1Q)
    }
    #[doc = "pll2_r selected as peripheral clock"]
    #[inline(always)]
    pub fn pll2_r(self) -> &'a mut crate::W<REG> {
        self.variant(FMCSEL::Pll2R)
    }
    #[doc = "PER selected as peripheral clock"]
    #[inline(always)]
    pub fn per(self) -> &'a mut crate::W<REG> {
        self.variant(FMCSEL::Per)
    }
}
#[doc = "Field `OCTOSPISEL` reader - QUADSPI kernel clock source selection"]
pub use FMCSEL_R as OCTOSPISEL_R;
#[doc = "Field `OCTOSPISEL` writer - QUADSPI kernel clock source selection"]
pub use FMCSEL_W as OCTOSPISEL_W;
#[doc = "SDMMC kernel clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDMMCSEL {
    #[doc = "0: pll1_q selected as peripheral clock"]
    Pll1Q = 0,
    #[doc = "1: pll2_r selected as peripheral clock"]
    Pll2R = 1,
}
impl From<SDMMCSEL> for bool {
    #[inline(always)]
    fn from(variant: SDMMCSEL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDMMCSEL` reader - SDMMC kernel clock source selection"]
pub type SDMMCSEL_R = crate::BitReader<SDMMCSEL>;
impl SDMMCSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SDMMCSEL {
        match self.bits {
            false => SDMMCSEL::Pll1Q,
            true => SDMMCSEL::Pll2R,
        }
    }
    #[doc = "pll1_q selected as peripheral clock"]
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == SDMMCSEL::Pll1Q
    }
    #[doc = "pll2_r selected as peripheral clock"]
    #[inline(always)]
    pub fn is_pll2_r(&self) -> bool {
        *self == SDMMCSEL::Pll2R
    }
}
#[doc = "Field `SDMMCSEL` writer - SDMMC kernel clock source selection"]
pub type SDMMCSEL_W<'a, REG> = crate::BitWriter<'a, REG, SDMMCSEL>;
impl<'a, REG> SDMMCSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "pll1_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut crate::W<REG> {
        self.variant(SDMMCSEL::Pll1Q)
    }
    #[doc = "pll2_r selected as peripheral clock"]
    #[inline(always)]
    pub fn pll2_r(self) -> &'a mut crate::W<REG> {
        self.variant(SDMMCSEL::Pll2R)
    }
}
#[doc = "per_ck clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKPERSEL {
    #[doc = "0: HSI selected as peripheral clock"]
    Hsi = 0,
    #[doc = "1: CSI selected as peripheral clock"]
    Csi = 1,
    #[doc = "2: HSE selected as peripheral clock"]
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
            0 => Some(CKPERSEL::Hsi),
            1 => Some(CKPERSEL::Csi),
            2 => Some(CKPERSEL::Hse),
            _ => None,
        }
    }
    #[doc = "HSI selected as peripheral clock"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == CKPERSEL::Hsi
    }
    #[doc = "CSI selected as peripheral clock"]
    #[inline(always)]
    pub fn is_csi(&self) -> bool {
        *self == CKPERSEL::Csi
    }
    #[doc = "HSE selected as peripheral clock"]
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
    #[doc = "HSI selected as peripheral clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut crate::W<REG> {
        self.variant(CKPERSEL::Hsi)
    }
    #[doc = "CSI selected as peripheral clock"]
    #[inline(always)]
    pub fn csi(self) -> &'a mut crate::W<REG> {
        self.variant(CKPERSEL::Csi)
    }
    #[doc = "HSE selected as peripheral clock"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(CKPERSEL::Hse)
    }
}
impl R {
    #[doc = "Bits 0:1 - FMC kernel clock source selection"]
    #[inline(always)]
    pub fn fmcsel(&self) -> FMCSEL_R {
        FMCSEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - QUADSPI kernel clock source selection"]
    #[inline(always)]
    pub fn octospisel(&self) -> OCTOSPISEL_R {
        OCTOSPISEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 16 - SDMMC kernel clock source selection"]
    #[inline(always)]
    pub fn sdmmcsel(&self) -> SDMMCSEL_R {
        SDMMCSEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 28:29 - per_ck clock source selection"]
    #[inline(always)]
    pub fn ckpersel(&self) -> CKPERSEL_R {
        CKPERSEL_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - FMC kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn fmcsel(&mut self) -> FMCSEL_W<D1CCIPRrs> {
        FMCSEL_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - QUADSPI kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn octospisel(&mut self) -> OCTOSPISEL_W<D1CCIPRrs> {
        OCTOSPISEL_W::new(self, 4)
    }
    #[doc = "Bit 16 - SDMMC kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmcsel(&mut self) -> SDMMCSEL_W<D1CCIPRrs> {
        SDMMCSEL_W::new(self, 16)
    }
    #[doc = "Bits 28:29 - per_ck clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn ckpersel(&mut self) -> CKPERSEL_W<D1CCIPRrs> {
        CKPERSEL_W::new(self, 28)
    }
}
#[doc = "RCC Domain 1 Kernel Clock Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d1ccipr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d1ccipr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D1CCIPRrs;
impl crate::RegisterSpec for D1CCIPRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d1ccipr::R`](R) reader structure"]
impl crate::Readable for D1CCIPRrs {}
#[doc = "`write(|w| ..)` method takes [`d1ccipr::W`](W) writer structure"]
impl crate::Writable for D1CCIPRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets D1CCIPR to value 0"]
impl crate::Resettable for D1CCIPRrs {
    const RESET_VALUE: u32 = 0;
}

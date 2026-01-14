///Register `D1CCIPR` reader
pub type R = crate::R<D1CCIPRrs>;
///Register `D1CCIPR` writer
pub type W = crate::W<D1CCIPRrs>;
/**FMC kernel clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FMCSEL {
    ///0: rcc_hclk3 selected as peripheral clock
    RccHclk3 = 0,
    ///1: pll1_q selected as peripheral clock
    Pll1Q = 1,
    ///2: pll2_r selected as peripheral clock
    Pll2R = 2,
    ///3: PER selected as peripheral clock
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
impl crate::IsEnum for FMCSEL {}
///Field `FMCSEL` reader - FMC kernel clock source selection
pub type FMCSEL_R = crate::FieldReader<FMCSEL>;
impl FMCSEL_R {
    ///Get enumerated values variant
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
    ///rcc_hclk3 selected as peripheral clock
    #[inline(always)]
    pub fn is_rcc_hclk3(&self) -> bool {
        *self == FMCSEL::RccHclk3
    }
    ///pll1_q selected as peripheral clock
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == FMCSEL::Pll1Q
    }
    ///pll2_r selected as peripheral clock
    #[inline(always)]
    pub fn is_pll2_r(&self) -> bool {
        *self == FMCSEL::Pll2R
    }
    ///PER selected as peripheral clock
    #[inline(always)]
    pub fn is_per(&self) -> bool {
        *self == FMCSEL::Per
    }
}
///Field `FMCSEL` writer - FMC kernel clock source selection
pub type FMCSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, FMCSEL, crate::Safe>;
impl<'a, REG> FMCSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///rcc_hclk3 selected as peripheral clock
    #[inline(always)]
    pub fn rcc_hclk3(self) -> &'a mut crate::W<REG> {
        self.variant(FMCSEL::RccHclk3)
    }
    ///pll1_q selected as peripheral clock
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut crate::W<REG> {
        self.variant(FMCSEL::Pll1Q)
    }
    ///pll2_r selected as peripheral clock
    #[inline(always)]
    pub fn pll2_r(self) -> &'a mut crate::W<REG> {
        self.variant(FMCSEL::Pll2R)
    }
    ///PER selected as peripheral clock
    #[inline(always)]
    pub fn per(self) -> &'a mut crate::W<REG> {
        self.variant(FMCSEL::Per)
    }
}
///Field `QSPISEL` reader - QUADSPI kernel clock source selection
pub use FMCSEL_R as QSPISEL_R;
///Field `QSPISEL` writer - QUADSPI kernel clock source selection
pub use FMCSEL_W as QSPISEL_W;
///Field `DSISEL` reader - DSI kernel clock source selection
pub type DSISEL_R = crate::BitReader;
///Field `DSISEL` writer - DSI kernel clock source selection
pub type DSISEL_W<'a, REG> = crate::BitWriter<'a, REG>;
/**SDMMC kernel clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDMMCSEL {
    ///0: pll1_q selected as peripheral clock
    Pll1Q = 0,
    ///1: pll2_r selected as peripheral clock
    Pll2R = 1,
}
impl From<SDMMCSEL> for bool {
    #[inline(always)]
    fn from(variant: SDMMCSEL) -> Self {
        variant as u8 != 0
    }
}
///Field `SDMMCSEL` reader - SDMMC kernel clock source selection
pub type SDMMCSEL_R = crate::BitReader<SDMMCSEL>;
impl SDMMCSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SDMMCSEL {
        match self.bits {
            false => SDMMCSEL::Pll1Q,
            true => SDMMCSEL::Pll2R,
        }
    }
    ///pll1_q selected as peripheral clock
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == SDMMCSEL::Pll1Q
    }
    ///pll2_r selected as peripheral clock
    #[inline(always)]
    pub fn is_pll2_r(&self) -> bool {
        *self == SDMMCSEL::Pll2R
    }
}
///Field `SDMMCSEL` writer - SDMMC kernel clock source selection
pub type SDMMCSEL_W<'a, REG> = crate::BitWriter<'a, REG, SDMMCSEL>;
impl<'a, REG> SDMMCSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///pll1_q selected as peripheral clock
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut crate::W<REG> {
        self.variant(SDMMCSEL::Pll1Q)
    }
    ///pll2_r selected as peripheral clock
    #[inline(always)]
    pub fn pll2_r(self) -> &'a mut crate::W<REG> {
        self.variant(SDMMCSEL::Pll2R)
    }
}
/**per_ck clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKPERSEL {
    ///0: HSI selected as peripheral clock
    Hsi = 0,
    ///1: CSI selected as peripheral clock
    Csi = 1,
    ///2: HSE selected as peripheral clock
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
            0 => Some(CKPERSEL::Hsi),
            1 => Some(CKPERSEL::Csi),
            2 => Some(CKPERSEL::Hse),
            _ => None,
        }
    }
    ///HSI selected as peripheral clock
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == CKPERSEL::Hsi
    }
    ///CSI selected as peripheral clock
    #[inline(always)]
    pub fn is_csi(&self) -> bool {
        *self == CKPERSEL::Csi
    }
    ///HSE selected as peripheral clock
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
    ///HSI selected as peripheral clock
    #[inline(always)]
    pub fn hsi(self) -> &'a mut crate::W<REG> {
        self.variant(CKPERSEL::Hsi)
    }
    ///CSI selected as peripheral clock
    #[inline(always)]
    pub fn csi(self) -> &'a mut crate::W<REG> {
        self.variant(CKPERSEL::Csi)
    }
    ///HSE selected as peripheral clock
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(CKPERSEL::Hse)
    }
}
impl R {
    ///Bits 0:1 - FMC kernel clock source selection
    #[inline(always)]
    pub fn fmcsel(&self) -> FMCSEL_R {
        FMCSEL_R::new((self.bits & 3) as u8)
    }
    ///Bits 4:5 - QUADSPI kernel clock source selection
    #[inline(always)]
    pub fn qspisel(&self) -> QSPISEL_R {
        QSPISEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 8 - DSI kernel clock source selection
    #[inline(always)]
    pub fn dsisel(&self) -> DSISEL_R {
        DSISEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 16 - SDMMC kernel clock source selection
    #[inline(always)]
    pub fn sdmmcsel(&self) -> SDMMCSEL_R {
        SDMMCSEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 28:29 - per_ck clock source selection
    #[inline(always)]
    pub fn ckpersel(&self) -> CKPERSEL_R {
        CKPERSEL_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("D1CCIPR")
            .field("fmcsel", &self.fmcsel())
            .field("qspisel", &self.qspisel())
            .field("dsisel", &self.dsisel())
            .field("sdmmcsel", &self.sdmmcsel())
            .field("ckpersel", &self.ckpersel())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - FMC kernel clock source selection
    #[inline(always)]
    pub fn fmcsel(&mut self) -> FMCSEL_W<'_, D1CCIPRrs> {
        FMCSEL_W::new(self, 0)
    }
    ///Bits 4:5 - QUADSPI kernel clock source selection
    #[inline(always)]
    pub fn qspisel(&mut self) -> QSPISEL_W<'_, D1CCIPRrs> {
        QSPISEL_W::new(self, 4)
    }
    ///Bit 8 - DSI kernel clock source selection
    #[inline(always)]
    pub fn dsisel(&mut self) -> DSISEL_W<'_, D1CCIPRrs> {
        DSISEL_W::new(self, 8)
    }
    ///Bit 16 - SDMMC kernel clock source selection
    #[inline(always)]
    pub fn sdmmcsel(&mut self) -> SDMMCSEL_W<'_, D1CCIPRrs> {
        SDMMCSEL_W::new(self, 16)
    }
    ///Bits 28:29 - per_ck clock source selection
    #[inline(always)]
    pub fn ckpersel(&mut self) -> CKPERSEL_W<'_, D1CCIPRrs> {
        CKPERSEL_W::new(self, 28)
    }
}
/**RCC Domain 1 Kernel Clock Configuration Register

You can [`read`](crate::Reg::read) this register and get [`d1ccipr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d1ccipr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM4.html#RCC:D1CCIPR)*/
pub struct D1CCIPRrs;
impl crate::RegisterSpec for D1CCIPRrs {
    type Ux = u32;
}
///`read()` method returns [`d1ccipr::R`](R) reader structure
impl crate::Readable for D1CCIPRrs {}
///`write(|w| ..)` method takes [`d1ccipr::W`](W) writer structure
impl crate::Writable for D1CCIPRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets D1CCIPR to value 0
impl crate::Resettable for D1CCIPRrs {}

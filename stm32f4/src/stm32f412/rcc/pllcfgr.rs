#[doc = "Register `PLLCFGR` reader"]
pub type R = crate::R<PLLCFGRrs>;
#[doc = "Register `PLLCFGR` writer"]
pub type W = crate::W<PLLCFGRrs>;
#[doc = "Field `PLLM` reader - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
pub type PLLM_R = crate::FieldReader;
#[doc = "Field `PLLM` writer - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
pub type PLLM_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PLLN` reader - Main PLL (PLL) multiplication factor for VCO"]
pub type PLLN_R = crate::FieldReader<u16>;
#[doc = "Field `PLLN` writer - Main PLL (PLL) multiplication factor for VCO"]
pub type PLLN_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Main PLL (PLL) division factor for main system clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLP {
    #[doc = "0: PLLP=2"]
    Div2 = 0,
    #[doc = "1: PLLP=4"]
    Div4 = 1,
    #[doc = "2: PLLP=6"]
    Div6 = 2,
    #[doc = "3: PLLP=8"]
    Div8 = 3,
}
impl From<PLLP> for u8 {
    #[inline(always)]
    fn from(variant: PLLP) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLLP {
    type Ux = u8;
}
#[doc = "Field `PLLP` reader - Main PLL (PLL) division factor for main system clock"]
pub type PLLP_R = crate::FieldReader<PLLP>;
impl PLLP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLLP {
        match self.bits {
            0 => PLLP::Div2,
            1 => PLLP::Div4,
            2 => PLLP::Div6,
            3 => PLLP::Div8,
            _ => unreachable!(),
        }
    }
    #[doc = "PLLP=2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLP::Div2
    }
    #[doc = "PLLP=4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLP::Div4
    }
    #[doc = "PLLP=6"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLP::Div6
    }
    #[doc = "PLLP=8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLP::Div8
    }
}
#[doc = "Field `PLLP` writer - Main PLL (PLL) division factor for main system clock"]
pub type PLLP_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, PLLP>;
impl<'a, REG> PLLP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PLLP=2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div2)
    }
    #[doc = "PLLP=4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div4)
    }
    #[doc = "PLLP=6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div6)
    }
    #[doc = "PLLP=8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div8)
    }
}
#[doc = "Main PLL(PLL) and audio PLL (PLLI2S) entry clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSRC {
    #[doc = "0: HSI clock selected as PLL and PLLI2S clock entry"]
    Hsi = 0,
    #[doc = "1: HSE oscillator clock selected as PLL and PLLI2S clock entry"]
    Hse = 1,
}
impl From<PLLSRC> for bool {
    #[inline(always)]
    fn from(variant: PLLSRC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLSRC` reader - Main PLL(PLL) and audio PLL (PLLI2S) entry clock source"]
pub type PLLSRC_R = crate::BitReader<PLLSRC>;
impl PLLSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLLSRC {
        match self.bits {
            false => PLLSRC::Hsi,
            true => PLLSRC::Hse,
        }
    }
    #[doc = "HSI clock selected as PLL and PLLI2S clock entry"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == PLLSRC::Hsi
    }
    #[doc = "HSE oscillator clock selected as PLL and PLLI2S clock entry"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == PLLSRC::Hse
    }
}
#[doc = "Field `PLLSRC` writer - Main PLL(PLL) and audio PLL (PLLI2S) entry clock source"]
pub type PLLSRC_W<'a, REG> = crate::BitWriter<'a, REG, PLLSRC>;
impl<'a, REG> PLLSRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSI clock selected as PLL and PLLI2S clock entry"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSRC::Hsi)
    }
    #[doc = "HSE oscillator clock selected as PLL and PLLI2S clock entry"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSRC::Hse)
    }
}
#[doc = "Field `PLLQ` reader - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
pub type PLLQ_R = crate::FieldReader;
#[doc = "Field `PLLQ` writer - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
pub type PLLQ_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PLLR` reader - Main PLL division factor for I2S, DFSDM clocks"]
pub type PLLR_R = crate::FieldReader;
#[doc = "Field `PLLR` writer - Main PLL division factor for I2S, DFSDM clocks"]
pub type PLLR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:5 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    pub fn pllm(&self) -> PLLM_R {
        PLLM_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:14 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln(&self) -> PLLN_R {
        PLLN_R::new(((self.bits >> 6) & 0x01ff) as u16)
    }
    #[doc = "Bits 16:17 - Main PLL (PLL) division factor for main system clock"]
    #[inline(always)]
    pub fn pllp(&self) -> PLLP_R {
        PLLP_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 22 - Main PLL(PLL) and audio PLL (PLLI2S) entry clock source"]
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
    #[inline(always)]
    pub fn pllq(&self) -> PLLQ_R {
        PLLQ_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:30 - Main PLL division factor for I2S, DFSDM clocks"]
    #[inline(always)]
    pub fn pllr(&self) -> PLLR_R {
        PLLR_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    #[must_use]
    pub fn pllm(&mut self) -> PLLM_W<PLLCFGRrs> {
        PLLM_W::new(self, 0)
    }
    #[doc = "Bits 6:14 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    #[must_use]
    pub fn plln(&mut self) -> PLLN_W<PLLCFGRrs> {
        PLLN_W::new(self, 6)
    }
    #[doc = "Bits 16:17 - Main PLL (PLL) division factor for main system clock"]
    #[inline(always)]
    #[must_use]
    pub fn pllp(&mut self) -> PLLP_W<PLLCFGRrs> {
        PLLP_W::new(self, 16)
    }
    #[doc = "Bit 22 - Main PLL(PLL) and audio PLL (PLLI2S) entry clock source"]
    #[inline(always)]
    #[must_use]
    pub fn pllsrc(&mut self) -> PLLSRC_W<PLLCFGRrs> {
        PLLSRC_W::new(self, 22)
    }
    #[doc = "Bits 24:27 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
    #[inline(always)]
    #[must_use]
    pub fn pllq(&mut self) -> PLLQ_W<PLLCFGRrs> {
        PLLQ_W::new(self, 24)
    }
    #[doc = "Bits 28:30 - Main PLL division factor for I2S, DFSDM clocks"]
    #[inline(always)]
    #[must_use]
    pub fn pllr(&mut self) -> PLLR_W<PLLCFGRrs> {
        PLLR_W::new(self, 28)
    }
}
#[doc = "PLL configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllcfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllcfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLLCFGRrs;
impl crate::RegisterSpec for PLLCFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pllcfgr::R`](R) reader structure"]
impl crate::Readable for PLLCFGRrs {}
#[doc = "`write(|w| ..)` method takes [`pllcfgr::W`](W) writer structure"]
impl crate::Writable for PLLCFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLLCFGR to value 0x2400_3010"]
impl crate::Resettable for PLLCFGRrs {
    const RESET_VALUE: u32 = 0x2400_3010;
}

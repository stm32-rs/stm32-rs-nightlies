#[doc = "Register `DCKCFGR2` reader"]
pub type R = crate::R<DCKCFGR2rs>;
#[doc = "Register `DCKCFGR2` writer"]
pub type W = crate::W<DCKCFGR2rs>;
#[doc = "FMPI2C1 kernel clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FMPI2C1SEL {
    #[doc = "0: APB clock selected as I2C clock"]
    Apb = 0,
    #[doc = "1: System clock selected as I2C clock"]
    Sysclk = 1,
    #[doc = "2: HSI clock selected as I2C clock"]
    Hsi = 2,
}
impl From<FMPI2C1SEL> for u8 {
    #[inline(always)]
    fn from(variant: FMPI2C1SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FMPI2C1SEL {
    type Ux = u8;
}
#[doc = "Field `FMPI2C1SEL` reader - FMPI2C1 kernel clock source selection"]
pub type FMPI2C1SEL_R = crate::FieldReader<FMPI2C1SEL>;
impl FMPI2C1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<FMPI2C1SEL> {
        match self.bits {
            0 => Some(FMPI2C1SEL::Apb),
            1 => Some(FMPI2C1SEL::Sysclk),
            2 => Some(FMPI2C1SEL::Hsi),
            _ => None,
        }
    }
    #[doc = "APB clock selected as I2C clock"]
    #[inline(always)]
    pub fn is_apb(&self) -> bool {
        *self == FMPI2C1SEL::Apb
    }
    #[doc = "System clock selected as I2C clock"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == FMPI2C1SEL::Sysclk
    }
    #[doc = "HSI clock selected as I2C clock"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == FMPI2C1SEL::Hsi
    }
}
#[doc = "Field `FMPI2C1SEL` writer - FMPI2C1 kernel clock source selection"]
pub type FMPI2C1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, FMPI2C1SEL>;
impl<'a, REG> FMPI2C1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "APB clock selected as I2C clock"]
    #[inline(always)]
    pub fn apb(self) -> &'a mut crate::W<REG> {
        self.variant(FMPI2C1SEL::Apb)
    }
    #[doc = "System clock selected as I2C clock"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(FMPI2C1SEL::Sysclk)
    }
    #[doc = "HSI clock selected as I2C clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut crate::W<REG> {
        self.variant(FMPI2C1SEL::Hsi)
    }
}
#[doc = "SDIO/USBFS clock selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CK48MSEL {
    #[doc = "0: 48MHz clock from PLL is selected"]
    Pll = 0,
    #[doc = "1: 48MHz clock from PLLSAI is selected"]
    Pllsai = 1,
}
impl From<CK48MSEL> for bool {
    #[inline(always)]
    fn from(variant: CK48MSEL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CK48MSEL` reader - SDIO/USBFS clock selection"]
pub type CK48MSEL_R = crate::BitReader<CK48MSEL>;
impl CK48MSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CK48MSEL {
        match self.bits {
            false => CK48MSEL::Pll,
            true => CK48MSEL::Pllsai,
        }
    }
    #[doc = "48MHz clock from PLL is selected"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == CK48MSEL::Pll
    }
    #[doc = "48MHz clock from PLLSAI is selected"]
    #[inline(always)]
    pub fn is_pllsai(&self) -> bool {
        *self == CK48MSEL::Pllsai
    }
}
#[doc = "Field `CK48MSEL` writer - SDIO/USBFS clock selection"]
pub type CK48MSEL_W<'a, REG> = crate::BitWriter<'a, REG, CK48MSEL>;
impl<'a, REG> CK48MSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "48MHz clock from PLL is selected"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut crate::W<REG> {
        self.variant(CK48MSEL::Pll)
    }
    #[doc = "48MHz clock from PLLSAI is selected"]
    #[inline(always)]
    pub fn pllsai(self) -> &'a mut crate::W<REG> {
        self.variant(CK48MSEL::Pllsai)
    }
}
#[doc = "SDIO clock selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDIOSEL {
    #[doc = "0: 48 MHz clock is selected as SD clock"]
    Ck48m = 0,
    #[doc = "1: System clock is selected as SD clock"]
    Sysclk = 1,
}
impl From<SDIOSEL> for bool {
    #[inline(always)]
    fn from(variant: SDIOSEL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDIOSEL` reader - SDIO clock selection"]
pub type SDIOSEL_R = crate::BitReader<SDIOSEL>;
impl SDIOSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SDIOSEL {
        match self.bits {
            false => SDIOSEL::Ck48m,
            true => SDIOSEL::Sysclk,
        }
    }
    #[doc = "48 MHz clock is selected as SD clock"]
    #[inline(always)]
    pub fn is_ck48m(&self) -> bool {
        *self == SDIOSEL::Ck48m
    }
    #[doc = "System clock is selected as SD clock"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == SDIOSEL::Sysclk
    }
}
#[doc = "Field `SDIOSEL` writer - SDIO clock selection"]
pub type SDIOSEL_W<'a, REG> = crate::BitWriter<'a, REG, SDIOSEL>;
impl<'a, REG> SDIOSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "48 MHz clock is selected as SD clock"]
    #[inline(always)]
    pub fn ck48m(self) -> &'a mut crate::W<REG> {
        self.variant(SDIOSEL::Ck48m)
    }
    #[doc = "System clock is selected as SD clock"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(SDIOSEL::Sysclk)
    }
}
impl R {
    #[doc = "Bits 22:23 - FMPI2C1 kernel clock source selection"]
    #[inline(always)]
    pub fn fmpi2c1sel(&self) -> FMPI2C1SEL_R {
        FMPI2C1SEL_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 27 - SDIO/USBFS clock selection"]
    #[inline(always)]
    pub fn ck48msel(&self) -> CK48MSEL_R {
        CK48MSEL_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - SDIO clock selection"]
    #[inline(always)]
    pub fn sdiosel(&self) -> SDIOSEL_R {
        SDIOSEL_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 22:23 - FMPI2C1 kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn fmpi2c1sel(&mut self) -> FMPI2C1SEL_W<DCKCFGR2rs> {
        FMPI2C1SEL_W::new(self, 22)
    }
    #[doc = "Bit 27 - SDIO/USBFS clock selection"]
    #[inline(always)]
    #[must_use]
    pub fn ck48msel(&mut self) -> CK48MSEL_W<DCKCFGR2rs> {
        CK48MSEL_W::new(self, 27)
    }
    #[doc = "Bit 28 - SDIO clock selection"]
    #[inline(always)]
    #[must_use]
    pub fn sdiosel(&mut self) -> SDIOSEL_W<DCKCFGR2rs> {
        SDIOSEL_W::new(self, 28)
    }
}
#[doc = "Dedicated Clock Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dckcfgr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dckcfgr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCKCFGR2rs;
impl crate::RegisterSpec for DCKCFGR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dckcfgr2::R`](R) reader structure"]
impl crate::Readable for DCKCFGR2rs {}
#[doc = "`write(|w| ..)` method takes [`dckcfgr2::W`](W) writer structure"]
impl crate::Writable for DCKCFGR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCKCFGR2 to value 0"]
impl crate::Resettable for DCKCFGR2rs {
    const RESET_VALUE: u32 = 0;
}

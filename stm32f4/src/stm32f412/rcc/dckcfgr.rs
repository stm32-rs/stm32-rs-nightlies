#[doc = "Register `DCKCFGR` reader"]
pub type R = crate::R<DCKCFGRrs>;
#[doc = "Register `DCKCFGR` writer"]
pub type W = crate::W<DCKCFGRrs>;
#[doc = "DFSDM1 audio clock selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKDFSDM1ASEL {
    #[doc = "0: CK_I2S_APB1 selected as audio clock"]
    I2s1 = 0,
    #[doc = "1: CK_I2S_APB2 selected as audio clock"]
    I2s2 = 1,
}
impl From<CKDFSDM1ASEL> for u8 {
    #[inline(always)]
    fn from(variant: CKDFSDM1ASEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CKDFSDM1ASEL {
    type Ux = u8;
}
#[doc = "Field `CKDFSDM1ASEL` reader - DFSDM1 audio clock selection"]
pub type CKDFSDM1ASEL_R = crate::FieldReader<CKDFSDM1ASEL>;
impl CKDFSDM1ASEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CKDFSDM1ASEL> {
        match self.bits {
            0 => Some(CKDFSDM1ASEL::I2s1),
            1 => Some(CKDFSDM1ASEL::I2s2),
            _ => None,
        }
    }
    #[doc = "CK_I2S_APB1 selected as audio clock"]
    #[inline(always)]
    pub fn is_i2s1(&self) -> bool {
        *self == CKDFSDM1ASEL::I2s1
    }
    #[doc = "CK_I2S_APB2 selected as audio clock"]
    #[inline(always)]
    pub fn is_i2s2(&self) -> bool {
        *self == CKDFSDM1ASEL::I2s2
    }
}
#[doc = "Field `CKDFSDM1ASEL` writer - DFSDM1 audio clock selection"]
pub type CKDFSDM1ASEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5, CKDFSDM1ASEL>;
impl<'a, REG> CKDFSDM1ASEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CK_I2S_APB1 selected as audio clock"]
    #[inline(always)]
    pub fn i2s1(self) -> &'a mut crate::W<REG> {
        self.variant(CKDFSDM1ASEL::I2s1)
    }
    #[doc = "CK_I2S_APB2 selected as audio clock"]
    #[inline(always)]
    pub fn i2s2(self) -> &'a mut crate::W<REG> {
        self.variant(CKDFSDM1ASEL::I2s2)
    }
}
#[doc = "Timers clocks prescalers selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMPRE {
    #[doc = "0: If the APB prescaler is configured 1, TIMxCLK = PCLKx. Otherwise, TIMxCLK = 2xPCLKx"]
    Mul1or2 = 0,
    #[doc = "1: If the APB prescaler is configured 1, 2 or 4, TIMxCLK = HCLK. Otherwise, TIMxCLK = 4xPCLKx"]
    Mul1or4 = 1,
}
impl From<TIMPRE> for bool {
    #[inline(always)]
    fn from(variant: TIMPRE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMPRE` reader - Timers clocks prescalers selection"]
pub type TIMPRE_R = crate::BitReader<TIMPRE>;
impl TIMPRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIMPRE {
        match self.bits {
            false => TIMPRE::Mul1or2,
            true => TIMPRE::Mul1or4,
        }
    }
    #[doc = "If the APB prescaler is configured 1, TIMxCLK = PCLKx. Otherwise, TIMxCLK = 2xPCLKx"]
    #[inline(always)]
    pub fn is_mul1or2(&self) -> bool {
        *self == TIMPRE::Mul1or2
    }
    #[doc = "If the APB prescaler is configured 1, 2 or 4, TIMxCLK = HCLK. Otherwise, TIMxCLK = 4xPCLKx"]
    #[inline(always)]
    pub fn is_mul1or4(&self) -> bool {
        *self == TIMPRE::Mul1or4
    }
}
#[doc = "Field `TIMPRE` writer - Timers clocks prescalers selection"]
pub type TIMPRE_W<'a, REG> = crate::BitWriter<'a, REG, TIMPRE>;
impl<'a, REG> TIMPRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "If the APB prescaler is configured 1, TIMxCLK = PCLKx. Otherwise, TIMxCLK = 2xPCLKx"]
    #[inline(always)]
    pub fn mul1or2(self) -> &'a mut crate::W<REG> {
        self.variant(TIMPRE::Mul1or2)
    }
    #[doc = "If the APB prescaler is configured 1, 2 or 4, TIMxCLK = HCLK. Otherwise, TIMxCLK = 4xPCLKx"]
    #[inline(always)]
    pub fn mul1or4(self) -> &'a mut crate::W<REG> {
        self.variant(TIMPRE::Mul1or4)
    }
}
#[doc = "I2S APB1 clocks source selection (I2S2/3)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2S1SRC {
    #[doc = "0: I2Sx clock frequency = f(PLLI2S_R)"]
    Plli2sr = 0,
    #[doc = "1: I2Sx clock frequency = I2S_CKIN Alternate function input frequency"]
    I2sCkin = 1,
    #[doc = "2: I2Sx clock frequency = f(PLL_R)"]
    Pllr = 2,
    #[doc = "3: I2Sx clock frequency = HSI/HSE depends on PLLSRC bit (PLLCFGR\\[22\\])"]
    HsiHse = 3,
}
impl From<I2S1SRC> for u8 {
    #[inline(always)]
    fn from(variant: I2S1SRC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I2S1SRC {
    type Ux = u8;
}
#[doc = "Field `I2S1SRC` reader - I2S APB1 clocks source selection (I2S2/3)"]
pub type I2S1SRC_R = crate::FieldReader<I2S1SRC>;
impl I2S1SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2S1SRC {
        match self.bits {
            0 => I2S1SRC::Plli2sr,
            1 => I2S1SRC::I2sCkin,
            2 => I2S1SRC::Pllr,
            3 => I2S1SRC::HsiHse,
            _ => unreachable!(),
        }
    }
    #[doc = "I2Sx clock frequency = f(PLLI2S_R)"]
    #[inline(always)]
    pub fn is_plli2sr(&self) -> bool {
        *self == I2S1SRC::Plli2sr
    }
    #[doc = "I2Sx clock frequency = I2S_CKIN Alternate function input frequency"]
    #[inline(always)]
    pub fn is_i2s_ckin(&self) -> bool {
        *self == I2S1SRC::I2sCkin
    }
    #[doc = "I2Sx clock frequency = f(PLL_R)"]
    #[inline(always)]
    pub fn is_pllr(&self) -> bool {
        *self == I2S1SRC::Pllr
    }
    #[doc = "I2Sx clock frequency = HSI/HSE depends on PLLSRC bit (PLLCFGR\\[22\\])"]
    #[inline(always)]
    pub fn is_hsi_hse(&self) -> bool {
        *self == I2S1SRC::HsiHse
    }
}
#[doc = "Field `I2S1SRC` writer - I2S APB1 clocks source selection (I2S2/3)"]
pub type I2S1SRC_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, I2S1SRC>;
impl<'a, REG> I2S1SRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "I2Sx clock frequency = f(PLLI2S_R)"]
    #[inline(always)]
    pub fn plli2sr(self) -> &'a mut crate::W<REG> {
        self.variant(I2S1SRC::Plli2sr)
    }
    #[doc = "I2Sx clock frequency = I2S_CKIN Alternate function input frequency"]
    #[inline(always)]
    pub fn i2s_ckin(self) -> &'a mut crate::W<REG> {
        self.variant(I2S1SRC::I2sCkin)
    }
    #[doc = "I2Sx clock frequency = f(PLL_R)"]
    #[inline(always)]
    pub fn pllr(self) -> &'a mut crate::W<REG> {
        self.variant(I2S1SRC::Pllr)
    }
    #[doc = "I2Sx clock frequency = HSI/HSE depends on PLLSRC bit (PLLCFGR\\[22\\])"]
    #[inline(always)]
    pub fn hsi_hse(self) -> &'a mut crate::W<REG> {
        self.variant(I2S1SRC::HsiHse)
    }
}
#[doc = "Field `I2S2SRC` reader - I2S APB2 clocks source selection (I2S1/4/5)"]
pub use I2S1SRC_R as I2S2SRC_R;
#[doc = "Field `I2S2SRC` writer - I2S APB2 clocks source selection (I2S1/4/5)"]
pub use I2S1SRC_W as I2S2SRC_W;
#[doc = "DFSDM1 Kernel clock selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKDFSDM1SEL {
    #[doc = "0: APB2 clock used as Kernel clock"]
    Apb2 = 0,
    #[doc = "1: System clock used as Kernel clock"]
    Sysclk = 1,
}
impl From<CKDFSDM1SEL> for bool {
    #[inline(always)]
    fn from(variant: CKDFSDM1SEL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKDFSDM1SEL` reader - DFSDM1 Kernel clock selection"]
pub type CKDFSDM1SEL_R = crate::BitReader<CKDFSDM1SEL>;
impl CKDFSDM1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CKDFSDM1SEL {
        match self.bits {
            false => CKDFSDM1SEL::Apb2,
            true => CKDFSDM1SEL::Sysclk,
        }
    }
    #[doc = "APB2 clock used as Kernel clock"]
    #[inline(always)]
    pub fn is_apb2(&self) -> bool {
        *self == CKDFSDM1SEL::Apb2
    }
    #[doc = "System clock used as Kernel clock"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == CKDFSDM1SEL::Sysclk
    }
}
#[doc = "Field `CKDFSDM1SEL` writer - DFSDM1 Kernel clock selection"]
pub type CKDFSDM1SEL_W<'a, REG> = crate::BitWriter<'a, REG, CKDFSDM1SEL>;
impl<'a, REG> CKDFSDM1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "APB2 clock used as Kernel clock"]
    #[inline(always)]
    pub fn apb2(self) -> &'a mut crate::W<REG> {
        self.variant(CKDFSDM1SEL::Apb2)
    }
    #[doc = "System clock used as Kernel clock"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(CKDFSDM1SEL::Sysclk)
    }
}
impl R {
    #[doc = "Bits 15:19 - DFSDM1 audio clock selection"]
    #[inline(always)]
    pub fn ckdfsdm1asel(&self) -> CKDFSDM1ASEL_R {
        CKDFSDM1ASEL_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - Timers clocks prescalers selection"]
    #[inline(always)]
    pub fn timpre(&self) -> TIMPRE_R {
        TIMPRE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - I2S APB1 clocks source selection (I2S2/3)"]
    #[inline(always)]
    pub fn i2s1src(&self) -> I2S1SRC_R {
        I2S1SRC_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bits 27:28 - I2S APB2 clocks source selection (I2S1/4/5)"]
    #[inline(always)]
    pub fn i2s2src(&self) -> I2S2SRC_R {
        I2S2SRC_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 31 - DFSDM1 Kernel clock selection"]
    #[inline(always)]
    pub fn ckdfsdm1sel(&self) -> CKDFSDM1SEL_R {
        CKDFSDM1SEL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 15:19 - DFSDM1 audio clock selection"]
    #[inline(always)]
    #[must_use]
    pub fn ckdfsdm1asel(&mut self) -> CKDFSDM1ASEL_W<DCKCFGRrs> {
        CKDFSDM1ASEL_W::new(self, 15)
    }
    #[doc = "Bit 24 - Timers clocks prescalers selection"]
    #[inline(always)]
    #[must_use]
    pub fn timpre(&mut self) -> TIMPRE_W<DCKCFGRrs> {
        TIMPRE_W::new(self, 24)
    }
    #[doc = "Bits 25:26 - I2S APB1 clocks source selection (I2S2/3)"]
    #[inline(always)]
    #[must_use]
    pub fn i2s1src(&mut self) -> I2S1SRC_W<DCKCFGRrs> {
        I2S1SRC_W::new(self, 25)
    }
    #[doc = "Bits 27:28 - I2S APB2 clocks source selection (I2S1/4/5)"]
    #[inline(always)]
    #[must_use]
    pub fn i2s2src(&mut self) -> I2S2SRC_W<DCKCFGRrs> {
        I2S2SRC_W::new(self, 27)
    }
    #[doc = "Bit 31 - DFSDM1 Kernel clock selection"]
    #[inline(always)]
    #[must_use]
    pub fn ckdfsdm1sel(&mut self) -> CKDFSDM1SEL_W<DCKCFGRrs> {
        CKDFSDM1SEL_W::new(self, 31)
    }
}
#[doc = "Dedicated Clock Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dckcfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dckcfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCKCFGRrs;
impl crate::RegisterSpec for DCKCFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dckcfgr::R`](R) reader structure"]
impl crate::Readable for DCKCFGRrs {}
#[doc = "`write(|w| ..)` method takes [`dckcfgr::W`](W) writer structure"]
impl crate::Writable for DCKCFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCKCFGR to value 0"]
impl crate::Resettable for DCKCFGRrs {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `DCKCFGR` reader"]
pub type R = crate::R<DCKCFGRrs>;
#[doc = "Register `DCKCFGR` writer"]
pub type W = crate::W<DCKCFGRrs>;
#[doc = "TIMPRE\n\nValue on reset: 0"]
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
#[doc = "Field `TIMPRE` reader - TIMPRE"]
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
#[doc = "Field `TIMPRE` writer - TIMPRE"]
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
#[doc = "I2SSRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2SSRC {
    #[doc = "0: I2Sx clock frequency = f(PLLCLK_R)"]
    Pllclkr = 0,
    #[doc = "1: I2Sx clock frequency = I2S_CKIN Alternate function input frequency"]
    I2sCkin = 1,
    #[doc = "3: I2Sx clock frequency = HSI/HSE depends on PLLSRC bit (PLLCFGR\\[22\\])"]
    HsiHse = 3,
}
impl From<I2SSRC> for u8 {
    #[inline(always)]
    fn from(variant: I2SSRC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I2SSRC {
    type Ux = u8;
}
#[doc = "Field `I2SSRC` reader - I2SSRC"]
pub type I2SSRC_R = crate::FieldReader<I2SSRC>;
impl I2SSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<I2SSRC> {
        match self.bits {
            0 => Some(I2SSRC::Pllclkr),
            1 => Some(I2SSRC::I2sCkin),
            3 => Some(I2SSRC::HsiHse),
            _ => None,
        }
    }
    #[doc = "I2Sx clock frequency = f(PLLCLK_R)"]
    #[inline(always)]
    pub fn is_pllclkr(&self) -> bool {
        *self == I2SSRC::Pllclkr
    }
    #[doc = "I2Sx clock frequency = I2S_CKIN Alternate function input frequency"]
    #[inline(always)]
    pub fn is_i2s_ckin(&self) -> bool {
        *self == I2SSRC::I2sCkin
    }
    #[doc = "I2Sx clock frequency = HSI/HSE depends on PLLSRC bit (PLLCFGR\\[22\\])"]
    #[inline(always)]
    pub fn is_hsi_hse(&self) -> bool {
        *self == I2SSRC::HsiHse
    }
}
#[doc = "Field `I2SSRC` writer - I2SSRC"]
pub type I2SSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, I2SSRC>;
impl<'a, REG> I2SSRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "I2Sx clock frequency = f(PLLCLK_R)"]
    #[inline(always)]
    pub fn pllclkr(self) -> &'a mut crate::W<REG> {
        self.variant(I2SSRC::Pllclkr)
    }
    #[doc = "I2Sx clock frequency = I2S_CKIN Alternate function input frequency"]
    #[inline(always)]
    pub fn i2s_ckin(self) -> &'a mut crate::W<REG> {
        self.variant(I2SSRC::I2sCkin)
    }
    #[doc = "I2Sx clock frequency = HSI/HSE depends on PLLSRC bit (PLLCFGR\\[22\\])"]
    #[inline(always)]
    pub fn hsi_hse(self) -> &'a mut crate::W<REG> {
        self.variant(I2SSRC::HsiHse)
    }
}
impl R {
    #[doc = "Bit 24 - TIMPRE"]
    #[inline(always)]
    pub fn timpre(&self) -> TIMPRE_R {
        TIMPRE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - I2SSRC"]
    #[inline(always)]
    pub fn i2ssrc(&self) -> I2SSRC_R {
        I2SSRC_R::new(((self.bits >> 25) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 24 - TIMPRE"]
    #[inline(always)]
    #[must_use]
    pub fn timpre(&mut self) -> TIMPRE_W<DCKCFGRrs> {
        TIMPRE_W::new(self, 24)
    }
    #[doc = "Bits 25:26 - I2SSRC"]
    #[inline(always)]
    #[must_use]
    pub fn i2ssrc(&mut self) -> I2SSRC_W<DCKCFGRrs> {
        I2SSRC_W::new(self, 25)
    }
}
#[doc = "DCKCFGR register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dckcfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dckcfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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

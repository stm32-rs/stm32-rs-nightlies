///Register `DCKCFGR` reader
pub type R = crate::R<DCKCFGRrs>;
///Register `DCKCFGR` writer
pub type W = crate::W<DCKCFGRrs>;
/**TIMPRE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMPRE {
    ///0: If the APB prescaler is configured 1, TIMxCLK = PCLKx. Otherwise, TIMxCLK = 2xPCLKx
    Mul1or2 = 0,
    ///1: If the APB prescaler is configured 1, 2 or 4, TIMxCLK = HCLK. Otherwise, TIMxCLK = 4xPCLKx
    Mul1or4 = 1,
}
impl From<TIMPRE> for bool {
    #[inline(always)]
    fn from(variant: TIMPRE) -> Self {
        variant as u8 != 0
    }
}
///Field `TIMPRE` reader - TIMPRE
pub type TIMPRE_R = crate::BitReader<TIMPRE>;
impl TIMPRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIMPRE {
        match self.bits {
            false => TIMPRE::Mul1or2,
            true => TIMPRE::Mul1or4,
        }
    }
    ///If the APB prescaler is configured 1, TIMxCLK = PCLKx. Otherwise, TIMxCLK = 2xPCLKx
    #[inline(always)]
    pub fn is_mul1or2(&self) -> bool {
        *self == TIMPRE::Mul1or2
    }
    ///If the APB prescaler is configured 1, 2 or 4, TIMxCLK = HCLK. Otherwise, TIMxCLK = 4xPCLKx
    #[inline(always)]
    pub fn is_mul1or4(&self) -> bool {
        *self == TIMPRE::Mul1or4
    }
}
///Field `TIMPRE` writer - TIMPRE
pub type TIMPRE_W<'a, REG> = crate::BitWriter<'a, REG, TIMPRE>;
impl<'a, REG> TIMPRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///If the APB prescaler is configured 1, TIMxCLK = PCLKx. Otherwise, TIMxCLK = 2xPCLKx
    #[inline(always)]
    pub fn mul1or2(self) -> &'a mut crate::W<REG> {
        self.variant(TIMPRE::Mul1or2)
    }
    ///If the APB prescaler is configured 1, 2 or 4, TIMxCLK = HCLK. Otherwise, TIMxCLK = 4xPCLKx
    #[inline(always)]
    pub fn mul1or4(self) -> &'a mut crate::W<REG> {
        self.variant(TIMPRE::Mul1or4)
    }
}
/**I2SSRC

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2SSRC {
    ///0: I2Sx clock frequency = f(PLLCLK_R)
    Pllclkr = 0,
    ///1: I2Sx clock frequency = I2S_CKIN Alternate function input frequency
    I2sCkin = 1,
    ///3: I2Sx clock frequency = HSI/HSE depends on PLLSRC bit (PLLCFGR\[22\])
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
impl crate::IsEnum for I2SSRC {}
///Field `I2SSRC` reader - I2SSRC
pub type I2SSRC_R = crate::FieldReader<I2SSRC>;
impl I2SSRC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<I2SSRC> {
        match self.bits {
            0 => Some(I2SSRC::Pllclkr),
            1 => Some(I2SSRC::I2sCkin),
            3 => Some(I2SSRC::HsiHse),
            _ => None,
        }
    }
    ///I2Sx clock frequency = f(PLLCLK_R)
    #[inline(always)]
    pub fn is_pllclkr(&self) -> bool {
        *self == I2SSRC::Pllclkr
    }
    ///I2Sx clock frequency = I2S_CKIN Alternate function input frequency
    #[inline(always)]
    pub fn is_i2s_ckin(&self) -> bool {
        *self == I2SSRC::I2sCkin
    }
    ///I2Sx clock frequency = HSI/HSE depends on PLLSRC bit (PLLCFGR\[22\])
    #[inline(always)]
    pub fn is_hsi_hse(&self) -> bool {
        *self == I2SSRC::HsiHse
    }
}
///Field `I2SSRC` writer - I2SSRC
pub type I2SSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, I2SSRC>;
impl<'a, REG> I2SSRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///I2Sx clock frequency = f(PLLCLK_R)
    #[inline(always)]
    pub fn pllclkr(self) -> &'a mut crate::W<REG> {
        self.variant(I2SSRC::Pllclkr)
    }
    ///I2Sx clock frequency = I2S_CKIN Alternate function input frequency
    #[inline(always)]
    pub fn i2s_ckin(self) -> &'a mut crate::W<REG> {
        self.variant(I2SSRC::I2sCkin)
    }
    ///I2Sx clock frequency = HSI/HSE depends on PLLSRC bit (PLLCFGR\[22\])
    #[inline(always)]
    pub fn hsi_hse(self) -> &'a mut crate::W<REG> {
        self.variant(I2SSRC::HsiHse)
    }
}
impl R {
    ///Bit 24 - TIMPRE
    #[inline(always)]
    pub fn timpre(&self) -> TIMPRE_R {
        TIMPRE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:26 - I2SSRC
    #[inline(always)]
    pub fn i2ssrc(&self) -> I2SSRC_R {
        I2SSRC_R::new(((self.bits >> 25) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCKCFGR")
            .field("timpre", &self.timpre())
            .field("i2ssrc", &self.i2ssrc())
            .finish()
    }
}
impl W {
    ///Bit 24 - TIMPRE
    #[inline(always)]
    pub fn timpre(&mut self) -> TIMPRE_W<'_, DCKCFGRrs> {
        TIMPRE_W::new(self, 24)
    }
    ///Bits 25:26 - I2SSRC
    #[inline(always)]
    pub fn i2ssrc(&mut self) -> I2SSRC_W<'_, DCKCFGRrs> {
        I2SSRC_W::new(self, 25)
    }
}
/**DCKCFGR register

You can [`read`](crate::Reg::read) this register and get [`dckcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dckcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F410.html#RCC:DCKCFGR)*/
pub struct DCKCFGRrs;
impl crate::RegisterSpec for DCKCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`dckcfgr::R`](R) reader structure
impl crate::Readable for DCKCFGRrs {}
///`write(|w| ..)` method takes [`dckcfgr::W`](W) writer structure
impl crate::Writable for DCKCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DCKCFGR to value 0
impl crate::Resettable for DCKCFGRrs {}

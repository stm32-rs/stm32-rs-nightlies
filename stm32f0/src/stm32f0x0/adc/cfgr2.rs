#[doc = "Register `CFGR2` reader"]
pub type R = crate::R<CFGR2rs>;
#[doc = "Register `CFGR2` writer"]
pub type W = crate::W<CFGR2rs>;
#[doc = "ADC clock mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKMODE {
    #[doc = "0: Asynchronous clock mode"]
    Adcclk = 0,
    #[doc = "1: Synchronous clock mode (PCLK/2)"]
    PclkDiv2 = 1,
    #[doc = "2: Sychronous clock mode (PCLK/4)"]
    PclkDiv4 = 2,
}
impl From<CKMODE> for u8 {
    #[inline(always)]
    fn from(variant: CKMODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CKMODE {
    type Ux = u8;
}
#[doc = "Field `CKMODE` reader - ADC clock mode"]
pub type CKMODE_R = crate::FieldReader<CKMODE>;
impl CKMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CKMODE> {
        match self.bits {
            0 => Some(CKMODE::Adcclk),
            1 => Some(CKMODE::PclkDiv2),
            2 => Some(CKMODE::PclkDiv4),
            _ => None,
        }
    }
    #[doc = "Asynchronous clock mode"]
    #[inline(always)]
    pub fn is_adcclk(&self) -> bool {
        *self == CKMODE::Adcclk
    }
    #[doc = "Synchronous clock mode (PCLK/2)"]
    #[inline(always)]
    pub fn is_pclk_div2(&self) -> bool {
        *self == CKMODE::PclkDiv2
    }
    #[doc = "Sychronous clock mode (PCLK/4)"]
    #[inline(always)]
    pub fn is_pclk_div4(&self) -> bool {
        *self == CKMODE::PclkDiv4
    }
}
#[doc = "Field `CKMODE` writer - ADC clock mode"]
pub type CKMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CKMODE>;
impl<'a, REG> CKMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Asynchronous clock mode"]
    #[inline(always)]
    pub fn adcclk(self) -> &'a mut crate::W<REG> {
        self.variant(CKMODE::Adcclk)
    }
    #[doc = "Synchronous clock mode (PCLK/2)"]
    #[inline(always)]
    pub fn pclk_div2(self) -> &'a mut crate::W<REG> {
        self.variant(CKMODE::PclkDiv2)
    }
    #[doc = "Sychronous clock mode (PCLK/4)"]
    #[inline(always)]
    pub fn pclk_div4(self) -> &'a mut crate::W<REG> {
        self.variant(CKMODE::PclkDiv4)
    }
}
impl R {
    #[doc = "Bits 30:31 - ADC clock mode"]
    #[inline(always)]
    pub fn ckmode(&self) -> CKMODE_R {
        CKMODE_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - ADC clock mode"]
    #[inline(always)]
    #[must_use]
    pub fn ckmode(&mut self) -> CKMODE_W<CFGR2rs> {
        CKMODE_W::new(self, 30)
    }
}
#[doc = "configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGR2rs;
impl crate::RegisterSpec for CFGR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr2::R`](R) reader structure"]
impl crate::Readable for CFGR2rs {}
#[doc = "`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure"]
impl crate::Writable for CFGR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR2 to value 0x8000"]
impl crate::Resettable for CFGR2rs {
    const RESET_VALUE: u32 = 0x8000;
}

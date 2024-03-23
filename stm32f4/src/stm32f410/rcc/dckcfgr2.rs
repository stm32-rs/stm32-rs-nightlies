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
#[doc = "LPTIM1SEL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPTIM1SEL {
    #[doc = "0: APB1 clock (PCLK1) selected as LPTILM1 clock"]
    Apb1 = 0,
    #[doc = "1: LSI clock is selected as LPTILM1 clock"]
    Lsi = 1,
    #[doc = "2: HSI clock is selected as LPTILM1 clock"]
    Hsi = 2,
    #[doc = "3: LSE clock is selected as LPTILM1 clock"]
    Lse = 3,
}
impl From<LPTIM1SEL> for u8 {
    #[inline(always)]
    fn from(variant: LPTIM1SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LPTIM1SEL {
    type Ux = u8;
}
#[doc = "Field `LPTIM1SEL` reader - LPTIM1SEL"]
pub type LPTIM1SEL_R = crate::FieldReader<LPTIM1SEL>;
impl LPTIM1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPTIM1SEL {
        match self.bits {
            0 => LPTIM1SEL::Apb1,
            1 => LPTIM1SEL::Lsi,
            2 => LPTIM1SEL::Hsi,
            3 => LPTIM1SEL::Lse,
            _ => unreachable!(),
        }
    }
    #[doc = "APB1 clock (PCLK1) selected as LPTILM1 clock"]
    #[inline(always)]
    pub fn is_apb1(&self) -> bool {
        *self == LPTIM1SEL::Apb1
    }
    #[doc = "LSI clock is selected as LPTILM1 clock"]
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == LPTIM1SEL::Lsi
    }
    #[doc = "HSI clock is selected as LPTILM1 clock"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == LPTIM1SEL::Hsi
    }
    #[doc = "LSE clock is selected as LPTILM1 clock"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == LPTIM1SEL::Lse
    }
}
#[doc = "Field `LPTIM1SEL` writer - LPTIM1SEL"]
pub type LPTIM1SEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, LPTIM1SEL>;
impl<'a, REG> LPTIM1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "APB1 clock (PCLK1) selected as LPTILM1 clock"]
    #[inline(always)]
    pub fn apb1(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::Apb1)
    }
    #[doc = "LSI clock is selected as LPTILM1 clock"]
    #[inline(always)]
    pub fn lsi(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::Lsi)
    }
    #[doc = "HSI clock is selected as LPTILM1 clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::Hsi)
    }
    #[doc = "LSE clock is selected as LPTILM1 clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::Lse)
    }
}
impl R {
    #[doc = "Bits 22:23 - FMPI2C1 kernel clock source selection"]
    #[inline(always)]
    pub fn fmpi2c1sel(&self) -> FMPI2C1SEL_R {
        FMPI2C1SEL_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 30:31 - LPTIM1SEL"]
    #[inline(always)]
    pub fn lptim1sel(&self) -> LPTIM1SEL_R {
        LPTIM1SEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 22:23 - FMPI2C1 kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn fmpi2c1sel(&mut self) -> FMPI2C1SEL_W<DCKCFGR2rs> {
        FMPI2C1SEL_W::new(self, 22)
    }
    #[doc = "Bits 30:31 - LPTIM1SEL"]
    #[inline(always)]
    #[must_use]
    pub fn lptim1sel(&mut self) -> LPTIM1SEL_W<DCKCFGR2rs> {
        LPTIM1SEL_W::new(self, 30)
    }
}
#[doc = "DCKCFGR2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dckcfgr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dckcfgr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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

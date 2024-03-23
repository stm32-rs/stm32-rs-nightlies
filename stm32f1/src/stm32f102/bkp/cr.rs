#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Tamper pin enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TPE {
    #[doc = "0: The TAMPER pin is free for general purpose I/O"]
    General = 0,
    #[doc = "1: Tamper alternate I/O function is activated"]
    Alternate = 1,
}
impl From<TPE> for bool {
    #[inline(always)]
    fn from(variant: TPE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TPE` reader - Tamper pin enable"]
pub type TPE_R = crate::BitReader<TPE>;
impl TPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TPE {
        match self.bits {
            false => TPE::General,
            true => TPE::Alternate,
        }
    }
    #[doc = "The TAMPER pin is free for general purpose I/O"]
    #[inline(always)]
    pub fn is_general(&self) -> bool {
        *self == TPE::General
    }
    #[doc = "Tamper alternate I/O function is activated"]
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == TPE::Alternate
    }
}
#[doc = "Field `TPE` writer - Tamper pin enable"]
pub type TPE_W<'a, REG> = crate::BitWriter<'a, REG, TPE>;
impl<'a, REG> TPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The TAMPER pin is free for general purpose I/O"]
    #[inline(always)]
    pub fn general(self) -> &'a mut crate::W<REG> {
        self.variant(TPE::General)
    }
    #[doc = "Tamper alternate I/O function is activated"]
    #[inline(always)]
    pub fn alternate(self) -> &'a mut crate::W<REG> {
        self.variant(TPE::Alternate)
    }
}
#[doc = "Tamper pin active level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TPAL {
    #[doc = "0: A high level on the TAMPER pin resets all data backup registers (if TPE bit is set)"]
    High = 0,
    #[doc = "1: A low level on the TAMPER pin resets all data backup registers (if TPE bit is set)"]
    Low = 1,
}
impl From<TPAL> for bool {
    #[inline(always)]
    fn from(variant: TPAL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TPAL` reader - Tamper pin active level"]
pub type TPAL_R = crate::BitReader<TPAL>;
impl TPAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TPAL {
        match self.bits {
            false => TPAL::High,
            true => TPAL::Low,
        }
    }
    #[doc = "A high level on the TAMPER pin resets all data backup registers (if TPE bit is set)"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == TPAL::High
    }
    #[doc = "A low level on the TAMPER pin resets all data backup registers (if TPE bit is set)"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == TPAL::Low
    }
}
#[doc = "Field `TPAL` writer - Tamper pin active level"]
pub type TPAL_W<'a, REG> = crate::BitWriter<'a, REG, TPAL>;
impl<'a, REG> TPAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A high level on the TAMPER pin resets all data backup registers (if TPE bit is set)"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(TPAL::High)
    }
    #[doc = "A low level on the TAMPER pin resets all data backup registers (if TPE bit is set)"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(TPAL::Low)
    }
}
impl R {
    #[doc = "Bit 0 - Tamper pin enable"]
    #[inline(always)]
    pub fn tpe(&self) -> TPE_R {
        TPE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tamper pin active level"]
    #[inline(always)]
    pub fn tpal(&self) -> TPAL_R {
        TPAL_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tamper pin enable"]
    #[inline(always)]
    #[must_use]
    pub fn tpe(&mut self) -> TPE_W<CRrs> {
        TPE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Tamper pin active level"]
    #[inline(always)]
    #[must_use]
    pub fn tpal(&mut self) -> TPAL_W<CRrs> {
        TPAL_W::new(self, 1)
    }
}
#[doc = "Backup control register (BKP_CR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CRrs {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}

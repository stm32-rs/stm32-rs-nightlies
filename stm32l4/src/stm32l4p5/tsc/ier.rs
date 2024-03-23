#[doc = "Register `IER` reader"]
pub type R = crate::R<IERrs>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IERrs>;
#[doc = "End of acquisition interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOAIE {
    #[doc = "0: End of acquisition interrupt disabled"]
    Disabled = 0,
    #[doc = "1: End of acquisition interrupt enabled"]
    Enabled = 1,
}
impl From<EOAIE> for bool {
    #[inline(always)]
    fn from(variant: EOAIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOAIE` reader - End of acquisition interrupt enable"]
pub type EOAIE_R = crate::BitReader<EOAIE>;
impl EOAIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EOAIE {
        match self.bits {
            false => EOAIE::Disabled,
            true => EOAIE::Enabled,
        }
    }
    #[doc = "End of acquisition interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EOAIE::Disabled
    }
    #[doc = "End of acquisition interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EOAIE::Enabled
    }
}
#[doc = "Field `EOAIE` writer - End of acquisition interrupt enable"]
pub type EOAIE_W<'a, REG> = crate::BitWriter<'a, REG, EOAIE>;
impl<'a, REG> EOAIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "End of acquisition interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOAIE::Disabled)
    }
    #[doc = "End of acquisition interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOAIE::Enabled)
    }
}
#[doc = "Max count error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCEIE {
    #[doc = "0: Max count error interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Max count error interrupt enabled"]
    Enabled = 1,
}
impl From<MCEIE> for bool {
    #[inline(always)]
    fn from(variant: MCEIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCEIE` reader - Max count error interrupt enable"]
pub type MCEIE_R = crate::BitReader<MCEIE>;
impl MCEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MCEIE {
        match self.bits {
            false => MCEIE::Disabled,
            true => MCEIE::Enabled,
        }
    }
    #[doc = "Max count error interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MCEIE::Disabled
    }
    #[doc = "Max count error interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MCEIE::Enabled
    }
}
#[doc = "Field `MCEIE` writer - Max count error interrupt enable"]
pub type MCEIE_W<'a, REG> = crate::BitWriter<'a, REG, MCEIE>;
impl<'a, REG> MCEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Max count error interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MCEIE::Disabled)
    }
    #[doc = "Max count error interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MCEIE::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - End of acquisition interrupt enable"]
    #[inline(always)]
    pub fn eoaie(&self) -> EOAIE_R {
        EOAIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Max count error interrupt enable"]
    #[inline(always)]
    pub fn mceie(&self) -> MCEIE_R {
        MCEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - End of acquisition interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eoaie(&mut self) -> EOAIE_W<IERrs> {
        EOAIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Max count error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn mceie(&mut self) -> MCEIE_W<IERrs> {
        MCEIE_W::new(self, 1)
    }
}
#[doc = "interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IERrs;
impl crate::RegisterSpec for IERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IERrs {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IERrs {
    const RESET_VALUE: u32 = 0;
}

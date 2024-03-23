#[doc = "Register `SSCGR` reader"]
pub type R = crate::R<SSCGRrs>;
#[doc = "Register `SSCGR` writer"]
pub type W = crate::W<SSCGRrs>;
#[doc = "Field `MODPER` reader - Modulation period"]
pub type MODPER_R = crate::FieldReader<u16>;
#[doc = "Field `MODPER` writer - Modulation period"]
pub type MODPER_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 13, u16>;
#[doc = "Field `INCSTEP` reader - Incrementation step"]
pub type INCSTEP_R = crate::FieldReader<u16>;
#[doc = "Field `INCSTEP` writer - Incrementation step"]
pub type INCSTEP_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 15, u16>;
#[doc = "Spread Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPREADSEL {
    #[doc = "0: Center spread"]
    Center = 0,
    #[doc = "1: Down spread"]
    Down = 1,
}
impl From<SPREADSEL> for bool {
    #[inline(always)]
    fn from(variant: SPREADSEL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPREADSEL` reader - Spread Select"]
pub type SPREADSEL_R = crate::BitReader<SPREADSEL>;
impl SPREADSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPREADSEL {
        match self.bits {
            false => SPREADSEL::Center,
            true => SPREADSEL::Down,
        }
    }
    #[doc = "Center spread"]
    #[inline(always)]
    pub fn is_center(&self) -> bool {
        *self == SPREADSEL::Center
    }
    #[doc = "Down spread"]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == SPREADSEL::Down
    }
}
#[doc = "Field `SPREADSEL` writer - Spread Select"]
pub type SPREADSEL_W<'a, REG> = crate::BitWriter<'a, REG, SPREADSEL>;
impl<'a, REG> SPREADSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Center spread"]
    #[inline(always)]
    pub fn center(self) -> &'a mut crate::W<REG> {
        self.variant(SPREADSEL::Center)
    }
    #[doc = "Down spread"]
    #[inline(always)]
    pub fn down(self) -> &'a mut crate::W<REG> {
        self.variant(SPREADSEL::Down)
    }
}
#[doc = "Spread spectrum modulation enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSCGEN {
    #[doc = "0: Spread spectrum modulation disabled"]
    Disabled = 0,
    #[doc = "1: Spread spectrum modulation enabled"]
    Enabled = 1,
}
impl From<SSCGEN> for bool {
    #[inline(always)]
    fn from(variant: SSCGEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSCGEN` reader - Spread spectrum modulation enable"]
pub type SSCGEN_R = crate::BitReader<SSCGEN>;
impl SSCGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SSCGEN {
        match self.bits {
            false => SSCGEN::Disabled,
            true => SSCGEN::Enabled,
        }
    }
    #[doc = "Spread spectrum modulation disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SSCGEN::Disabled
    }
    #[doc = "Spread spectrum modulation enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SSCGEN::Enabled
    }
}
#[doc = "Field `SSCGEN` writer - Spread spectrum modulation enable"]
pub type SSCGEN_W<'a, REG> = crate::BitWriter<'a, REG, SSCGEN>;
impl<'a, REG> SSCGEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Spread spectrum modulation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SSCGEN::Disabled)
    }
    #[doc = "Spread spectrum modulation enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SSCGEN::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:12 - Modulation period"]
    #[inline(always)]
    pub fn modper(&self) -> MODPER_R {
        MODPER_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 13:27 - Incrementation step"]
    #[inline(always)]
    pub fn incstep(&self) -> INCSTEP_R {
        INCSTEP_R::new(((self.bits >> 13) & 0x7fff) as u16)
    }
    #[doc = "Bit 30 - Spread Select"]
    #[inline(always)]
    pub fn spreadsel(&self) -> SPREADSEL_R {
        SPREADSEL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Spread spectrum modulation enable"]
    #[inline(always)]
    pub fn sscgen(&self) -> SSCGEN_R {
        SSCGEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:12 - Modulation period"]
    #[inline(always)]
    #[must_use]
    pub fn modper(&mut self) -> MODPER_W<SSCGRrs> {
        MODPER_W::new(self, 0)
    }
    #[doc = "Bits 13:27 - Incrementation step"]
    #[inline(always)]
    #[must_use]
    pub fn incstep(&mut self) -> INCSTEP_W<SSCGRrs> {
        INCSTEP_W::new(self, 13)
    }
    #[doc = "Bit 30 - Spread Select"]
    #[inline(always)]
    #[must_use]
    pub fn spreadsel(&mut self) -> SPREADSEL_W<SSCGRrs> {
        SPREADSEL_W::new(self, 30)
    }
    #[doc = "Bit 31 - Spread spectrum modulation enable"]
    #[inline(always)]
    #[must_use]
    pub fn sscgen(&mut self) -> SSCGEN_W<SSCGRrs> {
        SSCGEN_W::new(self, 31)
    }
}
#[doc = "spread spectrum clock generation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sscgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sscgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSCGRrs;
impl crate::RegisterSpec for SSCGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sscgr::R`](R) reader structure"]
impl crate::Readable for SSCGRrs {}
#[doc = "`write(|w| ..)` method takes [`sscgr::W`](W) writer structure"]
impl crate::Writable for SSCGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SSCGR to value 0"]
impl crate::Resettable for SSCGRrs {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CFGRrs>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CFGRrs>;
#[doc = "Low voltage enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVEN {
    #[doc = "0: Flash low voltage disabled"]
    Disabled = 0,
    #[doc = "1: Flash low voltage enabled"]
    Enabled = 1,
}
impl From<LVEN> for bool {
    #[inline(always)]
    fn from(variant: LVEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVEN` reader - Low voltage enable"]
pub type LVEN_R = crate::BitReader<LVEN>;
impl LVEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LVEN {
        match self.bits {
            false => LVEN::Disabled,
            true => LVEN::Enabled,
        }
    }
    #[doc = "Flash low voltage disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LVEN::Disabled
    }
    #[doc = "Flash low voltage enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LVEN::Enabled
    }
}
#[doc = "Field `LVEN` writer - Low voltage enable"]
pub type LVEN_W<'a, REG> = crate::BitWriter<'a, REG, LVEN>;
impl<'a, REG> LVEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flash low voltage disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LVEN::Disabled)
    }
    #[doc = "Flash low voltage enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LVEN::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Low voltage enable"]
    #[inline(always)]
    pub fn lven(&self) -> LVEN_R {
        LVEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low voltage enable"]
    #[inline(always)]
    #[must_use]
    pub fn lven(&mut self) -> LVEN_W<CFGRrs> {
        LVEN_W::new(self, 0)
    }
}
#[doc = "flash configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGRrs;
impl crate::RegisterSpec for CFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CFGRrs {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CFGRrs {
    const RESET_VALUE: u32 = 0;
}

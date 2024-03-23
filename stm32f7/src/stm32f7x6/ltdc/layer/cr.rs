#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Layer Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LEN {
    #[doc = "0: Layer disabled"]
    Disabled = 0,
    #[doc = "1: Layer enabled"]
    Enabled = 1,
}
impl From<LEN> for bool {
    #[inline(always)]
    fn from(variant: LEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEN` reader - Layer Enable"]
pub type LEN_R = crate::BitReader<LEN>;
impl LEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LEN {
        match self.bits {
            false => LEN::Disabled,
            true => LEN::Enabled,
        }
    }
    #[doc = "Layer disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LEN::Disabled
    }
    #[doc = "Layer enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LEN::Enabled
    }
}
#[doc = "Field `LEN` writer - Layer Enable"]
pub type LEN_W<'a, REG> = crate::BitWriter<'a, REG, LEN>;
impl<'a, REG> LEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Layer disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LEN::Disabled)
    }
    #[doc = "Layer enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LEN::Enabled)
    }
}
#[doc = "Color Keying Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COLKEN {
    #[doc = "0: Color keying disabled"]
    Disabled = 0,
    #[doc = "1: Color keying enabled"]
    Enabled = 1,
}
impl From<COLKEN> for bool {
    #[inline(always)]
    fn from(variant: COLKEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COLKEN` reader - Color Keying Enable"]
pub type COLKEN_R = crate::BitReader<COLKEN>;
impl COLKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COLKEN {
        match self.bits {
            false => COLKEN::Disabled,
            true => COLKEN::Enabled,
        }
    }
    #[doc = "Color keying disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COLKEN::Disabled
    }
    #[doc = "Color keying enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COLKEN::Enabled
    }
}
#[doc = "Field `COLKEN` writer - Color Keying Enable"]
pub type COLKEN_W<'a, REG> = crate::BitWriter<'a, REG, COLKEN>;
impl<'a, REG> COLKEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Color keying disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(COLKEN::Disabled)
    }
    #[doc = "Color keying enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(COLKEN::Enabled)
    }
}
#[doc = "Color Look-Up Table Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLUTEN {
    #[doc = "0: Color look-up table disabled"]
    Disabled = 0,
    #[doc = "1: Color look-up table enabled"]
    Enabled = 1,
}
impl From<CLUTEN> for bool {
    #[inline(always)]
    fn from(variant: CLUTEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLUTEN` reader - Color Look-Up Table Enable"]
pub type CLUTEN_R = crate::BitReader<CLUTEN>;
impl CLUTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLUTEN {
        match self.bits {
            false => CLUTEN::Disabled,
            true => CLUTEN::Enabled,
        }
    }
    #[doc = "Color look-up table disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CLUTEN::Disabled
    }
    #[doc = "Color look-up table enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CLUTEN::Enabled
    }
}
#[doc = "Field `CLUTEN` writer - Color Look-Up Table Enable"]
pub type CLUTEN_W<'a, REG> = crate::BitWriter<'a, REG, CLUTEN>;
impl<'a, REG> CLUTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Color look-up table disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CLUTEN::Disabled)
    }
    #[doc = "Color look-up table enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CLUTEN::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Layer Enable"]
    #[inline(always)]
    pub fn len(&self) -> LEN_R {
        LEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Color Keying Enable"]
    #[inline(always)]
    pub fn colken(&self) -> COLKEN_R {
        COLKEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Color Look-Up Table Enable"]
    #[inline(always)]
    pub fn cluten(&self) -> CLUTEN_R {
        CLUTEN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Layer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn len(&mut self) -> LEN_W<CRrs> {
        LEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Color Keying Enable"]
    #[inline(always)]
    #[must_use]
    pub fn colken(&mut self) -> COLKEN_W<CRrs> {
        COLKEN_W::new(self, 1)
    }
    #[doc = "Bit 4 - Color Look-Up Table Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cluten(&mut self) -> CLUTEN_W<CRrs> {
        CLUTEN_W::new(self, 4)
    }
}
#[doc = "Layerx Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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

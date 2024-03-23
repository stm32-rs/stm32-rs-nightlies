#[doc = "Register `CLRFR` writer"]
pub type W = crate::W<CLRFRrs>;
#[doc = "Clear overrun / underrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COVRUDRW {
    #[doc = "1: Clears the OVRUDR flag"]
    Clear = 1,
}
impl From<COVRUDRW> for bool {
    #[inline(always)]
    fn from(variant: COVRUDRW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COVRUDR` writer - Clear overrun / underrun"]
pub type COVRUDR_W<'a, REG> = crate::BitWriter<'a, REG, COVRUDRW>;
impl<'a, REG> COVRUDR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the OVRUDR flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(COVRUDRW::Clear)
    }
}
#[doc = "Mute detection flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMUTEDETW {
    #[doc = "1: Clears the MUTEDET flag"]
    Clear = 1,
}
impl From<CMUTEDETW> for bool {
    #[inline(always)]
    fn from(variant: CMUTEDETW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMUTEDET` writer - Mute detection flag"]
pub type CMUTEDET_W<'a, REG> = crate::BitWriter<'a, REG, CMUTEDETW>;
impl<'a, REG> CMUTEDET_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the MUTEDET flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CMUTEDETW::Clear)
    }
}
#[doc = "Clear wrong clock configuration flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CWCKCFGW {
    #[doc = "1: Clears the WCKCFG flag"]
    Clear = 1,
}
impl From<CWCKCFGW> for bool {
    #[inline(always)]
    fn from(variant: CWCKCFGW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CWCKCFG` writer - Clear wrong clock configuration flag"]
pub type CWCKCFG_W<'a, REG> = crate::BitWriter<'a, REG, CWCKCFGW>;
impl<'a, REG> CWCKCFG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the WCKCFG flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CWCKCFGW::Clear)
    }
}
#[doc = "Clear codec not ready flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCNRDYW {
    #[doc = "1: Clears the CNRDY flag"]
    Clear = 1,
}
impl From<CCNRDYW> for bool {
    #[inline(always)]
    fn from(variant: CCNRDYW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCNRDY` writer - Clear codec not ready flag"]
pub type CCNRDY_W<'a, REG> = crate::BitWriter<'a, REG, CCNRDYW>;
impl<'a, REG> CCNRDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the CNRDY flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CCNRDYW::Clear)
    }
}
#[doc = "Clear anticipated frame synchronization detection flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAFSDETW {
    #[doc = "1: Clears the AFSDET flag"]
    Clear = 1,
}
impl From<CAFSDETW> for bool {
    #[inline(always)]
    fn from(variant: CAFSDETW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAFSDET` writer - Clear anticipated frame synchronization detection flag"]
pub type CAFSDET_W<'a, REG> = crate::BitWriter<'a, REG, CAFSDETW>;
impl<'a, REG> CAFSDET_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the AFSDET flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CAFSDETW::Clear)
    }
}
#[doc = "Clear late frame synchronization detection flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLFSDETW {
    #[doc = "1: Clears the LFSDET flag"]
    Clear = 1,
}
impl From<CLFSDETW> for bool {
    #[inline(always)]
    fn from(variant: CLFSDETW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLFSDET` writer - Clear late frame synchronization detection flag"]
pub type CLFSDET_W<'a, REG> = crate::BitWriter<'a, REG, CLFSDETW>;
impl<'a, REG> CLFSDET_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the LFSDET flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CLFSDETW::Clear)
    }
}
impl W {
    #[doc = "Bit 0 - Clear overrun / underrun"]
    #[inline(always)]
    #[must_use]
    pub fn covrudr(&mut self) -> COVRUDR_W<CLRFRrs> {
        COVRUDR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Mute detection flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmutedet(&mut self) -> CMUTEDET_W<CLRFRrs> {
        CMUTEDET_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear wrong clock configuration flag"]
    #[inline(always)]
    #[must_use]
    pub fn cwckcfg(&mut self) -> CWCKCFG_W<CLRFRrs> {
        CWCKCFG_W::new(self, 2)
    }
    #[doc = "Bit 4 - Clear codec not ready flag"]
    #[inline(always)]
    #[must_use]
    pub fn ccnrdy(&mut self) -> CCNRDY_W<CLRFRrs> {
        CCNRDY_W::new(self, 4)
    }
    #[doc = "Bit 5 - Clear anticipated frame synchronization detection flag"]
    #[inline(always)]
    #[must_use]
    pub fn cafsdet(&mut self) -> CAFSDET_W<CLRFRrs> {
        CAFSDET_W::new(self, 5)
    }
    #[doc = "Bit 6 - Clear late frame synchronization detection flag"]
    #[inline(always)]
    #[must_use]
    pub fn clfsdet(&mut self) -> CLFSDET_W<CLRFRrs> {
        CLFSDET_W::new(self, 6)
    }
}
#[doc = "AClear flag register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clrfr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLRFRrs;
impl crate::RegisterSpec for CLRFRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`clrfr::W`](W) writer structure"]
impl crate::Writable for CLRFRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLRFR to value 0"]
impl crate::Resettable for CLRFRrs {
    const RESET_VALUE: u32 = 0;
}

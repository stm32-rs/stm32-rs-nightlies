#[doc = "Register `SCR` writer"]
pub type W = crate::W<SCRrs>;
#[doc = "CTAMP1F\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTAMP1FW {
    #[doc = "1: Clear tamper flag"]
    Clear = 1,
}
impl From<CTAMP1FW> for bool {
    #[inline(always)]
    fn from(variant: CTAMP1FW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTAMP1F` writer - CTAMP1F"]
pub type CTAMP1F_W<'a, REG> = crate::BitWriter<'a, REG, CTAMP1FW>;
impl<'a, REG> CTAMP1F_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear tamper flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CTAMP1FW::Clear)
    }
}
#[doc = "Field `CTAMP2F` writer - CTAMP2F"]
pub use CTAMP1F_W as CTAMP2F_W;
#[doc = "Field `CTAMP3F` writer - CTAMP3F"]
pub use CTAMP1F_W as CTAMP3F_W;
#[doc = "CITAMP3F\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CITAMP3FW {
    #[doc = "1: Clear tamper flag"]
    Clear = 1,
}
impl From<CITAMP3FW> for bool {
    #[inline(always)]
    fn from(variant: CITAMP3FW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CITAMP3F` writer - CITAMP3F"]
pub type CITAMP3F_W<'a, REG> = crate::BitWriter<'a, REG, CITAMP3FW>;
impl<'a, REG> CITAMP3F_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear tamper flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CITAMP3FW::Clear)
    }
}
#[doc = "Field `CITAMP5F` writer - CITAMP5F"]
pub use CITAMP3F_W as CITAMP5F_W;
#[doc = "Field `CITAMP6F` writer - CITAMP6F"]
pub use CITAMP3F_W as CITAMP6F_W;
#[doc = "Field `CITAMP8F` writer - CITAMP8F"]
pub use CITAMP3F_W as CITAMP8F_W;
impl W {
    #[doc = "Bit 0 - CTAMP1F"]
    #[inline(always)]
    #[must_use]
    pub fn ctamp1f(&mut self) -> CTAMP1F_W<SCRrs> {
        CTAMP1F_W::new(self, 0)
    }
    #[doc = "Bit 1 - CTAMP2F"]
    #[inline(always)]
    #[must_use]
    pub fn ctamp2f(&mut self) -> CTAMP2F_W<SCRrs> {
        CTAMP2F_W::new(self, 1)
    }
    #[doc = "Bit 2 - CTAMP3F"]
    #[inline(always)]
    #[must_use]
    pub fn ctamp3f(&mut self) -> CTAMP3F_W<SCRrs> {
        CTAMP3F_W::new(self, 2)
    }
    #[doc = "Bit 18 - CITAMP3F"]
    #[inline(always)]
    #[must_use]
    pub fn citamp3f(&mut self) -> CITAMP3F_W<SCRrs> {
        CITAMP3F_W::new(self, 18)
    }
    #[doc = "Bit 20 - CITAMP5F"]
    #[inline(always)]
    #[must_use]
    pub fn citamp5f(&mut self) -> CITAMP5F_W<SCRrs> {
        CITAMP5F_W::new(self, 20)
    }
    #[doc = "Bit 21 - CITAMP6F"]
    #[inline(always)]
    #[must_use]
    pub fn citamp6f(&mut self) -> CITAMP6F_W<SCRrs> {
        CITAMP6F_W::new(self, 21)
    }
    #[doc = "Bit 23 - CITAMP8F"]
    #[inline(always)]
    #[must_use]
    pub fn citamp8f(&mut self) -> CITAMP8F_W<SCRrs> {
        CITAMP8F_W::new(self, 23)
    }
}
#[doc = "TAMP status clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCRrs;
impl crate::RegisterSpec for SCRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`scr::W`](W) writer structure"]
impl crate::Writable for SCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for SCRrs {
    const RESET_VALUE: u32 = 0;
}

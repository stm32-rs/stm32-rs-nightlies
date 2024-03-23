#[doc = "Register `C1FCR` writer"]
pub type W = crate::W<C1FCRrs>;
#[doc = "transfer complete flag clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCFW {
    #[doc = "1: Clear flag"]
    Clear = 1,
}
impl From<TCFW> for bool {
    #[inline(always)]
    fn from(variant: TCFW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCF` writer - transfer complete flag clear"]
pub type TCF_W<'a, REG> = crate::BitWriter<'a, REG, TCFW>;
impl<'a, REG> TCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TCFW::Clear)
    }
}
#[doc = "Field `HTF` writer - half transfer flag clear"]
pub use TCF_W as HTF_W;
#[doc = "Field `DTEF` writer - data transfer error flag clear"]
pub use TCF_W as DTEF_W;
#[doc = "Field `ULEF` writer - update link transfer error flag clear"]
pub use TCF_W as ULEF_W;
#[doc = "Field `USEF` writer - user setting error flag clear"]
pub use TCF_W as USEF_W;
#[doc = "Field `SUSPF` writer - completed suspension flag clear"]
pub use TCF_W as SUSPF_W;
#[doc = "Field `TOF` writer - trigger overrun flag clear"]
pub use TCF_W as TOF_W;
impl W {
    #[doc = "Bit 8 - transfer complete flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn tcf(&mut self) -> TCF_W<C1FCRrs> {
        TCF_W::new(self, 8)
    }
    #[doc = "Bit 9 - half transfer flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn htf(&mut self) -> HTF_W<C1FCRrs> {
        HTF_W::new(self, 9)
    }
    #[doc = "Bit 10 - data transfer error flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn dtef(&mut self) -> DTEF_W<C1FCRrs> {
        DTEF_W::new(self, 10)
    }
    #[doc = "Bit 11 - update link transfer error flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn ulef(&mut self) -> ULEF_W<C1FCRrs> {
        ULEF_W::new(self, 11)
    }
    #[doc = "Bit 12 - user setting error flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn usef(&mut self) -> USEF_W<C1FCRrs> {
        USEF_W::new(self, 12)
    }
    #[doc = "Bit 13 - completed suspension flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn suspf(&mut self) -> SUSPF_W<C1FCRrs> {
        SUSPF_W::new(self, 13)
    }
    #[doc = "Bit 14 - trigger overrun flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn tof(&mut self) -> TOF_W<C1FCRrs> {
        TOF_W::new(self, 14)
    }
}
#[doc = "GPDMA channel 1 flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1fcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C1FCRrs;
impl crate::RegisterSpec for C1FCRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`c1fcr::W`](W) writer structure"]
impl crate::Writable for C1FCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C1FCR to value 0"]
impl crate::Resettable for C1FCRrs {
    const RESET_VALUE: u32 = 0;
}

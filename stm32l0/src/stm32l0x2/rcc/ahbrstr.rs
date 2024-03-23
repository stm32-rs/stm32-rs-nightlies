#[doc = "Register `AHBRSTR` reader"]
pub type R = crate::R<AHBRSTRrs>;
#[doc = "Register `AHBRSTR` writer"]
pub type W = crate::W<AHBRSTRrs>;
#[doc = "DMA reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMARSTW {
    #[doc = "1: Reset the module"]
    Reset = 1,
}
impl From<DMARSTW> for bool {
    #[inline(always)]
    fn from(variant: DMARSTW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMARST` reader - DMA reset"]
pub type DMARST_R = crate::BitReader<DMARSTW>;
impl DMARST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DMARSTW> {
        match self.bits {
            true => Some(DMARSTW::Reset),
            _ => None,
        }
    }
    #[doc = "Reset the module"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == DMARSTW::Reset
    }
}
#[doc = "Field `DMARST` writer - DMA reset"]
pub type DMARST_W<'a, REG> = crate::BitWriter<'a, REG, DMARSTW>;
impl<'a, REG> DMARST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset the module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(DMARSTW::Reset)
    }
}
#[doc = "Field `MIFRST` reader - Memory interface reset"]
pub use DMARST_R as MIFRST_R;
#[doc = "Field `CRCRST` reader - Test integration module reset"]
pub use DMARST_R as CRCRST_R;
#[doc = "Field `TOUCHRST` reader - Touch Sensing reset"]
pub use DMARST_R as TOUCHRST_R;
#[doc = "Field `RNGRST` reader - Random Number Generator module reset"]
pub use DMARST_R as RNGRST_R;
#[doc = "Field `CRYPRST` reader - Crypto module reset"]
pub use DMARST_R as CRYPRST_R;
#[doc = "Field `MIFRST` writer - Memory interface reset"]
pub use DMARST_W as MIFRST_W;
#[doc = "Field `CRCRST` writer - Test integration module reset"]
pub use DMARST_W as CRCRST_W;
#[doc = "Field `TOUCHRST` writer - Touch Sensing reset"]
pub use DMARST_W as TOUCHRST_W;
#[doc = "Field `RNGRST` writer - Random Number Generator module reset"]
pub use DMARST_W as RNGRST_W;
#[doc = "Field `CRYPRST` writer - Crypto module reset"]
pub use DMARST_W as CRYPRST_W;
impl R {
    #[doc = "Bit 0 - DMA reset"]
    #[inline(always)]
    pub fn dmarst(&self) -> DMARST_R {
        DMARST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Memory interface reset"]
    #[inline(always)]
    pub fn mifrst(&self) -> MIFRST_R {
        MIFRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Test integration module reset"]
    #[inline(always)]
    pub fn crcrst(&self) -> CRCRST_R {
        CRCRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Touch Sensing reset"]
    #[inline(always)]
    pub fn touchrst(&self) -> TOUCHRST_R {
        TOUCHRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Random Number Generator module reset"]
    #[inline(always)]
    pub fn rngrst(&self) -> RNGRST_R {
        RNGRST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Crypto module reset"]
    #[inline(always)]
    pub fn cryprst(&self) -> CRYPRST_R {
        CRYPRST_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA reset"]
    #[inline(always)]
    #[must_use]
    pub fn dmarst(&mut self) -> DMARST_W<AHBRSTRrs> {
        DMARST_W::new(self, 0)
    }
    #[doc = "Bit 8 - Memory interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn mifrst(&mut self) -> MIFRST_W<AHBRSTRrs> {
        MIFRST_W::new(self, 8)
    }
    #[doc = "Bit 12 - Test integration module reset"]
    #[inline(always)]
    #[must_use]
    pub fn crcrst(&mut self) -> CRCRST_W<AHBRSTRrs> {
        CRCRST_W::new(self, 12)
    }
    #[doc = "Bit 16 - Touch Sensing reset"]
    #[inline(always)]
    #[must_use]
    pub fn touchrst(&mut self) -> TOUCHRST_W<AHBRSTRrs> {
        TOUCHRST_W::new(self, 16)
    }
    #[doc = "Bit 20 - Random Number Generator module reset"]
    #[inline(always)]
    #[must_use]
    pub fn rngrst(&mut self) -> RNGRST_W<AHBRSTRrs> {
        RNGRST_W::new(self, 20)
    }
    #[doc = "Bit 24 - Crypto module reset"]
    #[inline(always)]
    #[must_use]
    pub fn cryprst(&mut self) -> CRYPRST_W<AHBRSTRrs> {
        CRYPRST_W::new(self, 24)
    }
}
#[doc = "AHB peripheral reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbrstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbrstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBRSTRrs;
impl crate::RegisterSpec for AHBRSTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbrstr::R`](R) reader structure"]
impl crate::Readable for AHBRSTRrs {}
#[doc = "`write(|w| ..)` method takes [`ahbrstr::W`](W) writer structure"]
impl crate::Writable for AHBRSTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBRSTR to value 0"]
impl crate::Resettable for AHBRSTRrs {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `AHBENR` reader"]
pub type R = crate::R<AHBENRrs>;
#[doc = "Register `AHBENR` writer"]
pub type W = crate::W<AHBENRrs>;
#[doc = "DMA clock enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAEN {
    #[doc = "0: Clock disabled"]
    Disabled = 0,
    #[doc = "1: Clock enabled"]
    Enabled = 1,
}
impl From<DMAEN> for bool {
    #[inline(always)]
    fn from(variant: DMAEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAEN` reader - DMA clock enable bit"]
pub type DMAEN_R = crate::BitReader<DMAEN>;
impl DMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMAEN {
        match self.bits {
            false => DMAEN::Disabled,
            true => DMAEN::Enabled,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAEN::Disabled
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAEN::Enabled
    }
}
#[doc = "Field `DMAEN` writer - DMA clock enable bit"]
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG, DMAEN>;
impl<'a, REG> DMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN::Disabled)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN::Enabled)
    }
}
#[doc = "Field `MIFEN` reader - NVM interface clock enable bit"]
pub use DMAEN_R as MIFEN_R;
#[doc = "Field `CRCEN` reader - CRC clock enable bit"]
pub use DMAEN_R as CRCEN_R;
#[doc = "Field `CRYPEN` reader - Crypto clock enable bit"]
pub use DMAEN_R as CRYPEN_R;
#[doc = "Field `MIFEN` writer - NVM interface clock enable bit"]
pub use DMAEN_W as MIFEN_W;
#[doc = "Field `CRCEN` writer - CRC clock enable bit"]
pub use DMAEN_W as CRCEN_W;
#[doc = "Field `CRYPEN` writer - Crypto clock enable bit"]
pub use DMAEN_W as CRYPEN_W;
impl R {
    #[doc = "Bit 0 - DMA clock enable bit"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - NVM interface clock enable bit"]
    #[inline(always)]
    pub fn mifen(&self) -> MIFEN_R {
        MIFEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC clock enable bit"]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 24 - Crypto clock enable bit"]
    #[inline(always)]
    pub fn crypen(&self) -> CRYPEN_R {
        CRYPEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA clock enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<AHBENRrs> {
        DMAEN_W::new(self, 0)
    }
    #[doc = "Bit 8 - NVM interface clock enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn mifen(&mut self) -> MIFEN_W<AHBENRrs> {
        MIFEN_W::new(self, 8)
    }
    #[doc = "Bit 12 - CRC clock enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CRCEN_W<AHBENRrs> {
        CRCEN_W::new(self, 12)
    }
    #[doc = "Bit 24 - Crypto clock enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn crypen(&mut self) -> CRYPEN_W<AHBENRrs> {
        CRYPEN_W::new(self, 24)
    }
}
#[doc = "AHB peripheral clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBENRrs;
impl crate::RegisterSpec for AHBENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbenr::R`](R) reader structure"]
impl crate::Readable for AHBENRrs {}
#[doc = "`write(|w| ..)` method takes [`ahbenr::W`](W) writer structure"]
impl crate::Writable for AHBENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBENR to value 0x0100"]
impl crate::Resettable for AHBENRrs {
    const RESET_VALUE: u32 = 0x0100;
}

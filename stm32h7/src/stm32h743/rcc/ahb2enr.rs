#[doc = "Register `AHB2ENR` reader"]
pub type R = crate::R<AHB2ENRrs>;
#[doc = "Register `AHB2ENR` writer"]
pub type W = crate::W<AHB2ENRrs>;
#[doc = "DCMI peripheral clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCMIEN {
    #[doc = "0: The selected clock is disabled"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled"]
    Enabled = 1,
}
impl From<DCMIEN> for bool {
    #[inline(always)]
    fn from(variant: DCMIEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCMIEN` reader - DCMI peripheral clock"]
pub type DCMIEN_R = crate::BitReader<DCMIEN>;
impl DCMIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DCMIEN {
        match self.bits {
            false => DCMIEN::Disabled,
            true => DCMIEN::Enabled,
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DCMIEN::Disabled
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DCMIEN::Enabled
    }
}
#[doc = "Field `DCMIEN` writer - DCMI peripheral clock"]
pub type DCMIEN_W<'a, REG> = crate::BitWriter<'a, REG, DCMIEN>;
impl<'a, REG> DCMIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DCMIEN::Disabled)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DCMIEN::Enabled)
    }
}
#[doc = "Field `CRYPTEN` reader - CRYPT peripheral clock enable"]
pub use DCMIEN_R as CRYPTEN_R;
#[doc = "Field `HASHEN` reader - HASH peripheral clock enable"]
pub use DCMIEN_R as HASHEN_R;
#[doc = "Field `RNGEN` reader - RNG peripheral clocks enable"]
pub use DCMIEN_R as RNGEN_R;
#[doc = "Field `SDMMC2EN` reader - SDMMC2 and SDMMC2 delay clock enable"]
pub use DCMIEN_R as SDMMC2EN_R;
#[doc = "Field `SRAM1EN` reader - SRAM1 block enable"]
pub use DCMIEN_R as SRAM1EN_R;
#[doc = "Field `SRAM2EN` reader - SRAM2 block enable"]
pub use DCMIEN_R as SRAM2EN_R;
#[doc = "Field `SRAM3EN` reader - SRAM3 block enable"]
pub use DCMIEN_R as SRAM3EN_R;
#[doc = "Field `CRYPTEN` writer - CRYPT peripheral clock enable"]
pub use DCMIEN_W as CRYPTEN_W;
#[doc = "Field `HASHEN` writer - HASH peripheral clock enable"]
pub use DCMIEN_W as HASHEN_W;
#[doc = "Field `RNGEN` writer - RNG peripheral clocks enable"]
pub use DCMIEN_W as RNGEN_W;
#[doc = "Field `SDMMC2EN` writer - SDMMC2 and SDMMC2 delay clock enable"]
pub use DCMIEN_W as SDMMC2EN_W;
#[doc = "Field `SRAM1EN` writer - SRAM1 block enable"]
pub use DCMIEN_W as SRAM1EN_W;
#[doc = "Field `SRAM2EN` writer - SRAM2 block enable"]
pub use DCMIEN_W as SRAM2EN_W;
#[doc = "Field `SRAM3EN` writer - SRAM3 block enable"]
pub use DCMIEN_W as SRAM3EN_W;
impl R {
    #[doc = "Bit 0 - DCMI peripheral clock"]
    #[inline(always)]
    pub fn dcmien(&self) -> DCMIEN_R {
        DCMIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - CRYPT peripheral clock enable"]
    #[inline(always)]
    pub fn crypten(&self) -> CRYPTEN_R {
        CRYPTEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HASH peripheral clock enable"]
    #[inline(always)]
    pub fn hashen(&self) -> HASHEN_R {
        HASHEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RNG peripheral clocks enable"]
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - SDMMC2 and SDMMC2 delay clock enable"]
    #[inline(always)]
    pub fn sdmmc2en(&self) -> SDMMC2EN_R {
        SDMMC2EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 29 - SRAM1 block enable"]
    #[inline(always)]
    pub fn sram1en(&self) -> SRAM1EN_R {
        SRAM1EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - SRAM2 block enable"]
    #[inline(always)]
    pub fn sram2en(&self) -> SRAM2EN_R {
        SRAM2EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - SRAM3 block enable"]
    #[inline(always)]
    pub fn sram3en(&self) -> SRAM3EN_R {
        SRAM3EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DCMI peripheral clock"]
    #[inline(always)]
    #[must_use]
    pub fn dcmien(&mut self) -> DCMIEN_W<AHB2ENRrs> {
        DCMIEN_W::new(self, 0)
    }
    #[doc = "Bit 4 - CRYPT peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn crypten(&mut self) -> CRYPTEN_W<AHB2ENRrs> {
        CRYPTEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - HASH peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn hashen(&mut self) -> HASHEN_W<AHB2ENRrs> {
        HASHEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - RNG peripheral clocks enable"]
    #[inline(always)]
    #[must_use]
    pub fn rngen(&mut self) -> RNGEN_W<AHB2ENRrs> {
        RNGEN_W::new(self, 6)
    }
    #[doc = "Bit 9 - SDMMC2 and SDMMC2 delay clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc2en(&mut self) -> SDMMC2EN_W<AHB2ENRrs> {
        SDMMC2EN_W::new(self, 9)
    }
    #[doc = "Bit 29 - SRAM1 block enable"]
    #[inline(always)]
    #[must_use]
    pub fn sram1en(&mut self) -> SRAM1EN_W<AHB2ENRrs> {
        SRAM1EN_W::new(self, 29)
    }
    #[doc = "Bit 30 - SRAM2 block enable"]
    #[inline(always)]
    #[must_use]
    pub fn sram2en(&mut self) -> SRAM2EN_W<AHB2ENRrs> {
        SRAM2EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - SRAM3 block enable"]
    #[inline(always)]
    #[must_use]
    pub fn sram3en(&mut self) -> SRAM3EN_W<AHB2ENRrs> {
        SRAM3EN_W::new(self, 31)
    }
}
#[doc = "RCC AHB2 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb2enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb2enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB2ENRrs;
impl crate::RegisterSpec for AHB2ENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb2enr::R`](R) reader structure"]
impl crate::Readable for AHB2ENRrs {}
#[doc = "`write(|w| ..)` method takes [`ahb2enr::W`](W) writer structure"]
impl crate::Writable for AHB2ENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB2ENR to value 0"]
impl crate::Resettable for AHB2ENRrs {
    const RESET_VALUE: u32 = 0;
}

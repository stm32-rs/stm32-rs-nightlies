#[doc = "Register `PLLI2SCFGR` reader"]
pub type R = crate::R<PLLI2SCFGRrs>;
#[doc = "Register `PLLI2SCFGR` writer"]
pub type W = crate::W<PLLI2SCFGRrs>;
#[doc = "Field `PLLI2SM` reader - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
pub type PLLI2SM_R = crate::FieldReader;
#[doc = "Field `PLLI2SM` writer - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
pub type PLLI2SM_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PLLI2SN` reader - PLLI2S multiplication factor for VCO"]
pub type PLLI2SN_R = crate::FieldReader<u16>;
#[doc = "Field `PLLI2SN` writer - PLLI2S multiplication factor for VCO"]
pub type PLLI2SN_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "PLLI2S entry clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLI2SSRC {
    #[doc = "0: HSE or HSI depending on PLLSRC of PLLCFGR"]
    HseHsi = 0,
    #[doc = "1: External AFI clock (CK_PLLI2S_EXT) selected as PLL clock entry"]
    External = 1,
}
impl From<PLLI2SSRC> for bool {
    #[inline(always)]
    fn from(variant: PLLI2SSRC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLI2SSRC` reader - PLLI2S entry clock source"]
pub type PLLI2SSRC_R = crate::BitReader<PLLI2SSRC>;
impl PLLI2SSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLLI2SSRC {
        match self.bits {
            false => PLLI2SSRC::HseHsi,
            true => PLLI2SSRC::External,
        }
    }
    #[doc = "HSE or HSI depending on PLLSRC of PLLCFGR"]
    #[inline(always)]
    pub fn is_hse_hsi(&self) -> bool {
        *self == PLLI2SSRC::HseHsi
    }
    #[doc = "External AFI clock (CK_PLLI2S_EXT) selected as PLL clock entry"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == PLLI2SSRC::External
    }
}
#[doc = "Field `PLLI2SSRC` writer - PLLI2S entry clock source"]
pub type PLLI2SSRC_W<'a, REG> = crate::BitWriter<'a, REG, PLLI2SSRC>;
impl<'a, REG> PLLI2SSRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSE or HSI depending on PLLSRC of PLLCFGR"]
    #[inline(always)]
    pub fn hse_hsi(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SSRC::HseHsi)
    }
    #[doc = "External AFI clock (CK_PLLI2S_EXT) selected as PLL clock entry"]
    #[inline(always)]
    pub fn external(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SSRC::External)
    }
}
#[doc = "Field `PLLI2SQ` reader - PLLI2S division factor for USB OTG FS/SDIO/RNG clock"]
pub type PLLI2SQ_R = crate::FieldReader;
#[doc = "Field `PLLI2SQ` writer - PLLI2S division factor for USB OTG FS/SDIO/RNG clock"]
pub type PLLI2SQ_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PLLI2SR` reader - PLLI2S division factor for I2S clocks"]
pub type PLLI2SR_R = crate::FieldReader;
#[doc = "Field `PLLI2SR` writer - PLLI2S division factor for I2S clocks"]
pub type PLLI2SR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:5 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    pub fn plli2sm(&self) -> PLLI2SM_R {
        PLLI2SM_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:14 - PLLI2S multiplication factor for VCO"]
    #[inline(always)]
    pub fn plli2sn(&self) -> PLLI2SN_R {
        PLLI2SN_R::new(((self.bits >> 6) & 0x01ff) as u16)
    }
    #[doc = "Bit 22 - PLLI2S entry clock source"]
    #[inline(always)]
    pub fn plli2ssrc(&self) -> PLLI2SSRC_R {
        PLLI2SSRC_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:27 - PLLI2S division factor for USB OTG FS/SDIO/RNG clock"]
    #[inline(always)]
    pub fn plli2sq(&self) -> PLLI2SQ_R {
        PLLI2SQ_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:30 - PLLI2S division factor for I2S clocks"]
    #[inline(always)]
    pub fn plli2sr(&self) -> PLLI2SR_R {
        PLLI2SR_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    #[must_use]
    pub fn plli2sm(&mut self) -> PLLI2SM_W<PLLI2SCFGRrs> {
        PLLI2SM_W::new(self, 0)
    }
    #[doc = "Bits 6:14 - PLLI2S multiplication factor for VCO"]
    #[inline(always)]
    #[must_use]
    pub fn plli2sn(&mut self) -> PLLI2SN_W<PLLI2SCFGRrs> {
        PLLI2SN_W::new(self, 6)
    }
    #[doc = "Bit 22 - PLLI2S entry clock source"]
    #[inline(always)]
    #[must_use]
    pub fn plli2ssrc(&mut self) -> PLLI2SSRC_W<PLLI2SCFGRrs> {
        PLLI2SSRC_W::new(self, 22)
    }
    #[doc = "Bits 24:27 - PLLI2S division factor for USB OTG FS/SDIO/RNG clock"]
    #[inline(always)]
    #[must_use]
    pub fn plli2sq(&mut self) -> PLLI2SQ_W<PLLI2SCFGRrs> {
        PLLI2SQ_W::new(self, 24)
    }
    #[doc = "Bits 28:30 - PLLI2S division factor for I2S clocks"]
    #[inline(always)]
    #[must_use]
    pub fn plli2sr(&mut self) -> PLLI2SR_W<PLLI2SCFGRrs> {
        PLLI2SR_W::new(self, 28)
    }
}
#[doc = "PLLI2S configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`plli2scfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`plli2scfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLLI2SCFGRrs;
impl crate::RegisterSpec for PLLI2SCFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`plli2scfgr::R`](R) reader structure"]
impl crate::Readable for PLLI2SCFGRrs {}
#[doc = "`write(|w| ..)` method takes [`plli2scfgr::W`](W) writer structure"]
impl crate::Writable for PLLI2SCFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLLI2SCFGR to value 0x2000_3000"]
impl crate::Resettable for PLLI2SCFGRrs {
    const RESET_VALUE: u32 = 0x2000_3000;
}

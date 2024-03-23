#[doc = "Register `ADF_CKGCR` reader"]
pub type R = crate::R<ADF_CKGCRrs>;
#[doc = "Register `ADF_CKGCR` writer"]
pub type W = crate::W<ADF_CKGCRrs>;
#[doc = "Field `CKGDEN` reader - CKGEN dividers enable"]
pub type CKGDEN_R = crate::BitReader;
#[doc = "Field `CKGDEN` writer - CKGEN dividers enable"]
pub type CKGDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCK0EN` reader - ADF_CCK0 clock enable"]
pub type CCK0EN_R = crate::BitReader;
#[doc = "Field `CCK0EN` writer - ADF_CCK0 clock enable"]
pub type CCK0EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCK1EN` reader - ADF_CCK1 clock enable"]
pub type CCK1EN_R = crate::BitReader;
#[doc = "Field `CCK1EN` writer - ADF_CCK1 clock enable"]
pub type CCK1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKGMOD` reader - Clock generator mode"]
pub type CKGMOD_R = crate::BitReader;
#[doc = "Field `CKGMOD` writer - Clock generator mode"]
pub type CKGMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCK0DIR` reader - ADF_CCK0 direction"]
pub type CCK0DIR_R = crate::BitReader;
#[doc = "Field `CCK0DIR` writer - ADF_CCK0 direction"]
pub type CCK0DIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCK1DIR` reader - ADF_CCK1 direction"]
pub type CCK1DIR_R = crate::BitReader;
#[doc = "Field `CCK1DIR` writer - ADF_CCK1 direction"]
pub type CCK1DIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGSENS` reader - CKGEN trigger sensitivity selection"]
pub type TRGSENS_R = crate::BitReader;
#[doc = "Field `TRGSENS` writer - CKGEN trigger sensitivity selection"]
pub type TRGSENS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGSRC` reader - Digital filter trigger signal selection"]
pub type TRGSRC_R = crate::FieldReader;
#[doc = "Field `TRGSRC` writer - Digital filter trigger signal selection"]
pub type TRGSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CCKDIV` reader - Divider to control the ADF_CCK clock"]
pub type CCKDIV_R = crate::FieldReader;
#[doc = "Field `CCKDIV` writer - Divider to control the ADF_CCK clock"]
pub type CCKDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PROCDIV` reader - Divider to control the serial interface clock"]
pub type PROCDIV_R = crate::FieldReader;
#[doc = "Field `PROCDIV` writer - Divider to control the serial interface clock"]
pub type PROCDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `CKGACTIVE` reader - Clock generator active flag"]
pub type CKGACTIVE_R = crate::BitReader;
#[doc = "Field `CKGACTIVE` writer - Clock generator active flag"]
pub type CKGACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CKGEN dividers enable"]
    #[inline(always)]
    pub fn ckgden(&self) -> CKGDEN_R {
        CKGDEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADF_CCK0 clock enable"]
    #[inline(always)]
    pub fn cck0en(&self) -> CCK0EN_R {
        CCK0EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADF_CCK1 clock enable"]
    #[inline(always)]
    pub fn cck1en(&self) -> CCK1EN_R {
        CCK1EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Clock generator mode"]
    #[inline(always)]
    pub fn ckgmod(&self) -> CKGMOD_R {
        CKGMOD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADF_CCK0 direction"]
    #[inline(always)]
    pub fn cck0dir(&self) -> CCK0DIR_R {
        CCK0DIR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ADF_CCK1 direction"]
    #[inline(always)]
    pub fn cck1dir(&self) -> CCK1DIR_R {
        CCK1DIR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - CKGEN trigger sensitivity selection"]
    #[inline(always)]
    pub fn trgsens(&self) -> TRGSENS_R {
        TRGSENS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Digital filter trigger signal selection"]
    #[inline(always)]
    pub fn trgsrc(&self) -> TRGSRC_R {
        TRGSRC_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Divider to control the ADF_CCK clock"]
    #[inline(always)]
    pub fn cckdiv(&self) -> CCKDIV_R {
        CCKDIV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:30 - Divider to control the serial interface clock"]
    #[inline(always)]
    pub fn procdiv(&self) -> PROCDIV_R {
        PROCDIV_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - Clock generator active flag"]
    #[inline(always)]
    pub fn ckgactive(&self) -> CKGACTIVE_R {
        CKGACTIVE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CKGEN dividers enable"]
    #[inline(always)]
    #[must_use]
    pub fn ckgden(&mut self) -> CKGDEN_W<ADF_CKGCRrs> {
        CKGDEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - ADF_CCK0 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn cck0en(&mut self) -> CCK0EN_W<ADF_CKGCRrs> {
        CCK0EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - ADF_CCK1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn cck1en(&mut self) -> CCK1EN_W<ADF_CKGCRrs> {
        CCK1EN_W::new(self, 2)
    }
    #[doc = "Bit 4 - Clock generator mode"]
    #[inline(always)]
    #[must_use]
    pub fn ckgmod(&mut self) -> CKGMOD_W<ADF_CKGCRrs> {
        CKGMOD_W::new(self, 4)
    }
    #[doc = "Bit 5 - ADF_CCK0 direction"]
    #[inline(always)]
    #[must_use]
    pub fn cck0dir(&mut self) -> CCK0DIR_W<ADF_CKGCRrs> {
        CCK0DIR_W::new(self, 5)
    }
    #[doc = "Bit 6 - ADF_CCK1 direction"]
    #[inline(always)]
    #[must_use]
    pub fn cck1dir(&mut self) -> CCK1DIR_W<ADF_CKGCRrs> {
        CCK1DIR_W::new(self, 6)
    }
    #[doc = "Bit 8 - CKGEN trigger sensitivity selection"]
    #[inline(always)]
    #[must_use]
    pub fn trgsens(&mut self) -> TRGSENS_W<ADF_CKGCRrs> {
        TRGSENS_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Digital filter trigger signal selection"]
    #[inline(always)]
    #[must_use]
    pub fn trgsrc(&mut self) -> TRGSRC_W<ADF_CKGCRrs> {
        TRGSRC_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Divider to control the ADF_CCK clock"]
    #[inline(always)]
    #[must_use]
    pub fn cckdiv(&mut self) -> CCKDIV_W<ADF_CKGCRrs> {
        CCKDIV_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - Divider to control the serial interface clock"]
    #[inline(always)]
    #[must_use]
    pub fn procdiv(&mut self) -> PROCDIV_W<ADF_CKGCRrs> {
        PROCDIV_W::new(self, 24)
    }
    #[doc = "Bit 31 - Clock generator active flag"]
    #[inline(always)]
    #[must_use]
    pub fn ckgactive(&mut self) -> CKGACTIVE_W<ADF_CKGCRrs> {
        CKGACTIVE_W::new(self, 31)
    }
}
#[doc = "ADF clock generator control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adf_ckgcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adf_ckgcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADF_CKGCRrs;
impl crate::RegisterSpec for ADF_CKGCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adf_ckgcr::R`](R) reader structure"]
impl crate::Readable for ADF_CKGCRrs {}
#[doc = "`write(|w| ..)` method takes [`adf_ckgcr::W`](W) writer structure"]
impl crate::Writable for ADF_CKGCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADF_CKGCR to value 0"]
impl crate::Resettable for ADF_CKGCRrs {
    const RESET_VALUE: u32 = 0;
}

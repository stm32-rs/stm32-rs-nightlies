#[doc = "Register `ADF_DFLT0CICR` reader"]
pub type R = crate::R<ADF_DFLT0CICRrs>;
#[doc = "Register `ADF_DFLT0CICR` writer"]
pub type W = crate::W<ADF_DFLT0CICRrs>;
#[doc = "Field `DATSRC` reader - Source data for the digital filter"]
pub type DATSRC_R = crate::FieldReader;
#[doc = "Field `DATSRC` writer - Source data for the digital filter"]
pub type DATSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CICMOD` reader - Select the CIC order"]
pub type CICMOD_R = crate::FieldReader;
#[doc = "Field `CICMOD` writer - Select the CIC order"]
pub type CICMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MCICD` reader - CIC decimation ratio selection"]
pub type MCICD_R = crate::FieldReader<u16>;
#[doc = "Field `MCICD` writer - CIC decimation ratio selection"]
pub type MCICD_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `SCALE` reader - Scaling factor selection"]
pub type SCALE_R = crate::FieldReader;
#[doc = "Field `SCALE` writer - Scaling factor selection"]
pub type SCALE_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:1 - Source data for the digital filter"]
    #[inline(always)]
    pub fn datsrc(&self) -> DATSRC_R {
        DATSRC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:6 - Select the CIC order"]
    #[inline(always)]
    pub fn cicmod(&self) -> CICMOD_R {
        CICMOD_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:16 - CIC decimation ratio selection"]
    #[inline(always)]
    pub fn mcicd(&self) -> MCICD_R {
        MCICD_R::new(((self.bits >> 8) & 0x01ff) as u16)
    }
    #[doc = "Bits 20:25 - Scaling factor selection"]
    #[inline(always)]
    pub fn scale(&self) -> SCALE_R {
        SCALE_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Source data for the digital filter"]
    #[inline(always)]
    #[must_use]
    pub fn datsrc(&mut self) -> DATSRC_W<ADF_DFLT0CICRrs> {
        DATSRC_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Select the CIC order"]
    #[inline(always)]
    #[must_use]
    pub fn cicmod(&mut self) -> CICMOD_W<ADF_DFLT0CICRrs> {
        CICMOD_W::new(self, 4)
    }
    #[doc = "Bits 8:16 - CIC decimation ratio selection"]
    #[inline(always)]
    #[must_use]
    pub fn mcicd(&mut self) -> MCICD_W<ADF_DFLT0CICRrs> {
        MCICD_W::new(self, 8)
    }
    #[doc = "Bits 20:25 - Scaling factor selection"]
    #[inline(always)]
    #[must_use]
    pub fn scale(&mut self) -> SCALE_W<ADF_DFLT0CICRrs> {
        SCALE_W::new(self, 20)
    }
}
#[doc = "ADF digital filer configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adf_dflt0cicr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adf_dflt0cicr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADF_DFLT0CICRrs;
impl crate::RegisterSpec for ADF_DFLT0CICRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adf_dflt0cicr::R`](R) reader structure"]
impl crate::Readable for ADF_DFLT0CICRrs {}
#[doc = "`write(|w| ..)` method takes [`adf_dflt0cicr::W`](W) writer structure"]
impl crate::Writable for ADF_DFLT0CICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADF_DFLT0CICR to value 0"]
impl crate::Resettable for ADF_DFLT0CICRrs {
    const RESET_VALUE: u32 = 0;
}

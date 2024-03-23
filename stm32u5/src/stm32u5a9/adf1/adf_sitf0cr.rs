#[doc = "Register `ADF_SITF0CR` reader"]
pub type R = crate::R<ADF_SITF0CRrs>;
#[doc = "Register `ADF_SITF0CR` writer"]
pub type W = crate::W<ADF_SITF0CRrs>;
#[doc = "Field `SITFEN` reader - SITFEN"]
pub type SITFEN_R = crate::BitReader;
#[doc = "Field `SITFEN` writer - SITFEN"]
pub type SITFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCKSRC` reader - SCKSRC"]
pub type SCKSRC_R = crate::FieldReader;
#[doc = "Field `SCKSRC` writer - SCKSRC"]
pub type SCKSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SITFMOD` reader - SITFMOD"]
pub type SITFMOD_R = crate::FieldReader;
#[doc = "Field `SITFMOD` writer - SITFMOD"]
pub type SITFMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `STH` reader - STH"]
pub type STH_R = crate::FieldReader;
#[doc = "Field `STH` writer - STH"]
pub type STH_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SITFACTIVE` reader - SITFACTIVE"]
pub type SITFACTIVE_R = crate::BitReader;
#[doc = "Field `SITFACTIVE` writer - SITFACTIVE"]
pub type SITFACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SITFEN"]
    #[inline(always)]
    pub fn sitfen(&self) -> SITFEN_R {
        SITFEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - SCKSRC"]
    #[inline(always)]
    pub fn scksrc(&self) -> SCKSRC_R {
        SCKSRC_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 4:5 - SITFMOD"]
    #[inline(always)]
    pub fn sitfmod(&self) -> SITFMOD_R {
        SITFMOD_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:12 - STH"]
    #[inline(always)]
    pub fn sth(&self) -> STH_R {
        STH_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - SITFACTIVE"]
    #[inline(always)]
    pub fn sitfactive(&self) -> SITFACTIVE_R {
        SITFACTIVE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SITFEN"]
    #[inline(always)]
    #[must_use]
    pub fn sitfen(&mut self) -> SITFEN_W<ADF_SITF0CRrs> {
        SITFEN_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - SCKSRC"]
    #[inline(always)]
    #[must_use]
    pub fn scksrc(&mut self) -> SCKSRC_W<ADF_SITF0CRrs> {
        SCKSRC_W::new(self, 1)
    }
    #[doc = "Bits 4:5 - SITFMOD"]
    #[inline(always)]
    #[must_use]
    pub fn sitfmod(&mut self) -> SITFMOD_W<ADF_SITF0CRrs> {
        SITFMOD_W::new(self, 4)
    }
    #[doc = "Bits 8:12 - STH"]
    #[inline(always)]
    #[must_use]
    pub fn sth(&mut self) -> STH_W<ADF_SITF0CRrs> {
        STH_W::new(self, 8)
    }
    #[doc = "Bit 31 - SITFACTIVE"]
    #[inline(always)]
    #[must_use]
    pub fn sitfactive(&mut self) -> SITFACTIVE_W<ADF_SITF0CRrs> {
        SITFACTIVE_W::new(self, 31)
    }
}
#[doc = "ADF serial interface control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adf_sitf0cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adf_sitf0cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADF_SITF0CRrs;
impl crate::RegisterSpec for ADF_SITF0CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adf_sitf0cr::R`](R) reader structure"]
impl crate::Readable for ADF_SITF0CRrs {}
#[doc = "`write(|w| ..)` method takes [`adf_sitf0cr::W`](W) writer structure"]
impl crate::Writable for ADF_SITF0CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADF_SITF0CR to value 0x1f00"]
impl crate::Resettable for ADF_SITF0CRrs {
    const RESET_VALUE: u32 = 0x1f00;
}

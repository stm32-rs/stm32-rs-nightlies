#[doc = "Register `WPCR2` reader"]
pub type R = crate::R<WPCR2rs>;
#[doc = "Register `WPCR2` writer"]
pub type W = crate::W<WPCR2rs>;
#[doc = "Field `TCLKPREP` reader - tCLK-PREPARE"]
pub type TCLKPREP_R = crate::FieldReader;
#[doc = "Field `TCLKPREP` writer - tCLK-PREPARE"]
pub type TCLKPREP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TCLKZERO` reader - tCLK-ZERO"]
pub type TCLKZERO_R = crate::FieldReader;
#[doc = "Field `TCLKZERO` writer - tCLK-ZERO"]
pub type TCLKZERO_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `THSPREP` reader - tHS-PREPARE"]
pub type THSPREP_R = crate::FieldReader;
#[doc = "Field `THSPREP` writer - tHS-PREPARE"]
pub type THSPREP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `THSTRAIL` reader - tHSTRAIL"]
pub type THSTRAIL_R = crate::FieldReader;
#[doc = "Field `THSTRAIL` writer - tHSTRAIL"]
pub type THSTRAIL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - tCLK-PREPARE"]
    #[inline(always)]
    pub fn tclkprep(&self) -> TCLKPREP_R {
        TCLKPREP_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - tCLK-ZERO"]
    #[inline(always)]
    pub fn tclkzero(&self) -> TCLKZERO_R {
        TCLKZERO_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - tHS-PREPARE"]
    #[inline(always)]
    pub fn thsprep(&self) -> THSPREP_R {
        THSPREP_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - tHSTRAIL"]
    #[inline(always)]
    pub fn thstrail(&self) -> THSTRAIL_R {
        THSTRAIL_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - tCLK-PREPARE"]
    #[inline(always)]
    #[must_use]
    pub fn tclkprep(&mut self) -> TCLKPREP_W<WPCR2rs> {
        TCLKPREP_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - tCLK-ZERO"]
    #[inline(always)]
    #[must_use]
    pub fn tclkzero(&mut self) -> TCLKZERO_W<WPCR2rs> {
        TCLKZERO_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - tHS-PREPARE"]
    #[inline(always)]
    #[must_use]
    pub fn thsprep(&mut self) -> THSPREP_W<WPCR2rs> {
        THSPREP_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - tHSTRAIL"]
    #[inline(always)]
    #[must_use]
    pub fn thstrail(&mut self) -> THSTRAIL_W<WPCR2rs> {
        THSTRAIL_W::new(self, 24)
    }
}
#[doc = "DSI wrapper PHY configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpcr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpcr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WPCR2rs;
impl crate::RegisterSpec for WPCR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wpcr2::R`](R) reader structure"]
impl crate::Readable for WPCR2rs {}
#[doc = "`write(|w| ..)` method takes [`wpcr2::W`](W) writer structure"]
impl crate::Writable for WPCR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WPCR2 to value 0"]
impl crate::Resettable for WPCR2rs {
    const RESET_VALUE: u32 = 0;
}

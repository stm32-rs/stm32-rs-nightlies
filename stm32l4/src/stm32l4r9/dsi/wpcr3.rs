#[doc = "Register `WPCR3` reader"]
pub type R = crate::R<WPCR3rs>;
#[doc = "Register `WPCR3` writer"]
pub type W = crate::W<WPCR3rs>;
#[doc = "Field `THSZERO` reader - tHS-ZERO"]
pub type THSZERO_R = crate::FieldReader;
#[doc = "Field `THSZERO` writer - tHS-ZERO"]
pub type THSZERO_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TLPXD` reader - tLPX for Data lanes"]
pub type TLPXD_R = crate::FieldReader;
#[doc = "Field `TLPXD` writer - tLPX for Data lanes"]
pub type TLPXD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `THSEXIT` reader - tHSEXIT"]
pub type THSEXIT_R = crate::FieldReader;
#[doc = "Field `THSEXIT` writer - tHSEXIT"]
pub type THSEXIT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TLPXC` reader - tLPXC for Clock lane"]
pub type TLPXC_R = crate::FieldReader;
#[doc = "Field `TLPXC` writer - tLPXC for Clock lane"]
pub type TLPXC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - tHS-ZERO"]
    #[inline(always)]
    pub fn thszero(&self) -> THSZERO_R {
        THSZERO_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - tLPX for Data lanes"]
    #[inline(always)]
    pub fn tlpxd(&self) -> TLPXD_R {
        TLPXD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - tHSEXIT"]
    #[inline(always)]
    pub fn thsexit(&self) -> THSEXIT_R {
        THSEXIT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - tLPXC for Clock lane"]
    #[inline(always)]
    pub fn tlpxc(&self) -> TLPXC_R {
        TLPXC_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - tHS-ZERO"]
    #[inline(always)]
    #[must_use]
    pub fn thszero(&mut self) -> THSZERO_W<WPCR3rs> {
        THSZERO_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - tLPX for Data lanes"]
    #[inline(always)]
    #[must_use]
    pub fn tlpxd(&mut self) -> TLPXD_W<WPCR3rs> {
        TLPXD_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - tHSEXIT"]
    #[inline(always)]
    #[must_use]
    pub fn thsexit(&mut self) -> THSEXIT_W<WPCR3rs> {
        THSEXIT_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - tLPXC for Clock lane"]
    #[inline(always)]
    #[must_use]
    pub fn tlpxc(&mut self) -> TLPXC_W<WPCR3rs> {
        TLPXC_W::new(self, 24)
    }
}
#[doc = "DSI_WPCR3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpcr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpcr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WPCR3rs;
impl crate::RegisterSpec for WPCR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wpcr3::R`](R) reader structure"]
impl crate::Readable for WPCR3rs {}
#[doc = "`write(|w| ..)` method takes [`wpcr3::W`](W) writer structure"]
impl crate::Writable for WPCR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WPCR3 to value 0"]
impl crate::Resettable for WPCR3rs {
    const RESET_VALUE: u32 = 0;
}

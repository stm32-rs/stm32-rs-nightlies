#[doc = "Register `EECR3` reader"]
pub type R = crate::R<EECR3rs>;
#[doc = "Register `EECR3` writer"]
pub type W = crate::W<EECR3rs>;
#[doc = "Field `EE6F` reader - EE6F"]
pub type EE6F_R = crate::FieldReader;
#[doc = "Field `EE6F` writer - EE6F"]
pub type EE6F_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EE7F` reader - EE7F"]
pub type EE7F_R = crate::FieldReader;
#[doc = "Field `EE7F` writer - EE7F"]
pub type EE7F_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EE8F` reader - EE8F"]
pub type EE8F_R = crate::FieldReader;
#[doc = "Field `EE8F` writer - EE8F"]
pub type EE8F_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EE9F` reader - EE9F"]
pub type EE9F_R = crate::FieldReader;
#[doc = "Field `EE9F` writer - EE9F"]
pub type EE9F_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EE10F` reader - EE10F"]
pub type EE10F_R = crate::FieldReader;
#[doc = "Field `EE10F` writer - EE10F"]
pub type EE10F_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EEVSD` reader - EEVSD"]
pub type EEVSD_R = crate::FieldReader;
#[doc = "Field `EEVSD` writer - EEVSD"]
pub type EEVSD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - EE6F"]
    #[inline(always)]
    pub fn ee6f(&self) -> EE6F_R {
        EE6F_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 6:9 - EE7F"]
    #[inline(always)]
    pub fn ee7f(&self) -> EE7F_R {
        EE7F_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EE8F"]
    #[inline(always)]
    pub fn ee8f(&self) -> EE8F_R {
        EE8F_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 18:21 - EE9F"]
    #[inline(always)]
    pub fn ee9f(&self) -> EE9F_R {
        EE9F_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - EE10F"]
    #[inline(always)]
    pub fn ee10f(&self) -> EE10F_R {
        EE10F_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 30:31 - EEVSD"]
    #[inline(always)]
    pub fn eevsd(&self) -> EEVSD_R {
        EEVSD_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EE6F"]
    #[inline(always)]
    #[must_use]
    pub fn ee6f(&mut self) -> EE6F_W<EECR3rs> {
        EE6F_W::new(self, 0)
    }
    #[doc = "Bits 6:9 - EE7F"]
    #[inline(always)]
    #[must_use]
    pub fn ee7f(&mut self) -> EE7F_W<EECR3rs> {
        EE7F_W::new(self, 6)
    }
    #[doc = "Bits 12:15 - EE8F"]
    #[inline(always)]
    #[must_use]
    pub fn ee8f(&mut self) -> EE8F_W<EECR3rs> {
        EE8F_W::new(self, 12)
    }
    #[doc = "Bits 18:21 - EE9F"]
    #[inline(always)]
    #[must_use]
    pub fn ee9f(&mut self) -> EE9F_W<EECR3rs> {
        EE9F_W::new(self, 18)
    }
    #[doc = "Bits 24:27 - EE10F"]
    #[inline(always)]
    #[must_use]
    pub fn ee10f(&mut self) -> EE10F_W<EECR3rs> {
        EE10F_W::new(self, 24)
    }
    #[doc = "Bits 30:31 - EEVSD"]
    #[inline(always)]
    #[must_use]
    pub fn eevsd(&mut self) -> EEVSD_W<EECR3rs> {
        EEVSD_W::new(self, 30)
    }
}
#[doc = "Timer External Event Control Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eecr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eecr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EECR3rs;
impl crate::RegisterSpec for EECR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eecr3::R`](R) reader structure"]
impl crate::Readable for EECR3rs {}
#[doc = "`write(|w| ..)` method takes [`eecr3::W`](W) writer structure"]
impl crate::Writable for EECR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EECR3 to value 0"]
impl crate::Resettable for EECR3rs {
    const RESET_VALUE: u32 = 0;
}

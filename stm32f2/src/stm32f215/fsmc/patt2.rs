#[doc = "Register `PATT2` reader"]
pub type R = crate::R<PATT2rs>;
#[doc = "Register `PATT2` writer"]
pub type W = crate::W<PATT2rs>;
#[doc = "Field `ATTSET` reader - Attribute memory x setup time"]
pub type ATTSET_R = crate::FieldReader;
#[doc = "Field `ATTSET` writer - Attribute memory x setup time"]
pub type ATTSET_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ATTWAIT` reader - Attribute memory x wait time"]
pub type ATTWAIT_R = crate::FieldReader;
#[doc = "Field `ATTWAIT` writer - Attribute memory x wait time"]
pub type ATTWAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ATTHOLD` reader - Attribute memory x hold time"]
pub type ATTHOLD_R = crate::FieldReader;
#[doc = "Field `ATTHOLD` writer - Attribute memory x hold time"]
pub type ATTHOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ATTHIZ` reader - Attribute memory x databus HiZ time"]
pub type ATTHIZ_R = crate::FieldReader;
#[doc = "Field `ATTHIZ` writer - Attribute memory x databus HiZ time"]
pub type ATTHIZ_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Attribute memory x setup time"]
    #[inline(always)]
    pub fn attset(&self) -> ATTSET_R {
        ATTSET_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Attribute memory x wait time"]
    #[inline(always)]
    pub fn attwait(&self) -> ATTWAIT_R {
        ATTWAIT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Attribute memory x hold time"]
    #[inline(always)]
    pub fn atthold(&self) -> ATTHOLD_R {
        ATTHOLD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Attribute memory x databus HiZ time"]
    #[inline(always)]
    pub fn atthiz(&self) -> ATTHIZ_R {
        ATTHIZ_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Attribute memory x setup time"]
    #[inline(always)]
    #[must_use]
    pub fn attset(&mut self) -> ATTSET_W<PATT2rs> {
        ATTSET_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Attribute memory x wait time"]
    #[inline(always)]
    #[must_use]
    pub fn attwait(&mut self) -> ATTWAIT_W<PATT2rs> {
        ATTWAIT_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Attribute memory x hold time"]
    #[inline(always)]
    #[must_use]
    pub fn atthold(&mut self) -> ATTHOLD_W<PATT2rs> {
        ATTHOLD_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Attribute memory x databus HiZ time"]
    #[inline(always)]
    #[must_use]
    pub fn atthiz(&mut self) -> ATTHIZ_W<PATT2rs> {
        ATTHIZ_W::new(self, 24)
    }
}
#[doc = "Attribute memory space timing register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`patt2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`patt2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PATT2rs;
impl crate::RegisterSpec for PATT2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`patt2::R`](R) reader structure"]
impl crate::Readable for PATT2rs {}
#[doc = "`write(|w| ..)` method takes [`patt2::W`](W) writer structure"]
impl crate::Writable for PATT2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PATT2 to value 0xfcfc_fcfc"]
impl crate::Resettable for PATT2rs {
    const RESET_VALUE: u32 = 0xfcfc_fcfc;
}

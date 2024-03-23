#[doc = "Register `PATT4` reader"]
pub type R = crate::R<PATT4rs>;
#[doc = "Register `PATT4` writer"]
pub type W = crate::W<PATT4rs>;
#[doc = "Field `ATTSET` reader - ATTSETx"]
pub type ATTSET_R = crate::FieldReader;
#[doc = "Field `ATTSET` writer - ATTSETx"]
pub type ATTSET_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ATTWAIT` reader - ATTWAITx"]
pub type ATTWAIT_R = crate::FieldReader;
#[doc = "Field `ATTWAIT` writer - ATTWAITx"]
pub type ATTWAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ATTHOLD` reader - ATTHOLDx"]
pub type ATTHOLD_R = crate::FieldReader;
#[doc = "Field `ATTHOLD` writer - ATTHOLDx"]
pub type ATTHOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ATTHIZ` reader - ATTHIZx"]
pub type ATTHIZ_R = crate::FieldReader;
#[doc = "Field `ATTHIZ` writer - ATTHIZx"]
pub type ATTHIZ_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - ATTSETx"]
    #[inline(always)]
    pub fn attset(&self) -> ATTSET_R {
        ATTSET_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - ATTWAITx"]
    #[inline(always)]
    pub fn attwait(&self) -> ATTWAIT_R {
        ATTWAIT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - ATTHOLDx"]
    #[inline(always)]
    pub fn atthold(&self) -> ATTHOLD_R {
        ATTHOLD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - ATTHIZx"]
    #[inline(always)]
    pub fn atthiz(&self) -> ATTHIZ_R {
        ATTHIZ_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ATTSETx"]
    #[inline(always)]
    #[must_use]
    pub fn attset(&mut self) -> ATTSET_W<PATT4rs> {
        ATTSET_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - ATTWAITx"]
    #[inline(always)]
    #[must_use]
    pub fn attwait(&mut self) -> ATTWAIT_W<PATT4rs> {
        ATTWAIT_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - ATTHOLDx"]
    #[inline(always)]
    #[must_use]
    pub fn atthold(&mut self) -> ATTHOLD_W<PATT4rs> {
        ATTHOLD_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - ATTHIZx"]
    #[inline(always)]
    #[must_use]
    pub fn atthiz(&mut self) -> ATTHIZ_W<PATT4rs> {
        ATTHIZ_W::new(self, 24)
    }
}
#[doc = "Attribute memory space timing register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`patt4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`patt4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PATT4rs;
impl crate::RegisterSpec for PATT4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`patt4::R`](R) reader structure"]
impl crate::Readable for PATT4rs {}
#[doc = "`write(|w| ..)` method takes [`patt4::W`](W) writer structure"]
impl crate::Writable for PATT4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PATT4 to value 0xfcfc_fcfc"]
impl crate::Resettable for PATT4rs {
    const RESET_VALUE: u32 = 0xfcfc_fcfc;
}

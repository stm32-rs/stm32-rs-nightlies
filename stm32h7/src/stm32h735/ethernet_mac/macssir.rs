#[doc = "Register `MACSSIR` reader"]
pub type R = crate::R<MACSSIRrs>;
#[doc = "Register `MACSSIR` writer"]
pub type W = crate::W<MACSSIRrs>;
#[doc = "Field `SNSINC` reader - Sub-nanosecond Increment Value"]
pub type SNSINC_R = crate::FieldReader;
#[doc = "Field `SNSINC` writer - Sub-nanosecond Increment Value"]
pub type SNSINC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SSINC` reader - Sub-second Increment Value"]
pub type SSINC_R = crate::FieldReader;
#[doc = "Field `SSINC` writer - Sub-second Increment Value"]
pub type SSINC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 8:15 - Sub-nanosecond Increment Value"]
    #[inline(always)]
    pub fn snsinc(&self) -> SNSINC_R {
        SNSINC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Sub-second Increment Value"]
    #[inline(always)]
    pub fn ssinc(&self) -> SSINC_R {
        SSINC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - Sub-nanosecond Increment Value"]
    #[inline(always)]
    #[must_use]
    pub fn snsinc(&mut self) -> SNSINC_W<MACSSIRrs> {
        SNSINC_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Sub-second Increment Value"]
    #[inline(always)]
    #[must_use]
    pub fn ssinc(&mut self) -> SSINC_W<MACSSIRrs> {
        SSINC_W::new(self, 16)
    }
}
#[doc = "Sub-second increment register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macssir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macssir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACSSIRrs;
impl crate::RegisterSpec for MACSSIRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macssir::R`](R) reader structure"]
impl crate::Readable for MACSSIRrs {}
#[doc = "`write(|w| ..)` method takes [`macssir::W`](W) writer structure"]
impl crate::Writable for MACSSIRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACSSIR to value 0"]
impl crate::Resettable for MACSSIRrs {
    const RESET_VALUE: u32 = 0;
}

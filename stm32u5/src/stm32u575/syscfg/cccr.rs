#[doc = "Register `CCCR` reader"]
pub type R = crate::R<CCCRrs>;
#[doc = "Register `CCCR` writer"]
pub type W = crate::W<CCCRrs>;
#[doc = "Field `NCC1` reader - NCC1"]
pub type NCC1_R = crate::FieldReader;
#[doc = "Field `NCC1` writer - NCC1"]
pub type NCC1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PCC1` reader - PCC1"]
pub type PCC1_R = crate::FieldReader;
#[doc = "Field `PCC1` writer - PCC1"]
pub type PCC1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NCC2` reader - NCC2"]
pub type NCC2_R = crate::FieldReader;
#[doc = "Field `NCC2` writer - NCC2"]
pub type NCC2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PCC2` reader - PCC2"]
pub type PCC2_R = crate::FieldReader;
#[doc = "Field `PCC2` writer - PCC2"]
pub type PCC2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - NCC1"]
    #[inline(always)]
    pub fn ncc1(&self) -> NCC1_R {
        NCC1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PCC1"]
    #[inline(always)]
    pub fn pcc1(&self) -> PCC1_R {
        PCC1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - NCC2"]
    #[inline(always)]
    pub fn ncc2(&self) -> NCC2_R {
        NCC2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PCC2"]
    #[inline(always)]
    pub fn pcc2(&self) -> PCC2_R {
        PCC2_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - NCC1"]
    #[inline(always)]
    #[must_use]
    pub fn ncc1(&mut self) -> NCC1_W<CCCRrs> {
        NCC1_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - PCC1"]
    #[inline(always)]
    #[must_use]
    pub fn pcc1(&mut self) -> PCC1_W<CCCRrs> {
        PCC1_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - NCC2"]
    #[inline(always)]
    #[must_use]
    pub fn ncc2(&mut self) -> NCC2_W<CCCRrs> {
        NCC2_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - PCC2"]
    #[inline(always)]
    #[must_use]
    pub fn pcc2(&mut self) -> PCC2_W<CCCRrs> {
        PCC2_W::new(self, 12)
    }
}
#[doc = "compensation cell code register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCCRrs;
impl crate::RegisterSpec for CCCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cccr::R`](R) reader structure"]
impl crate::Readable for CCCRrs {}
#[doc = "`write(|w| ..)` method takes [`cccr::W`](W) writer structure"]
impl crate::Writable for CCCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCCR to value 0x7878"]
impl crate::Resettable for CCCRrs {
    const RESET_VALUE: u32 = 0x7878;
}

#[doc = "Register `CCR` reader"]
pub type R = crate::R<CCRrs>;
#[doc = "Register `CCR` writer"]
pub type W = crate::W<CCRrs>;
#[doc = "Field `OTRIM1` reader - DAC Channel 1 offset trimming value"]
pub type OTRIM1_R = crate::FieldReader;
#[doc = "Field `OTRIM1` writer - DAC Channel 1 offset trimming value"]
pub type OTRIM1_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
#[doc = "Field `OTRIM2` reader - DAC Channel 2 offset trimming value"]
pub type OTRIM2_R = crate::FieldReader;
#[doc = "Field `OTRIM2` writer - DAC Channel 2 offset trimming value"]
pub type OTRIM2_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - DAC Channel 1 offset trimming value"]
    #[inline(always)]
    pub fn otrim1(&self) -> OTRIM1_R {
        OTRIM1_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - DAC Channel 2 offset trimming value"]
    #[inline(always)]
    pub fn otrim2(&self) -> OTRIM2_R {
        OTRIM2_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - DAC Channel 1 offset trimming value"]
    #[inline(always)]
    #[must_use]
    pub fn otrim1(&mut self) -> OTRIM1_W<CCRrs> {
        OTRIM1_W::new(self, 0)
    }
    #[doc = "Bits 16:20 - DAC Channel 2 offset trimming value"]
    #[inline(always)]
    #[must_use]
    pub fn otrim2(&mut self) -> OTRIM2_W<CCRrs> {
        OTRIM2_W::new(self, 16)
    }
}
#[doc = "calibration control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCRrs;
impl crate::RegisterSpec for CCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr::R`](R) reader structure"]
impl crate::Readable for CCRrs {}
#[doc = "`write(|w| ..)` method takes [`ccr::W`](W) writer structure"]
impl crate::Writable for CCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCR to value 0"]
impl crate::Resettable for CCRrs {
    const RESET_VALUE: u32 = 0;
}

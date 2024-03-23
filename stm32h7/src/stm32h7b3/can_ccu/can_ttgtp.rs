#[doc = "Register `CAN_TTGTP` reader"]
pub type R = crate::R<CAN_TTGTPrs>;
#[doc = "Register `CAN_TTGTP` writer"]
pub type W = crate::W<CAN_TTGTPrs>;
#[doc = "Field `NCL` reader - Time Preset"]
pub type NCL_R = crate::FieldReader<u16>;
#[doc = "Field `NCL` writer - Time Preset"]
pub type NCL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CTP` reader - Cycle Time Target Phase"]
pub type CTP_R = crate::FieldReader<u16>;
#[doc = "Field `CTP` writer - Cycle Time Target Phase"]
pub type CTP_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Time Preset"]
    #[inline(always)]
    pub fn ncl(&self) -> NCL_R {
        NCL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Cycle Time Target Phase"]
    #[inline(always)]
    pub fn ctp(&self) -> CTP_R {
        CTP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Time Preset"]
    #[inline(always)]
    #[must_use]
    pub fn ncl(&mut self) -> NCL_W<CAN_TTGTPrs> {
        NCL_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Cycle Time Target Phase"]
    #[inline(always)]
    #[must_use]
    pub fn ctp(&mut self) -> CTP_W<CAN_TTGTPrs> {
        CTP_W::new(self, 16)
    }
}
#[doc = "FDCAN TT Global Time Preset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can_ttgtp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`can_ttgtp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAN_TTGTPrs;
impl crate::RegisterSpec for CAN_TTGTPrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_ttgtp::R`](R) reader structure"]
impl crate::Readable for CAN_TTGTPrs {}
#[doc = "`write(|w| ..)` method takes [`can_ttgtp::W`](W) writer structure"]
impl crate::Writable for CAN_TTGTPrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAN_TTGTP to value 0"]
impl crate::Resettable for CAN_TTGTPrs {
    const RESET_VALUE: u32 = 0;
}

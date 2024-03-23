#[doc = "Register `HSICFGR` reader"]
pub type R = crate::R<HSICFGRrs>;
#[doc = "Register `HSICFGR` writer"]
pub type W = crate::W<HSICFGRrs>;
#[doc = "Field `HSICAL` reader - HSI clock calibration"]
pub type HSICAL_R = crate::FieldReader<u16>;
#[doc = "Field `HSICAL` writer - HSI clock calibration"]
pub type HSICAL_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `HSITRIM` reader - HSI clock trimming"]
pub type HSITRIM_R = crate::FieldReader;
#[doc = "Field `HSITRIM` writer - HSI clock trimming"]
pub type HSITRIM_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:11 - HSI clock calibration"]
    #[inline(always)]
    pub fn hsical(&self) -> HSICAL_R {
        HSICAL_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 24:30 - HSI clock trimming"]
    #[inline(always)]
    pub fn hsitrim(&self) -> HSITRIM_R {
        HSITRIM_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - HSI clock calibration"]
    #[inline(always)]
    #[must_use]
    pub fn hsical(&mut self) -> HSICAL_W<HSICFGRrs> {
        HSICAL_W::new(self, 0)
    }
    #[doc = "Bits 24:30 - HSI clock trimming"]
    #[inline(always)]
    #[must_use]
    pub fn hsitrim(&mut self) -> HSITRIM_W<HSICFGRrs> {
        HSITRIM_W::new(self, 24)
    }
}
#[doc = "RCC HSI configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsicfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsicfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSICFGRrs;
impl crate::RegisterSpec for HSICFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hsicfgr::R`](R) reader structure"]
impl crate::Readable for HSICFGRrs {}
#[doc = "`write(|w| ..)` method takes [`hsicfgr::W`](W) writer structure"]
impl crate::Writable for HSICFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HSICFGR to value 0"]
impl crate::Resettable for HSICFGRrs {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `CSICFGR` reader"]
pub type R = crate::R<CSICFGRrs>;
#[doc = "Register `CSICFGR` writer"]
pub type W = crate::W<CSICFGRrs>;
#[doc = "Field `CSICAL` reader - CSI clock calibration"]
pub type CSICAL_R = crate::FieldReader<u16>;
#[doc = "Field `CSICAL` writer - CSI clock calibration"]
pub type CSICAL_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `CSITRIM` reader - CSI clock trimming"]
pub type CSITRIM_R = crate::FieldReader;
#[doc = "Field `CSITRIM` writer - CSI clock trimming"]
pub type CSITRIM_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:9 - CSI clock calibration"]
    #[inline(always)]
    pub fn csical(&self) -> CSICAL_R {
        CSICAL_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 24:29 - CSI clock trimming"]
    #[inline(always)]
    pub fn csitrim(&self) -> CSITRIM_R {
        CSITRIM_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - CSI clock calibration"]
    #[inline(always)]
    #[must_use]
    pub fn csical(&mut self) -> CSICAL_W<CSICFGRrs> {
        CSICAL_W::new(self, 0)
    }
    #[doc = "Bits 24:29 - CSI clock trimming"]
    #[inline(always)]
    #[must_use]
    pub fn csitrim(&mut self) -> CSITRIM_W<CSICFGRrs> {
        CSITRIM_W::new(self, 24)
    }
}
#[doc = "RCC CSI configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csicfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csicfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSICFGRrs;
impl crate::RegisterSpec for CSICFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csicfgr::R`](R) reader structure"]
impl crate::Readable for CSICFGRrs {}
#[doc = "`write(|w| ..)` method takes [`csicfgr::W`](W) writer structure"]
impl crate::Writable for CSICFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSICFGR to value 0"]
impl crate::Resettable for CSICFGRrs {
    const RESET_VALUE: u32 = 0;
}

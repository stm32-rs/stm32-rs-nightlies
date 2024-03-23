#[doc = "Register `DLYCFGR` reader"]
pub type R = crate::R<DLYCFGRrs>;
#[doc = "Register `DLYCFGR` writer"]
pub type W = crate::W<DLYCFGRrs>;
#[doc = "Field `OCTOSPI1_DLY` reader - Delay sampling configuration on OCTOSPI1"]
pub type OCTOSPI1_DLY_R = crate::FieldReader;
#[doc = "Field `OCTOSPI1_DLY` writer - Delay sampling configuration on OCTOSPI1"]
pub type OCTOSPI1_DLY_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `OCTOSPI2_DLY` reader - Delay sampling configuration on OCTOSPI2"]
pub type OCTOSPI2_DLY_R = crate::FieldReader;
#[doc = "Field `OCTOSPI2_DLY` writer - Delay sampling configuration on OCTOSPI2"]
pub type OCTOSPI2_DLY_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Delay sampling configuration on OCTOSPI1"]
    #[inline(always)]
    pub fn octospi1_dly(&self) -> OCTOSPI1_DLY_R {
        OCTOSPI1_DLY_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Delay sampling configuration on OCTOSPI2"]
    #[inline(always)]
    pub fn octospi2_dly(&self) -> OCTOSPI2_DLY_R {
        OCTOSPI2_DLY_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Delay sampling configuration on OCTOSPI1"]
    #[inline(always)]
    #[must_use]
    pub fn octospi1_dly(&mut self) -> OCTOSPI1_DLY_W<DLYCFGRrs> {
        OCTOSPI1_DLY_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Delay sampling configuration on OCTOSPI2"]
    #[inline(always)]
    #[must_use]
    pub fn octospi2_dly(&mut self) -> OCTOSPI2_DLY_W<DLYCFGRrs> {
        OCTOSPI2_DLY_W::new(self, 4)
    }
}
#[doc = "Delay configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dlycfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dlycfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DLYCFGRrs;
impl crate::RegisterSpec for DLYCFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dlycfgr::R`](R) reader structure"]
impl crate::Readable for DLYCFGRrs {}
#[doc = "`write(|w| ..)` method takes [`dlycfgr::W`](W) writer structure"]
impl crate::Writable for DLYCFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DLYCFGR to value 0"]
impl crate::Resettable for DLYCFGRrs {
    const RESET_VALUE: u32 = 0;
}

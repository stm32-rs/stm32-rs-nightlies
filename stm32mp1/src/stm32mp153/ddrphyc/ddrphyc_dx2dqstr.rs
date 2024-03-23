#[doc = "Register `DDRPHYC_DX2DQSTR` reader"]
pub type R = crate::R<DDRPHYC_DX2DQSTRrs>;
#[doc = "Register `DDRPHYC_DX2DQSTR` writer"]
pub type W = crate::W<DDRPHYC_DX2DQSTRrs>;
#[doc = "Field `R0DGSL` reader - R0DGSL"]
pub type R0DGSL_R = crate::FieldReader;
#[doc = "Field `R0DGSL` writer - R0DGSL"]
pub type R0DGSL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `R0DGPS` reader - R0DGPS"]
pub type R0DGPS_R = crate::FieldReader;
#[doc = "Field `R0DGPS` writer - R0DGPS"]
pub type R0DGPS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DQSDLY` reader - DQSDLY"]
pub type DQSDLY_R = crate::FieldReader;
#[doc = "Field `DQSDLY` writer - DQSDLY"]
pub type DQSDLY_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DQSNDLY` reader - DQSNDLY"]
pub type DQSNDLY_R = crate::FieldReader;
#[doc = "Field `DQSNDLY` writer - DQSNDLY"]
pub type DQSNDLY_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DMDLY` reader - DMDLY"]
pub type DMDLY_R = crate::FieldReader;
#[doc = "Field `DMDLY` writer - DMDLY"]
pub type DMDLY_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2 - R0DGSL"]
    #[inline(always)]
    pub fn r0dgsl(&self) -> R0DGSL_R {
        R0DGSL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 12:13 - R0DGPS"]
    #[inline(always)]
    pub fn r0dgps(&self) -> R0DGPS_R {
        R0DGPS_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 20:22 - DQSDLY"]
    #[inline(always)]
    pub fn dqsdly(&self) -> DQSDLY_R {
        DQSDLY_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:25 - DQSNDLY"]
    #[inline(always)]
    pub fn dqsndly(&self) -> DQSNDLY_R {
        DQSNDLY_R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bits 26:29 - DMDLY"]
    #[inline(always)]
    pub fn dmdly(&self) -> DMDLY_R {
        DMDLY_R::new(((self.bits >> 26) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - R0DGSL"]
    #[inline(always)]
    #[must_use]
    pub fn r0dgsl(&mut self) -> R0DGSL_W<DDRPHYC_DX2DQSTRrs> {
        R0DGSL_W::new(self, 0)
    }
    #[doc = "Bits 12:13 - R0DGPS"]
    #[inline(always)]
    #[must_use]
    pub fn r0dgps(&mut self) -> R0DGPS_W<DDRPHYC_DX2DQSTRrs> {
        R0DGPS_W::new(self, 12)
    }
    #[doc = "Bits 20:22 - DQSDLY"]
    #[inline(always)]
    #[must_use]
    pub fn dqsdly(&mut self) -> DQSDLY_W<DDRPHYC_DX2DQSTRrs> {
        DQSDLY_W::new(self, 20)
    }
    #[doc = "Bits 23:25 - DQSNDLY"]
    #[inline(always)]
    #[must_use]
    pub fn dqsndly(&mut self) -> DQSNDLY_W<DDRPHYC_DX2DQSTRrs> {
        DQSNDLY_W::new(self, 23)
    }
    #[doc = "Bits 26:29 - DMDLY"]
    #[inline(always)]
    #[must_use]
    pub fn dmdly(&mut self) -> DMDLY_W<DDRPHYC_DX2DQSTRrs> {
        DMDLY_W::new(self, 26)
    }
}
#[doc = "DDRPHYC byte lane 2 DQST register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrphyc_dx2dqstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrphyc_dx2dqstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRPHYC_DX2DQSTRrs;
impl crate::RegisterSpec for DDRPHYC_DX2DQSTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrphyc_dx2dqstr::R`](R) reader structure"]
impl crate::Readable for DDRPHYC_DX2DQSTRrs {}
#[doc = "`write(|w| ..)` method takes [`ddrphyc_dx2dqstr::W`](W) writer structure"]
impl crate::Writable for DDRPHYC_DX2DQSTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRPHYC_DX2DQSTR to value 0x3db0_2000"]
impl crate::Resettable for DDRPHYC_DX2DQSTRrs {
    const RESET_VALUE: u32 = 0x3db0_2000;
}

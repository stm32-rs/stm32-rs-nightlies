#[doc = "Register `DCMI_ESCR` reader"]
pub type R = crate::R<DCMI_ESCRrs>;
#[doc = "Register `DCMI_ESCR` writer"]
pub type W = crate::W<DCMI_ESCRrs>;
#[doc = "Field `FSC` reader - FSC"]
pub type FSC_R = crate::FieldReader;
#[doc = "Field `FSC` writer - FSC"]
pub type FSC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LSC` reader - LSC"]
pub type LSC_R = crate::FieldReader;
#[doc = "Field `LSC` writer - LSC"]
pub type LSC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LEC` reader - LEC"]
pub type LEC_R = crate::FieldReader;
#[doc = "Field `LEC` writer - LEC"]
pub type LEC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FEC` reader - FEC"]
pub type FEC_R = crate::FieldReader;
#[doc = "Field `FEC` writer - FEC"]
pub type FEC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - FSC"]
    #[inline(always)]
    pub fn fsc(&self) -> FSC_R {
        FSC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - LSC"]
    #[inline(always)]
    pub fn lsc(&self) -> LSC_R {
        LSC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - LEC"]
    #[inline(always)]
    pub fn lec(&self) -> LEC_R {
        LEC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - FEC"]
    #[inline(always)]
    pub fn fec(&self) -> FEC_R {
        FEC_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - FSC"]
    #[inline(always)]
    #[must_use]
    pub fn fsc(&mut self) -> FSC_W<DCMI_ESCRrs> {
        FSC_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - LSC"]
    #[inline(always)]
    #[must_use]
    pub fn lsc(&mut self) -> LSC_W<DCMI_ESCRrs> {
        LSC_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - LEC"]
    #[inline(always)]
    #[must_use]
    pub fn lec(&mut self) -> LEC_W<DCMI_ESCRrs> {
        LEC_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - FEC"]
    #[inline(always)]
    #[must_use]
    pub fn fec(&mut self) -> FEC_W<DCMI_ESCRrs> {
        FEC_W::new(self, 24)
    }
}
#[doc = "DCMI embedded synchronization code register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcmi_escr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcmi_escr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCMI_ESCRrs;
impl crate::RegisterSpec for DCMI_ESCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcmi_escr::R`](R) reader structure"]
impl crate::Readable for DCMI_ESCRrs {}
#[doc = "`write(|w| ..)` method takes [`dcmi_escr::W`](W) writer structure"]
impl crate::Writable for DCMI_ESCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCMI_ESCR to value 0"]
impl crate::Resettable for DCMI_ESCRrs {
    const RESET_VALUE: u32 = 0;
}

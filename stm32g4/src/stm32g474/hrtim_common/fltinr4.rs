#[doc = "Register `FLTINR4` reader"]
pub type R = crate::R<FLTINR4rs>;
#[doc = "Register `FLTINR4` writer"]
pub type W = crate::W<FLTINR4rs>;
#[doc = "Field `FLT5BLKE` reader - FLT5BLKE"]
pub type FLT5BLKE_R = crate::BitReader;
#[doc = "Field `FLT5BLKE` writer - FLT5BLKE"]
pub type FLT5BLKE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT5BLKS` reader - FLT5BLKS"]
pub type FLT5BLKS_R = crate::BitReader;
#[doc = "Field `FLT5BLKS` writer - FLT5BLKS"]
pub type FLT5BLKS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT5CNT` reader - FLT5CNT"]
pub type FLT5CNT_R = crate::FieldReader;
#[doc = "Field `FLT5CNT` writer - FLT5CNT"]
pub type FLT5CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FLT5CRES` reader - FLT5CRES"]
pub type FLT5CRES_R = crate::BitReader;
#[doc = "Field `FLT5CRES` writer - FLT5CRES"]
pub type FLT5CRES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT5RSTM` reader - FLT5RSTM"]
pub type FLT5RSTM_R = crate::BitReader;
#[doc = "Field `FLT5RSTM` writer - FLT5RSTM"]
pub type FLT5RSTM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT6BLKE` reader - FLT6BLKE"]
pub type FLT6BLKE_R = crate::BitReader;
#[doc = "Field `FLT6BLKE` writer - FLT6BLKE"]
pub type FLT6BLKE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT6BLKS` reader - FLT6BLKS"]
pub type FLT6BLKS_R = crate::BitReader;
#[doc = "Field `FLT6BLKS` writer - FLT6BLKS"]
pub type FLT6BLKS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT6CNT` reader - FLT6CNT"]
pub type FLT6CNT_R = crate::FieldReader;
#[doc = "Field `FLT6CNT` writer - FLT6CNT"]
pub type FLT6CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FLT6CRES` reader - FLT6CRES"]
pub type FLT6CRES_R = crate::BitReader;
#[doc = "Field `FLT6CRES` writer - FLT6CRES"]
pub type FLT6CRES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT6RSTM` reader - FLT6RSTM"]
pub type FLT6RSTM_R = crate::BitReader;
#[doc = "Field `FLT6RSTM` writer - FLT6RSTM"]
pub type FLT6RSTM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - FLT5BLKE"]
    #[inline(always)]
    pub fn flt5blke(&self) -> FLT5BLKE_R {
        FLT5BLKE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FLT5BLKS"]
    #[inline(always)]
    pub fn flt5blks(&self) -> FLT5BLKS_R {
        FLT5BLKS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - FLT5CNT"]
    #[inline(always)]
    pub fn flt5cnt(&self) -> FLT5CNT_R {
        FLT5CNT_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - FLT5CRES"]
    #[inline(always)]
    pub fn flt5cres(&self) -> FLT5CRES_R {
        FLT5CRES_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - FLT5RSTM"]
    #[inline(always)]
    pub fn flt5rstm(&self) -> FLT5RSTM_R {
        FLT5RSTM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - FLT6BLKE"]
    #[inline(always)]
    pub fn flt6blke(&self) -> FLT6BLKE_R {
        FLT6BLKE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - FLT6BLKS"]
    #[inline(always)]
    pub fn flt6blks(&self) -> FLT6BLKS_R {
        FLT6BLKS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13 - FLT6CNT"]
    #[inline(always)]
    pub fn flt6cnt(&self) -> FLT6CNT_R {
        FLT6CNT_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 14 - FLT6CRES"]
    #[inline(always)]
    pub fn flt6cres(&self) -> FLT6CRES_R {
        FLT6CRES_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - FLT6RSTM"]
    #[inline(always)]
    pub fn flt6rstm(&self) -> FLT6RSTM_R {
        FLT6RSTM_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FLT5BLKE"]
    #[inline(always)]
    #[must_use]
    pub fn flt5blke(&mut self) -> FLT5BLKE_W<FLTINR4rs> {
        FLT5BLKE_W::new(self, 0)
    }
    #[doc = "Bit 1 - FLT5BLKS"]
    #[inline(always)]
    #[must_use]
    pub fn flt5blks(&mut self) -> FLT5BLKS_W<FLTINR4rs> {
        FLT5BLKS_W::new(self, 1)
    }
    #[doc = "Bits 2:5 - FLT5CNT"]
    #[inline(always)]
    #[must_use]
    pub fn flt5cnt(&mut self) -> FLT5CNT_W<FLTINR4rs> {
        FLT5CNT_W::new(self, 2)
    }
    #[doc = "Bit 6 - FLT5CRES"]
    #[inline(always)]
    #[must_use]
    pub fn flt5cres(&mut self) -> FLT5CRES_W<FLTINR4rs> {
        FLT5CRES_W::new(self, 6)
    }
    #[doc = "Bit 7 - FLT5RSTM"]
    #[inline(always)]
    #[must_use]
    pub fn flt5rstm(&mut self) -> FLT5RSTM_W<FLTINR4rs> {
        FLT5RSTM_W::new(self, 7)
    }
    #[doc = "Bit 8 - FLT6BLKE"]
    #[inline(always)]
    #[must_use]
    pub fn flt6blke(&mut self) -> FLT6BLKE_W<FLTINR4rs> {
        FLT6BLKE_W::new(self, 8)
    }
    #[doc = "Bit 9 - FLT6BLKS"]
    #[inline(always)]
    #[must_use]
    pub fn flt6blks(&mut self) -> FLT6BLKS_W<FLTINR4rs> {
        FLT6BLKS_W::new(self, 9)
    }
    #[doc = "Bits 10:13 - FLT6CNT"]
    #[inline(always)]
    #[must_use]
    pub fn flt6cnt(&mut self) -> FLT6CNT_W<FLTINR4rs> {
        FLT6CNT_W::new(self, 10)
    }
    #[doc = "Bit 14 - FLT6CRES"]
    #[inline(always)]
    #[must_use]
    pub fn flt6cres(&mut self) -> FLT6CRES_W<FLTINR4rs> {
        FLT6CRES_W::new(self, 14)
    }
    #[doc = "Bit 15 - FLT6RSTM"]
    #[inline(always)]
    #[must_use]
    pub fn flt6rstm(&mut self) -> FLT6RSTM_W<FLTINR4rs> {
        FLT6RSTM_W::new(self, 15)
    }
}
#[doc = "HRTIM Fault Input Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fltinr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fltinr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLTINR4rs;
impl crate::RegisterSpec for FLTINR4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fltinr4::R`](R) reader structure"]
impl crate::Readable for FLTINR4rs {}
#[doc = "`write(|w| ..)` method takes [`fltinr4::W`](W) writer structure"]
impl crate::Writable for FLTINR4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLTINR4 to value 0"]
impl crate::Resettable for FLTINR4rs {
    const RESET_VALUE: u32 = 0;
}

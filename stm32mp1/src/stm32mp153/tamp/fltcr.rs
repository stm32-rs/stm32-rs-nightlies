#[doc = "Register `FLTCR` reader"]
pub type R = crate::R<FLTCRrs>;
#[doc = "Register `FLTCR` writer"]
pub type W = crate::W<FLTCRrs>;
#[doc = "Field `TAMPFREQ` reader - TAMPFREQ"]
pub type TAMPFREQ_R = crate::FieldReader;
#[doc = "Field `TAMPFREQ` writer - TAMPFREQ"]
pub type TAMPFREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TAMPFLT` reader - TAMPFLT"]
pub type TAMPFLT_R = crate::FieldReader;
#[doc = "Field `TAMPFLT` writer - TAMPFLT"]
pub type TAMPFLT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TAMPPRCH` reader - TAMPPRCH"]
pub type TAMPPRCH_R = crate::FieldReader;
#[doc = "Field `TAMPPRCH` writer - TAMPPRCH"]
pub type TAMPPRCH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TAMPPUDIS` reader - TAMPPUDIS"]
pub type TAMPPUDIS_R = crate::BitReader;
#[doc = "Field `TAMPPUDIS` writer - TAMPPUDIS"]
pub type TAMPPUDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - TAMPFREQ"]
    #[inline(always)]
    pub fn tampfreq(&self) -> TAMPFREQ_R {
        TAMPFREQ_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - TAMPFLT"]
    #[inline(always)]
    pub fn tampflt(&self) -> TAMPFLT_R {
        TAMPFLT_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - TAMPPRCH"]
    #[inline(always)]
    pub fn tampprch(&self) -> TAMPPRCH_R {
        TAMPPRCH_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - TAMPPUDIS"]
    #[inline(always)]
    pub fn tamppudis(&self) -> TAMPPUDIS_R {
        TAMPPUDIS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - TAMPFREQ"]
    #[inline(always)]
    #[must_use]
    pub fn tampfreq(&mut self) -> TAMPFREQ_W<FLTCRrs> {
        TAMPFREQ_W::new(self, 0)
    }
    #[doc = "Bits 3:4 - TAMPFLT"]
    #[inline(always)]
    #[must_use]
    pub fn tampflt(&mut self) -> TAMPFLT_W<FLTCRrs> {
        TAMPFLT_W::new(self, 3)
    }
    #[doc = "Bits 5:6 - TAMPPRCH"]
    #[inline(always)]
    #[must_use]
    pub fn tampprch(&mut self) -> TAMPPRCH_W<FLTCRrs> {
        TAMPPRCH_W::new(self, 5)
    }
    #[doc = "Bit 7 - TAMPPUDIS"]
    #[inline(always)]
    #[must_use]
    pub fn tamppudis(&mut self) -> TAMPPUDIS_W<FLTCRrs> {
        TAMPPUDIS_W::new(self, 7)
    }
}
#[doc = "This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fltcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fltcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLTCRrs;
impl crate::RegisterSpec for FLTCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fltcr::R`](R) reader structure"]
impl crate::Readable for FLTCRrs {}
#[doc = "`write(|w| ..)` method takes [`fltcr::W`](W) writer structure"]
impl crate::Writable for FLTCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLTCR to value 0"]
impl crate::Resettable for FLTCRrs {
    const RESET_VALUE: u32 = 0;
}

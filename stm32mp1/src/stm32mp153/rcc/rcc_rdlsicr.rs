#[doc = "Register `RCC_RDLSICR` reader"]
pub type R = crate::R<RCC_RDLSICRrs>;
#[doc = "Register `RCC_RDLSICR` writer"]
pub type W = crate::W<RCC_RDLSICRrs>;
#[doc = "Field `LSION` reader - LSION"]
pub type LSION_R = crate::BitReader;
#[doc = "Field `LSION` writer - LSION"]
pub type LSION_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSIRDY` reader - LSIRDY"]
pub type LSIRDY_R = crate::BitReader;
#[doc = "Field `MRD` reader - MRD"]
pub type MRD_R = crate::FieldReader;
#[doc = "Field `MRD` writer - MRD"]
pub type MRD_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `EADLY` reader - EADLY"]
pub type EADLY_R = crate::FieldReader;
#[doc = "Field `EADLY` writer - EADLY"]
pub type EADLY_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SPARE` reader - SPARE"]
pub type SPARE_R = crate::FieldReader;
#[doc = "Field `SPARE` writer - SPARE"]
pub type SPARE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - LSION"]
    #[inline(always)]
    pub fn lsion(&self) -> LSION_R {
        LSION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSIRDY"]
    #[inline(always)]
    pub fn lsirdy(&self) -> LSIRDY_R {
        LSIRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 16:20 - MRD"]
    #[inline(always)]
    pub fn mrd(&self) -> MRD_R {
        MRD_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:26 - EADLY"]
    #[inline(always)]
    pub fn eadly(&self) -> EADLY_R {
        EADLY_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:31 - SPARE"]
    #[inline(always)]
    pub fn spare(&self) -> SPARE_R {
        SPARE_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - LSION"]
    #[inline(always)]
    #[must_use]
    pub fn lsion(&mut self) -> LSION_W<RCC_RDLSICRrs> {
        LSION_W::new(self, 0)
    }
    #[doc = "Bits 16:20 - MRD"]
    #[inline(always)]
    #[must_use]
    pub fn mrd(&mut self) -> MRD_W<RCC_RDLSICRrs> {
        MRD_W::new(self, 16)
    }
    #[doc = "Bits 24:26 - EADLY"]
    #[inline(always)]
    #[must_use]
    pub fn eadly(&mut self) -> EADLY_W<RCC_RDLSICRrs> {
        EADLY_W::new(self, 24)
    }
    #[doc = "Bits 27:31 - SPARE"]
    #[inline(always)]
    #[must_use]
    pub fn spare(&mut self) -> SPARE_W<RCC_RDLSICRrs> {
        SPARE_W::new(self, 27)
    }
}
#[doc = "This register is used to control the minimum NRST active duration and LSI function.0 to 7 wait states are inserted for word, half-word and byte accesses. Wait states are inserted in case of successive accesses to this register.This register is reset by the por_rst reset, and it is located into the VDD domain. If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_rdlsicr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_rdlsicr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_RDLSICRrs;
impl crate::RegisterSpec for RCC_RDLSICRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_rdlsicr::R`](R) reader structure"]
impl crate::Readable for RCC_RDLSICRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_rdlsicr::W`](W) writer structure"]
impl crate::Writable for RCC_RDLSICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_RDLSICR to value 0"]
impl crate::Resettable for RCC_RDLSICRrs {
    const RESET_VALUE: u32 = 0;
}

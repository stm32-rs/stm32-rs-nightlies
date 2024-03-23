#[doc = "Register `LTDC_GCR` reader"]
pub type R = crate::R<LTDC_GCRrs>;
#[doc = "Register `LTDC_GCR` writer"]
pub type W = crate::W<LTDC_GCRrs>;
#[doc = "Field `LTDCEN` reader - LTDCEN"]
pub type LTDCEN_R = crate::BitReader;
#[doc = "Field `LTDCEN` writer - LTDCEN"]
pub type LTDCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBW` reader - DBW"]
pub type DBW_R = crate::FieldReader;
#[doc = "Field `DGW` reader - DGW"]
pub type DGW_R = crate::FieldReader;
#[doc = "Field `DRW` reader - DRW"]
pub type DRW_R = crate::FieldReader;
#[doc = "Field `DEN` reader - DEN"]
pub type DEN_R = crate::BitReader;
#[doc = "Field `DEN` writer - DEN"]
pub type DEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCPOL` reader - PCPOL"]
pub type PCPOL_R = crate::BitReader;
#[doc = "Field `PCPOL` writer - PCPOL"]
pub type PCPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEPOL` reader - DEPOL"]
pub type DEPOL_R = crate::BitReader;
#[doc = "Field `DEPOL` writer - DEPOL"]
pub type DEPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSPOL` reader - VSPOL"]
pub type VSPOL_R = crate::BitReader;
#[doc = "Field `VSPOL` writer - VSPOL"]
pub type VSPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSPOL` reader - HSPOL"]
pub type HSPOL_R = crate::BitReader;
#[doc = "Field `HSPOL` writer - HSPOL"]
pub type HSPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LTDCEN"]
    #[inline(always)]
    pub fn ltdcen(&self) -> LTDCEN_R {
        LTDCEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - DBW"]
    #[inline(always)]
    pub fn dbw(&self) -> DBW_R {
        DBW_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - DGW"]
    #[inline(always)]
    pub fn dgw(&self) -> DGW_R {
        DGW_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - DRW"]
    #[inline(always)]
    pub fn drw(&self) -> DRW_R {
        DRW_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 16 - DEN"]
    #[inline(always)]
    pub fn den(&self) -> DEN_R {
        DEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 28 - PCPOL"]
    #[inline(always)]
    pub fn pcpol(&self) -> PCPOL_R {
        PCPOL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DEPOL"]
    #[inline(always)]
    pub fn depol(&self) -> DEPOL_R {
        DEPOL_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - VSPOL"]
    #[inline(always)]
    pub fn vspol(&self) -> VSPOL_R {
        VSPOL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - HSPOL"]
    #[inline(always)]
    pub fn hspol(&self) -> HSPOL_R {
        HSPOL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LTDCEN"]
    #[inline(always)]
    #[must_use]
    pub fn ltdcen(&mut self) -> LTDCEN_W<LTDC_GCRrs> {
        LTDCEN_W::new(self, 0)
    }
    #[doc = "Bit 16 - DEN"]
    #[inline(always)]
    #[must_use]
    pub fn den(&mut self) -> DEN_W<LTDC_GCRrs> {
        DEN_W::new(self, 16)
    }
    #[doc = "Bit 28 - PCPOL"]
    #[inline(always)]
    #[must_use]
    pub fn pcpol(&mut self) -> PCPOL_W<LTDC_GCRrs> {
        PCPOL_W::new(self, 28)
    }
    #[doc = "Bit 29 - DEPOL"]
    #[inline(always)]
    #[must_use]
    pub fn depol(&mut self) -> DEPOL_W<LTDC_GCRrs> {
        DEPOL_W::new(self, 29)
    }
    #[doc = "Bit 30 - VSPOL"]
    #[inline(always)]
    #[must_use]
    pub fn vspol(&mut self) -> VSPOL_W<LTDC_GCRrs> {
        VSPOL_W::new(self, 30)
    }
    #[doc = "Bit 31 - HSPOL"]
    #[inline(always)]
    #[must_use]
    pub fn hspol(&mut self) -> HSPOL_W<LTDC_GCRrs> {
        HSPOL_W::new(self, 31)
    }
}
#[doc = "This register defines the global configuration of the LCD-TFT controller.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ltdc_gcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ltdc_gcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LTDC_GCRrs;
impl crate::RegisterSpec for LTDC_GCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ltdc_gcr::R`](R) reader structure"]
impl crate::Readable for LTDC_GCRrs {}
#[doc = "`write(|w| ..)` method takes [`ltdc_gcr::W`](W) writer structure"]
impl crate::Writable for LTDC_GCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LTDC_GCR to value 0x2220"]
impl crate::Resettable for LTDC_GCRrs {
    const RESET_VALUE: u32 = 0x2220;
}

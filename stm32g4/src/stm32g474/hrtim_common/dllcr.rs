#[doc = "Register `DLLCR` reader"]
pub type R = crate::R<DLLCRrs>;
#[doc = "Register `DLLCR` writer"]
pub type W = crate::W<DLLCRrs>;
#[doc = "Field `CAL` reader - DLL Calibration Start"]
pub type CAL_R = crate::BitReader;
#[doc = "Field `CAL` writer - DLL Calibration Start"]
pub type CAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALEN` reader - DLL Calibration Enable"]
pub type CALEN_R = crate::BitReader;
#[doc = "Field `CALEN` writer - DLL Calibration Enable"]
pub type CALEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALRTE` reader - DLL Calibration rate"]
pub type CALRTE_R = crate::FieldReader;
#[doc = "Field `CALRTE` writer - DLL Calibration rate"]
pub type CALRTE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - DLL Calibration Start"]
    #[inline(always)]
    pub fn cal(&self) -> CAL_R {
        CAL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DLL Calibration Enable"]
    #[inline(always)]
    pub fn calen(&self) -> CALEN_R {
        CALEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - DLL Calibration rate"]
    #[inline(always)]
    pub fn calrte(&self) -> CALRTE_R {
        CALRTE_R::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DLL Calibration Start"]
    #[inline(always)]
    #[must_use]
    pub fn cal(&mut self) -> CAL_W<DLLCRrs> {
        CAL_W::new(self, 0)
    }
    #[doc = "Bit 1 - DLL Calibration Enable"]
    #[inline(always)]
    #[must_use]
    pub fn calen(&mut self) -> CALEN_W<DLLCRrs> {
        CALEN_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - DLL Calibration rate"]
    #[inline(always)]
    #[must_use]
    pub fn calrte(&mut self) -> CALRTE_W<DLLCRrs> {
        CALRTE_W::new(self, 2)
    }
}
#[doc = "DLL Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dllcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dllcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DLLCRrs;
impl crate::RegisterSpec for DLLCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dllcr::R`](R) reader structure"]
impl crate::Readable for DLLCRrs {}
#[doc = "`write(|w| ..)` method takes [`dllcr::W`](W) writer structure"]
impl crate::Writable for DLLCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DLLCR to value 0"]
impl crate::Resettable for DLLCRrs {
    const RESET_VALUE: u32 = 0;
}

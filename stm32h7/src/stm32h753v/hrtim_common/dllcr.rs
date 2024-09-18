///Register `DLLCR` reader
pub type R = crate::R<DLLCRrs>;
///Register `DLLCR` writer
pub type W = crate::W<DLLCRrs>;
///Field `CAL` reader - DLL Calibration Start
pub type CAL_R = crate::BitReader;
///Field `CAL` writer - DLL Calibration Start
pub type CAL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CALEN` reader - DLL Calibration Enable
pub type CALEN_R = crate::BitReader;
///Field `CALEN` writer - DLL Calibration Enable
pub type CALEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CALRTE` reader - DLL Calibration rate
pub type CALRTE_R = crate::FieldReader;
///Field `CALRTE` writer - DLL Calibration rate
pub type CALRTE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - DLL Calibration Start
    #[inline(always)]
    pub fn cal(&self) -> CAL_R {
        CAL_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DLL Calibration Enable
    #[inline(always)]
    pub fn calen(&self) -> CALEN_R {
        CALEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - DLL Calibration rate
    #[inline(always)]
    pub fn calrte(&self) -> CALRTE_R {
        CALRTE_R::new(((self.bits >> 2) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DLLCR")
            .field("calrte", &self.calrte())
            .field("calen", &self.calen())
            .field("cal", &self.cal())
            .finish()
    }
}
impl W {
    ///Bit 0 - DLL Calibration Start
    #[inline(always)]
    #[must_use]
    pub fn cal(&mut self) -> CAL_W<DLLCRrs> {
        CAL_W::new(self, 0)
    }
    ///Bit 1 - DLL Calibration Enable
    #[inline(always)]
    #[must_use]
    pub fn calen(&mut self) -> CALEN_W<DLLCRrs> {
        CALEN_W::new(self, 1)
    }
    ///Bits 2:3 - DLL Calibration rate
    #[inline(always)]
    #[must_use]
    pub fn calrte(&mut self) -> CALRTE_W<DLLCRrs> {
        CALRTE_W::new(self, 2)
    }
}
/**DLL Control Register

You can [`read`](crate::Reg::read) this register and get [`dllcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dllcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753.html#HRTIM_Common:DLLCR)*/
pub struct DLLCRrs;
impl crate::RegisterSpec for DLLCRrs {
    type Ux = u32;
}
///`read()` method returns [`dllcr::R`](R) reader structure
impl crate::Readable for DLLCRrs {}
///`write(|w| ..)` method takes [`dllcr::W`](W) writer structure
impl crate::Writable for DLLCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DLLCR to value 0
impl crate::Resettable for DLLCRrs {
    const RESET_VALUE: u32 = 0;
}

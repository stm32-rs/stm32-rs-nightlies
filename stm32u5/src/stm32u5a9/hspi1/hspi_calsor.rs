#[doc = "Register `HSPI_CALSOR` reader"]
pub type R = crate::R<HSPI_CALSORrs>;
#[doc = "Register `HSPI_CALSOR` writer"]
pub type W = crate::W<HSPI_CALSORrs>;
#[doc = "Field `FINE` reader - 6: 0\\]: Fine calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet)."]
pub type FINE_R = crate::FieldReader;
#[doc = "Field `FINE` writer - 6: 0\\]: Fine calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet)."]
pub type FINE_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `COARSE` reader - 4: 0\\]: Coarse calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet)."]
pub type COARSE_R = crate::FieldReader;
#[doc = "Field `COARSE` writer - 4: 0\\]: Coarse calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet)."]
pub type COARSE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:6 - 6: 0\\]: Fine calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet)."]
    #[inline(always)]
    pub fn fine(&self) -> FINE_R {
        FINE_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:20 - 4: 0\\]: Coarse calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet)."]
    #[inline(always)]
    pub fn coarse(&self) -> COARSE_R {
        COARSE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6: 0\\]: Fine calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet)."]
    #[inline(always)]
    #[must_use]
    pub fn fine(&mut self) -> FINE_W<HSPI_CALSORrs> {
        FINE_W::new(self, 0)
    }
    #[doc = "Bits 16:20 - 4: 0\\]: Coarse calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet)."]
    #[inline(always)]
    #[must_use]
    pub fn coarse(&mut self) -> COARSE_W<HSPI_CALSORrs> {
        COARSE_W::new(self, 16)
    }
}
#[doc = "HSPI DLL slave output calibration configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hspi_calsor::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hspi_calsor::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSPI_CALSORrs;
impl crate::RegisterSpec for HSPI_CALSORrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hspi_calsor::R`](R) reader structure"]
impl crate::Readable for HSPI_CALSORrs {}
#[doc = "`write(|w| ..)` method takes [`hspi_calsor::W`](W) writer structure"]
impl crate::Writable for HSPI_CALSORrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HSPI_CALSOR to value 0"]
impl crate::Resettable for HSPI_CALSORrs {
    const RESET_VALUE: u32 = 0;
}

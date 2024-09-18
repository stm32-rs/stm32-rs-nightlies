///Register `HSPI_CALSOR` reader
pub type R = crate::R<HSPI_CALSORrs>;
///Register `HSPI_CALSOR` writer
pub type W = crate::W<HSPI_CALSORrs>;
///Field `FINE` reader - 6: 0\]: Fine calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet).
pub type FINE_R = crate::FieldReader;
///Field `FINE` writer - 6: 0\]: Fine calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet).
pub type FINE_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `COARSE` reader - 4: 0\]: Coarse calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet).
pub type COARSE_R = crate::FieldReader;
///Field `COARSE` writer - 4: 0\]: Coarse calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet).
pub type COARSE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:6 - 6: 0\]: Fine calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet).
    #[inline(always)]
    pub fn fine(&self) -> FINE_R {
        FINE_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 16:20 - 4: 0\]: Coarse calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet).
    #[inline(always)]
    pub fn coarse(&self) -> COARSE_R {
        COARSE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSPI_CALSOR")
            .field("fine", &self.fine())
            .field("coarse", &self.coarse())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - 6: 0\]: Fine calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet).
    #[inline(always)]
    #[must_use]
    pub fn fine(&mut self) -> FINE_W<HSPI_CALSORrs> {
        FINE_W::new(self, 0)
    }
    ///Bits 16:20 - 4: 0\]: Coarse calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet).
    #[inline(always)]
    #[must_use]
    pub fn coarse(&mut self) -> COARSE_W<HSPI_CALSORrs> {
        COARSE_W::new(self, 16)
    }
}
/**HSPI DLL slave output calibration configuration

You can [`read`](crate::Reg::read) this register and get [`hspi_calsor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hspi_calsor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#HSPI1:HSPI_CALSOR)*/
pub struct HSPI_CALSORrs;
impl crate::RegisterSpec for HSPI_CALSORrs {
    type Ux = u32;
}
///`read()` method returns [`hspi_calsor::R`](R) reader structure
impl crate::Readable for HSPI_CALSORrs {}
///`write(|w| ..)` method takes [`hspi_calsor::W`](W) writer structure
impl crate::Writable for HSPI_CALSORrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HSPI_CALSOR to value 0
impl crate::Resettable for HSPI_CALSORrs {
    const RESET_VALUE: u32 = 0;
}

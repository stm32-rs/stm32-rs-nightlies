#[doc = "Register `HSPI_DCR4` reader"]
pub type R = crate::R<HSPI_DCR4rs>;
#[doc = "Register `HSPI_DCR4` writer"]
pub type W = crate::W<HSPI_DCR4rs>;
#[doc = "Field `REFRESH` reader - Refresh rate This field enables the refresh rate feature. The nCS is released every REFRESH+1 clock cycles for writes, and REFRESH+4 clock cycles for reads. Note: These two values can be extended with few clock cycles when refresh occurs during a byte transmission in single, dual or quad mode, because the byte transmission must be completed. others: Maximum communication length is set to REFRESH+1 clock cycles."]
pub type REFRESH_R = crate::FieldReader<u32>;
#[doc = "Field `REFRESH` writer - Refresh rate This field enables the refresh rate feature. The nCS is released every REFRESH+1 clock cycles for writes, and REFRESH+4 clock cycles for reads. Note: These two values can be extended with few clock cycles when refresh occurs during a byte transmission in single, dual or quad mode, because the byte transmission must be completed. others: Maximum communication length is set to REFRESH+1 clock cycles."]
pub type REFRESH_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Refresh rate This field enables the refresh rate feature. The nCS is released every REFRESH+1 clock cycles for writes, and REFRESH+4 clock cycles for reads. Note: These two values can be extended with few clock cycles when refresh occurs during a byte transmission in single, dual or quad mode, because the byte transmission must be completed. others: Maximum communication length is set to REFRESH+1 clock cycles."]
    #[inline(always)]
    pub fn refresh(&self) -> REFRESH_R {
        REFRESH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Refresh rate This field enables the refresh rate feature. The nCS is released every REFRESH+1 clock cycles for writes, and REFRESH+4 clock cycles for reads. Note: These two values can be extended with few clock cycles when refresh occurs during a byte transmission in single, dual or quad mode, because the byte transmission must be completed. others: Maximum communication length is set to REFRESH+1 clock cycles."]
    #[inline(always)]
    #[must_use]
    pub fn refresh(&mut self) -> REFRESH_W<HSPI_DCR4rs> {
        REFRESH_W::new(self, 0)
    }
}
#[doc = "HSPI device configuration register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hspi_dcr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hspi_dcr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSPI_DCR4rs;
impl crate::RegisterSpec for HSPI_DCR4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hspi_dcr4::R`](R) reader structure"]
impl crate::Readable for HSPI_DCR4rs {}
#[doc = "`write(|w| ..)` method takes [`hspi_dcr4::W`](W) writer structure"]
impl crate::Writable for HSPI_DCR4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HSPI_DCR4 to value 0"]
impl crate::Resettable for HSPI_DCR4rs {
    const RESET_VALUE: u32 = 0;
}

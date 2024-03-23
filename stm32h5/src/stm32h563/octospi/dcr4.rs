#[doc = "Register `DCR4` reader"]
pub type R = crate::R<DCR4rs>;
#[doc = "Register `DCR4` writer"]
pub type W = crate::W<DCR4rs>;
#[doc = "Field `REFRESH` reader - Refresh rate This field enables the refresh rate feature. The NCS is released every REFRESH + 1 clock cycles for writes, and REFRESH + 4 clock cycles for reads. Note: These two values can be extended with few clock cycles when refresh occurs during a byte transmission in Single-, Dual- or Quad-SPI mode, because the byte transmission must be completed. others: Maximum communication length is set to REFRESH + 1 clock cycles."]
pub type REFRESH_R = crate::FieldReader<u32>;
#[doc = "Field `REFRESH` writer - Refresh rate This field enables the refresh rate feature. The NCS is released every REFRESH + 1 clock cycles for writes, and REFRESH + 4 clock cycles for reads. Note: These two values can be extended with few clock cycles when refresh occurs during a byte transmission in Single-, Dual- or Quad-SPI mode, because the byte transmission must be completed. others: Maximum communication length is set to REFRESH + 1 clock cycles."]
pub type REFRESH_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Refresh rate This field enables the refresh rate feature. The NCS is released every REFRESH + 1 clock cycles for writes, and REFRESH + 4 clock cycles for reads. Note: These two values can be extended with few clock cycles when refresh occurs during a byte transmission in Single-, Dual- or Quad-SPI mode, because the byte transmission must be completed. others: Maximum communication length is set to REFRESH + 1 clock cycles."]
    #[inline(always)]
    pub fn refresh(&self) -> REFRESH_R {
        REFRESH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Refresh rate This field enables the refresh rate feature. The NCS is released every REFRESH + 1 clock cycles for writes, and REFRESH + 4 clock cycles for reads. Note: These two values can be extended with few clock cycles when refresh occurs during a byte transmission in Single-, Dual- or Quad-SPI mode, because the byte transmission must be completed. others: Maximum communication length is set to REFRESH + 1 clock cycles."]
    #[inline(always)]
    #[must_use]
    pub fn refresh(&mut self) -> REFRESH_W<DCR4rs> {
        REFRESH_W::new(self, 0)
    }
}
#[doc = "OCTOSPI device configuration register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCR4rs;
impl crate::RegisterSpec for DCR4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcr4::R`](R) reader structure"]
impl crate::Readable for DCR4rs {}
#[doc = "`write(|w| ..)` method takes [`dcr4::W`](W) writer structure"]
impl crate::Writable for DCR4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCR4 to value 0"]
impl crate::Resettable for DCR4rs {
    const RESET_VALUE: u32 = 0;
}

///Register `OCRDYR` reader
pub type R = crate::R<OCRDYRrs>;
///Field `HSIRDY` reader - HSIRDY
pub type HSIRDY_R = crate::BitReader;
///Field `HSIDIVRDY` reader - HSIDIVRDY
pub type HSIDIVRDY_R = crate::BitReader;
///Field `CSIRDY` reader - CSIRDY
pub type CSIRDY_R = crate::BitReader;
///Field `HSERDY` reader - HSERDY
pub type HSERDY_R = crate::BitReader;
///Field `MPUCKRDY` reader - MPUCKRDY
pub type MPUCKRDY_R = crate::BitReader;
///Field `AXICKRDY` reader - AXICKRDY
pub type AXICKRDY_R = crate::BitReader;
///Field `CKREST` reader - CKREST
pub type CKREST_R = crate::BitReader;
impl R {
    ///Bit 0 - HSIRDY
    #[inline(always)]
    pub fn hsirdy(&self) -> HSIRDY_R {
        HSIRDY_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - HSIDIVRDY
    #[inline(always)]
    pub fn hsidivrdy(&self) -> HSIDIVRDY_R {
        HSIDIVRDY_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - CSIRDY
    #[inline(always)]
    pub fn csirdy(&self) -> CSIRDY_R {
        CSIRDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - HSERDY
    #[inline(always)]
    pub fn hserdy(&self) -> HSERDY_R {
        HSERDY_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 23 - MPUCKRDY
    #[inline(always)]
    pub fn mpuckrdy(&self) -> MPUCKRDY_R {
        MPUCKRDY_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - AXICKRDY
    #[inline(always)]
    pub fn axickrdy(&self) -> AXICKRDY_R {
        AXICKRDY_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - CKREST
    #[inline(always)]
    pub fn ckrest(&self) -> CKREST_R {
        CKREST_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OCRDYR")
            .field("hsirdy", &self.hsirdy())
            .field("hsidivrdy", &self.hsidivrdy())
            .field("csirdy", &self.csirdy())
            .field("hserdy", &self.hserdy())
            .field("mpuckrdy", &self.mpuckrdy())
            .field("axickrdy", &self.axickrdy())
            .field("ckrest", &self.ckrest())
            .finish()
    }
}
/**This is a read-only access register, It contains the status flags of oscillators. Writing has no effect.

You can [`read`](crate::Reg::read) this register and get [`ocrdyr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:OCRDYR)*/
pub struct OCRDYRrs;
impl crate::RegisterSpec for OCRDYRrs {
    type Ux = u32;
}
///`read()` method returns [`ocrdyr::R`](R) reader structure
impl crate::Readable for OCRDYRrs {}
///`reset()` method sets OCRDYR to value 0
impl crate::Resettable for OCRDYRrs {}

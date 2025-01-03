///Register `SCR` reader
pub type R = crate::R<SCRrs>;
///Field `CALRAF` reader - CALRAF
pub type CALRAF_R = crate::BitReader;
///Field `CALRBF` reader - CALRBF
pub type CALRBF_R = crate::BitReader;
///Field `CWUTF` reader - CWUTF
pub type CWUTF_R = crate::BitReader;
///Field `CTSF` reader - CTSF
pub type CTSF_R = crate::BitReader;
///Field `CTSOVF` reader - CTSOVF
pub type CTSOVF_R = crate::BitReader;
///Field `CITSF` reader - CITSF
pub type CITSF_R = crate::BitReader;
///Field `CSSRUF` reader - CSSRUF
pub type CSSRUF_R = crate::BitReader;
impl R {
    ///Bit 0 - CALRAF
    #[inline(always)]
    pub fn calraf(&self) -> CALRAF_R {
        CALRAF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CALRBF
    #[inline(always)]
    pub fn calrbf(&self) -> CALRBF_R {
        CALRBF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CWUTF
    #[inline(always)]
    pub fn cwutf(&self) -> CWUTF_R {
        CWUTF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CTSF
    #[inline(always)]
    pub fn ctsf(&self) -> CTSF_R {
        CTSF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CTSOVF
    #[inline(always)]
    pub fn ctsovf(&self) -> CTSOVF_R {
        CTSOVF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - CITSF
    #[inline(always)]
    pub fn citsf(&self) -> CITSF_R {
        CITSF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CSSRUF
    #[inline(always)]
    pub fn cssruf(&self) -> CSSRUF_R {
        CSSRUF_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCR")
            .field("calraf", &self.calraf())
            .field("calrbf", &self.calrbf())
            .field("cwutf", &self.cwutf())
            .field("ctsf", &self.ctsf())
            .field("ctsovf", &self.ctsovf())
            .field("citsf", &self.citsf())
            .field("cssruf", &self.cssruf())
            .finish()
    }
}
/**status clear register

You can [`read`](crate::Reg::read) this register and get [`scr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#RTC:SCR)*/
pub struct SCRrs;
impl crate::RegisterSpec for SCRrs {
    type Ux = u32;
}
///`read()` method returns [`scr::R`](R) reader structure
impl crate::Readable for SCRrs {}
///`reset()` method sets SCR to value 0
impl crate::Resettable for SCRrs {
    const RESET_VALUE: u32 = 0;
}

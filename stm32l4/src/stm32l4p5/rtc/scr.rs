///Register `SCR` reader
pub type R = crate::R<SCRrs>;
/**CALRAF

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALRAF {
    ///1: Clear interrupt flag
    Clear = 1,
}
impl From<CALRAF> for bool {
    #[inline(always)]
    fn from(variant: CALRAF) -> Self {
        variant as u8 != 0
    }
}
///Field `CALRAF` reader - CALRAF
pub type CALRAF_R = crate::BitReader<CALRAF>;
impl CALRAF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CALRAF> {
        match self.bits {
            true => Some(CALRAF::Clear),
            _ => None,
        }
    }
    ///Clear interrupt flag
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CALRAF::Clear
    }
}
///Field `CALRBF` reader - CALRBF
pub use CALRAF_R as CALRBF_R;
///Field `CWUTF` reader - CWUTF
pub use CALRAF_R as CWUTF_R;
///Field `CTSF` reader - CTSF
pub use CALRAF_R as CTSF_R;
///Field `CTSOVF` reader - CTSOVF
pub use CALRAF_R as CTSOVF_R;
///Field `CITSF` reader - CITSF
pub use CALRAF_R as CITSF_R;
///Field `CSSRUF` reader - CSSRUF
pub use CALRAF_R as CSSRUF_R;
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
impl crate::Resettable for SCRrs {}

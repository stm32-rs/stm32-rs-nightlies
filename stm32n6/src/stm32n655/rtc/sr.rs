///Register `SR` reader
pub type R = crate::R<SRrs>;
///Field `ALRAF` reader - Alarm A flag
pub type ALRAF_R = crate::BitReader;
///Field `ALRBF` reader - Alarm B flag
pub type ALRBF_R = crate::BitReader;
///Field `WUTF` reader - Wake-up timer flag
pub type WUTF_R = crate::BitReader;
///Field `TSF` reader - Timestamp flag
pub type TSF_R = crate::BitReader;
///Field `TSOVF` reader - Timestamp overflow flag
pub type TSOVF_R = crate::BitReader;
///Field `ITSF` reader - Internal timestamp flag
pub type ITSF_R = crate::BitReader;
///Field `SSRUF` reader - SSR underflow flag
pub type SSRUF_R = crate::BitReader;
impl R {
    ///Bit 0 - Alarm A flag
    #[inline(always)]
    pub fn alraf(&self) -> ALRAF_R {
        ALRAF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Alarm B flag
    #[inline(always)]
    pub fn alrbf(&self) -> ALRBF_R {
        ALRBF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Wake-up timer flag
    #[inline(always)]
    pub fn wutf(&self) -> WUTF_R {
        WUTF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Timestamp flag
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Timestamp overflow flag
    #[inline(always)]
    pub fn tsovf(&self) -> TSOVF_R {
        TSOVF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Internal timestamp flag
    #[inline(always)]
    pub fn itsf(&self) -> ITSF_R {
        ITSF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - SSR underflow flag
    #[inline(always)]
    pub fn ssruf(&self) -> SSRUF_R {
        SSRUF_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("alraf", &self.alraf())
            .field("alrbf", &self.alrbf())
            .field("wutf", &self.wutf())
            .field("tsf", &self.tsf())
            .field("tsovf", &self.tsovf())
            .field("itsf", &self.itsf())
            .field("ssruf", &self.ssruf())
            .finish()
    }
}
/**RTC status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RTC:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}

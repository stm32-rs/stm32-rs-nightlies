///Register `CCIPR7` reader
pub type R = crate::R<CCIPR7rs>;
///Register `CCIPR7` writer
pub type W = crate::W<CCIPR7rs>;
///Field `PERSEL` reader - Source selection for the PER kernel clock
pub type PERSEL_R = crate::FieldReader;
///Field `PERSEL` writer - Source selection for the PER kernel clock
pub type PERSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PSSISEL` reader - Source selection for the PSSI kernel clock
pub type PSSISEL_R = crate::FieldReader;
///Field `PSSISEL` writer - Source selection for the PSSI kernel clock
pub type PSSISEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RTCSEL` reader - Source selection for the RTC kernel clock
pub type RTCSEL_R = crate::FieldReader;
///Field `RTCSEL` writer - Source selection for the RTC kernel clock
pub type RTCSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RTCPRE` reader - RTC Prog clock divider selection (for clock ck_icn_p_risaf)
pub type RTCPRE_R = crate::FieldReader;
///Field `RTCPRE` writer - RTC Prog clock divider selection (for clock ck_icn_p_risaf)
pub type RTCPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `SAI1SEL` reader - Source selection for the SAI1 kernel clock
pub type SAI1SEL_R = crate::FieldReader;
///Field `SAI1SEL` writer - Source selection for the SAI1 kernel clock
pub type SAI1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SAI2SEL` reader - Source selection for the SAI2 kernel clock
pub type SAI2SEL_R = crate::FieldReader;
///Field `SAI2SEL` writer - Source selection for the SAI2 kernel clock
pub type SAI2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - Source selection for the PER kernel clock
    #[inline(always)]
    pub fn persel(&self) -> PERSEL_R {
        PERSEL_R::new((self.bits & 7) as u8)
    }
    ///Bits 4:5 - Source selection for the PSSI kernel clock
    #[inline(always)]
    pub fn pssisel(&self) -> PSSISEL_R {
        PSSISEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 8:9 - Source selection for the RTC kernel clock
    #[inline(always)]
    pub fn rtcsel(&self) -> RTCSEL_R {
        RTCSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 12:17 - RTC Prog clock divider selection (for clock ck_icn_p_risaf)
    #[inline(always)]
    pub fn rtcpre(&self) -> RTCPRE_R {
        RTCPRE_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    ///Bits 20:22 - Source selection for the SAI1 kernel clock
    #[inline(always)]
    pub fn sai1sel(&self) -> SAI1SEL_R {
        SAI1SEL_R::new(((self.bits >> 20) & 7) as u8)
    }
    ///Bits 24:26 - Source selection for the SAI2 kernel clock
    #[inline(always)]
    pub fn sai2sel(&self) -> SAI2SEL_R {
        SAI2SEL_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCIPR7")
            .field("persel", &self.persel())
            .field("pssisel", &self.pssisel())
            .field("rtcsel", &self.rtcsel())
            .field("rtcpre", &self.rtcpre())
            .field("sai1sel", &self.sai1sel())
            .field("sai2sel", &self.sai2sel())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Source selection for the PER kernel clock
    #[inline(always)]
    pub fn persel(&mut self) -> PERSEL_W<'_, CCIPR7rs> {
        PERSEL_W::new(self, 0)
    }
    ///Bits 4:5 - Source selection for the PSSI kernel clock
    #[inline(always)]
    pub fn pssisel(&mut self) -> PSSISEL_W<'_, CCIPR7rs> {
        PSSISEL_W::new(self, 4)
    }
    ///Bits 8:9 - Source selection for the RTC kernel clock
    #[inline(always)]
    pub fn rtcsel(&mut self) -> RTCSEL_W<'_, CCIPR7rs> {
        RTCSEL_W::new(self, 8)
    }
    ///Bits 12:17 - RTC Prog clock divider selection (for clock ck_icn_p_risaf)
    #[inline(always)]
    pub fn rtcpre(&mut self) -> RTCPRE_W<'_, CCIPR7rs> {
        RTCPRE_W::new(self, 12)
    }
    ///Bits 20:22 - Source selection for the SAI1 kernel clock
    #[inline(always)]
    pub fn sai1sel(&mut self) -> SAI1SEL_W<'_, CCIPR7rs> {
        SAI1SEL_W::new(self, 20)
    }
    ///Bits 24:26 - Source selection for the SAI2 kernel clock
    #[inline(always)]
    pub fn sai2sel(&mut self) -> SAI2SEL_W<'_, CCIPR7rs> {
        SAI2SEL_W::new(self, 24)
    }
}
/**RCC clock configuration for independent peripheral register7

You can [`read`](crate::Reg::read) this register and get [`ccipr7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RCC:CCIPR7)*/
pub struct CCIPR7rs;
impl crate::RegisterSpec for CCIPR7rs {
    type Ux = u32;
}
///`read()` method returns [`ccipr7::R`](R) reader structure
impl crate::Readable for CCIPR7rs {}
///`write(|w| ..)` method takes [`ccipr7::W`](W) writer structure
impl crate::Writable for CCIPR7rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCIPR7 to value 0
impl crate::Resettable for CCIPR7rs {}

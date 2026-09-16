///Register `CCIPR1` reader
pub type R = crate::R<CCIPR1rs>;
///Register `CCIPR1` writer
pub type W = crate::W<CCIPR1rs>;
///Field `ADF1SEL` reader - Source selection for the ADF1 kernel clock
pub type ADF1SEL_R = crate::FieldReader;
///Field `ADF1SEL` writer - Source selection for the ADF1 kernel clock
pub type ADF1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `ADC12SEL` reader - Source selection for the ADC12 kernel clock
pub type ADC12SEL_R = crate::FieldReader;
///Field `ADC12SEL` writer - Source selection for the ADC12 kernel clock
pub type ADC12SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `ADCPRE` reader - ADC12 Prog clock divider selection (for clock ck_icn_p_adf1)
pub type ADCPRE_R = crate::FieldReader;
///Field `ADCPRE` writer - ADC12 Prog clock divider selection (for clock ck_icn_p_adf1)
pub type ADCPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DCMIPPSEL` reader - Source selection for the DCMIPP kernel clock
pub type DCMIPPSEL_R = crate::FieldReader;
///Field `DCMIPPSEL` writer - Source selection for the DCMIPP kernel clock
pub type DCMIPPSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:2 - Source selection for the ADF1 kernel clock
    #[inline(always)]
    pub fn adf1sel(&self) -> ADF1SEL_R {
        ADF1SEL_R::new((self.bits & 7) as u8)
    }
    ///Bits 4:6 - Source selection for the ADC12 kernel clock
    #[inline(always)]
    pub fn adc12sel(&self) -> ADC12SEL_R {
        ADC12SEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:15 - ADC12 Prog clock divider selection (for clock ck_icn_p_adf1)
    #[inline(always)]
    pub fn adcpre(&self) -> ADCPRE_R {
        ADCPRE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 20:21 - Source selection for the DCMIPP kernel clock
    #[inline(always)]
    pub fn dcmippsel(&self) -> DCMIPPSEL_R {
        DCMIPPSEL_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCIPR1")
            .field("adf1sel", &self.adf1sel())
            .field("adc12sel", &self.adc12sel())
            .field("adcpre", &self.adcpre())
            .field("dcmippsel", &self.dcmippsel())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Source selection for the ADF1 kernel clock
    #[inline(always)]
    pub fn adf1sel(&mut self) -> ADF1SEL_W<'_, CCIPR1rs> {
        ADF1SEL_W::new(self, 0)
    }
    ///Bits 4:6 - Source selection for the ADC12 kernel clock
    #[inline(always)]
    pub fn adc12sel(&mut self) -> ADC12SEL_W<'_, CCIPR1rs> {
        ADC12SEL_W::new(self, 4)
    }
    ///Bits 8:15 - ADC12 Prog clock divider selection (for clock ck_icn_p_adf1)
    #[inline(always)]
    pub fn adcpre(&mut self) -> ADCPRE_W<'_, CCIPR1rs> {
        ADCPRE_W::new(self, 8)
    }
    ///Bits 20:21 - Source selection for the DCMIPP kernel clock
    #[inline(always)]
    pub fn dcmippsel(&mut self) -> DCMIPPSEL_W<'_, CCIPR1rs> {
        DCMIPPSEL_W::new(self, 20)
    }
}
/**RCC clock configuration for independent peripheral register1

You can [`read`](crate::Reg::read) this register and get [`ccipr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RCC:CCIPR1)*/
pub struct CCIPR1rs;
impl crate::RegisterSpec for CCIPR1rs {
    type Ux = u32;
}
///`read()` method returns [`ccipr1::R`](R) reader structure
impl crate::Readable for CCIPR1rs {}
///`write(|w| ..)` method takes [`ccipr1::W`](W) writer structure
impl crate::Writable for CCIPR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCIPR1 to value 0
impl crate::Resettable for CCIPR1rs {}

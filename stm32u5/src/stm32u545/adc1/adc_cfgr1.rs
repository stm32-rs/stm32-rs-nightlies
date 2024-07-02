///Register `ADC_CFGR1` reader
pub type R = crate::R<ADC_CFGR1rs>;
///Register `ADC_CFGR1` writer
pub type W = crate::W<ADC_CFGR1rs>;
///Field `DMNGT` reader - DMNGT
pub type DMNGT_R = crate::FieldReader;
///Field `DMNGT` writer - DMNGT
pub type DMNGT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RES` reader - RES
pub type RES_R = crate::FieldReader;
///Field `RES` writer - RES
pub type RES_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `EXTSEL` reader - EXTSEL
pub type EXTSEL_R = crate::FieldReader;
///Field `EXTSEL` writer - EXTSEL
pub type EXTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `EXTEN` reader - EXTEN
pub type EXTEN_R = crate::FieldReader;
///Field `EXTEN` writer - EXTEN
pub type EXTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OVRMOD` reader - OVRMOD
pub type OVRMOD_R = crate::BitReader;
///Field `OVRMOD` writer - OVRMOD
pub type OVRMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CONT` reader - CONT
pub type CONT_R = crate::BitReader;
///Field `CONT` writer - CONT
pub type CONT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AUTDLY` reader - AUTDLY
pub type AUTDLY_R = crate::BitReader;
///Field `AUTDLY` writer - AUTDLY
pub type AUTDLY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DISCEN` reader - DISCEN
pub type DISCEN_R = crate::BitReader;
///Field `DISCEN` writer - DISCEN
pub type DISCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DISCNUM` reader - DISCNUM
pub type DISCNUM_R = crate::FieldReader;
///Field `DISCNUM` writer - DISCNUM
pub type DISCNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `JDISCEN` reader - JDISCEN
pub type JDISCEN_R = crate::BitReader;
///Field `JDISCEN` writer - JDISCEN
pub type JDISCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD1SGL` reader - AWD1SGL
pub type AWD1SGL_R = crate::BitReader;
///Field `AWD1SGL` writer - AWD1SGL
pub type AWD1SGL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD1EN` reader - AWD1EN
pub type AWD1EN_R = crate::BitReader;
///Field `AWD1EN` writer - AWD1EN
pub type AWD1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JAWD1EN` reader - JAWD1EN
pub type JAWD1EN_R = crate::BitReader;
///Field `JAWD1EN` writer - JAWD1EN
pub type JAWD1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JAUTO` reader - JAUTO
pub type JAUTO_R = crate::BitReader;
///Field `JAUTO` writer - JAUTO
pub type JAUTO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD1CH` reader - AWD1CH
pub type AWD1CH_R = crate::FieldReader;
///Field `AWD1CH` writer - AWD1CH
pub type AWD1CH_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:1 - DMNGT
    #[inline(always)]
    pub fn dmngt(&self) -> DMNGT_R {
        DMNGT_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - RES
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 5:9 - EXTSEL
    #[inline(always)]
    pub fn extsel(&self) -> EXTSEL_R {
        EXTSEL_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    ///Bits 10:11 - EXTEN
    #[inline(always)]
    pub fn exten(&self) -> EXTEN_R {
        EXTEN_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bit 12 - OVRMOD
    #[inline(always)]
    pub fn ovrmod(&self) -> OVRMOD_R {
        OVRMOD_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - CONT
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - AUTDLY
    #[inline(always)]
    pub fn autdly(&self) -> AUTDLY_R {
        AUTDLY_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - DISCEN
    #[inline(always)]
    pub fn discen(&self) -> DISCEN_R {
        DISCEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:19 - DISCNUM
    #[inline(always)]
    pub fn discnum(&self) -> DISCNUM_R {
        DISCNUM_R::new(((self.bits >> 17) & 7) as u8)
    }
    ///Bit 20 - JDISCEN
    #[inline(always)]
    pub fn jdiscen(&self) -> JDISCEN_R {
        JDISCEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 22 - AWD1SGL
    #[inline(always)]
    pub fn awd1sgl(&self) -> AWD1SGL_R {
        AWD1SGL_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - AWD1EN
    #[inline(always)]
    pub fn awd1en(&self) -> AWD1EN_R {
        AWD1EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - JAWD1EN
    #[inline(always)]
    pub fn jawd1en(&self) -> JAWD1EN_R {
        JAWD1EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - JAUTO
    #[inline(always)]
    pub fn jauto(&self) -> JAUTO_R {
        JAUTO_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bits 26:30 - AWD1CH
    #[inline(always)]
    pub fn awd1ch(&self) -> AWD1CH_R {
        AWD1CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_CFGR1")
            .field("awd1ch", &self.awd1ch())
            .field("jauto", &self.jauto())
            .field("jawd1en", &self.jawd1en())
            .field("awd1en", &self.awd1en())
            .field("awd1sgl", &self.awd1sgl())
            .field("jdiscen", &self.jdiscen())
            .field("discnum", &self.discnum())
            .field("discen", &self.discen())
            .field("autdly", &self.autdly())
            .field("cont", &self.cont())
            .field("ovrmod", &self.ovrmod())
            .field("exten", &self.exten())
            .field("extsel", &self.extsel())
            .field("res", &self.res())
            .field("dmngt", &self.dmngt())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - DMNGT
    #[inline(always)]
    #[must_use]
    pub fn dmngt(&mut self) -> DMNGT_W<ADC_CFGR1rs> {
        DMNGT_W::new(self, 0)
    }
    ///Bits 2:3 - RES
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<ADC_CFGR1rs> {
        RES_W::new(self, 2)
    }
    ///Bits 5:9 - EXTSEL
    #[inline(always)]
    #[must_use]
    pub fn extsel(&mut self) -> EXTSEL_W<ADC_CFGR1rs> {
        EXTSEL_W::new(self, 5)
    }
    ///Bits 10:11 - EXTEN
    #[inline(always)]
    #[must_use]
    pub fn exten(&mut self) -> EXTEN_W<ADC_CFGR1rs> {
        EXTEN_W::new(self, 10)
    }
    ///Bit 12 - OVRMOD
    #[inline(always)]
    #[must_use]
    pub fn ovrmod(&mut self) -> OVRMOD_W<ADC_CFGR1rs> {
        OVRMOD_W::new(self, 12)
    }
    ///Bit 13 - CONT
    #[inline(always)]
    #[must_use]
    pub fn cont(&mut self) -> CONT_W<ADC_CFGR1rs> {
        CONT_W::new(self, 13)
    }
    ///Bit 14 - AUTDLY
    #[inline(always)]
    #[must_use]
    pub fn autdly(&mut self) -> AUTDLY_W<ADC_CFGR1rs> {
        AUTDLY_W::new(self, 14)
    }
    ///Bit 16 - DISCEN
    #[inline(always)]
    #[must_use]
    pub fn discen(&mut self) -> DISCEN_W<ADC_CFGR1rs> {
        DISCEN_W::new(self, 16)
    }
    ///Bits 17:19 - DISCNUM
    #[inline(always)]
    #[must_use]
    pub fn discnum(&mut self) -> DISCNUM_W<ADC_CFGR1rs> {
        DISCNUM_W::new(self, 17)
    }
    ///Bit 20 - JDISCEN
    #[inline(always)]
    #[must_use]
    pub fn jdiscen(&mut self) -> JDISCEN_W<ADC_CFGR1rs> {
        JDISCEN_W::new(self, 20)
    }
    ///Bit 22 - AWD1SGL
    #[inline(always)]
    #[must_use]
    pub fn awd1sgl(&mut self) -> AWD1SGL_W<ADC_CFGR1rs> {
        AWD1SGL_W::new(self, 22)
    }
    ///Bit 23 - AWD1EN
    #[inline(always)]
    #[must_use]
    pub fn awd1en(&mut self) -> AWD1EN_W<ADC_CFGR1rs> {
        AWD1EN_W::new(self, 23)
    }
    ///Bit 24 - JAWD1EN
    #[inline(always)]
    #[must_use]
    pub fn jawd1en(&mut self) -> JAWD1EN_W<ADC_CFGR1rs> {
        JAWD1EN_W::new(self, 24)
    }
    ///Bit 25 - JAUTO
    #[inline(always)]
    #[must_use]
    pub fn jauto(&mut self) -> JAUTO_W<ADC_CFGR1rs> {
        JAUTO_W::new(self, 25)
    }
    ///Bits 26:30 - AWD1CH
    #[inline(always)]
    #[must_use]
    pub fn awd1ch(&mut self) -> AWD1CH_W<ADC_CFGR1rs> {
        AWD1CH_W::new(self, 26)
    }
}
/**ADC configuration register

You can [`read`](crate::Reg::read) this register and get [`adc_cfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_cfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#ADC1:ADC_CFGR1)*/
pub struct ADC_CFGR1rs;
impl crate::RegisterSpec for ADC_CFGR1rs {
    type Ux = u32;
}
///`read()` method returns [`adc_cfgr1::R`](R) reader structure
impl crate::Readable for ADC_CFGR1rs {}
///`write(|w| ..)` method takes [`adc_cfgr1::W`](W) writer structure
impl crate::Writable for ADC_CFGR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ADC_CFGR1 to value 0x8000_0000
impl crate::Resettable for ADC_CFGR1rs {
    const RESET_VALUE: u32 = 0x8000_0000;
}

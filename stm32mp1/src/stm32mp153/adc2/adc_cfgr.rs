#[doc = "Register `ADC_CFGR` reader"]
pub type R = crate::R<ADC_CFGRrs>;
#[doc = "Register `ADC_CFGR` writer"]
pub type W = crate::W<ADC_CFGRrs>;
#[doc = "Field `DMNGT` reader - DMNGT"]
pub type DMNGT_R = crate::FieldReader;
#[doc = "Field `DMNGT` writer - DMNGT"]
pub type DMNGT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RES` reader - RES"]
pub type RES_R = crate::FieldReader;
#[doc = "Field `RES` writer - RES"]
pub type RES_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `EXTSEL` reader - EXTSEL"]
pub type EXTSEL_R = crate::FieldReader;
#[doc = "Field `EXTSEL` writer - EXTSEL"]
pub type EXTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `EXTEN` reader - EXTEN"]
pub type EXTEN_R = crate::FieldReader;
#[doc = "Field `EXTEN` writer - EXTEN"]
pub type EXTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OVRMOD` reader - OVRMOD"]
pub type OVRMOD_R = crate::BitReader;
#[doc = "Field `OVRMOD` writer - OVRMOD"]
pub type OVRMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONT` reader - CONT"]
pub type CONT_R = crate::BitReader;
#[doc = "Field `CONT` writer - CONT"]
pub type CONT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTDLY` reader - AUTDLY"]
pub type AUTDLY_R = crate::BitReader;
#[doc = "Field `AUTDLY` writer - AUTDLY"]
pub type AUTDLY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISCEN` reader - DISCEN"]
pub type DISCEN_R = crate::BitReader;
#[doc = "Field `DISCEN` writer - DISCEN"]
pub type DISCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISCNUM` reader - DISCNUM"]
pub type DISCNUM_R = crate::FieldReader;
#[doc = "Field `DISCNUM` writer - DISCNUM"]
pub type DISCNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `JDISCEN` reader - JDISCEN"]
pub type JDISCEN_R = crate::BitReader;
#[doc = "Field `JDISCEN` writer - JDISCEN"]
pub type JDISCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JQM` reader - JQM"]
pub type JQM_R = crate::BitReader;
#[doc = "Field `JQM` writer - JQM"]
pub type JQM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD1SGL` reader - AWD1SGL"]
pub type AWD1SGL_R = crate::BitReader;
#[doc = "Field `AWD1SGL` writer - AWD1SGL"]
pub type AWD1SGL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD1EN` reader - AWD1EN"]
pub type AWD1EN_R = crate::BitReader;
#[doc = "Field `AWD1EN` writer - AWD1EN"]
pub type AWD1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JAWD1EN` reader - JAWD1EN"]
pub type JAWD1EN_R = crate::BitReader;
#[doc = "Field `JAWD1EN` writer - JAWD1EN"]
pub type JAWD1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JAUTO` reader - JAUTO"]
pub type JAUTO_R = crate::BitReader;
#[doc = "Field `JAUTO` writer - JAUTO"]
pub type JAUTO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD1CH` reader - AWD1CH"]
pub type AWD1CH_R = crate::FieldReader;
#[doc = "Field `AWD1CH` writer - AWD1CH"]
pub type AWD1CH_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `JQDIS` reader - JQDIS"]
pub type JQDIS_R = crate::BitReader;
#[doc = "Field `JQDIS` writer - JQDIS"]
pub type JQDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - DMNGT"]
    #[inline(always)]
    pub fn dmngt(&self) -> DMNGT_R {
        DMNGT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - RES"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:9 - EXTSEL"]
    #[inline(always)]
    pub fn extsel(&self) -> EXTSEL_R {
        EXTSEL_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:11 - EXTEN"]
    #[inline(always)]
    pub fn exten(&self) -> EXTEN_R {
        EXTEN_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - OVRMOD"]
    #[inline(always)]
    pub fn ovrmod(&self) -> OVRMOD_R {
        OVRMOD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CONT"]
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - AUTDLY"]
    #[inline(always)]
    pub fn autdly(&self) -> AUTDLY_R {
        AUTDLY_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - DISCEN"]
    #[inline(always)]
    pub fn discen(&self) -> DISCEN_R {
        DISCEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - DISCNUM"]
    #[inline(always)]
    pub fn discnum(&self) -> DISCNUM_R {
        DISCNUM_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 20 - JDISCEN"]
    #[inline(always)]
    pub fn jdiscen(&self) -> JDISCEN_R {
        JDISCEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - JQM"]
    #[inline(always)]
    pub fn jqm(&self) -> JQM_R {
        JQM_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - AWD1SGL"]
    #[inline(always)]
    pub fn awd1sgl(&self) -> AWD1SGL_R {
        AWD1SGL_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - AWD1EN"]
    #[inline(always)]
    pub fn awd1en(&self) -> AWD1EN_R {
        AWD1EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - JAWD1EN"]
    #[inline(always)]
    pub fn jawd1en(&self) -> JAWD1EN_R {
        JAWD1EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - JAUTO"]
    #[inline(always)]
    pub fn jauto(&self) -> JAUTO_R {
        JAUTO_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:30 - AWD1CH"]
    #[inline(always)]
    pub fn awd1ch(&self) -> AWD1CH_R {
        AWD1CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - JQDIS"]
    #[inline(always)]
    pub fn jqdis(&self) -> JQDIS_R {
        JQDIS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - DMNGT"]
    #[inline(always)]
    #[must_use]
    pub fn dmngt(&mut self) -> DMNGT_W<ADC_CFGRrs> {
        DMNGT_W::new(self, 0)
    }
    #[doc = "Bits 2:4 - RES"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<ADC_CFGRrs> {
        RES_W::new(self, 2)
    }
    #[doc = "Bits 5:9 - EXTSEL"]
    #[inline(always)]
    #[must_use]
    pub fn extsel(&mut self) -> EXTSEL_W<ADC_CFGRrs> {
        EXTSEL_W::new(self, 5)
    }
    #[doc = "Bits 10:11 - EXTEN"]
    #[inline(always)]
    #[must_use]
    pub fn exten(&mut self) -> EXTEN_W<ADC_CFGRrs> {
        EXTEN_W::new(self, 10)
    }
    #[doc = "Bit 12 - OVRMOD"]
    #[inline(always)]
    #[must_use]
    pub fn ovrmod(&mut self) -> OVRMOD_W<ADC_CFGRrs> {
        OVRMOD_W::new(self, 12)
    }
    #[doc = "Bit 13 - CONT"]
    #[inline(always)]
    #[must_use]
    pub fn cont(&mut self) -> CONT_W<ADC_CFGRrs> {
        CONT_W::new(self, 13)
    }
    #[doc = "Bit 14 - AUTDLY"]
    #[inline(always)]
    #[must_use]
    pub fn autdly(&mut self) -> AUTDLY_W<ADC_CFGRrs> {
        AUTDLY_W::new(self, 14)
    }
    #[doc = "Bit 16 - DISCEN"]
    #[inline(always)]
    #[must_use]
    pub fn discen(&mut self) -> DISCEN_W<ADC_CFGRrs> {
        DISCEN_W::new(self, 16)
    }
    #[doc = "Bits 17:19 - DISCNUM"]
    #[inline(always)]
    #[must_use]
    pub fn discnum(&mut self) -> DISCNUM_W<ADC_CFGRrs> {
        DISCNUM_W::new(self, 17)
    }
    #[doc = "Bit 20 - JDISCEN"]
    #[inline(always)]
    #[must_use]
    pub fn jdiscen(&mut self) -> JDISCEN_W<ADC_CFGRrs> {
        JDISCEN_W::new(self, 20)
    }
    #[doc = "Bit 21 - JQM"]
    #[inline(always)]
    #[must_use]
    pub fn jqm(&mut self) -> JQM_W<ADC_CFGRrs> {
        JQM_W::new(self, 21)
    }
    #[doc = "Bit 22 - AWD1SGL"]
    #[inline(always)]
    #[must_use]
    pub fn awd1sgl(&mut self) -> AWD1SGL_W<ADC_CFGRrs> {
        AWD1SGL_W::new(self, 22)
    }
    #[doc = "Bit 23 - AWD1EN"]
    #[inline(always)]
    #[must_use]
    pub fn awd1en(&mut self) -> AWD1EN_W<ADC_CFGRrs> {
        AWD1EN_W::new(self, 23)
    }
    #[doc = "Bit 24 - JAWD1EN"]
    #[inline(always)]
    #[must_use]
    pub fn jawd1en(&mut self) -> JAWD1EN_W<ADC_CFGRrs> {
        JAWD1EN_W::new(self, 24)
    }
    #[doc = "Bit 25 - JAUTO"]
    #[inline(always)]
    #[must_use]
    pub fn jauto(&mut self) -> JAUTO_W<ADC_CFGRrs> {
        JAUTO_W::new(self, 25)
    }
    #[doc = "Bits 26:30 - AWD1CH"]
    #[inline(always)]
    #[must_use]
    pub fn awd1ch(&mut self) -> AWD1CH_W<ADC_CFGRrs> {
        AWD1CH_W::new(self, 26)
    }
    #[doc = "Bit 31 - JQDIS"]
    #[inline(always)]
    #[must_use]
    pub fn jqdis(&mut self) -> JQDIS_W<ADC_CFGRrs> {
        JQDIS_W::new(self, 31)
    }
}
#[doc = "ADC configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC_CFGRrs;
impl crate::RegisterSpec for ADC_CFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_cfgr::R`](R) reader structure"]
impl crate::Readable for ADC_CFGRrs {}
#[doc = "`write(|w| ..)` method takes [`adc_cfgr::W`](W) writer structure"]
impl crate::Writable for ADC_CFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC_CFGR to value 0x8000_0000"]
impl crate::Resettable for ADC_CFGRrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}

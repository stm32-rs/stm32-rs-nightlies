///Register `CFGR` reader
pub type R = crate::R<CFGRrs>;
///Register `CFGR` writer
pub type W = crate::W<CFGRrs>;
///Field `DMNGT` reader - DMNGT
pub type DMNGT_R = crate::FieldReader;
///Field `DMNGT` writer - DMNGT
pub type DMNGT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RES` reader - RES
pub type RES_R = crate::FieldReader;
///Field `RES` writer - RES
pub type RES_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
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
///Field `JQM` reader - JQM
pub type JQM_R = crate::BitReader;
///Field `JQM` writer - JQM
pub type JQM_W<'a, REG> = crate::BitWriter<'a, REG>;
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
///Field `JQDIS` reader - JQDIS
pub type JQDIS_R = crate::BitReader;
///Field `JQDIS` writer - JQDIS
pub type JQDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - DMNGT
    #[inline(always)]
    pub fn dmngt(&self) -> DMNGT_R {
        DMNGT_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:4 - RES
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 2) & 7) as u8)
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
    ///Bit 21 - JQM
    #[inline(always)]
    pub fn jqm(&self) -> JQM_R {
        JQM_R::new(((self.bits >> 21) & 1) != 0)
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
    ///Bit 31 - JQDIS
    #[inline(always)]
    pub fn jqdis(&self) -> JQDIS_R {
        JQDIS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR")
            .field("dmngt", &self.dmngt())
            .field("res", &self.res())
            .field("extsel", &self.extsel())
            .field("exten", &self.exten())
            .field("ovrmod", &self.ovrmod())
            .field("cont", &self.cont())
            .field("autdly", &self.autdly())
            .field("discen", &self.discen())
            .field("discnum", &self.discnum())
            .field("jdiscen", &self.jdiscen())
            .field("jqm", &self.jqm())
            .field("awd1sgl", &self.awd1sgl())
            .field("awd1en", &self.awd1en())
            .field("jawd1en", &self.jawd1en())
            .field("jauto", &self.jauto())
            .field("awd1ch", &self.awd1ch())
            .field("jqdis", &self.jqdis())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - DMNGT
    #[inline(always)]
    pub fn dmngt(&mut self) -> DMNGT_W<'_, CFGRrs> {
        DMNGT_W::new(self, 0)
    }
    ///Bits 2:4 - RES
    #[inline(always)]
    pub fn res(&mut self) -> RES_W<'_, CFGRrs> {
        RES_W::new(self, 2)
    }
    ///Bits 5:9 - EXTSEL
    #[inline(always)]
    pub fn extsel(&mut self) -> EXTSEL_W<'_, CFGRrs> {
        EXTSEL_W::new(self, 5)
    }
    ///Bits 10:11 - EXTEN
    #[inline(always)]
    pub fn exten(&mut self) -> EXTEN_W<'_, CFGRrs> {
        EXTEN_W::new(self, 10)
    }
    ///Bit 12 - OVRMOD
    #[inline(always)]
    pub fn ovrmod(&mut self) -> OVRMOD_W<'_, CFGRrs> {
        OVRMOD_W::new(self, 12)
    }
    ///Bit 13 - CONT
    #[inline(always)]
    pub fn cont(&mut self) -> CONT_W<'_, CFGRrs> {
        CONT_W::new(self, 13)
    }
    ///Bit 14 - AUTDLY
    #[inline(always)]
    pub fn autdly(&mut self) -> AUTDLY_W<'_, CFGRrs> {
        AUTDLY_W::new(self, 14)
    }
    ///Bit 16 - DISCEN
    #[inline(always)]
    pub fn discen(&mut self) -> DISCEN_W<'_, CFGRrs> {
        DISCEN_W::new(self, 16)
    }
    ///Bits 17:19 - DISCNUM
    #[inline(always)]
    pub fn discnum(&mut self) -> DISCNUM_W<'_, CFGRrs> {
        DISCNUM_W::new(self, 17)
    }
    ///Bit 20 - JDISCEN
    #[inline(always)]
    pub fn jdiscen(&mut self) -> JDISCEN_W<'_, CFGRrs> {
        JDISCEN_W::new(self, 20)
    }
    ///Bit 21 - JQM
    #[inline(always)]
    pub fn jqm(&mut self) -> JQM_W<'_, CFGRrs> {
        JQM_W::new(self, 21)
    }
    ///Bit 22 - AWD1SGL
    #[inline(always)]
    pub fn awd1sgl(&mut self) -> AWD1SGL_W<'_, CFGRrs> {
        AWD1SGL_W::new(self, 22)
    }
    ///Bit 23 - AWD1EN
    #[inline(always)]
    pub fn awd1en(&mut self) -> AWD1EN_W<'_, CFGRrs> {
        AWD1EN_W::new(self, 23)
    }
    ///Bit 24 - JAWD1EN
    #[inline(always)]
    pub fn jawd1en(&mut self) -> JAWD1EN_W<'_, CFGRrs> {
        JAWD1EN_W::new(self, 24)
    }
    ///Bit 25 - JAUTO
    #[inline(always)]
    pub fn jauto(&mut self) -> JAUTO_W<'_, CFGRrs> {
        JAUTO_W::new(self, 25)
    }
    ///Bits 26:30 - AWD1CH
    #[inline(always)]
    pub fn awd1ch(&mut self) -> AWD1CH_W<'_, CFGRrs> {
        AWD1CH_W::new(self, 26)
    }
    ///Bit 31 - JQDIS
    #[inline(always)]
    pub fn jqdis(&mut self) -> JQDIS_W<'_, CFGRrs> {
        JQDIS_W::new(self, 31)
    }
}
/**ADC configuration register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ADC:CFGR)*/
pub struct CFGRrs;
impl crate::RegisterSpec for CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`cfgr::R`](R) reader structure
impl crate::Readable for CFGRrs {}
///`write(|w| ..)` method takes [`cfgr::W`](W) writer structure
impl crate::Writable for CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR to value 0x8000_0000
impl crate::Resettable for CFGRrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}

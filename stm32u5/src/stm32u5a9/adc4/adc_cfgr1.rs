///Register `ADC_CFGR1` reader
pub type R = crate::R<ADC_CFGR1rs>;
///Register `ADC_CFGR1` writer
pub type W = crate::W<ADC_CFGR1rs>;
///Field `DMAEN` reader - DMAEN
pub type DMAEN_R = crate::BitReader;
///Field `DMAEN` writer - DMAEN
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMACFG` reader - DMACFG
pub type DMACFG_R = crate::BitReader;
///Field `DMACFG` writer - DMACFG
pub type DMACFG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RES` reader - RES
pub type RES_R = crate::FieldReader;
///Field `RES` writer - RES
pub type RES_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SCANDIR` reader - SCANDIR
pub type SCANDIR_R = crate::BitReader;
///Field `SCANDIR` writer - SCANDIR
pub type SCANDIR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALIGN` reader - ALIGN
pub type ALIGN_R = crate::BitReader;
///Field `ALIGN` writer - ALIGN
pub type ALIGN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXTSEL` reader - EXTSEL
pub type EXTSEL_R = crate::FieldReader;
///Field `EXTSEL` writer - EXTSEL
pub type EXTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
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
///Field `WAIT` reader - WAIT
pub type WAIT_R = crate::BitReader;
///Field `WAIT` writer - WAIT
pub type WAIT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DISCEN` reader - DISCEN
pub type DISCEN_R = crate::BitReader;
///Field `DISCEN` writer - DISCEN
pub type DISCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHSELRMOD` reader - CHSELRMOD
pub type CHSELRMOD_R = crate::BitReader;
///Field `CHSELRMOD` writer - CHSELRMOD
pub type CHSELRMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD1SGL` reader - AWD1SGL
pub type AWD1SGL_R = crate::BitReader;
///Field `AWD1SGL` writer - AWD1SGL
pub type AWD1SGL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD1EN` reader - AWD1EN
pub type AWD1EN_R = crate::BitReader;
///Field `AWD1EN` writer - AWD1EN
pub type AWD1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD1CH` reader - AWD1CH
pub type AWD1CH_R = crate::FieldReader;
///Field `AWD1CH` writer - AWD1CH
pub type AWD1CH_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bit 0 - DMAEN
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMACFG
    #[inline(always)]
    pub fn dmacfg(&self) -> DMACFG_R {
        DMACFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - RES
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bit 4 - SCANDIR
    #[inline(always)]
    pub fn scandir(&self) -> SCANDIR_R {
        SCANDIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - ALIGN
    #[inline(always)]
    pub fn align(&self) -> ALIGN_R {
        ALIGN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:8 - EXTSEL
    #[inline(always)]
    pub fn extsel(&self) -> EXTSEL_R {
        EXTSEL_R::new(((self.bits >> 6) & 7) as u8)
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
    ///Bit 14 - WAIT
    #[inline(always)]
    pub fn wait(&self) -> WAIT_R {
        WAIT_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - DISCEN
    #[inline(always)]
    pub fn discen(&self) -> DISCEN_R {
        DISCEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 21 - CHSELRMOD
    #[inline(always)]
    pub fn chselrmod(&self) -> CHSELRMOD_R {
        CHSELRMOD_R::new(((self.bits >> 21) & 1) != 0)
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
            .field("awd1en", &self.awd1en())
            .field("awd1sgl", &self.awd1sgl())
            .field("chselrmod", &self.chselrmod())
            .field("discen", &self.discen())
            .field("wait", &self.wait())
            .field("cont", &self.cont())
            .field("ovrmod", &self.ovrmod())
            .field("exten", &self.exten())
            .field("extsel", &self.extsel())
            .field("align", &self.align())
            .field("scandir", &self.scandir())
            .field("res", &self.res())
            .field("dmacfg", &self.dmacfg())
            .field("dmaen", &self.dmaen())
            .finish()
    }
}
impl W {
    ///Bit 0 - DMAEN
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<ADC_CFGR1rs> {
        DMAEN_W::new(self, 0)
    }
    ///Bit 1 - DMACFG
    #[inline(always)]
    #[must_use]
    pub fn dmacfg(&mut self) -> DMACFG_W<ADC_CFGR1rs> {
        DMACFG_W::new(self, 1)
    }
    ///Bits 2:3 - RES
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<ADC_CFGR1rs> {
        RES_W::new(self, 2)
    }
    ///Bit 4 - SCANDIR
    #[inline(always)]
    #[must_use]
    pub fn scandir(&mut self) -> SCANDIR_W<ADC_CFGR1rs> {
        SCANDIR_W::new(self, 4)
    }
    ///Bit 5 - ALIGN
    #[inline(always)]
    #[must_use]
    pub fn align(&mut self) -> ALIGN_W<ADC_CFGR1rs> {
        ALIGN_W::new(self, 5)
    }
    ///Bits 6:8 - EXTSEL
    #[inline(always)]
    #[must_use]
    pub fn extsel(&mut self) -> EXTSEL_W<ADC_CFGR1rs> {
        EXTSEL_W::new(self, 6)
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
    ///Bit 14 - WAIT
    #[inline(always)]
    #[must_use]
    pub fn wait(&mut self) -> WAIT_W<ADC_CFGR1rs> {
        WAIT_W::new(self, 14)
    }
    ///Bit 16 - DISCEN
    #[inline(always)]
    #[must_use]
    pub fn discen(&mut self) -> DISCEN_W<ADC_CFGR1rs> {
        DISCEN_W::new(self, 16)
    }
    ///Bit 21 - CHSELRMOD
    #[inline(always)]
    #[must_use]
    pub fn chselrmod(&mut self) -> CHSELRMOD_W<ADC_CFGR1rs> {
        CHSELRMOD_W::new(self, 21)
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
    ///Bits 26:30 - AWD1CH
    #[inline(always)]
    #[must_use]
    pub fn awd1ch(&mut self) -> AWD1CH_W<ADC_CFGR1rs> {
        AWD1CH_W::new(self, 26)
    }
}
/**ADC configuration register

You can [`read`](crate::Reg::read) this register and get [`adc_cfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_cfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#ADC4:ADC_CFGR1)*/
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
///`reset()` method sets ADC_CFGR1 to value 0
impl crate::Resettable for ADC_CFGR1rs {
    const RESET_VALUE: u32 = 0;
}

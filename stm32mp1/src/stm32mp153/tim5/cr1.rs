///Register `CR1` reader
pub type R = crate::R<CR1rs>;
///Register `CR1` writer
pub type W = crate::W<CR1rs>;
///Field `CEN` reader - CEN
pub type CEN_R = crate::BitReader;
///Field `CEN` writer - CEN
pub type CEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UDIS` reader - UDIS
pub type UDIS_R = crate::BitReader;
///Field `UDIS` writer - UDIS
pub type UDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `URS` reader - URS
pub type URS_R = crate::BitReader;
///Field `URS` writer - URS
pub type URS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPM` reader - OPM
pub type OPM_R = crate::BitReader;
///Field `OPM` writer - OPM
pub type OPM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIR` reader - DIR
pub type DIR_R = crate::BitReader;
///Field `DIR` writer - DIR
pub type DIR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMS` reader - CMS
pub type CMS_R = crate::FieldReader;
///Field `CMS` writer - CMS
pub type CMS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ARPE` reader - ARPE
pub type ARPE_R = crate::BitReader;
///Field `ARPE` writer - ARPE
pub type ARPE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CKD` reader - CKD
pub type CKD_R = crate::FieldReader;
///Field `CKD` writer - CKD
pub type CKD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `UIFREMAP` reader - UIFREMAP
pub type UIFREMAP_R = crate::BitReader;
///Field `UIFREMAP` writer - UIFREMAP
pub type UIFREMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - CEN
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - UDIS
    #[inline(always)]
    pub fn udis(&self) -> UDIS_R {
        UDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - URS
    #[inline(always)]
    pub fn urs(&self) -> URS_R {
        URS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - OPM
    #[inline(always)]
    pub fn opm(&self) -> OPM_R {
        OPM_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - DIR
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:6 - CMS
    #[inline(always)]
    pub fn cms(&self) -> CMS_R {
        CMS_R::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bit 7 - ARPE
    #[inline(always)]
    pub fn arpe(&self) -> ARPE_R {
        ARPE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - CKD
    #[inline(always)]
    pub fn ckd(&self) -> CKD_R {
        CKD_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 11 - UIFREMAP
    #[inline(always)]
    pub fn uifremap(&self) -> UIFREMAP_R {
        UIFREMAP_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR1")
            .field("cen", &self.cen())
            .field("udis", &self.udis())
            .field("urs", &self.urs())
            .field("opm", &self.opm())
            .field("dir", &self.dir())
            .field("cms", &self.cms())
            .field("arpe", &self.arpe())
            .field("ckd", &self.ckd())
            .field("uifremap", &self.uifremap())
            .finish()
    }
}
impl W {
    ///Bit 0 - CEN
    #[inline(always)]
    pub fn cen(&mut self) -> CEN_W<'_, CR1rs> {
        CEN_W::new(self, 0)
    }
    ///Bit 1 - UDIS
    #[inline(always)]
    pub fn udis(&mut self) -> UDIS_W<'_, CR1rs> {
        UDIS_W::new(self, 1)
    }
    ///Bit 2 - URS
    #[inline(always)]
    pub fn urs(&mut self) -> URS_W<'_, CR1rs> {
        URS_W::new(self, 2)
    }
    ///Bit 3 - OPM
    #[inline(always)]
    pub fn opm(&mut self) -> OPM_W<'_, CR1rs> {
        OPM_W::new(self, 3)
    }
    ///Bit 4 - DIR
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W<'_, CR1rs> {
        DIR_W::new(self, 4)
    }
    ///Bits 5:6 - CMS
    #[inline(always)]
    pub fn cms(&mut self) -> CMS_W<'_, CR1rs> {
        CMS_W::new(self, 5)
    }
    ///Bit 7 - ARPE
    #[inline(always)]
    pub fn arpe(&mut self) -> ARPE_W<'_, CR1rs> {
        ARPE_W::new(self, 7)
    }
    ///Bits 8:9 - CKD
    #[inline(always)]
    pub fn ckd(&mut self) -> CKD_W<'_, CR1rs> {
        CKD_W::new(self, 8)
    }
    ///Bit 11 - UIFREMAP
    #[inline(always)]
    pub fn uifremap(&mut self) -> UIFREMAP_W<'_, CR1rs> {
        UIFREMAP_W::new(self, 11)
    }
}
/**TIM5 control register 1

You can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM5:CR1)*/
pub struct CR1rs;
impl crate::RegisterSpec for CR1rs {
    type Ux = u16;
}
///`read()` method returns [`cr1::R`](R) reader structure
impl crate::Readable for CR1rs {}
///`write(|w| ..)` method takes [`cr1::W`](W) writer structure
impl crate::Writable for CR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR1 to value 0
impl crate::Resettable for CR1rs {}

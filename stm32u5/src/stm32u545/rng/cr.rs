///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `RNGEN` reader - True random number generator enable
pub type RNGEN_R = crate::BitReader;
///Field `RNGEN` writer - True random number generator enable
pub type RNGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IE` reader - Interrupt Enable
pub type IE_R = crate::BitReader;
///Field `IE` writer - Interrupt Enable
pub type IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CED` reader - Clock error detection
pub type CED_R = crate::BitReader;
///Field `CED` writer - Clock error detection
pub type CED_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ARDIS` reader - Auto reset disable
pub type ARDIS_R = crate::BitReader;
///Field `ARDIS` writer - Auto reset disable
pub type ARDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RNG_CONFIG3` reader - RNG configuration 3
pub type RNG_CONFIG3_R = crate::FieldReader;
///Field `RNG_CONFIG3` writer - RNG configuration 3
pub type RNG_CONFIG3_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `NISTC` reader - Non NIST compliant
pub type NISTC_R = crate::BitReader;
///Field `NISTC` writer - Non NIST compliant
pub type NISTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RNG_CONFIG2` reader - RNG configuration 2
pub type RNG_CONFIG2_R = crate::FieldReader;
///Field `RNG_CONFIG2` writer - RNG configuration 2
pub type RNG_CONFIG2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `CLKDIV` reader - Clock divider factor
pub type CLKDIV_R = crate::FieldReader;
///Field `CLKDIV` writer - Clock divider factor
pub type CLKDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `RNG_CONFIG1` reader - RNG configuration 1
pub type RNG_CONFIG1_R = crate::FieldReader;
///Field `RNG_CONFIG1` writer - RNG configuration 1
pub type RNG_CONFIG1_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `CONDRST` reader - Conditioning soft reset
pub type CONDRST_R = crate::BitReader;
///Field `CONDRST` writer - Conditioning soft reset
pub type CONDRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CONFIGLOCK` reader - RNG Config Lock
pub type CONFIGLOCK_R = crate::BitReader;
///Field `CONFIGLOCK` writer - RNG Config Lock
pub type CONFIGLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 2 - True random number generator enable
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Interrupt Enable
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - Clock error detection
    #[inline(always)]
    pub fn ced(&self) -> CED_R {
        CED_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - Auto reset disable
    #[inline(always)]
    pub fn ardis(&self) -> ARDIS_R {
        ARDIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:11 - RNG configuration 3
    #[inline(always)]
    pub fn rng_config3(&self) -> RNG_CONFIG3_R {
        RNG_CONFIG3_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 12 - Non NIST compliant
    #[inline(always)]
    pub fn nistc(&self) -> NISTC_R {
        NISTC_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:15 - RNG configuration 2
    #[inline(always)]
    pub fn rng_config2(&self) -> RNG_CONFIG2_R {
        RNG_CONFIG2_R::new(((self.bits >> 13) & 7) as u8)
    }
    ///Bits 16:19 - Clock divider factor
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:25 - RNG configuration 1
    #[inline(always)]
    pub fn rng_config1(&self) -> RNG_CONFIG1_R {
        RNG_CONFIG1_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
    ///Bit 30 - Conditioning soft reset
    #[inline(always)]
    pub fn condrst(&self) -> CONDRST_R {
        CONDRST_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - RNG Config Lock
    #[inline(always)]
    pub fn configlock(&self) -> CONFIGLOCK_R {
        CONFIGLOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("configlock", &self.configlock())
            .field("condrst", &self.condrst())
            .field("rng_config1", &self.rng_config1())
            .field("clkdiv", &self.clkdiv())
            .field("rng_config2", &self.rng_config2())
            .field("nistc", &self.nistc())
            .field("rng_config3", &self.rng_config3())
            .field("ardis", &self.ardis())
            .field("ced", &self.ced())
            .field("ie", &self.ie())
            .field("rngen", &self.rngen())
            .finish()
    }
}
impl W {
    ///Bit 2 - True random number generator enable
    #[inline(always)]
    pub fn rngen(&mut self) -> RNGEN_W<CRrs> {
        RNGEN_W::new(self, 2)
    }
    ///Bit 3 - Interrupt Enable
    #[inline(always)]
    pub fn ie(&mut self) -> IE_W<CRrs> {
        IE_W::new(self, 3)
    }
    ///Bit 5 - Clock error detection
    #[inline(always)]
    pub fn ced(&mut self) -> CED_W<CRrs> {
        CED_W::new(self, 5)
    }
    ///Bit 7 - Auto reset disable
    #[inline(always)]
    pub fn ardis(&mut self) -> ARDIS_W<CRrs> {
        ARDIS_W::new(self, 7)
    }
    ///Bits 8:11 - RNG configuration 3
    #[inline(always)]
    pub fn rng_config3(&mut self) -> RNG_CONFIG3_W<CRrs> {
        RNG_CONFIG3_W::new(self, 8)
    }
    ///Bit 12 - Non NIST compliant
    #[inline(always)]
    pub fn nistc(&mut self) -> NISTC_W<CRrs> {
        NISTC_W::new(self, 12)
    }
    ///Bits 13:15 - RNG configuration 2
    #[inline(always)]
    pub fn rng_config2(&mut self) -> RNG_CONFIG2_W<CRrs> {
        RNG_CONFIG2_W::new(self, 13)
    }
    ///Bits 16:19 - Clock divider factor
    #[inline(always)]
    pub fn clkdiv(&mut self) -> CLKDIV_W<CRrs> {
        CLKDIV_W::new(self, 16)
    }
    ///Bits 20:25 - RNG configuration 1
    #[inline(always)]
    pub fn rng_config1(&mut self) -> RNG_CONFIG1_W<CRrs> {
        RNG_CONFIG1_W::new(self, 20)
    }
    ///Bit 30 - Conditioning soft reset
    #[inline(always)]
    pub fn condrst(&mut self) -> CONDRST_W<CRrs> {
        CONDRST_W::new(self, 30)
    }
    ///Bit 31 - RNG Config Lock
    #[inline(always)]
    pub fn configlock(&mut self) -> CONFIGLOCK_W<CRrs> {
        CONFIGLOCK_W::new(self, 31)
    }
}
/**control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#RNG:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}

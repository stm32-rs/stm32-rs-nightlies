///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `LPDS` reader - Low-power deep sleep
pub type LPDS_R = crate::BitReader;
///Field `LPDS` writer - Low-power deep sleep
pub type LPDS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDDS` reader - Power down deepsleep
pub type PDDS_R = crate::BitReader;
///Field `PDDS` writer - Power down deepsleep
pub type PDDS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CWUF` reader - Clear wakeup flag
pub type CWUF_R = crate::BitReader;
///Field `CWUF` writer - Clear wakeup flag
pub type CWUF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSBF` reader - Clear standby flag
pub type CSBF_R = crate::BitReader;
///Field `CSBF` writer - Clear standby flag
pub type CSBF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PVDE` reader - Power voltage detector enable
pub type PVDE_R = crate::BitReader;
///Field `PVDE` writer - Power voltage detector enable
pub type PVDE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLS` reader - PVD level selection
pub type PLS_R = crate::FieldReader;
///Field `PLS` writer - PVD level selection
pub type PLS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DBP` reader - Disable backup domain write protection
pub type DBP_R = crate::BitReader;
///Field `DBP` writer - Disable backup domain write protection
pub type DBP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ENSD1` reader - ENable SD1 ADC
pub type ENSD1_R = crate::BitReader;
///Field `ENSD1` writer - ENable SD1 ADC
pub type ENSD1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ENSD2` reader - ENable SD2 ADC
pub type ENSD2_R = crate::BitReader;
///Field `ENSD2` writer - ENable SD2 ADC
pub type ENSD2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ENSD3` reader - ENable SD3 ADC
pub type ENSD3_R = crate::BitReader;
///Field `ENSD3` writer - ENable SD3 ADC
pub type ENSD3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Low-power deep sleep
    #[inline(always)]
    pub fn lpds(&self) -> LPDS_R {
        LPDS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Power down deepsleep
    #[inline(always)]
    pub fn pdds(&self) -> PDDS_R {
        PDDS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Clear wakeup flag
    #[inline(always)]
    pub fn cwuf(&self) -> CWUF_R {
        CWUF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Clear standby flag
    #[inline(always)]
    pub fn csbf(&self) -> CSBF_R {
        CSBF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Power voltage detector enable
    #[inline(always)]
    pub fn pvde(&self) -> PVDE_R {
        PVDE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:7 - PVD level selection
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 5) & 7) as u8)
    }
    ///Bit 8 - Disable backup domain write protection
    #[inline(always)]
    pub fn dbp(&self) -> DBP_R {
        DBP_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - ENable SD1 ADC
    #[inline(always)]
    pub fn ensd1(&self) -> ENSD1_R {
        ENSD1_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - ENable SD2 ADC
    #[inline(always)]
    pub fn ensd2(&self) -> ENSD2_R {
        ENSD2_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - ENable SD3 ADC
    #[inline(always)]
    pub fn ensd3(&self) -> ENSD3_R {
        ENSD3_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("lpds", &self.lpds())
            .field("pdds", &self.pdds())
            .field("cwuf", &self.cwuf())
            .field("csbf", &self.csbf())
            .field("pvde", &self.pvde())
            .field("pls", &self.pls())
            .field("dbp", &self.dbp())
            .field("ensd1", &self.ensd1())
            .field("ensd2", &self.ensd2())
            .field("ensd3", &self.ensd3())
            .finish()
    }
}
impl W {
    ///Bit 0 - Low-power deep sleep
    #[inline(always)]
    pub fn lpds(&mut self) -> LPDS_W<'_, CRrs> {
        LPDS_W::new(self, 0)
    }
    ///Bit 1 - Power down deepsleep
    #[inline(always)]
    pub fn pdds(&mut self) -> PDDS_W<'_, CRrs> {
        PDDS_W::new(self, 1)
    }
    ///Bit 2 - Clear wakeup flag
    #[inline(always)]
    pub fn cwuf(&mut self) -> CWUF_W<'_, CRrs> {
        CWUF_W::new(self, 2)
    }
    ///Bit 3 - Clear standby flag
    #[inline(always)]
    pub fn csbf(&mut self) -> CSBF_W<'_, CRrs> {
        CSBF_W::new(self, 3)
    }
    ///Bit 4 - Power voltage detector enable
    #[inline(always)]
    pub fn pvde(&mut self) -> PVDE_W<'_, CRrs> {
        PVDE_W::new(self, 4)
    }
    ///Bits 5:7 - PVD level selection
    #[inline(always)]
    pub fn pls(&mut self) -> PLS_W<'_, CRrs> {
        PLS_W::new(self, 5)
    }
    ///Bit 8 - Disable backup domain write protection
    #[inline(always)]
    pub fn dbp(&mut self) -> DBP_W<'_, CRrs> {
        DBP_W::new(self, 8)
    }
    ///Bit 9 - ENable SD1 ADC
    #[inline(always)]
    pub fn ensd1(&mut self) -> ENSD1_W<'_, CRrs> {
        ENSD1_W::new(self, 9)
    }
    ///Bit 10 - ENable SD2 ADC
    #[inline(always)]
    pub fn ensd2(&mut self) -> ENSD2_W<'_, CRrs> {
        ENSD2_W::new(self, 10)
    }
    ///Bit 11 - ENable SD3 ADC
    #[inline(always)]
    pub fn ensd3(&mut self) -> ENSD3_W<'_, CRrs> {
        ENSD3_W::new(self, 11)
    }
}
/**power control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F301.html#PWR:CR)*/
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

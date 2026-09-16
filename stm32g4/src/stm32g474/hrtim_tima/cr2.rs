///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
///Field `DCDE` reader - Dual Channel DAC trigger enable
pub type DCDE_R = crate::BitReader;
///Field `DCDE` writer - Dual Channel DAC trigger enable
pub type DCDE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DCDS` reader - Dual Channel DAC Step trigger
pub type DCDS_R = crate::BitReader;
///Field `DCDS` writer - Dual Channel DAC Step trigger
pub type DCDS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DCDR` reader - Dual Channel DAC Reset trigger
pub type DCDR_R = crate::BitReader;
///Field `DCDR` writer - Dual Channel DAC Reset trigger
pub type DCDR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UDM` reader - Up-Down Mode
pub type UDM_R = crate::BitReader;
///Field `UDM` writer - Up-Down Mode
pub type UDM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ROM` reader - Roll-Over Mode
pub type ROM_R = crate::FieldReader;
///Field `ROM` writer - Roll-Over Mode
pub type ROM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OUTROM` reader - Output Roll-Over Mode
pub type OUTROM_R = crate::FieldReader;
///Field `OUTROM` writer - Output Roll-Over Mode
pub type OUTROM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ADROM` reader - ADC Roll-Over Mode
pub type ADROM_R = crate::FieldReader;
///Field `ADROM` writer - ADC Roll-Over Mode
pub type ADROM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `BMROM` reader - Burst Mode Roll-Over Mode
pub type BMROM_R = crate::FieldReader;
///Field `BMROM` writer - Burst Mode Roll-Over Mode
pub type BMROM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `FEROM` reader - Fault and Event Roll-Over Mode
pub type FEROM_R = crate::FieldReader;
///Field `FEROM` writer - Fault and Event Roll-Over Mode
pub type FEROM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `GTCMP1` reader - Greater than Compare 1 PWM mode
pub type GTCMP1_R = crate::BitReader;
///Field `GTCMP1` writer - Greater than Compare 1 PWM mode
pub type GTCMP1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GTCMP3` reader - Greater than Compare 3 PWM mode
pub type GTCMP3_R = crate::BitReader;
///Field `GTCMP3` writer - Greater than Compare 3 PWM mode
pub type GTCMP3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRGHLF` reader - Triggered-half mode
pub type TRGHLF_R = crate::BitReader;
///Field `TRGHLF` writer - Triggered-half mode
pub type TRGHLF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Dual Channel DAC trigger enable
    #[inline(always)]
    pub fn dcde(&self) -> DCDE_R {
        DCDE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Dual Channel DAC Step trigger
    #[inline(always)]
    pub fn dcds(&self) -> DCDS_R {
        DCDS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Dual Channel DAC Reset trigger
    #[inline(always)]
    pub fn dcdr(&self) -> DCDR_R {
        DCDR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - Up-Down Mode
    #[inline(always)]
    pub fn udm(&self) -> UDM_R {
        UDM_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 6:7 - Roll-Over Mode
    #[inline(always)]
    pub fn rom(&self) -> ROM_R {
        ROM_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - Output Roll-Over Mode
    #[inline(always)]
    pub fn outrom(&self) -> OUTROM_R {
        OUTROM_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - ADC Roll-Over Mode
    #[inline(always)]
    pub fn adrom(&self) -> ADROM_R {
        ADROM_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - Burst Mode Roll-Over Mode
    #[inline(always)]
    pub fn bmrom(&self) -> BMROM_R {
        BMROM_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - Fault and Event Roll-Over Mode
    #[inline(always)]
    pub fn ferom(&self) -> FEROM_R {
        FEROM_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bit 16 - Greater than Compare 1 PWM mode
    #[inline(always)]
    pub fn gtcmp1(&self) -> GTCMP1_R {
        GTCMP1_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Greater than Compare 3 PWM mode
    #[inline(always)]
    pub fn gtcmp3(&self) -> GTCMP3_R {
        GTCMP3_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 20 - Triggered-half mode
    #[inline(always)]
    pub fn trghlf(&self) -> TRGHLF_R {
        TRGHLF_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("trghlf", &self.trghlf())
            .field("gtcmp3", &self.gtcmp3())
            .field("gtcmp1", &self.gtcmp1())
            .field("ferom", &self.ferom())
            .field("bmrom", &self.bmrom())
            .field("adrom", &self.adrom())
            .field("outrom", &self.outrom())
            .field("rom", &self.rom())
            .field("udm", &self.udm())
            .field("dcdr", &self.dcdr())
            .field("dcds", &self.dcds())
            .field("dcde", &self.dcde())
            .finish()
    }
}
impl W {
    ///Bit 0 - Dual Channel DAC trigger enable
    #[inline(always)]
    pub fn dcde(&mut self) -> DCDE_W<'_, CR2rs> {
        DCDE_W::new(self, 0)
    }
    ///Bit 1 - Dual Channel DAC Step trigger
    #[inline(always)]
    pub fn dcds(&mut self) -> DCDS_W<'_, CR2rs> {
        DCDS_W::new(self, 1)
    }
    ///Bit 2 - Dual Channel DAC Reset trigger
    #[inline(always)]
    pub fn dcdr(&mut self) -> DCDR_W<'_, CR2rs> {
        DCDR_W::new(self, 2)
    }
    ///Bit 4 - Up-Down Mode
    #[inline(always)]
    pub fn udm(&mut self) -> UDM_W<'_, CR2rs> {
        UDM_W::new(self, 4)
    }
    ///Bits 6:7 - Roll-Over Mode
    #[inline(always)]
    pub fn rom(&mut self) -> ROM_W<'_, CR2rs> {
        ROM_W::new(self, 6)
    }
    ///Bits 8:9 - Output Roll-Over Mode
    #[inline(always)]
    pub fn outrom(&mut self) -> OUTROM_W<'_, CR2rs> {
        OUTROM_W::new(self, 8)
    }
    ///Bits 10:11 - ADC Roll-Over Mode
    #[inline(always)]
    pub fn adrom(&mut self) -> ADROM_W<'_, CR2rs> {
        ADROM_W::new(self, 10)
    }
    ///Bits 12:13 - Burst Mode Roll-Over Mode
    #[inline(always)]
    pub fn bmrom(&mut self) -> BMROM_W<'_, CR2rs> {
        BMROM_W::new(self, 12)
    }
    ///Bits 14:15 - Fault and Event Roll-Over Mode
    #[inline(always)]
    pub fn ferom(&mut self) -> FEROM_W<'_, CR2rs> {
        FEROM_W::new(self, 14)
    }
    ///Bit 16 - Greater than Compare 1 PWM mode
    #[inline(always)]
    pub fn gtcmp1(&mut self) -> GTCMP1_W<'_, CR2rs> {
        GTCMP1_W::new(self, 16)
    }
    ///Bit 17 - Greater than Compare 3 PWM mode
    #[inline(always)]
    pub fn gtcmp3(&mut self) -> GTCMP3_W<'_, CR2rs> {
        GTCMP3_W::new(self, 17)
    }
    ///Bit 20 - Triggered-half mode
    #[inline(always)]
    pub fn trghlf(&mut self) -> TRGHLF_W<'_, CR2rs> {
        TRGHLF_W::new(self, 20)
    }
}
/**HRTIM Timerx Control Register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#HRTIM_TIMA:CR2)*/
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
///`read()` method returns [`cr2::R`](R) reader structure
impl crate::Readable for CR2rs {}
///`write(|w| ..)` method takes [`cr2::W`](W) writer structure
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2rs {}

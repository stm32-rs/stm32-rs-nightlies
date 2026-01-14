///Register `ENGTRIM2` reader
pub type R = crate::R<ENGTRIM2rs>;
///Register `ENGTRIM2` writer
pub type W = crate::W<ENGTRIM2rs>;
///Field `BOFTRIMEN` reader - BOFTRIMEN: trimming BOF enabled - 1: trimming bit applied from ENGTRIM2 register - 0: trimming bit applied from OBL (can be read on TRIMR register)
pub type BOFTRIMEN_R = crate::BitReader;
///Field `BOFTRIMEN` writer - BOFTRIMEN: trimming BOF enabled - 1: trimming bit applied from ENGTRIM2 register - 0: trimming bit applied from OBL (can be read on TRIMR register)
pub type BOFTRIMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BOF_TRIM` reader - SMPS_TRIM: SMPS Output Voltage Trimming By default, this value is not applied, but taken from the engi bytes; if ENGTRIM.BOFTRIMEN=1, the SMPS output voltage can be controlled by this register.
pub type BOF_TRIM_R = crate::FieldReader;
///Field `BOF_TRIM` writer - SMPS_TRIM: SMPS Output Voltage Trimming By default, this value is not applied, but taken from the engi bytes; if ENGTRIM.BOFTRIMEN=1, the SMPS output voltage can be controlled by this register.
pub type BOF_TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bit 0 - BOFTRIMEN: trimming BOF enabled - 1: trimming bit applied from ENGTRIM2 register - 0: trimming bit applied from OBL (can be read on TRIMR register)
    #[inline(always)]
    pub fn boftrimen(&self) -> BOFTRIMEN_R {
        BOFTRIMEN_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:3 - SMPS_TRIM: SMPS Output Voltage Trimming By default, this value is not applied, but taken from the engi bytes; if ENGTRIM.BOFTRIMEN=1, the SMPS output voltage can be controlled by this register.
    #[inline(always)]
    pub fn bof_trim(&self) -> BOF_TRIM_R {
        BOF_TRIM_R::new(((self.bits >> 1) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENGTRIM2")
            .field("boftrimen", &self.boftrimen())
            .field("bof_trim", &self.bof_trim())
            .finish()
    }
}
impl W {
    ///Bit 0 - BOFTRIMEN: trimming BOF enabled - 1: trimming bit applied from ENGTRIM2 register - 0: trimming bit applied from OBL (can be read on TRIMR register)
    #[inline(always)]
    pub fn boftrimen(&mut self) -> BOFTRIMEN_W<'_, ENGTRIM2rs> {
        BOFTRIMEN_W::new(self, 0)
    }
    ///Bits 1:3 - SMPS_TRIM: SMPS Output Voltage Trimming By default, this value is not applied, but taken from the engi bytes; if ENGTRIM.BOFTRIMEN=1, the SMPS output voltage can be controlled by this register.
    #[inline(always)]
    pub fn bof_trim(&mut self) -> BOF_TRIM_W<'_, ENGTRIM2rs> {
        BOF_TRIM_W::new(self, 1)
    }
}
/**ENGTRIM2 register

You can [`read`](crate::Reg::read) this register and get [`engtrim2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`engtrim2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#PWRC:ENGTRIM2)*/
pub struct ENGTRIM2rs;
impl crate::RegisterSpec for ENGTRIM2rs {
    type Ux = u32;
}
///`read()` method returns [`engtrim2::R`](R) reader structure
impl crate::Readable for ENGTRIM2rs {}
///`write(|w| ..)` method takes [`engtrim2::W`](W) writer structure
impl crate::Writable for ENGTRIM2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ENGTRIM2 to value 0
impl crate::Resettable for ENGTRIM2rs {}

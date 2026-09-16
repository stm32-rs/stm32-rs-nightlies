///Register `CM55TCMCR` reader
pub type R = crate::R<CM55TCMCRrs>;
///Register `CM55TCMCR` writer
pub type W = crate::W<CM55TCMCRrs>;
///Field `CFGITCMSZ` reader - Select ITCM memory size
pub type CFGITCMSZ_R = crate::FieldReader;
///Field `CFGITCMSZ` writer - Select ITCM memory size
pub type CFGITCMSZ_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `CFGDTCMSZ` reader - Select DTCM memory size
pub type CFGDTCMSZ_R = crate::FieldReader;
///Field `CFGDTCMSZ` writer - Select DTCM memory size
pub type CFGDTCMSZ_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `LOCKTCM` reader - Disable writes to registers associated with the TCM region
pub type LOCKTCM_R = crate::BitReader;
///Field `LOCKTCM` writer - Disable writes to registers associated with the TCM region
pub type LOCKTCM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LOCKITGU` reader - Disable writes to registers associated with the ITCM interface security gating.
pub type LOCKITGU_R = crate::BitReader;
///Field `LOCKITGU` writer - Disable writes to registers associated with the ITCM interface security gating.
pub type LOCKITGU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LOCKDTGU` reader - Disable writes to registers associated with the DTCM interface security gating.
pub type LOCKDTGU_R = crate::BitReader;
///Field `LOCKDTGU` writer - Disable writes to registers associated with the DTCM interface security gating.
pub type LOCKDTGU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITCMWSDISABLE` reader - Disable wait-state applied by default on extended ITCM memory.
pub type ITCMWSDISABLE_R = crate::BitReader;
///Field `ITCMWSDISABLE` writer - Disable wait-state applied by default on extended ITCM memory.
pub type ITCMWSDISABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTCMWSDISABLE` reader - Disable wait-state applied by default on extended DTCM memory.
pub type DTCMWSDISABLE_R = crate::BitReader;
///Field `DTCMWSDISABLE` writer - Disable wait-state applied by default on extended DTCM memory.
pub type DTCMWSDISABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:3 - Select ITCM memory size
    #[inline(always)]
    pub fn cfgitcmsz(&self) -> CFGITCMSZ_R {
        CFGITCMSZ_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Select DTCM memory size
    #[inline(always)]
    pub fn cfgdtcmsz(&self) -> CFGDTCMSZ_R {
        CFGDTCMSZ_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bit 16 - Disable writes to registers associated with the TCM region
    #[inline(always)]
    pub fn locktcm(&self) -> LOCKTCM_R {
        LOCKTCM_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Disable writes to registers associated with the ITCM interface security gating.
    #[inline(always)]
    pub fn lockitgu(&self) -> LOCKITGU_R {
        LOCKITGU_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Disable writes to registers associated with the DTCM interface security gating.
    #[inline(always)]
    pub fn lockdtgu(&self) -> LOCKDTGU_R {
        LOCKDTGU_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 23 - Disable wait-state applied by default on extended ITCM memory.
    #[inline(always)]
    pub fn itcmwsdisable(&self) -> ITCMWSDISABLE_R {
        ITCMWSDISABLE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Disable wait-state applied by default on extended DTCM memory.
    #[inline(always)]
    pub fn dtcmwsdisable(&self) -> DTCMWSDISABLE_R {
        DTCMWSDISABLE_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CM55TCMCR")
            .field("cfgitcmsz", &self.cfgitcmsz())
            .field("cfgdtcmsz", &self.cfgdtcmsz())
            .field("locktcm", &self.locktcm())
            .field("lockitgu", &self.lockitgu())
            .field("lockdtgu", &self.lockdtgu())
            .field("itcmwsdisable", &self.itcmwsdisable())
            .field("dtcmwsdisable", &self.dtcmwsdisable())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Select ITCM memory size
    #[inline(always)]
    pub fn cfgitcmsz(&mut self) -> CFGITCMSZ_W<'_, CM55TCMCRrs> {
        CFGITCMSZ_W::new(self, 0)
    }
    ///Bits 4:7 - Select DTCM memory size
    #[inline(always)]
    pub fn cfgdtcmsz(&mut self) -> CFGDTCMSZ_W<'_, CM55TCMCRrs> {
        CFGDTCMSZ_W::new(self, 4)
    }
    ///Bit 16 - Disable writes to registers associated with the TCM region
    #[inline(always)]
    pub fn locktcm(&mut self) -> LOCKTCM_W<'_, CM55TCMCRrs> {
        LOCKTCM_W::new(self, 16)
    }
    ///Bit 17 - Disable writes to registers associated with the ITCM interface security gating.
    #[inline(always)]
    pub fn lockitgu(&mut self) -> LOCKITGU_W<'_, CM55TCMCRrs> {
        LOCKITGU_W::new(self, 17)
    }
    ///Bit 18 - Disable writes to registers associated with the DTCM interface security gating.
    #[inline(always)]
    pub fn lockdtgu(&mut self) -> LOCKDTGU_W<'_, CM55TCMCRrs> {
        LOCKDTGU_W::new(self, 18)
    }
    ///Bit 23 - Disable wait-state applied by default on extended ITCM memory.
    #[inline(always)]
    pub fn itcmwsdisable(&mut self) -> ITCMWSDISABLE_W<'_, CM55TCMCRrs> {
        ITCMWSDISABLE_W::new(self, 23)
    }
    ///Bit 24 - Disable wait-state applied by default on extended DTCM memory.
    #[inline(always)]
    pub fn dtcmwsdisable(&mut self) -> DTCMWSDISABLE_W<'_, CM55TCMCRrs> {
        DTCMWSDISABLE_W::new(self, 24)
    }
}
/**SYSCFG Cortex-M55 TCM control register

You can [`read`](crate::Reg::read) this register and get [`cm55tcmcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm55tcmcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#SYSCFG:CM55TCMCR)*/
pub struct CM55TCMCRrs;
impl crate::RegisterSpec for CM55TCMCRrs {
    type Ux = u32;
}
///`read()` method returns [`cm55tcmcr::R`](R) reader structure
impl crate::Readable for CM55TCMCRrs {}
///`write(|w| ..)` method takes [`cm55tcmcr::W`](W) writer structure
impl crate::Writable for CM55TCMCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CM55TCMCR to value 0x87
impl crate::Resettable for CM55TCMCRrs {
    const RESET_VALUE: u32 = 0x87;
}

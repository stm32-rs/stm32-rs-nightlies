#[doc = "Register `PECR` reader"]
pub type R = crate::R<PECRrs>;
#[doc = "Register `PECR` writer"]
pub type W = crate::W<PECRrs>;
#[doc = "Field `PELOCK` reader - FLASH_PECR and data EEPROM lock"]
pub type PELOCK_R = crate::BitReader;
#[doc = "Field `PELOCK` writer - FLASH_PECR and data EEPROM lock"]
pub type PELOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRGLOCK` reader - Program memory lock"]
pub type PRGLOCK_R = crate::BitReader;
#[doc = "Field `PRGLOCK` writer - Program memory lock"]
pub type PRGLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPTLOCK` reader - Option bytes block lock"]
pub type OPTLOCK_R = crate::BitReader;
#[doc = "Field `OPTLOCK` writer - Option bytes block lock"]
pub type OPTLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROG` reader - Program memory selection"]
pub type PROG_R = crate::BitReader;
#[doc = "Field `PROG` writer - Program memory selection"]
pub type PROG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA` reader - Data EEPROM selection"]
pub type DATA_R = crate::BitReader;
#[doc = "Field `DATA` writer - Data EEPROM selection"]
pub type DATA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTDW` reader - Fixed time data write for Byte, Half Word and Word programming"]
pub type FTDW_R = crate::BitReader;
#[doc = "Field `FTDW` writer - Fixed time data write for Byte, Half Word and Word programming"]
pub type FTDW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERASE` reader - Page or Double Word erase mode"]
pub type ERASE_R = crate::BitReader;
#[doc = "Field `ERASE` writer - Page or Double Word erase mode"]
pub type ERASE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPRG` reader - Half Page/Double Word programming mode"]
pub type FPRG_R = crate::BitReader;
#[doc = "Field `FPRG` writer - Half Page/Double Word programming mode"]
pub type FPRG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARALLELBANK` reader - Parallel bank mode"]
pub type PARALLELBANK_R = crate::BitReader;
#[doc = "Field `PARALLELBANK` writer - Parallel bank mode"]
pub type PARALLELBANK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOPIE` reader - End of programming interrupt enable"]
pub type EOPIE_R = crate::BitReader;
#[doc = "Field `EOPIE` writer - End of programming interrupt enable"]
pub type EOPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ERRIE_R = crate::BitReader;
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OBL_LAUNCH` reader - Launch the option byte loading"]
pub type OBL_LAUNCH_R = crate::BitReader;
#[doc = "Field `OBL_LAUNCH` writer - Launch the option byte loading"]
pub type OBL_LAUNCH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - FLASH_PECR and data EEPROM lock"]
    #[inline(always)]
    pub fn pelock(&self) -> PELOCK_R {
        PELOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Program memory lock"]
    #[inline(always)]
    pub fn prglock(&self) -> PRGLOCK_R {
        PRGLOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Option bytes block lock"]
    #[inline(always)]
    pub fn optlock(&self) -> OPTLOCK_R {
        OPTLOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Program memory selection"]
    #[inline(always)]
    pub fn prog(&self) -> PROG_R {
        PROG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data EEPROM selection"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Fixed time data write for Byte, Half Word and Word programming"]
    #[inline(always)]
    pub fn ftdw(&self) -> FTDW_R {
        FTDW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Page or Double Word erase mode"]
    #[inline(always)]
    pub fn erase(&self) -> ERASE_R {
        ERASE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Half Page/Double Word programming mode"]
    #[inline(always)]
    pub fn fprg(&self) -> FPRG_R {
        FPRG_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - Parallel bank mode"]
    #[inline(always)]
    pub fn parallelbank(&self) -> PARALLELBANK_R {
        PARALLELBANK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - End of programming interrupt enable"]
    #[inline(always)]
    pub fn eopie(&self) -> EOPIE_R {
        EOPIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Launch the option byte loading"]
    #[inline(always)]
    pub fn obl_launch(&self) -> OBL_LAUNCH_R {
        OBL_LAUNCH_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FLASH_PECR and data EEPROM lock"]
    #[inline(always)]
    #[must_use]
    pub fn pelock(&mut self) -> PELOCK_W<PECRrs> {
        PELOCK_W::new(self, 0)
    }
    #[doc = "Bit 1 - Program memory lock"]
    #[inline(always)]
    #[must_use]
    pub fn prglock(&mut self) -> PRGLOCK_W<PECRrs> {
        PRGLOCK_W::new(self, 1)
    }
    #[doc = "Bit 2 - Option bytes block lock"]
    #[inline(always)]
    #[must_use]
    pub fn optlock(&mut self) -> OPTLOCK_W<PECRrs> {
        OPTLOCK_W::new(self, 2)
    }
    #[doc = "Bit 3 - Program memory selection"]
    #[inline(always)]
    #[must_use]
    pub fn prog(&mut self) -> PROG_W<PECRrs> {
        PROG_W::new(self, 3)
    }
    #[doc = "Bit 4 - Data EEPROM selection"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<PECRrs> {
        DATA_W::new(self, 4)
    }
    #[doc = "Bit 8 - Fixed time data write for Byte, Half Word and Word programming"]
    #[inline(always)]
    #[must_use]
    pub fn ftdw(&mut self) -> FTDW_W<PECRrs> {
        FTDW_W::new(self, 8)
    }
    #[doc = "Bit 9 - Page or Double Word erase mode"]
    #[inline(always)]
    #[must_use]
    pub fn erase(&mut self) -> ERASE_W<PECRrs> {
        ERASE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Half Page/Double Word programming mode"]
    #[inline(always)]
    #[must_use]
    pub fn fprg(&mut self) -> FPRG_W<PECRrs> {
        FPRG_W::new(self, 10)
    }
    #[doc = "Bit 15 - Parallel bank mode"]
    #[inline(always)]
    #[must_use]
    pub fn parallelbank(&mut self) -> PARALLELBANK_W<PECRrs> {
        PARALLELBANK_W::new(self, 15)
    }
    #[doc = "Bit 16 - End of programming interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eopie(&mut self) -> EOPIE_W<PECRrs> {
        EOPIE_W::new(self, 16)
    }
    #[doc = "Bit 17 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<PECRrs> {
        ERRIE_W::new(self, 17)
    }
    #[doc = "Bit 18 - Launch the option byte loading"]
    #[inline(always)]
    #[must_use]
    pub fn obl_launch(&mut self) -> OBL_LAUNCH_W<PECRrs> {
        OBL_LAUNCH_W::new(self, 18)
    }
}
#[doc = "Program/erase control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pecr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pecr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PECRrs;
impl crate::RegisterSpec for PECRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pecr::R`](R) reader structure"]
impl crate::Readable for PECRrs {}
#[doc = "`write(|w| ..)` method takes [`pecr::W`](W) writer structure"]
impl crate::Writable for PECRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PECR to value 0x07"]
impl crate::Resettable for PECRrs {
    const RESET_VALUE: u32 = 0x07;
}

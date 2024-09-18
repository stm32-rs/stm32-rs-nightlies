///Register `FMC_BWTR2` reader
pub type R = crate::R<FMC_BWTR2rs>;
///Register `FMC_BWTR2` writer
pub type W = crate::W<FMC_BWTR2rs>;
///Field `ADDSET` reader - ADDSET
pub type ADDSET_R = crate::FieldReader;
///Field `ADDSET` writer - ADDSET
pub type ADDSET_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ADDHLD` reader - ADDHLD
pub type ADDHLD_R = crate::FieldReader;
///Field `ADDHLD` writer - ADDHLD
pub type ADDHLD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DATAST` reader - DATAST
pub type DATAST_R = crate::FieldReader;
///Field `DATAST` writer - DATAST
pub type DATAST_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `BUSTURN` reader - BUSTURN
pub type BUSTURN_R = crate::FieldReader;
///Field `BUSTURN` writer - BUSTURN
pub type BUSTURN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ACCMOD` reader - ACCMOD
pub type ACCMOD_R = crate::FieldReader;
///Field `ACCMOD` writer - ACCMOD
pub type ACCMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DATAHLD` reader - DATAHLD
pub type DATAHLD_R = crate::FieldReader;
///Field `DATAHLD` writer - DATAHLD
pub type DATAHLD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:3 - ADDSET
    #[inline(always)]
    pub fn addset(&self) -> ADDSET_R {
        ADDSET_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - ADDHLD
    #[inline(always)]
    pub fn addhld(&self) -> ADDHLD_R {
        ADDHLD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:15 - DATAST
    #[inline(always)]
    pub fn datast(&self) -> DATAST_R {
        DATAST_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:19 - BUSTURN
    #[inline(always)]
    pub fn busturn(&self) -> BUSTURN_R {
        BUSTURN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 28:29 - ACCMOD
    #[inline(always)]
    pub fn accmod(&self) -> ACCMOD_R {
        ACCMOD_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bits 30:31 - DATAHLD
    #[inline(always)]
    pub fn datahld(&self) -> DATAHLD_R {
        DATAHLD_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FMC_BWTR2")
            .field("addset", &self.addset())
            .field("addhld", &self.addhld())
            .field("datast", &self.datast())
            .field("busturn", &self.busturn())
            .field("accmod", &self.accmod())
            .field("datahld", &self.datahld())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - ADDSET
    #[inline(always)]
    #[must_use]
    pub fn addset(&mut self) -> ADDSET_W<FMC_BWTR2rs> {
        ADDSET_W::new(self, 0)
    }
    ///Bits 4:7 - ADDHLD
    #[inline(always)]
    #[must_use]
    pub fn addhld(&mut self) -> ADDHLD_W<FMC_BWTR2rs> {
        ADDHLD_W::new(self, 4)
    }
    ///Bits 8:15 - DATAST
    #[inline(always)]
    #[must_use]
    pub fn datast(&mut self) -> DATAST_W<FMC_BWTR2rs> {
        DATAST_W::new(self, 8)
    }
    ///Bits 16:19 - BUSTURN
    #[inline(always)]
    #[must_use]
    pub fn busturn(&mut self) -> BUSTURN_W<FMC_BWTR2rs> {
        BUSTURN_W::new(self, 16)
    }
    ///Bits 28:29 - ACCMOD
    #[inline(always)]
    #[must_use]
    pub fn accmod(&mut self) -> ACCMOD_W<FMC_BWTR2rs> {
        ACCMOD_W::new(self, 28)
    }
    ///Bits 30:31 - DATAHLD
    #[inline(always)]
    #[must_use]
    pub fn datahld(&mut self) -> DATAHLD_W<FMC_BWTR2rs> {
        DATAHLD_W::new(self, 30)
    }
}
/**This register contains the control information of each memory bank. It is used for SRAMs, FRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access.

You can [`read`](crate::Reg::read) this register and get [`fmc_bwtr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc_bwtr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FMC:FMC_BWTR2)*/
pub struct FMC_BWTR2rs;
impl crate::RegisterSpec for FMC_BWTR2rs {
    type Ux = u32;
}
///`read()` method returns [`fmc_bwtr2::R`](R) reader structure
impl crate::Readable for FMC_BWTR2rs {}
///`write(|w| ..)` method takes [`fmc_bwtr2::W`](W) writer structure
impl crate::Writable for FMC_BWTR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FMC_BWTR2 to value 0x000f_ffff
impl crate::Resettable for FMC_BWTR2rs {
    const RESET_VALUE: u32 = 0x000f_ffff;
}

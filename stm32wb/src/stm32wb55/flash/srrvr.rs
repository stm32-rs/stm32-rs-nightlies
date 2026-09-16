///Register `SRRVR` reader
pub type R = crate::R<SRRVRrs>;
///Register `SRRVR` writer
pub type W = crate::W<SRRVRrs>;
///Field `SBRV` reader - cortex M0 access control register
pub type SBRV_R = crate::FieldReader<u32>;
///Field `SBRV` writer - cortex M0 access control register
pub type SBRV_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
///Field `SBRSA` reader - Secure backup SRAM2a start address
pub type SBRSA_R = crate::FieldReader;
///Field `SBRSA` writer - Secure backup SRAM2a start address
pub type SBRSA_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `BRSD` reader - backup SRAM2a security disable
pub type BRSD_R = crate::BitReader;
///Field `BRSD` writer - backup SRAM2a security disable
pub type BRSD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SNBRSA` reader - Secure non backup SRAM2a start address
pub type SNBRSA_R = crate::FieldReader;
///Field `SNBRSA` writer - Secure non backup SRAM2a start address
pub type SNBRSA_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `NBRSD` reader - non-backup SRAM2b security disable
pub type NBRSD_R = crate::BitReader;
///Field `NBRSD` writer - non-backup SRAM2b security disable
pub type NBRSD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `C2OPT` reader - CPU2 cortex M0 boot reset vector memory selection
pub type C2OPT_R = crate::BitReader;
///Field `C2OPT` writer - CPU2 cortex M0 boot reset vector memory selection
pub type C2OPT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:17 - cortex M0 access control register
    #[inline(always)]
    pub fn sbrv(&self) -> SBRV_R {
        SBRV_R::new(self.bits & 0x0003_ffff)
    }
    ///Bits 18:22 - Secure backup SRAM2a start address
    #[inline(always)]
    pub fn sbrsa(&self) -> SBRSA_R {
        SBRSA_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    ///Bit 23 - backup SRAM2a security disable
    #[inline(always)]
    pub fn brsd(&self) -> BRSD_R {
        BRSD_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 25:29 - Secure non backup SRAM2a start address
    #[inline(always)]
    pub fn snbrsa(&self) -> SNBRSA_R {
        SNBRSA_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
    ///Bit 30 - non-backup SRAM2b security disable
    #[inline(always)]
    pub fn nbrsd(&self) -> NBRSD_R {
        NBRSD_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - CPU2 cortex M0 boot reset vector memory selection
    #[inline(always)]
    pub fn c2opt(&self) -> C2OPT_R {
        C2OPT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRRVR")
            .field("sbrv", &self.sbrv())
            .field("sbrsa", &self.sbrsa())
            .field("brsd", &self.brsd())
            .field("snbrsa", &self.snbrsa())
            .field("c2opt", &self.c2opt())
            .field("nbrsd", &self.nbrsd())
            .finish()
    }
}
impl W {
    ///Bits 0:17 - cortex M0 access control register
    #[inline(always)]
    pub fn sbrv(&mut self) -> SBRV_W<'_, SRRVRrs> {
        SBRV_W::new(self, 0)
    }
    ///Bits 18:22 - Secure backup SRAM2a start address
    #[inline(always)]
    pub fn sbrsa(&mut self) -> SBRSA_W<'_, SRRVRrs> {
        SBRSA_W::new(self, 18)
    }
    ///Bit 23 - backup SRAM2a security disable
    #[inline(always)]
    pub fn brsd(&mut self) -> BRSD_W<'_, SRRVRrs> {
        BRSD_W::new(self, 23)
    }
    ///Bits 25:29 - Secure non backup SRAM2a start address
    #[inline(always)]
    pub fn snbrsa(&mut self) -> SNBRSA_W<'_, SRRVRrs> {
        SNBRSA_W::new(self, 25)
    }
    ///Bit 30 - non-backup SRAM2b security disable
    #[inline(always)]
    pub fn nbrsd(&mut self) -> NBRSD_W<'_, SRRVRrs> {
        NBRSD_W::new(self, 30)
    }
    ///Bit 31 - CPU2 cortex M0 boot reset vector memory selection
    #[inline(always)]
    pub fn c2opt(&mut self) -> C2OPT_W<'_, SRRVRrs> {
        C2OPT_W::new(self, 31)
    }
}
/**Secure SRAM2 start address and cortex M0 reset vector register

You can [`read`](crate::Reg::read) this register and get [`srrvr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srrvr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#Flash:SRRVR)*/
pub struct SRRVRrs;
impl crate::RegisterSpec for SRRVRrs {
    type Ux = u32;
}
///`read()` method returns [`srrvr::R`](R) reader structure
impl crate::Readable for SRRVRrs {}
///`write(|w| ..)` method takes [`srrvr::W`](W) writer structure
impl crate::Writable for SRRVRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SRRVR to value 0x0100_0000
impl crate::Resettable for SRRVRrs {
    const RESET_VALUE: u32 = 0x0100_0000;
}

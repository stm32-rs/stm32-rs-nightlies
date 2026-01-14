///Register `SECCR` reader
pub type R = crate::R<SECCRrs>;
///Register `SECCR` writer
pub type W = crate::W<SECCRrs>;
///Field `LOCK` reader - configuration lock bit This bit locks the FLASH_SECCR register. The correct write sequence to FLASH_SECKEYR register unlocks this bit. If a wrong sequence is executed, or if the unlock sequence to FLASH_NSKEYR is performed twice, this bit remains locked until the next system reset. LOCK can be set by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When LOCK changes from 0 to 1, the other bits of FLASH_SECCR register do not change.
pub type LOCK_R = crate::BitReader;
///Field `LOCK` writer - configuration lock bit This bit locks the FLASH_SECCR register. The correct write sequence to FLASH_SECKEYR register unlocks this bit. If a wrong sequence is executed, or if the unlock sequence to FLASH_NSKEYR is performed twice, this bit remains locked until the next system reset. LOCK can be set by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When LOCK changes from 0 to 1, the other bits of FLASH_SECCR register do not change.
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PG` reader - programming control bit PG can be programmed only when LOCK is cleared to 0. PG allows programming in Bank1 and Bank2.
pub type PG_R = crate::BitReader;
///Field `PG` writer - programming control bit PG can be programmed only when LOCK is cleared to 0. PG allows programming in Bank1 and Bank2.
pub type PG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SER` reader - sector erase request Setting SER bit to 1 requests a sector erase. SER can be programmed only when LOCK is cleared to 0. If BER and MER are also set, a PGSERR is raised.
pub type SER_R = crate::BitReader;
///Field `SER` writer - sector erase request Setting SER bit to 1 requests a sector erase. SER can be programmed only when LOCK is cleared to 0. If BER and MER are also set, a PGSERR is raised.
pub type SER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BER` reader - erase request Setting BER bit to 1 requests a bank erase operation (user flash memory only). BER can be programmed only when LOCK is cleared to 0. If MER and SER are also set, a PGSERR is raised. Note: Write protection error is triggered when a bank erase is required and some sectors are protected.
pub type BER_R = crate::BitReader;
///Field `BER` writer - erase request Setting BER bit to 1 requests a bank erase operation (user flash memory only). BER can be programmed only when LOCK is cleared to 0. If MER and SER are also set, a PGSERR is raised. Note: Write protection error is triggered when a bank erase is required and some sectors are protected.
pub type BER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FW` reader - write forcing control bit FW forces a write operation even if the write buffer is not full. In this case all bits not written are set to 1 by hardware. FW can be programmed only when LOCK is cleared to 0. The embedded flash memory resets FW when the corresponding operation has been acknowledged. Note: Using a force-write operation prevents the application from updating later the missing bits with something else than 1, because it is likely that it leads to permanent ECC error. Write forcing is effective only if the write buffer is not empty and was filled by secure access (in particular, FW does not start several write operations when the force-write operations are performed consecutively). Since there is just one write buffer, FW can force a write in bank1 or bank2.
pub type FW_R = crate::BitReader;
///Field `FW` writer - write forcing control bit FW forces a write operation even if the write buffer is not full. In this case all bits not written are set to 1 by hardware. FW can be programmed only when LOCK is cleared to 0. The embedded flash memory resets FW when the corresponding operation has been acknowledged. Note: Using a force-write operation prevents the application from updating later the missing bits with something else than 1, because it is likely that it leads to permanent ECC error. Write forcing is effective only if the write buffer is not empty and was filled by secure access (in particular, FW does not start several write operations when the force-write operations are performed consecutively). Since there is just one write buffer, FW can force a write in bank1 or bank2.
pub type FW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STRT` reader - erase start control bit STRT bit is used to start a sector erase or a bank erase operation. STRT can be programmed only when LOCK is cleared to 0. STRT is reseted at the end of the operation or when an error occurs. It cannot be reset by software.
pub type STRT_R = crate::BitReader;
///Field `STRT` writer - erase start control bit STRT bit is used to start a sector erase or a bank erase operation. STRT can be programmed only when LOCK is cleared to 0. STRT is reseted at the end of the operation or when an error occurs. It cannot be reset by software.
pub type STRT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SNB` reader - sector erase selection number These bits are used to select the target sector for an erase operation (they are unused otherwise). SNB can be programmed only when LOCK is cleared to 0. ..
pub type SNB_R = crate::FieldReader;
///Field `SNB` writer - sector erase selection number These bits are used to select the target sector for an erase operation (they are unused otherwise). SNB can be programmed only when LOCK is cleared to 0. ..
pub type SNB_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `MER` reader - mass erase request Setting MER bit to 1 requests a mass erase operation (user flash memory only). MER can be programmed only when LOCK is cleared to 0. If BER or SER are also set, a PGSERR is raised. Error is triggered when a mass erase is required and some sectors are protected.
pub type MER_R = crate::BitReader;
///Field `MER` writer - mass erase request Setting MER bit to 1 requests a mass erase operation (user flash memory only). MER can be programmed only when LOCK is cleared to 0. If BER or SER are also set, a PGSERR is raised. Error is triggered when a mass erase is required and some sectors are protected.
pub type MER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOPIE` reader - end of operation interrupt control bit Setting EOPIE bit to 1 enables the generation of an interrupt at the end of a program/erase operation. EOPIE can be programmed only when LOCK is cleared to 0.
pub type EOPIE_R = crate::BitReader;
///Field `EOPIE` writer - end of operation interrupt control bit Setting EOPIE bit to 1 enables the generation of an interrupt at the end of a program/erase operation. EOPIE can be programmed only when LOCK is cleared to 0.
pub type EOPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WRPERRIE` reader - write protection error interrupt enable bit When WRPERRIE bit is set to 1, an interrupt is generated when a protection error occurs during a program operation. WRPERRIE can be programmed only when LOCK is cleared to 0.
pub type WRPERRIE_R = crate::BitReader;
///Field `WRPERRIE` writer - write protection error interrupt enable bit When WRPERRIE bit is set to 1, an interrupt is generated when a protection error occurs during a program operation. WRPERRIE can be programmed only when LOCK is cleared to 0.
pub type WRPERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PGSERRIE` reader - programming sequence error interrupt enable bit When PGSERRIE bit is set to 1, an interrupt is generated when a sequence error occurs during a program operation. PGSERRIE can be programmed only when LOCK is cleared to 0.
pub type PGSERRIE_R = crate::BitReader;
///Field `PGSERRIE` writer - programming sequence error interrupt enable bit When PGSERRIE bit is set to 1, an interrupt is generated when a sequence error occurs during a program operation. PGSERRIE can be programmed only when LOCK is cleared to 0.
pub type PGSERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STRBERRIE` reader - strobe error interrupt enable bit When STRBERRIE bit is set to 1, an interrupt is generated when a strobe error occurs (the master programs several times the same byte in the write buffer) during a write operation. STRBERRIE can be programmed only when LOCK is cleared to 0.
pub type STRBERRIE_R = crate::BitReader;
///Field `STRBERRIE` writer - strobe error interrupt enable bit When STRBERRIE bit is set to 1, an interrupt is generated when a strobe error occurs (the master programs several times the same byte in the write buffer) during a write operation. STRBERRIE can be programmed only when LOCK is cleared to 0.
pub type STRBERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INCERRIE` reader - inconsistency error interrupt enable bit When INCERRIE bit is set to 1, an interrupt is generated when an inconsistency error occurs during a write operation. INCERRIE can be programmed only when LOCK is cleared to 0.
pub type INCERRIE_R = crate::BitReader;
///Field `INCERRIE` writer - inconsistency error interrupt enable bit When INCERRIE bit is set to 1, an interrupt is generated when an inconsistency error occurs during a write operation. INCERRIE can be programmed only when LOCK is cleared to 0.
pub type INCERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OBKERRIE` reader - OBK general error interrupt enable bit OBKERRIE enables generating an interrupt in case of OBK specific access error. OBKERRIE can be programmed only when LOCK is cleared to 0.
pub type OBKERRIE_R = crate::BitReader;
///Field `OBKERRIE` writer - OBK general error interrupt enable bit OBKERRIE enables generating an interrupt in case of OBK specific access error. OBKERRIE can be programmed only when LOCK is cleared to 0.
pub type OBKERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OBKWERRIE` reader - OBK write error interrupt enable bit OBKWERRIE enables generation of interrupt in case of OBK specific write error. OBKWERRIE can be programmed only when LOCK is cleared to 0.
pub type OBKWERRIE_R = crate::BitReader;
///Field `OBKWERRIE` writer - OBK write error interrupt enable bit OBKWERRIE enables generation of interrupt in case of OBK specific write error. OBKWERRIE can be programmed only when LOCK is cleared to 0.
pub type OBKWERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INV` reader - Flash memory security state invert. This bit inverts the flash memory security state.
pub type INV_R = crate::BitReader;
///Field `INV` writer - Flash memory security state invert. This bit inverts the flash memory security state.
pub type INV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKSEL` reader - bank selector bit BKSEL can only be programmed when LOCK is cleared to 0. The bit selects physical bank, SWAP_BANK setting is ignored.
pub type BKSEL_R = crate::BitReader;
///Field `BKSEL` writer - bank selector bit BKSEL can only be programmed when LOCK is cleared to 0. The bit selects physical bank, SWAP_BANK setting is ignored.
pub type BKSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - configuration lock bit This bit locks the FLASH_SECCR register. The correct write sequence to FLASH_SECKEYR register unlocks this bit. If a wrong sequence is executed, or if the unlock sequence to FLASH_NSKEYR is performed twice, this bit remains locked until the next system reset. LOCK can be set by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When LOCK changes from 0 to 1, the other bits of FLASH_SECCR register do not change.
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - programming control bit PG can be programmed only when LOCK is cleared to 0. PG allows programming in Bank1 and Bank2.
    #[inline(always)]
    pub fn pg(&self) -> PG_R {
        PG_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - sector erase request Setting SER bit to 1 requests a sector erase. SER can be programmed only when LOCK is cleared to 0. If BER and MER are also set, a PGSERR is raised.
    #[inline(always)]
    pub fn ser(&self) -> SER_R {
        SER_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - erase request Setting BER bit to 1 requests a bank erase operation (user flash memory only). BER can be programmed only when LOCK is cleared to 0. If MER and SER are also set, a PGSERR is raised. Note: Write protection error is triggered when a bank erase is required and some sectors are protected.
    #[inline(always)]
    pub fn ber(&self) -> BER_R {
        BER_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - write forcing control bit FW forces a write operation even if the write buffer is not full. In this case all bits not written are set to 1 by hardware. FW can be programmed only when LOCK is cleared to 0. The embedded flash memory resets FW when the corresponding operation has been acknowledged. Note: Using a force-write operation prevents the application from updating later the missing bits with something else than 1, because it is likely that it leads to permanent ECC error. Write forcing is effective only if the write buffer is not empty and was filled by secure access (in particular, FW does not start several write operations when the force-write operations are performed consecutively). Since there is just one write buffer, FW can force a write in bank1 or bank2.
    #[inline(always)]
    pub fn fw(&self) -> FW_R {
        FW_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - erase start control bit STRT bit is used to start a sector erase or a bank erase operation. STRT can be programmed only when LOCK is cleared to 0. STRT is reseted at the end of the operation or when an error occurs. It cannot be reset by software.
    #[inline(always)]
    pub fn strt(&self) -> STRT_R {
        STRT_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:12 - sector erase selection number These bits are used to select the target sector for an erase operation (they are unused otherwise). SNB can be programmed only when LOCK is cleared to 0. ..
    #[inline(always)]
    pub fn snb(&self) -> SNB_R {
        SNB_R::new(((self.bits >> 6) & 0x7f) as u8)
    }
    ///Bit 15 - mass erase request Setting MER bit to 1 requests a mass erase operation (user flash memory only). MER can be programmed only when LOCK is cleared to 0. If BER or SER are also set, a PGSERR is raised. Error is triggered when a mass erase is required and some sectors are protected.
    #[inline(always)]
    pub fn mer(&self) -> MER_R {
        MER_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - end of operation interrupt control bit Setting EOPIE bit to 1 enables the generation of an interrupt at the end of a program/erase operation. EOPIE can be programmed only when LOCK is cleared to 0.
    #[inline(always)]
    pub fn eopie(&self) -> EOPIE_R {
        EOPIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - write protection error interrupt enable bit When WRPERRIE bit is set to 1, an interrupt is generated when a protection error occurs during a program operation. WRPERRIE can be programmed only when LOCK is cleared to 0.
    #[inline(always)]
    pub fn wrperrie(&self) -> WRPERRIE_R {
        WRPERRIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - programming sequence error interrupt enable bit When PGSERRIE bit is set to 1, an interrupt is generated when a sequence error occurs during a program operation. PGSERRIE can be programmed only when LOCK is cleared to 0.
    #[inline(always)]
    pub fn pgserrie(&self) -> PGSERRIE_R {
        PGSERRIE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - strobe error interrupt enable bit When STRBERRIE bit is set to 1, an interrupt is generated when a strobe error occurs (the master programs several times the same byte in the write buffer) during a write operation. STRBERRIE can be programmed only when LOCK is cleared to 0.
    #[inline(always)]
    pub fn strberrie(&self) -> STRBERRIE_R {
        STRBERRIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - inconsistency error interrupt enable bit When INCERRIE bit is set to 1, an interrupt is generated when an inconsistency error occurs during a write operation. INCERRIE can be programmed only when LOCK is cleared to 0.
    #[inline(always)]
    pub fn incerrie(&self) -> INCERRIE_R {
        INCERRIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - OBK general error interrupt enable bit OBKERRIE enables generating an interrupt in case of OBK specific access error. OBKERRIE can be programmed only when LOCK is cleared to 0.
    #[inline(always)]
    pub fn obkerrie(&self) -> OBKERRIE_R {
        OBKERRIE_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - OBK write error interrupt enable bit OBKWERRIE enables generation of interrupt in case of OBK specific write error. OBKWERRIE can be programmed only when LOCK is cleared to 0.
    #[inline(always)]
    pub fn obkwerrie(&self) -> OBKWERRIE_R {
        OBKWERRIE_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 29 - Flash memory security state invert. This bit inverts the flash memory security state.
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 31 - bank selector bit BKSEL can only be programmed when LOCK is cleared to 0. The bit selects physical bank, SWAP_BANK setting is ignored.
    #[inline(always)]
    pub fn bksel(&self) -> BKSEL_R {
        BKSEL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECCR")
            .field("lock", &self.lock())
            .field("pg", &self.pg())
            .field("ser", &self.ser())
            .field("ber", &self.ber())
            .field("fw", &self.fw())
            .field("strt", &self.strt())
            .field("snb", &self.snb())
            .field("mer", &self.mer())
            .field("eopie", &self.eopie())
            .field("wrperrie", &self.wrperrie())
            .field("pgserrie", &self.pgserrie())
            .field("strberrie", &self.strberrie())
            .field("incerrie", &self.incerrie())
            .field("obkerrie", &self.obkerrie())
            .field("obkwerrie", &self.obkwerrie())
            .field("inv", &self.inv())
            .field("bksel", &self.bksel())
            .finish()
    }
}
impl W {
    ///Bit 0 - configuration lock bit This bit locks the FLASH_SECCR register. The correct write sequence to FLASH_SECKEYR register unlocks this bit. If a wrong sequence is executed, or if the unlock sequence to FLASH_NSKEYR is performed twice, this bit remains locked until the next system reset. LOCK can be set by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When LOCK changes from 0 to 1, the other bits of FLASH_SECCR register do not change.
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<'_, SECCRrs> {
        LOCK_W::new(self, 0)
    }
    ///Bit 1 - programming control bit PG can be programmed only when LOCK is cleared to 0. PG allows programming in Bank1 and Bank2.
    #[inline(always)]
    pub fn pg(&mut self) -> PG_W<'_, SECCRrs> {
        PG_W::new(self, 1)
    }
    ///Bit 2 - sector erase request Setting SER bit to 1 requests a sector erase. SER can be programmed only when LOCK is cleared to 0. If BER and MER are also set, a PGSERR is raised.
    #[inline(always)]
    pub fn ser(&mut self) -> SER_W<'_, SECCRrs> {
        SER_W::new(self, 2)
    }
    ///Bit 3 - erase request Setting BER bit to 1 requests a bank erase operation (user flash memory only). BER can be programmed only when LOCK is cleared to 0. If MER and SER are also set, a PGSERR is raised. Note: Write protection error is triggered when a bank erase is required and some sectors are protected.
    #[inline(always)]
    pub fn ber(&mut self) -> BER_W<'_, SECCRrs> {
        BER_W::new(self, 3)
    }
    ///Bit 4 - write forcing control bit FW forces a write operation even if the write buffer is not full. In this case all bits not written are set to 1 by hardware. FW can be programmed only when LOCK is cleared to 0. The embedded flash memory resets FW when the corresponding operation has been acknowledged. Note: Using a force-write operation prevents the application from updating later the missing bits with something else than 1, because it is likely that it leads to permanent ECC error. Write forcing is effective only if the write buffer is not empty and was filled by secure access (in particular, FW does not start several write operations when the force-write operations are performed consecutively). Since there is just one write buffer, FW can force a write in bank1 or bank2.
    #[inline(always)]
    pub fn fw(&mut self) -> FW_W<'_, SECCRrs> {
        FW_W::new(self, 4)
    }
    ///Bit 5 - erase start control bit STRT bit is used to start a sector erase or a bank erase operation. STRT can be programmed only when LOCK is cleared to 0. STRT is reseted at the end of the operation or when an error occurs. It cannot be reset by software.
    #[inline(always)]
    pub fn strt(&mut self) -> STRT_W<'_, SECCRrs> {
        STRT_W::new(self, 5)
    }
    ///Bits 6:12 - sector erase selection number These bits are used to select the target sector for an erase operation (they are unused otherwise). SNB can be programmed only when LOCK is cleared to 0. ..
    #[inline(always)]
    pub fn snb(&mut self) -> SNB_W<'_, SECCRrs> {
        SNB_W::new(self, 6)
    }
    ///Bit 15 - mass erase request Setting MER bit to 1 requests a mass erase operation (user flash memory only). MER can be programmed only when LOCK is cleared to 0. If BER or SER are also set, a PGSERR is raised. Error is triggered when a mass erase is required and some sectors are protected.
    #[inline(always)]
    pub fn mer(&mut self) -> MER_W<'_, SECCRrs> {
        MER_W::new(self, 15)
    }
    ///Bit 16 - end of operation interrupt control bit Setting EOPIE bit to 1 enables the generation of an interrupt at the end of a program/erase operation. EOPIE can be programmed only when LOCK is cleared to 0.
    #[inline(always)]
    pub fn eopie(&mut self) -> EOPIE_W<'_, SECCRrs> {
        EOPIE_W::new(self, 16)
    }
    ///Bit 17 - write protection error interrupt enable bit When WRPERRIE bit is set to 1, an interrupt is generated when a protection error occurs during a program operation. WRPERRIE can be programmed only when LOCK is cleared to 0.
    #[inline(always)]
    pub fn wrperrie(&mut self) -> WRPERRIE_W<'_, SECCRrs> {
        WRPERRIE_W::new(self, 17)
    }
    ///Bit 18 - programming sequence error interrupt enable bit When PGSERRIE bit is set to 1, an interrupt is generated when a sequence error occurs during a program operation. PGSERRIE can be programmed only when LOCK is cleared to 0.
    #[inline(always)]
    pub fn pgserrie(&mut self) -> PGSERRIE_W<'_, SECCRrs> {
        PGSERRIE_W::new(self, 18)
    }
    ///Bit 19 - strobe error interrupt enable bit When STRBERRIE bit is set to 1, an interrupt is generated when a strobe error occurs (the master programs several times the same byte in the write buffer) during a write operation. STRBERRIE can be programmed only when LOCK is cleared to 0.
    #[inline(always)]
    pub fn strberrie(&mut self) -> STRBERRIE_W<'_, SECCRrs> {
        STRBERRIE_W::new(self, 19)
    }
    ///Bit 20 - inconsistency error interrupt enable bit When INCERRIE bit is set to 1, an interrupt is generated when an inconsistency error occurs during a write operation. INCERRIE can be programmed only when LOCK is cleared to 0.
    #[inline(always)]
    pub fn incerrie(&mut self) -> INCERRIE_W<'_, SECCRrs> {
        INCERRIE_W::new(self, 20)
    }
    ///Bit 21 - OBK general error interrupt enable bit OBKERRIE enables generating an interrupt in case of OBK specific access error. OBKERRIE can be programmed only when LOCK is cleared to 0.
    #[inline(always)]
    pub fn obkerrie(&mut self) -> OBKERRIE_W<'_, SECCRrs> {
        OBKERRIE_W::new(self, 21)
    }
    ///Bit 22 - OBK write error interrupt enable bit OBKWERRIE enables generation of interrupt in case of OBK specific write error. OBKWERRIE can be programmed only when LOCK is cleared to 0.
    #[inline(always)]
    pub fn obkwerrie(&mut self) -> OBKWERRIE_W<'_, SECCRrs> {
        OBKWERRIE_W::new(self, 22)
    }
    ///Bit 29 - Flash memory security state invert. This bit inverts the flash memory security state.
    #[inline(always)]
    pub fn inv(&mut self) -> INV_W<'_, SECCRrs> {
        INV_W::new(self, 29)
    }
    ///Bit 31 - bank selector bit BKSEL can only be programmed when LOCK is cleared to 0. The bit selects physical bank, SWAP_BANK setting is ignored.
    #[inline(always)]
    pub fn bksel(&mut self) -> BKSEL_W<'_, SECCRrs> {
        BKSEL_W::new(self, 31)
    }
}
/**FLASH secure control register

You can [`read`](crate::Reg::read) this register and get [`seccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#FLASH:SECCR)*/
pub struct SECCRrs;
impl crate::RegisterSpec for SECCRrs {
    type Ux = u32;
}
///`read()` method returns [`seccr::R`](R) reader structure
impl crate::Readable for SECCRrs {}
///`write(|w| ..)` method takes [`seccr::W`](W) writer structure
impl crate::Writable for SECCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECCR to value 0x01
impl crate::Resettable for SECCRrs {
    const RESET_VALUE: u32 = 0x01;
}

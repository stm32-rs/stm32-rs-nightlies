///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `LOCK` reader - Bank 1 configuration lock bit This bit locks the FLASH_CR1 register. The correct write sequence to FLASH_KEYR1 register unlocks this bit. If a wrong sequence is executed, or if the unlock sequence to FLASH_KEYR1 is performed twice, this bit remains locked until the next system reset. LOCK1 can be set by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When LOCK1 changes from 0 to 1, the other bits of FLASH_CR1 register do not change.
pub type LOCK_R = crate::BitReader;
///Field `LOCK` writer - Bank 1 configuration lock bit This bit locks the FLASH_CR1 register. The correct write sequence to FLASH_KEYR1 register unlocks this bit. If a wrong sequence is executed, or if the unlock sequence to FLASH_KEYR1 is performed twice, this bit remains locked until the next system reset. LOCK1 can be set by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When LOCK1 changes from 0 to 1, the other bits of FLASH_CR1 register do not change.
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PG` reader - Bank 1 internal buffer control bit Setting PG1 bit to 1 enables internal buffer for write operations to bank 1. This allows preparing program operations even if a sector or bank erase is ongoing. PG1 can be programmed only when LOCK1 is cleared to 0. When PG1 is reset, the internal buffer is disabled for write operations to bank 1, and all the data stored in the buffer but not sent to the operation queue are lost.
pub type PG_R = crate::BitReader;
///Field `PG` writer - Bank 1 internal buffer control bit Setting PG1 bit to 1 enables internal buffer for write operations to bank 1. This allows preparing program operations even if a sector or bank erase is ongoing. PG1 can be programmed only when LOCK1 is cleared to 0. When PG1 is reset, the internal buffer is disabled for write operations to bank 1, and all the data stored in the buffer but not sent to the operation queue are lost.
pub type PG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SER` reader - Bank 1 sector erase request Setting SER1 bit to 1 requests a sector erase on bank 1. SER1 can be programmed only when LOCK1 is cleared to 0. BER1 has a higher priority than SER1: if both bits are set, the embedded Flash memory executes a bank erase. Note: Write protection error is triggered when a sector erase is required on a protected sector.
pub type SER_R = crate::BitReader;
///Field `SER` writer - Bank 1 sector erase request Setting SER1 bit to 1 requests a sector erase on bank 1. SER1 can be programmed only when LOCK1 is cleared to 0. BER1 has a higher priority than SER1: if both bits are set, the embedded Flash memory executes a bank erase. Note: Write protection error is triggered when a sector erase is required on a protected sector.
pub type SER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BER` reader - Bank 1 erase request Setting BER1 bit to 1 requests a bank erase operation on bank 1 (user Flash memory only). BER1 can be programmed only when LOCK1 is cleared to 0. BER1 has a higher priority than SER1: if both are set, the embedded Flash memory executes a bank erase. Note: Write protection error is triggered when a bank erase is required and some sectors are protected.
pub type BER_R = crate::BitReader;
///Field `BER` writer - Bank 1 erase request Setting BER1 bit to 1 requests a bank erase operation on bank 1 (user Flash memory only). BER1 can be programmed only when LOCK1 is cleared to 0. BER1 has a higher priority than SER1: if both are set, the embedded Flash memory executes a bank erase. Note: Write protection error is triggered when a bank erase is required and some sectors are protected.
pub type BER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FW` reader - Bank 1 write forcing control bit FW1 forces a write operation even if the write buffer is not full. In this case all bits not written are set to 1 by hardware. FW1 can be programmed only when LOCK1 is cleared to 0. The embedded Flash memory resets FW1 when the corresponding operation has been acknowledged. Note: Using a force-write operation prevents the application from updating later the missing bits with something else than 1, because it is likely that it will lead to permanent ECC error. Write forcing is effective only if the write buffer is not empty (in particular, FW1 does not start several write operations when the force-write operations are performed consecutively).
pub type FW_R = crate::BitReader;
///Field `FW` writer - Bank 1 write forcing control bit FW1 forces a write operation even if the write buffer is not full. In this case all bits not written are set to 1 by hardware. FW1 can be programmed only when LOCK1 is cleared to 0. The embedded Flash memory resets FW1 when the corresponding operation has been acknowledged. Note: Using a force-write operation prevents the application from updating later the missing bits with something else than 1, because it is likely that it will lead to permanent ECC error. Write forcing is effective only if the write buffer is not empty (in particular, FW1 does not start several write operations when the force-write operations are performed consecutively).
pub type FW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `START` reader - Bank 1 erase start control bit START1 bit is used to start a sector erase or a bank erase operation. START1 can be programmed only when LOCK1 is cleared to 0. The embedded Flash memory resets START1 when the corresponding operation has been acknowledged. The user application cannot access any embedded Flash memory register until the operation is acknowledged.
pub type START_R = crate::BitReader;
///Field `START` writer - Bank 1 erase start control bit START1 bit is used to start a sector erase or a bank erase operation. START1 can be programmed only when LOCK1 is cleared to 0. The embedded Flash memory resets START1 when the corresponding operation has been acknowledged. The user application cannot access any embedded Flash memory register until the operation is acknowledged.
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SNB` reader - Bank 1 sector erase selection number These bits are used to select the target sector for an erase operation (they are unused otherwise). SSN1 can be programmed only when LOCK1 is cleared to 0. .. ... ... Note: Bank 1 is limited to 16 and 64 sectors on STM32H7B0 and STM32H7A3xG devices, respectively.
pub type SNB_R = crate::FieldReader;
///Field `SNB` writer - Bank 1 sector erase selection number These bits are used to select the target sector for an erase operation (they are unused otherwise). SSN1 can be programmed only when LOCK1 is cleared to 0. .. ... ... Note: Bank 1 is limited to 16 and 64 sectors on STM32H7B0 and STM32H7A3xG devices, respectively.
pub type SNB_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `CRC_EN` reader - Bank 1 CRC control bit Setting CRC_EN bit to 1 enables the CRC calculation on bank 1. CRC_EN does not start CRC calculation but enables CRC configuration through FLASH_CRCCR1 register. When CRC calculation is performed on bank 1, it can only be disabled by setting CRC_EN bit to 0. Resetting CRC_EN clears CRC configuration and resets the content of FLASH_CRCDATAR register. Clearing CRC_EN to 0 sets CRCDATA to 0x0. CRC_EN can be programmed only when LOCK1 is cleared to 0.
pub type CRC_EN_R = crate::BitReader;
///Field `CRC_EN` writer - Bank 1 CRC control bit Setting CRC_EN bit to 1 enables the CRC calculation on bank 1. CRC_EN does not start CRC calculation but enables CRC configuration through FLASH_CRCCR1 register. When CRC calculation is performed on bank 1, it can only be disabled by setting CRC_EN bit to 0. Resetting CRC_EN clears CRC configuration and resets the content of FLASH_CRCDATAR register. Clearing CRC_EN to 0 sets CRCDATA to 0x0. CRC_EN can be programmed only when LOCK1 is cleared to 0.
pub type CRC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOPIE` reader - Bank 1 end-of-program interrupt control bit Setting EOPIE1 bit to 1 enables the generation of an interrupt at the end of a program operation to bank 1. EOPIE1 can be programmed only when LOCK1 is cleared to 0.
pub type EOPIE_R = crate::BitReader;
///Field `EOPIE` writer - Bank 1 end-of-program interrupt control bit Setting EOPIE1 bit to 1 enables the generation of an interrupt at the end of a program operation to bank 1. EOPIE1 can be programmed only when LOCK1 is cleared to 0.
pub type EOPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WRPERRIE` reader - Bank 1 write protection error interrupt enable bit When WRPERRIE1 bit is set to 1, an interrupt is generated when a protection error occurs during a program operation to bank 1. WRPERRIE1 can be programmed only when LOCK1 is cleared to 0.
pub type WRPERRIE_R = crate::BitReader;
///Field `WRPERRIE` writer - Bank 1 write protection error interrupt enable bit When WRPERRIE1 bit is set to 1, an interrupt is generated when a protection error occurs during a program operation to bank 1. WRPERRIE1 can be programmed only when LOCK1 is cleared to 0.
pub type WRPERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PGSERRIE` reader - Bank 1 programming sequence error interrupt enable bit When PGSERRIE1 bit is set to 1, an interrupt is generated when a sequence error occurs during a program operation to bank 1. PGSERRIE1 can be programmed only when LOCK1 is cleared to 0.
pub type PGSERRIE_R = crate::BitReader;
///Field `PGSERRIE` writer - Bank 1 programming sequence error interrupt enable bit When PGSERRIE1 bit is set to 1, an interrupt is generated when a sequence error occurs during a program operation to bank 1. PGSERRIE1 can be programmed only when LOCK1 is cleared to 0.
pub type PGSERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STRBERRIE` reader - Bank 1 strobe error interrupt enable bit When STRBERRIE1 bit is set to 1, an interrupt is generated when a strobe error occurs (the master programs several times the same byte in the write buffer) during a write operation to bank 1. STRBERRIE1 can be programmed only when LOCK1 is cleared to 0.
pub type STRBERRIE_R = crate::BitReader;
///Field `STRBERRIE` writer - Bank 1 strobe error interrupt enable bit When STRBERRIE1 bit is set to 1, an interrupt is generated when a strobe error occurs (the master programs several times the same byte in the write buffer) during a write operation to bank 1. STRBERRIE1 can be programmed only when LOCK1 is cleared to 0.
pub type STRBERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INCERRIE` reader - Bank 1 inconsistency error interrupt enable bit When INCERRIE1 bit is set to 1, an interrupt is generated when an inconsistency error occurs during a write operation to bank 1. INCERRIE1 can be programmed only when LOCK1 is cleared to 0.
pub type INCERRIE_R = crate::BitReader;
///Field `INCERRIE` writer - Bank 1 inconsistency error interrupt enable bit When INCERRIE1 bit is set to 1, an interrupt is generated when an inconsistency error occurs during a write operation to bank 1. INCERRIE1 can be programmed only when LOCK1 is cleared to 0.
pub type INCERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RDPERRIE` reader - Bank 1 read protection error interrupt enable bit When RDPERRIE1 bit is set to 1, an interrupt is generated when a read protection error occurs (access to an address protected by PCROP or by RDP level 1) during a read operation from bank 1. RDPERRIE1 can be programmed only when LOCK1 is cleared to 0.
pub type RDPERRIE_R = crate::BitReader;
///Field `RDPERRIE` writer - Bank 1 read protection error interrupt enable bit When RDPERRIE1 bit is set to 1, an interrupt is generated when a read protection error occurs (access to an address protected by PCROP or by RDP level 1) during a read operation from bank 1. RDPERRIE1 can be programmed only when LOCK1 is cleared to 0.
pub type RDPERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RDSERRIE` reader - Bank 1 secure error interrupt enable bit When RDSERRIE1 bit is set to 1, an interrupt is generated when a secure error (access to a secure-only protected address) occurs during a read operation from bank 1. RDSERRIE1 can be programmed only when LOCK1 is cleared to 0.
pub type RDSERRIE_R = crate::BitReader;
///Field `RDSERRIE` writer - Bank 1 secure error interrupt enable bit When RDSERRIE1 bit is set to 1, an interrupt is generated when a secure error (access to a secure-only protected address) occurs during a read operation from bank 1. RDSERRIE1 can be programmed only when LOCK1 is cleared to 0.
pub type RDSERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SNECCERRIE` reader - Bank 1 ECC single correction error interrupt enable bit When SNECCERRIE1 bit is set to 1, an interrupt is generated when an ECC single correction error occurs during a read operation from bank 1. SNECCERRIE1 can be programmed only when LOCK1 is cleared to 0.
pub type SNECCERRIE_R = crate::BitReader;
///Field `SNECCERRIE` writer - Bank 1 ECC single correction error interrupt enable bit When SNECCERRIE1 bit is set to 1, an interrupt is generated when an ECC single correction error occurs during a read operation from bank 1. SNECCERRIE1 can be programmed only when LOCK1 is cleared to 0.
pub type SNECCERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBECCERRIE` reader - Bank 1 ECC double detection error interrupt enable bit When DBECCERRIE1 bit is set to 1, an interrupt is generated when an ECC double detection error occurs during a read operation from bank 1. DBECCERRIE1 can be programmed only when LOCK1 is cleared to 0.
pub type DBECCERRIE_R = crate::BitReader;
///Field `DBECCERRIE` writer - Bank 1 ECC double detection error interrupt enable bit When DBECCERRIE1 bit is set to 1, an interrupt is generated when an ECC double detection error occurs during a read operation from bank 1. DBECCERRIE1 can be programmed only when LOCK1 is cleared to 0.
pub type DBECCERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCENDIE` reader - Bank 1 CRC end of calculation interrupt enable bit When CRCENDIE1 bit is set to 1, an interrupt is generated when the CRC computation has completed on bank 1. CRCENDIE1 can be programmed only when LOCK1 is cleared to 0.
pub type CRCENDIE_R = crate::BitReader;
///Field `CRCENDIE` writer - Bank 1 CRC end of calculation interrupt enable bit When CRCENDIE1 bit is set to 1, an interrupt is generated when the CRC computation has completed on bank 1. CRCENDIE1 can be programmed only when LOCK1 is cleared to 0.
pub type CRCENDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCRDERRIE` reader - Bank 1 CRC read error interrupt enable bit When CRCRDERRIE1 bit is set to 1, an interrupt is generated when a protected area (PCROP or secure-only) has been detected during the last CRC computation on bank 1. CRCRDERRIE1 can be programmed only when LOCK1 is cleared to 0.
pub type CRCRDERRIE_R = crate::BitReader;
///Field `CRCRDERRIE` writer - Bank 1 CRC read error interrupt enable bit When CRCRDERRIE1 bit is set to 1, an interrupt is generated when a protected area (PCROP or secure-only) has been detected during the last CRC computation on bank 1. CRCRDERRIE1 can be programmed only when LOCK1 is cleared to 0.
pub type CRCRDERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Bank 1 configuration lock bit This bit locks the FLASH_CR1 register. The correct write sequence to FLASH_KEYR1 register unlocks this bit. If a wrong sequence is executed, or if the unlock sequence to FLASH_KEYR1 is performed twice, this bit remains locked until the next system reset. LOCK1 can be set by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When LOCK1 changes from 0 to 1, the other bits of FLASH_CR1 register do not change.
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Bank 1 internal buffer control bit Setting PG1 bit to 1 enables internal buffer for write operations to bank 1. This allows preparing program operations even if a sector or bank erase is ongoing. PG1 can be programmed only when LOCK1 is cleared to 0. When PG1 is reset, the internal buffer is disabled for write operations to bank 1, and all the data stored in the buffer but not sent to the operation queue are lost.
    #[inline(always)]
    pub fn pg(&self) -> PG_R {
        PG_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Bank 1 sector erase request Setting SER1 bit to 1 requests a sector erase on bank 1. SER1 can be programmed only when LOCK1 is cleared to 0. BER1 has a higher priority than SER1: if both bits are set, the embedded Flash memory executes a bank erase. Note: Write protection error is triggered when a sector erase is required on a protected sector.
    #[inline(always)]
    pub fn ser(&self) -> SER_R {
        SER_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Bank 1 erase request Setting BER1 bit to 1 requests a bank erase operation on bank 1 (user Flash memory only). BER1 can be programmed only when LOCK1 is cleared to 0. BER1 has a higher priority than SER1: if both are set, the embedded Flash memory executes a bank erase. Note: Write protection error is triggered when a bank erase is required and some sectors are protected.
    #[inline(always)]
    pub fn ber(&self) -> BER_R {
        BER_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Bank 1 write forcing control bit FW1 forces a write operation even if the write buffer is not full. In this case all bits not written are set to 1 by hardware. FW1 can be programmed only when LOCK1 is cleared to 0. The embedded Flash memory resets FW1 when the corresponding operation has been acknowledged. Note: Using a force-write operation prevents the application from updating later the missing bits with something else than 1, because it is likely that it will lead to permanent ECC error. Write forcing is effective only if the write buffer is not empty (in particular, FW1 does not start several write operations when the force-write operations are performed consecutively).
    #[inline(always)]
    pub fn fw(&self) -> FW_R {
        FW_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Bank 1 erase start control bit START1 bit is used to start a sector erase or a bank erase operation. START1 can be programmed only when LOCK1 is cleared to 0. The embedded Flash memory resets START1 when the corresponding operation has been acknowledged. The user application cannot access any embedded Flash memory register until the operation is acknowledged.
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:12 - Bank 1 sector erase selection number These bits are used to select the target sector for an erase operation (they are unused otherwise). SSN1 can be programmed only when LOCK1 is cleared to 0. .. ... ... Note: Bank 1 is limited to 16 and 64 sectors on STM32H7B0 and STM32H7A3xG devices, respectively.
    #[inline(always)]
    pub fn snb(&self) -> SNB_R {
        SNB_R::new(((self.bits >> 6) & 0x7f) as u8)
    }
    ///Bit 15 - Bank 1 CRC control bit Setting CRC_EN bit to 1 enables the CRC calculation on bank 1. CRC_EN does not start CRC calculation but enables CRC configuration through FLASH_CRCCR1 register. When CRC calculation is performed on bank 1, it can only be disabled by setting CRC_EN bit to 0. Resetting CRC_EN clears CRC configuration and resets the content of FLASH_CRCDATAR register. Clearing CRC_EN to 0 sets CRCDATA to 0x0. CRC_EN can be programmed only when LOCK1 is cleared to 0.
    #[inline(always)]
    pub fn crc_en(&self) -> CRC_EN_R {
        CRC_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Bank 1 end-of-program interrupt control bit Setting EOPIE1 bit to 1 enables the generation of an interrupt at the end of a program operation to bank 1. EOPIE1 can be programmed only when LOCK1 is cleared to 0.
    #[inline(always)]
    pub fn eopie(&self) -> EOPIE_R {
        EOPIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Bank 1 write protection error interrupt enable bit When WRPERRIE1 bit is set to 1, an interrupt is generated when a protection error occurs during a program operation to bank 1. WRPERRIE1 can be programmed only when LOCK1 is cleared to 0.
    #[inline(always)]
    pub fn wrperrie(&self) -> WRPERRIE_R {
        WRPERRIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Bank 1 programming sequence error interrupt enable bit When PGSERRIE1 bit is set to 1, an interrupt is generated when a sequence error occurs during a program operation to bank 1. PGSERRIE1 can be programmed only when LOCK1 is cleared to 0.
    #[inline(always)]
    pub fn pgserrie(&self) -> PGSERRIE_R {
        PGSERRIE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Bank 1 strobe error interrupt enable bit When STRBERRIE1 bit is set to 1, an interrupt is generated when a strobe error occurs (the master programs several times the same byte in the write buffer) during a write operation to bank 1. STRBERRIE1 can be programmed only when LOCK1 is cleared to 0.
    #[inline(always)]
    pub fn strberrie(&self) -> STRBERRIE_R {
        STRBERRIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 21 - Bank 1 inconsistency error interrupt enable bit When INCERRIE1 bit is set to 1, an interrupt is generated when an inconsistency error occurs during a write operation to bank 1. INCERRIE1 can be programmed only when LOCK1 is cleared to 0.
    #[inline(always)]
    pub fn incerrie(&self) -> INCERRIE_R {
        INCERRIE_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 23 - Bank 1 read protection error interrupt enable bit When RDPERRIE1 bit is set to 1, an interrupt is generated when a read protection error occurs (access to an address protected by PCROP or by RDP level 1) during a read operation from bank 1. RDPERRIE1 can be programmed only when LOCK1 is cleared to 0.
    #[inline(always)]
    pub fn rdperrie(&self) -> RDPERRIE_R {
        RDPERRIE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Bank 1 secure error interrupt enable bit When RDSERRIE1 bit is set to 1, an interrupt is generated when a secure error (access to a secure-only protected address) occurs during a read operation from bank 1. RDSERRIE1 can be programmed only when LOCK1 is cleared to 0.
    #[inline(always)]
    pub fn rdserrie(&self) -> RDSERRIE_R {
        RDSERRIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Bank 1 ECC single correction error interrupt enable bit When SNECCERRIE1 bit is set to 1, an interrupt is generated when an ECC single correction error occurs during a read operation from bank 1. SNECCERRIE1 can be programmed only when LOCK1 is cleared to 0.
    #[inline(always)]
    pub fn sneccerrie(&self) -> SNECCERRIE_R {
        SNECCERRIE_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Bank 1 ECC double detection error interrupt enable bit When DBECCERRIE1 bit is set to 1, an interrupt is generated when an ECC double detection error occurs during a read operation from bank 1. DBECCERRIE1 can be programmed only when LOCK1 is cleared to 0.
    #[inline(always)]
    pub fn dbeccerrie(&self) -> DBECCERRIE_R {
        DBECCERRIE_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Bank 1 CRC end of calculation interrupt enable bit When CRCENDIE1 bit is set to 1, an interrupt is generated when the CRC computation has completed on bank 1. CRCENDIE1 can be programmed only when LOCK1 is cleared to 0.
    #[inline(always)]
    pub fn crcendie(&self) -> CRCENDIE_R {
        CRCENDIE_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Bank 1 CRC read error interrupt enable bit When CRCRDERRIE1 bit is set to 1, an interrupt is generated when a protected area (PCROP or secure-only) has been detected during the last CRC computation on bank 1. CRCRDERRIE1 can be programmed only when LOCK1 is cleared to 0.
    #[inline(always)]
    pub fn crcrderrie(&self) -> CRCRDERRIE_R {
        CRCRDERRIE_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("lock", &self.lock())
            .field("pg", &self.pg())
            .field("ser", &self.ser())
            .field("ber", &self.ber())
            .field("fw", &self.fw())
            .field("start", &self.start())
            .field("snb", &self.snb())
            .field("crc_en", &self.crc_en())
            .field("eopie", &self.eopie())
            .field("wrperrie", &self.wrperrie())
            .field("pgserrie", &self.pgserrie())
            .field("strberrie", &self.strberrie())
            .field("incerrie", &self.incerrie())
            .field("rdperrie", &self.rdperrie())
            .field("rdserrie", &self.rdserrie())
            .field("sneccerrie", &self.sneccerrie())
            .field("dbeccerrie", &self.dbeccerrie())
            .field("crcendie", &self.crcendie())
            .field("crcrderrie", &self.crcrderrie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Bank 1 configuration lock bit This bit locks the FLASH_CR1 register. The correct write sequence to FLASH_KEYR1 register unlocks this bit. If a wrong sequence is executed, or if the unlock sequence to FLASH_KEYR1 is performed twice, this bit remains locked until the next system reset. LOCK1 can be set by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When LOCK1 changes from 0 to 1, the other bits of FLASH_CR1 register do not change.
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<'_, CRrs> {
        LOCK_W::new(self, 0)
    }
    ///Bit 1 - Bank 1 internal buffer control bit Setting PG1 bit to 1 enables internal buffer for write operations to bank 1. This allows preparing program operations even if a sector or bank erase is ongoing. PG1 can be programmed only when LOCK1 is cleared to 0. When PG1 is reset, the internal buffer is disabled for write operations to bank 1, and all the data stored in the buffer but not sent to the operation queue are lost.
    #[inline(always)]
    pub fn pg(&mut self) -> PG_W<'_, CRrs> {
        PG_W::new(self, 1)
    }
    ///Bit 2 - Bank 1 sector erase request Setting SER1 bit to 1 requests a sector erase on bank 1. SER1 can be programmed only when LOCK1 is cleared to 0. BER1 has a higher priority than SER1: if both bits are set, the embedded Flash memory executes a bank erase. Note: Write protection error is triggered when a sector erase is required on a protected sector.
    #[inline(always)]
    pub fn ser(&mut self) -> SER_W<'_, CRrs> {
        SER_W::new(self, 2)
    }
    ///Bit 3 - Bank 1 erase request Setting BER1 bit to 1 requests a bank erase operation on bank 1 (user Flash memory only). BER1 can be programmed only when LOCK1 is cleared to 0. BER1 has a higher priority than SER1: if both are set, the embedded Flash memory executes a bank erase. Note: Write protection error is triggered when a bank erase is required and some sectors are protected.
    #[inline(always)]
    pub fn ber(&mut self) -> BER_W<'_, CRrs> {
        BER_W::new(self, 3)
    }
    ///Bit 4 - Bank 1 write forcing control bit FW1 forces a write operation even if the write buffer is not full. In this case all bits not written are set to 1 by hardware. FW1 can be programmed only when LOCK1 is cleared to 0. The embedded Flash memory resets FW1 when the corresponding operation has been acknowledged. Note: Using a force-write operation prevents the application from updating later the missing bits with something else than 1, because it is likely that it will lead to permanent ECC error. Write forcing is effective only if the write buffer is not empty (in particular, FW1 does not start several write operations when the force-write operations are performed consecutively).
    #[inline(always)]
    pub fn fw(&mut self) -> FW_W<'_, CRrs> {
        FW_W::new(self, 4)
    }
    ///Bit 5 - Bank 1 erase start control bit START1 bit is used to start a sector erase or a bank erase operation. START1 can be programmed only when LOCK1 is cleared to 0. The embedded Flash memory resets START1 when the corresponding operation has been acknowledged. The user application cannot access any embedded Flash memory register until the operation is acknowledged.
    #[inline(always)]
    pub fn start(&mut self) -> START_W<'_, CRrs> {
        START_W::new(self, 5)
    }
    ///Bits 6:12 - Bank 1 sector erase selection number These bits are used to select the target sector for an erase operation (they are unused otherwise). SSN1 can be programmed only when LOCK1 is cleared to 0. .. ... ... Note: Bank 1 is limited to 16 and 64 sectors on STM32H7B0 and STM32H7A3xG devices, respectively.
    #[inline(always)]
    pub fn snb(&mut self) -> SNB_W<'_, CRrs> {
        SNB_W::new(self, 6)
    }
    ///Bit 15 - Bank 1 CRC control bit Setting CRC_EN bit to 1 enables the CRC calculation on bank 1. CRC_EN does not start CRC calculation but enables CRC configuration through FLASH_CRCCR1 register. When CRC calculation is performed on bank 1, it can only be disabled by setting CRC_EN bit to 0. Resetting CRC_EN clears CRC configuration and resets the content of FLASH_CRCDATAR register. Clearing CRC_EN to 0 sets CRCDATA to 0x0. CRC_EN can be programmed only when LOCK1 is cleared to 0.
    #[inline(always)]
    pub fn crc_en(&mut self) -> CRC_EN_W<'_, CRrs> {
        CRC_EN_W::new(self, 15)
    }
    ///Bit 16 - Bank 1 end-of-program interrupt control bit Setting EOPIE1 bit to 1 enables the generation of an interrupt at the end of a program operation to bank 1. EOPIE1 can be programmed only when LOCK1 is cleared to 0.
    #[inline(always)]
    pub fn eopie(&mut self) -> EOPIE_W<'_, CRrs> {
        EOPIE_W::new(self, 16)
    }
    ///Bit 17 - Bank 1 write protection error interrupt enable bit When WRPERRIE1 bit is set to 1, an interrupt is generated when a protection error occurs during a program operation to bank 1. WRPERRIE1 can be programmed only when LOCK1 is cleared to 0.
    #[inline(always)]
    pub fn wrperrie(&mut self) -> WRPERRIE_W<'_, CRrs> {
        WRPERRIE_W::new(self, 17)
    }
    ///Bit 18 - Bank 1 programming sequence error interrupt enable bit When PGSERRIE1 bit is set to 1, an interrupt is generated when a sequence error occurs during a program operation to bank 1. PGSERRIE1 can be programmed only when LOCK1 is cleared to 0.
    #[inline(always)]
    pub fn pgserrie(&mut self) -> PGSERRIE_W<'_, CRrs> {
        PGSERRIE_W::new(self, 18)
    }
    ///Bit 19 - Bank 1 strobe error interrupt enable bit When STRBERRIE1 bit is set to 1, an interrupt is generated when a strobe error occurs (the master programs several times the same byte in the write buffer) during a write operation to bank 1. STRBERRIE1 can be programmed only when LOCK1 is cleared to 0.
    #[inline(always)]
    pub fn strberrie(&mut self) -> STRBERRIE_W<'_, CRrs> {
        STRBERRIE_W::new(self, 19)
    }
    ///Bit 21 - Bank 1 inconsistency error interrupt enable bit When INCERRIE1 bit is set to 1, an interrupt is generated when an inconsistency error occurs during a write operation to bank 1. INCERRIE1 can be programmed only when LOCK1 is cleared to 0.
    #[inline(always)]
    pub fn incerrie(&mut self) -> INCERRIE_W<'_, CRrs> {
        INCERRIE_W::new(self, 21)
    }
    ///Bit 23 - Bank 1 read protection error interrupt enable bit When RDPERRIE1 bit is set to 1, an interrupt is generated when a read protection error occurs (access to an address protected by PCROP or by RDP level 1) during a read operation from bank 1. RDPERRIE1 can be programmed only when LOCK1 is cleared to 0.
    #[inline(always)]
    pub fn rdperrie(&mut self) -> RDPERRIE_W<'_, CRrs> {
        RDPERRIE_W::new(self, 23)
    }
    ///Bit 24 - Bank 1 secure error interrupt enable bit When RDSERRIE1 bit is set to 1, an interrupt is generated when a secure error (access to a secure-only protected address) occurs during a read operation from bank 1. RDSERRIE1 can be programmed only when LOCK1 is cleared to 0.
    #[inline(always)]
    pub fn rdserrie(&mut self) -> RDSERRIE_W<'_, CRrs> {
        RDSERRIE_W::new(self, 24)
    }
    ///Bit 25 - Bank 1 ECC single correction error interrupt enable bit When SNECCERRIE1 bit is set to 1, an interrupt is generated when an ECC single correction error occurs during a read operation from bank 1. SNECCERRIE1 can be programmed only when LOCK1 is cleared to 0.
    #[inline(always)]
    pub fn sneccerrie(&mut self) -> SNECCERRIE_W<'_, CRrs> {
        SNECCERRIE_W::new(self, 25)
    }
    ///Bit 26 - Bank 1 ECC double detection error interrupt enable bit When DBECCERRIE1 bit is set to 1, an interrupt is generated when an ECC double detection error occurs during a read operation from bank 1. DBECCERRIE1 can be programmed only when LOCK1 is cleared to 0.
    #[inline(always)]
    pub fn dbeccerrie(&mut self) -> DBECCERRIE_W<'_, CRrs> {
        DBECCERRIE_W::new(self, 26)
    }
    ///Bit 27 - Bank 1 CRC end of calculation interrupt enable bit When CRCENDIE1 bit is set to 1, an interrupt is generated when the CRC computation has completed on bank 1. CRCENDIE1 can be programmed only when LOCK1 is cleared to 0.
    #[inline(always)]
    pub fn crcendie(&mut self) -> CRCENDIE_W<'_, CRrs> {
        CRCENDIE_W::new(self, 27)
    }
    ///Bit 28 - Bank 1 CRC read error interrupt enable bit When CRCRDERRIE1 bit is set to 1, an interrupt is generated when a protected area (PCROP or secure-only) has been detected during the last CRC computation on bank 1. CRCRDERRIE1 can be programmed only when LOCK1 is cleared to 0.
    #[inline(always)]
    pub fn crcrderrie(&mut self) -> CRCRDERRIE_W<'_, CRrs> {
        CRCRDERRIE_W::new(self, 28)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
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
///`reset()` method sets CR to value 0x01
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0x01;
}

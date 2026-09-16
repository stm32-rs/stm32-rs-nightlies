///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `LOCK` reader - Configuration lock bit When this bit is set write to all other bits in this register, and to FLASH_IER register, are ignored. Clearing this bit requires the correct write sequence to FLASH_KEYR register (see this register for details). If a wrong sequence is executed, or if the unlock sequence is performed twice, this bit remains locked until the next system reset. During the write access to set LOCK bit from 0 to 1, it is possible to change the other bits of this register.
pub type LOCK_R = crate::BitReader;
///Field `LOCK` writer - Configuration lock bit When this bit is set write to all other bits in this register, and to FLASH_IER register, are ignored. Clearing this bit requires the correct write sequence to FLASH_KEYR register (see this register for details). If a wrong sequence is executed, or if the unlock sequence is performed twice, this bit remains locked until the next system reset. During the write access to set LOCK bit from 0 to 1, it is possible to change the other bits of this register.
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PG` reader - Internal buffer control bit Setting this bit enables internal buffer for write operations. This allows preparing program operations even if a sector or bank erase is ongoing. When PG is cleared, the internal buffer is disabled for write operations, and all the data stored in the buffer but not sent to the operation queue are lost.
pub type PG_R = crate::BitReader;
///Field `PG` writer - Internal buffer control bit Setting this bit enables internal buffer for write operations. This allows preparing program operations even if a sector or bank erase is ongoing. When PG is cleared, the internal buffer is disabled for write operations, and all the data stored in the buffer but not sent to the operation queue are lost.
pub type PG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SER` reader - Sector erase request Setting this bit requests a sector erase. Write protection error is triggered when a sector erase is required on at least one protected sector. BER has a higher priority than SER: if both bits are set, the embedded Flash memory executes a bank erase.
pub type SER_R = crate::BitReader;
///Field `SER` writer - Sector erase request Setting this bit requests a sector erase. Write protection error is triggered when a sector erase is required on at least one protected sector. BER has a higher priority than SER: if both bits are set, the embedded Flash memory executes a bank erase.
pub type SER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BER` reader - Bank erase request Setting this bit requests a bank erase operation (user Flash memory only). Write protection error is triggered when a bank erase is required and some sectors are protected. BER has a higher priority than SER: if both are set, the embedded Flash memory executes a bank erase.
pub type BER_R = crate::BitReader;
///Field `BER` writer - Bank erase request Setting this bit requests a bank erase operation (user Flash memory only). Write protection error is triggered when a bank erase is required and some sectors are protected. BER has a higher priority than SER: if both are set, the embedded Flash memory executes a bank erase.
pub type BER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FW` reader - Force write This bit forces a write operation even if the write buffer is not full. In this case all bits not written are set by hardware. The embedded Flash memory resets FW when the corresponding operation has been acknowledged. Note: Using a force-write operation prevents the application from updating later the missing bits with something else than 1, because it is likely that it will lead to permanent ECC error. Write forcing is effective only if the write buffer is not empty (in particular, FW does not start several write operations when the force-write operations are performed consecutively).
pub type FW_R = crate::BitReader;
///Field `FW` writer - Force write This bit forces a write operation even if the write buffer is not full. In this case all bits not written are set by hardware. The embedded Flash memory resets FW when the corresponding operation has been acknowledged. Note: Using a force-write operation prevents the application from updating later the missing bits with something else than 1, because it is likely that it will lead to permanent ECC error. Write forcing is effective only if the write buffer is not empty (in particular, FW does not start several write operations when the force-write operations are performed consecutively).
pub type FW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `START` reader - Erase start control bit This bit is used to start a sector erase or a bank erase operation. The embedded Flash memory resets START when the corresponding operation has been acknowledged. The user application cannot access any embedded Flash memory register until the operation is acknowledged.
pub type START_R = crate::BitReader;
///Field `START` writer - Erase start control bit This bit is used to start a sector erase or a bank erase operation. The embedded Flash memory resets START when the corresponding operation has been acknowledged. The user application cannot access any embedded Flash memory register until the operation is acknowledged.
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SSN` reader - Sector erase selection number These bits are used to select the target sector for an erase operation (they are unused otherwise). ...
pub type SSN_R = crate::FieldReader;
///Field `SSN` writer - Sector erase selection number These bits are used to select the target sector for an erase operation (they are unused otherwise). ...
pub type SSN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PG_OTP` reader - Program Enable for OTP Area Set this bit to enable write operations to OTP area.
pub type PG_OTP_R = crate::BitReader;
///Field `PG_OTP` writer - Program Enable for OTP Area Set this bit to enable write operations to OTP area.
pub type PG_OTP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRC_EN` reader - CRC enable Setting this bit enables the CRC calculation. CRC_EN does not start CRC calculation but enables CRC configuration through FLASH_CRCCR register. When CRC calculation is performed it can be disabled by clearing CRC_EN bit. Doing so sets CRCDATA to 0x0, clears CRC configuration and resets the content of FLASH_CRCDATAR register.
pub type CRC_EN_R = crate::BitReader;
///Field `CRC_EN` writer - CRC enable Setting this bit enables the CRC calculation. CRC_EN does not start CRC calculation but enables CRC configuration through FLASH_CRCCR register. When CRC calculation is performed it can be disabled by clearing CRC_EN bit. Doing so sets CRCDATA to 0x0, clears CRC configuration and resets the content of FLASH_CRCDATAR register.
pub type CRC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALL_BANKS` reader - All banks select bit When this bit is set the erase is done on all Flash Memory sectors. ALL_BANKS is used only if a bank erase is required (BER=1). In all others operations, this control bit is ignored.
pub type ALL_BANKS_R = crate::BitReader;
///Field `ALL_BANKS` writer - All banks select bit When this bit is set the erase is done on all Flash Memory sectors. ALL_BANKS is used only if a bank erase is required (BER=1). In all others operations, this control bit is ignored.
pub type ALL_BANKS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Configuration lock bit When this bit is set write to all other bits in this register, and to FLASH_IER register, are ignored. Clearing this bit requires the correct write sequence to FLASH_KEYR register (see this register for details). If a wrong sequence is executed, or if the unlock sequence is performed twice, this bit remains locked until the next system reset. During the write access to set LOCK bit from 0 to 1, it is possible to change the other bits of this register.
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Internal buffer control bit Setting this bit enables internal buffer for write operations. This allows preparing program operations even if a sector or bank erase is ongoing. When PG is cleared, the internal buffer is disabled for write operations, and all the data stored in the buffer but not sent to the operation queue are lost.
    #[inline(always)]
    pub fn pg(&self) -> PG_R {
        PG_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Sector erase request Setting this bit requests a sector erase. Write protection error is triggered when a sector erase is required on at least one protected sector. BER has a higher priority than SER: if both bits are set, the embedded Flash memory executes a bank erase.
    #[inline(always)]
    pub fn ser(&self) -> SER_R {
        SER_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Bank erase request Setting this bit requests a bank erase operation (user Flash memory only). Write protection error is triggered when a bank erase is required and some sectors are protected. BER has a higher priority than SER: if both are set, the embedded Flash memory executes a bank erase.
    #[inline(always)]
    pub fn ber(&self) -> BER_R {
        BER_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Force write This bit forces a write operation even if the write buffer is not full. In this case all bits not written are set by hardware. The embedded Flash memory resets FW when the corresponding operation has been acknowledged. Note: Using a force-write operation prevents the application from updating later the missing bits with something else than 1, because it is likely that it will lead to permanent ECC error. Write forcing is effective only if the write buffer is not empty (in particular, FW does not start several write operations when the force-write operations are performed consecutively).
    #[inline(always)]
    pub fn fw(&self) -> FW_R {
        FW_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Erase start control bit This bit is used to start a sector erase or a bank erase operation. The embedded Flash memory resets START when the corresponding operation has been acknowledged. The user application cannot access any embedded Flash memory register until the operation is acknowledged.
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:7 - Sector erase selection number These bits are used to select the target sector for an erase operation (they are unused otherwise). ...
    #[inline(always)]
    pub fn ssn(&self) -> SSN_R {
        SSN_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bit 16 - Program Enable for OTP Area Set this bit to enable write operations to OTP area.
    #[inline(always)]
    pub fn pg_otp(&self) -> PG_OTP_R {
        PG_OTP_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - CRC enable Setting this bit enables the CRC calculation. CRC_EN does not start CRC calculation but enables CRC configuration through FLASH_CRCCR register. When CRC calculation is performed it can be disabled by clearing CRC_EN bit. Doing so sets CRCDATA to 0x0, clears CRC configuration and resets the content of FLASH_CRCDATAR register.
    #[inline(always)]
    pub fn crc_en(&self) -> CRC_EN_R {
        CRC_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 24 - All banks select bit When this bit is set the erase is done on all Flash Memory sectors. ALL_BANKS is used only if a bank erase is required (BER=1). In all others operations, this control bit is ignored.
    #[inline(always)]
    pub fn all_banks(&self) -> ALL_BANKS_R {
        ALL_BANKS_R::new(((self.bits >> 24) & 1) != 0)
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
            .field("ssn", &self.ssn())
            .field("pg_otp", &self.pg_otp())
            .field("crc_en", &self.crc_en())
            .field("all_banks", &self.all_banks())
            .finish()
    }
}
impl W {
    ///Bit 0 - Configuration lock bit When this bit is set write to all other bits in this register, and to FLASH_IER register, are ignored. Clearing this bit requires the correct write sequence to FLASH_KEYR register (see this register for details). If a wrong sequence is executed, or if the unlock sequence is performed twice, this bit remains locked until the next system reset. During the write access to set LOCK bit from 0 to 1, it is possible to change the other bits of this register.
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<'_, CRrs> {
        LOCK_W::new(self, 0)
    }
    ///Bit 1 - Internal buffer control bit Setting this bit enables internal buffer for write operations. This allows preparing program operations even if a sector or bank erase is ongoing. When PG is cleared, the internal buffer is disabled for write operations, and all the data stored in the buffer but not sent to the operation queue are lost.
    #[inline(always)]
    pub fn pg(&mut self) -> PG_W<'_, CRrs> {
        PG_W::new(self, 1)
    }
    ///Bit 2 - Sector erase request Setting this bit requests a sector erase. Write protection error is triggered when a sector erase is required on at least one protected sector. BER has a higher priority than SER: if both bits are set, the embedded Flash memory executes a bank erase.
    #[inline(always)]
    pub fn ser(&mut self) -> SER_W<'_, CRrs> {
        SER_W::new(self, 2)
    }
    ///Bit 3 - Bank erase request Setting this bit requests a bank erase operation (user Flash memory only). Write protection error is triggered when a bank erase is required and some sectors are protected. BER has a higher priority than SER: if both are set, the embedded Flash memory executes a bank erase.
    #[inline(always)]
    pub fn ber(&mut self) -> BER_W<'_, CRrs> {
        BER_W::new(self, 3)
    }
    ///Bit 4 - Force write This bit forces a write operation even if the write buffer is not full. In this case all bits not written are set by hardware. The embedded Flash memory resets FW when the corresponding operation has been acknowledged. Note: Using a force-write operation prevents the application from updating later the missing bits with something else than 1, because it is likely that it will lead to permanent ECC error. Write forcing is effective only if the write buffer is not empty (in particular, FW does not start several write operations when the force-write operations are performed consecutively).
    #[inline(always)]
    pub fn fw(&mut self) -> FW_W<'_, CRrs> {
        FW_W::new(self, 4)
    }
    ///Bit 5 - Erase start control bit This bit is used to start a sector erase or a bank erase operation. The embedded Flash memory resets START when the corresponding operation has been acknowledged. The user application cannot access any embedded Flash memory register until the operation is acknowledged.
    #[inline(always)]
    pub fn start(&mut self) -> START_W<'_, CRrs> {
        START_W::new(self, 5)
    }
    ///Bits 6:7 - Sector erase selection number These bits are used to select the target sector for an erase operation (they are unused otherwise). ...
    #[inline(always)]
    pub fn ssn(&mut self) -> SSN_W<'_, CRrs> {
        SSN_W::new(self, 6)
    }
    ///Bit 16 - Program Enable for OTP Area Set this bit to enable write operations to OTP area.
    #[inline(always)]
    pub fn pg_otp(&mut self) -> PG_OTP_W<'_, CRrs> {
        PG_OTP_W::new(self, 16)
    }
    ///Bit 17 - CRC enable Setting this bit enables the CRC calculation. CRC_EN does not start CRC calculation but enables CRC configuration through FLASH_CRCCR register. When CRC calculation is performed it can be disabled by clearing CRC_EN bit. Doing so sets CRCDATA to 0x0, clears CRC configuration and resets the content of FLASH_CRCDATAR register.
    #[inline(always)]
    pub fn crc_en(&mut self) -> CRC_EN_W<'_, CRrs> {
        CRC_EN_W::new(self, 17)
    }
    ///Bit 24 - All banks select bit When this bit is set the erase is done on all Flash Memory sectors. ALL_BANKS is used only if a bank erase is required (BER=1). In all others operations, this control bit is ignored.
    #[inline(always)]
    pub fn all_banks(&mut self) -> ALL_BANKS_W<'_, CRrs> {
        ALL_BANKS_W::new(self, 24)
    }
}
/**FLASH control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FLASH:CR)*/
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

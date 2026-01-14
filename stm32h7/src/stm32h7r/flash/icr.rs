///Register `ICR` writer
pub type W = crate::W<ICRrs>;
///Field `EOPF` writer - End-of-program flag clear Setting this bit clears EOPF flag in FLASH_ISR register.
pub type EOPF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WRPERRF` writer - Write protection error flag clear Setting this bit clears WRPERRF flag in FLASH_ISR register.
pub type WRPERRF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PGSERRF` writer - Programming sequence error flag clear Setting this bit clears PGSERRF flag in FLASH_ISR register.
pub type PGSERRF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STRBERRF` writer - Strobe error flag clear Setting this bit clears STRBERRF flag in FLASH_ISR register.
pub type STRBERRF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OBLERRF` writer - Option byte loading error flag clear Setting this bit clears OBLERRF flag in FLASH_ISR register.
pub type OBLERRF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INCERRF` writer - Inconsistency error flag clear Setting this bit clears INCERRF flag in FLASH_ISR register.
pub type INCERRF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RDSERRF` writer - Read security error flag clear Setting this bit clears RDSERRF flag in FLASH_ISR register.
pub type RDSERRF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SNECCERRF` writer - ECC single error flag clear Setting this bit clears SNECCERRF flag in FLASH_ISR register. If the DBECCERRF flag of FLASH_ISR register is also cleared, FLASH_ECCFAR register is reset to zero as well.
pub type SNECCERRF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBECCERRF` writer - ECC double error flag clear Setting this bit clears DBECCERRF flag in FLASH_ISR register. If the SNECCERRF flag of FLASH_ISR register is also cleared, FLASH_ECCFAR register is reset to zero as well.
pub type DBECCERRF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCENDF` writer - CRC end flag clear Setting this bit clears CRCENDF flag in FLASH_ISR register.
pub type CRCENDF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCRDERRF` writer - CRC error flag clear Setting this bit clears CRCRDERRF flag in FLASH_ISR register.
pub type CRCRDERRF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<ICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 16 - End-of-program flag clear Setting this bit clears EOPF flag in FLASH_ISR register.
    #[inline(always)]
    pub fn eopf(&mut self) -> EOPF_W<'_, ICRrs> {
        EOPF_W::new(self, 16)
    }
    ///Bit 17 - Write protection error flag clear Setting this bit clears WRPERRF flag in FLASH_ISR register.
    #[inline(always)]
    pub fn wrperrf(&mut self) -> WRPERRF_W<'_, ICRrs> {
        WRPERRF_W::new(self, 17)
    }
    ///Bit 18 - Programming sequence error flag clear Setting this bit clears PGSERRF flag in FLASH_ISR register.
    #[inline(always)]
    pub fn pgserrf(&mut self) -> PGSERRF_W<'_, ICRrs> {
        PGSERRF_W::new(self, 18)
    }
    ///Bit 19 - Strobe error flag clear Setting this bit clears STRBERRF flag in FLASH_ISR register.
    #[inline(always)]
    pub fn strberrf(&mut self) -> STRBERRF_W<'_, ICRrs> {
        STRBERRF_W::new(self, 19)
    }
    ///Bit 20 - Option byte loading error flag clear Setting this bit clears OBLERRF flag in FLASH_ISR register.
    #[inline(always)]
    pub fn oblerrf(&mut self) -> OBLERRF_W<'_, ICRrs> {
        OBLERRF_W::new(self, 20)
    }
    ///Bit 21 - Inconsistency error flag clear Setting this bit clears INCERRF flag in FLASH_ISR register.
    #[inline(always)]
    pub fn incerrf(&mut self) -> INCERRF_W<'_, ICRrs> {
        INCERRF_W::new(self, 21)
    }
    ///Bit 24 - Read security error flag clear Setting this bit clears RDSERRF flag in FLASH_ISR register.
    #[inline(always)]
    pub fn rdserrf(&mut self) -> RDSERRF_W<'_, ICRrs> {
        RDSERRF_W::new(self, 24)
    }
    ///Bit 25 - ECC single error flag clear Setting this bit clears SNECCERRF flag in FLASH_ISR register. If the DBECCERRF flag of FLASH_ISR register is also cleared, FLASH_ECCFAR register is reset to zero as well.
    #[inline(always)]
    pub fn sneccerrf(&mut self) -> SNECCERRF_W<'_, ICRrs> {
        SNECCERRF_W::new(self, 25)
    }
    ///Bit 26 - ECC double error flag clear Setting this bit clears DBECCERRF flag in FLASH_ISR register. If the SNECCERRF flag of FLASH_ISR register is also cleared, FLASH_ECCFAR register is reset to zero as well.
    #[inline(always)]
    pub fn dbeccerrf(&mut self) -> DBECCERRF_W<'_, ICRrs> {
        DBECCERRF_W::new(self, 26)
    }
    ///Bit 27 - CRC end flag clear Setting this bit clears CRCENDF flag in FLASH_ISR register.
    #[inline(always)]
    pub fn crcendf(&mut self) -> CRCENDF_W<'_, ICRrs> {
        CRCENDF_W::new(self, 27)
    }
    ///Bit 28 - CRC error flag clear Setting this bit clears CRCRDERRF flag in FLASH_ISR register.
    #[inline(always)]
    pub fn crcrderrf(&mut self) -> CRCRDERRF_W<'_, ICRrs> {
        CRCRDERRF_W::new(self, 28)
    }
}
/**FLASH interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FLASH:ICR)*/
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`icr::W`](W) writer structure
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICR to value 0
impl crate::Resettable for ICRrs {}

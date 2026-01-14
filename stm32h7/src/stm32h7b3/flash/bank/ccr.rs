///Register `CCR` writer
pub type W = crate::W<CCRrs>;
///Field `CLR_EOP` writer - Bank 1 EOP1 flag clear bit Setting this bit to 1 resets to 0 EOP1 flag in FLASH_SR1 register.
pub type CLR_EOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLR_WRPERR` writer - Bank 1 WRPERR1 flag clear bit Setting this bit to 1 resets to 0 WRPERR1 flag in FLASH_SR1 register.
pub type CLR_WRPERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLR_PGSERR` writer - Bank 1 PGSERR1 flag clear bit Setting this bit to 1 resets to 0 PGSERR1 flag in FLASH_SR1 register.
pub type CLR_PGSERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLR_STRBERR` writer - Bank 1 STRBERR1 flag clear bit Setting this bit to 1 resets to 0 STRBERR1 flag in FLASH_SR1 register.
pub type CLR_STRBERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLR_INCERR` writer - Bank 1 INCERR1 flag clear bit Setting this bit to 1 resets to 0 INCERR1 flag in FLASH_SR1 register.
pub type CLR_INCERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLR_RDPERR` writer - Bank 1 RDPERR1 flag clear bit Setting this bit to 1 resets to 0 RDPERR1 flag in FLASH_SR1 register.
pub type CLR_RDPERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLR_RDSERR` writer - Bank 1 RDSERR1 flag clear bit Setting this bit to 1 resets to 0 RDSERR1 flag in FLASH_SR1 register.
pub type CLR_RDSERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLR_SNECCERR` writer - Bank 1 SNECCERR1 flag clear bit Setting this bit to 1 resets to 0 SNECCERR1 flag in FLASH_SR1 register. If the DBECCERR1 flag of FLASH_SR1 register is cleared to 0, FLASH_ECC_FA1R register is reset to 0 as well.
pub type CLR_SNECCERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLR_DBECCERR` writer - Bank 1 DBECCERR1 flag clear bit Setting this bit to 1 resets to 0 DBECCERR1 flag in FLASH_SR1 register. If the SNECCERR1 flag of FLASH_SR1 register is cleared to 0, FLASH_ECC_FA1R register is reset to 0 as well.
pub type CLR_DBECCERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLR_CRCEND` writer - Bank 1 CRCEND1 flag clear bit Setting this bit to 1 resets to 0 CRCEND1 flag in FLASH_SR1 register.
pub type CLR_CRCEND_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLR_CRCRDERR` writer - Bank 1 CRCRDERR1 flag clear bit Setting this bit to 1 resets to 0 CRCRDERR1 flag in FLASH_SR1 register.
pub type CLR_CRCRDERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<CCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 16 - Bank 1 EOP1 flag clear bit Setting this bit to 1 resets to 0 EOP1 flag in FLASH_SR1 register.
    #[inline(always)]
    pub fn clr_eop(&mut self) -> CLR_EOP_W<'_, CCRrs> {
        CLR_EOP_W::new(self, 16)
    }
    ///Bit 17 - Bank 1 WRPERR1 flag clear bit Setting this bit to 1 resets to 0 WRPERR1 flag in FLASH_SR1 register.
    #[inline(always)]
    pub fn clr_wrperr(&mut self) -> CLR_WRPERR_W<'_, CCRrs> {
        CLR_WRPERR_W::new(self, 17)
    }
    ///Bit 18 - Bank 1 PGSERR1 flag clear bit Setting this bit to 1 resets to 0 PGSERR1 flag in FLASH_SR1 register.
    #[inline(always)]
    pub fn clr_pgserr(&mut self) -> CLR_PGSERR_W<'_, CCRrs> {
        CLR_PGSERR_W::new(self, 18)
    }
    ///Bit 19 - Bank 1 STRBERR1 flag clear bit Setting this bit to 1 resets to 0 STRBERR1 flag in FLASH_SR1 register.
    #[inline(always)]
    pub fn clr_strberr(&mut self) -> CLR_STRBERR_W<'_, CCRrs> {
        CLR_STRBERR_W::new(self, 19)
    }
    ///Bit 21 - Bank 1 INCERR1 flag clear bit Setting this bit to 1 resets to 0 INCERR1 flag in FLASH_SR1 register.
    #[inline(always)]
    pub fn clr_incerr(&mut self) -> CLR_INCERR_W<'_, CCRrs> {
        CLR_INCERR_W::new(self, 21)
    }
    ///Bit 23 - Bank 1 RDPERR1 flag clear bit Setting this bit to 1 resets to 0 RDPERR1 flag in FLASH_SR1 register.
    #[inline(always)]
    pub fn clr_rdperr(&mut self) -> CLR_RDPERR_W<'_, CCRrs> {
        CLR_RDPERR_W::new(self, 23)
    }
    ///Bit 24 - Bank 1 RDSERR1 flag clear bit Setting this bit to 1 resets to 0 RDSERR1 flag in FLASH_SR1 register.
    #[inline(always)]
    pub fn clr_rdserr(&mut self) -> CLR_RDSERR_W<'_, CCRrs> {
        CLR_RDSERR_W::new(self, 24)
    }
    ///Bit 25 - Bank 1 SNECCERR1 flag clear bit Setting this bit to 1 resets to 0 SNECCERR1 flag in FLASH_SR1 register. If the DBECCERR1 flag of FLASH_SR1 register is cleared to 0, FLASH_ECC_FA1R register is reset to 0 as well.
    #[inline(always)]
    pub fn clr_sneccerr(&mut self) -> CLR_SNECCERR_W<'_, CCRrs> {
        CLR_SNECCERR_W::new(self, 25)
    }
    ///Bit 26 - Bank 1 DBECCERR1 flag clear bit Setting this bit to 1 resets to 0 DBECCERR1 flag in FLASH_SR1 register. If the SNECCERR1 flag of FLASH_SR1 register is cleared to 0, FLASH_ECC_FA1R register is reset to 0 as well.
    #[inline(always)]
    pub fn clr_dbeccerr(&mut self) -> CLR_DBECCERR_W<'_, CCRrs> {
        CLR_DBECCERR_W::new(self, 26)
    }
    ///Bit 27 - Bank 1 CRCEND1 flag clear bit Setting this bit to 1 resets to 0 CRCEND1 flag in FLASH_SR1 register.
    #[inline(always)]
    pub fn clr_crcend(&mut self) -> CLR_CRCEND_W<'_, CCRrs> {
        CLR_CRCEND_W::new(self, 27)
    }
    ///Bit 28 - Bank 1 CRCRDERR1 flag clear bit Setting this bit to 1 resets to 0 CRCRDERR1 flag in FLASH_SR1 register.
    #[inline(always)]
    pub fn clr_crcrderr(&mut self) -> CLR_CRCRDERR_W<'_, CCRrs> {
        CLR_CRCRDERR_W::new(self, 28)
    }
}
/**

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CCRrs;
impl crate::RegisterSpec for CCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ccr::W`](W) writer structure
impl crate::Writable for CCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCR to value 0
impl crate::Resettable for CCRrs {}

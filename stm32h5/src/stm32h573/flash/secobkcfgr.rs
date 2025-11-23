///Register `SECOBKCFGR` reader
pub type R = crate::R<SECOBKCFGRrs>;
///Register `SECOBKCFGR` writer
pub type W = crate::W<SECOBKCFGRrs>;
///Field `LOCK` reader - OBKCFGR lock option configuration bit This bit locks the FLASH_OBKCFGR register. The correct write sequence to FLASH_SECOBKKEYR register unlocks this bit. If a wrong sequence is executed, or if the unlock sequence to FLASH_SECOBKKEYR is performed twice, this bit remains locked until the next system reset. LOCK can be set by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When LOCK changes from 0 to 1, the other bits of FLASH_NSCR register do not change.
pub type LOCK_R = crate::BitReader;
///Field `LOCK` writer - OBKCFGR lock option configuration bit This bit locks the FLASH_OBKCFGR register. The correct write sequence to FLASH_SECOBKKEYR register unlocks this bit. If a wrong sequence is executed, or if the unlock sequence to FLASH_SECOBKKEYR is performed twice, this bit remains locked until the next system reset. LOCK can be set by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When LOCK changes from 0 to 1, the other bits of FLASH_NSCR register do not change.
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWAP_SECT_REQ` reader - OBK swap sector request bit When set, all the OBKs which have not been updated in the alternate sector is copied from current sector to alternate one. The SWAP_OFFSET value must be a certain minimum value in order for the swap to be launched in OBK-HDPLdifferent 0. Minimum value is 16 for OBK-HDPL=1, 144 for OBK-HDPL=2 and 192 for OBK-HDPL=3.
pub type SWAP_SECT_REQ_R = crate::BitReader;
///Field `SWAP_SECT_REQ` writer - OBK swap sector request bit When set, all the OBKs which have not been updated in the alternate sector is copied from current sector to alternate one. The SWAP_OFFSET value must be a certain minimum value in order for the swap to be launched in OBK-HDPLdifferent 0. Minimum value is 16 for OBK-HDPL=1, 144 for OBK-HDPL=2 and 192 for OBK-HDPL=3.
pub type SWAP_SECT_REQ_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALT_SECT` reader - alternate sector bit This bit must not change while filling the write buffer, otherwise an error is generated
pub type ALT_SECT_R = crate::BitReader;
///Field `ALT_SECT` writer - alternate sector bit This bit must not change while filling the write buffer, otherwise an error is generated
pub type ALT_SECT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALT_SECT_ERASE` reader - alternate sector erase bit When ALT_SECT bit is set, use this bit to generate an erase command for the OBK alternate sector. It is set only by Software and cleared when the OBK swap operation is completed or an error occurs (PGSERR). It is reseted at the same time as the BUSY bit.
pub type ALT_SECT_ERASE_R = crate::BitReader;
///Field `ALT_SECT_ERASE` writer - alternate sector erase bit When ALT_SECT bit is set, use this bit to generate an erase command for the OBK alternate sector. It is set only by Software and cleared when the OBK swap operation is completed or an error occurs (PGSERR). It is reseted at the same time as the BUSY bit.
pub type ALT_SECT_ERASE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWAP_OFFSET` reader - key index (offset /16 bits) pointing for next swap.
pub type SWAP_OFFSET_R = crate::FieldReader<u16>;
///Field `SWAP_OFFSET` writer - key index (offset /16 bits) pointing for next swap.
pub type SWAP_OFFSET_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bit 0 - OBKCFGR lock option configuration bit This bit locks the FLASH_OBKCFGR register. The correct write sequence to FLASH_SECOBKKEYR register unlocks this bit. If a wrong sequence is executed, or if the unlock sequence to FLASH_SECOBKKEYR is performed twice, this bit remains locked until the next system reset. LOCK can be set by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When LOCK changes from 0 to 1, the other bits of FLASH_NSCR register do not change.
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - OBK swap sector request bit When set, all the OBKs which have not been updated in the alternate sector is copied from current sector to alternate one. The SWAP_OFFSET value must be a certain minimum value in order for the swap to be launched in OBK-HDPLdifferent 0. Minimum value is 16 for OBK-HDPL=1, 144 for OBK-HDPL=2 and 192 for OBK-HDPL=3.
    #[inline(always)]
    pub fn swap_sect_req(&self) -> SWAP_SECT_REQ_R {
        SWAP_SECT_REQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - alternate sector bit This bit must not change while filling the write buffer, otherwise an error is generated
    #[inline(always)]
    pub fn alt_sect(&self) -> ALT_SECT_R {
        ALT_SECT_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - alternate sector erase bit When ALT_SECT bit is set, use this bit to generate an erase command for the OBK alternate sector. It is set only by Software and cleared when the OBK swap operation is completed or an error occurs (PGSERR). It is reseted at the same time as the BUSY bit.
    #[inline(always)]
    pub fn alt_sect_erase(&self) -> ALT_SECT_ERASE_R {
        ALT_SECT_ERASE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 16:24 - key index (offset /16 bits) pointing for next swap.
    #[inline(always)]
    pub fn swap_offset(&self) -> SWAP_OFFSET_R {
        SWAP_OFFSET_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECOBKCFGR")
            .field("lock", &self.lock())
            .field("swap_sect_req", &self.swap_sect_req())
            .field("alt_sect", &self.alt_sect())
            .field("alt_sect_erase", &self.alt_sect_erase())
            .field("swap_offset", &self.swap_offset())
            .finish()
    }
}
impl W {
    ///Bit 0 - OBKCFGR lock option configuration bit This bit locks the FLASH_OBKCFGR register. The correct write sequence to FLASH_SECOBKKEYR register unlocks this bit. If a wrong sequence is executed, or if the unlock sequence to FLASH_SECOBKKEYR is performed twice, this bit remains locked until the next system reset. LOCK can be set by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When LOCK changes from 0 to 1, the other bits of FLASH_NSCR register do not change.
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<'_, SECOBKCFGRrs> {
        LOCK_W::new(self, 0)
    }
    ///Bit 1 - OBK swap sector request bit When set, all the OBKs which have not been updated in the alternate sector is copied from current sector to alternate one. The SWAP_OFFSET value must be a certain minimum value in order for the swap to be launched in OBK-HDPLdifferent 0. Minimum value is 16 for OBK-HDPL=1, 144 for OBK-HDPL=2 and 192 for OBK-HDPL=3.
    #[inline(always)]
    pub fn swap_sect_req(&mut self) -> SWAP_SECT_REQ_W<'_, SECOBKCFGRrs> {
        SWAP_SECT_REQ_W::new(self, 1)
    }
    ///Bit 2 - alternate sector bit This bit must not change while filling the write buffer, otherwise an error is generated
    #[inline(always)]
    pub fn alt_sect(&mut self) -> ALT_SECT_W<'_, SECOBKCFGRrs> {
        ALT_SECT_W::new(self, 2)
    }
    ///Bit 3 - alternate sector erase bit When ALT_SECT bit is set, use this bit to generate an erase command for the OBK alternate sector. It is set only by Software and cleared when the OBK swap operation is completed or an error occurs (PGSERR). It is reseted at the same time as the BUSY bit.
    #[inline(always)]
    pub fn alt_sect_erase(&mut self) -> ALT_SECT_ERASE_W<'_, SECOBKCFGRrs> {
        ALT_SECT_ERASE_W::new(self, 3)
    }
    ///Bits 16:24 - key index (offset /16 bits) pointing for next swap.
    #[inline(always)]
    pub fn swap_offset(&mut self) -> SWAP_OFFSET_W<'_, SECOBKCFGRrs> {
        SWAP_OFFSET_W::new(self, 16)
    }
}
/**FLASH secure OBK configuration register

You can [`read`](crate::Reg::read) this register and get [`secobkcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secobkcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#FLASH:SECOBKCFGR)*/
pub struct SECOBKCFGRrs;
impl crate::RegisterSpec for SECOBKCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`secobkcfgr::R`](R) reader structure
impl crate::Readable for SECOBKCFGRrs {}
///`write(|w| ..)` method takes [`secobkcfgr::W`](W) writer structure
impl crate::Writable for SECOBKCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECOBKCFGR to value 0x01ff_0000
impl crate::Resettable for SECOBKCFGRrs {
    const RESET_VALUE: u32 = 0x01ff_0000;
}

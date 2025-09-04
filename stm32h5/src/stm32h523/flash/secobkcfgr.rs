///Register `SECOBKCFGR` reader
pub type R = crate::R<SECOBKCFGRrs>;
///Register `SECOBKCFGR` writer
pub type W = crate::W<SECOBKCFGRrs>;
///Field `LOCK` reader - OBKCFGR lock option configuration bit
pub type LOCK_R = crate::BitReader;
///Field `LOCK` writer - OBKCFGR lock option configuration bit
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWAP_SECT_REQ` reader - OBK swap sector request bit
pub type SWAP_SECT_REQ_R = crate::BitReader;
///Field `SWAP_SECT_REQ` writer - OBK swap sector request bit
pub type SWAP_SECT_REQ_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALT_SECT` reader - alternate sector bit
pub type ALT_SECT_R = crate::BitReader;
///Field `ALT_SECT` writer - alternate sector bit
pub type ALT_SECT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALT_SECT_ERASE` reader - alternate sector erase bit
pub type ALT_SECT_ERASE_R = crate::BitReader;
///Field `ALT_SECT_ERASE` writer - alternate sector erase bit
pub type ALT_SECT_ERASE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWAP_OFFSET` reader - key index (offset /16 bits) pointing for next swap.
pub type SWAP_OFFSET_R = crate::FieldReader<u16>;
///Field `SWAP_OFFSET` writer - key index (offset /16 bits) pointing for next swap.
pub type SWAP_OFFSET_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bit 0 - OBKCFGR lock option configuration bit
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - OBK swap sector request bit
    #[inline(always)]
    pub fn swap_sect_req(&self) -> SWAP_SECT_REQ_R {
        SWAP_SECT_REQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - alternate sector bit
    #[inline(always)]
    pub fn alt_sect(&self) -> ALT_SECT_R {
        ALT_SECT_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - alternate sector erase bit
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
    ///Bit 0 - OBKCFGR lock option configuration bit
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<SECOBKCFGRrs> {
        LOCK_W::new(self, 0)
    }
    ///Bit 1 - OBK swap sector request bit
    #[inline(always)]
    pub fn swap_sect_req(&mut self) -> SWAP_SECT_REQ_W<SECOBKCFGRrs> {
        SWAP_SECT_REQ_W::new(self, 1)
    }
    ///Bit 2 - alternate sector bit
    #[inline(always)]
    pub fn alt_sect(&mut self) -> ALT_SECT_W<SECOBKCFGRrs> {
        ALT_SECT_W::new(self, 2)
    }
    ///Bit 3 - alternate sector erase bit
    #[inline(always)]
    pub fn alt_sect_erase(&mut self) -> ALT_SECT_ERASE_W<SECOBKCFGRrs> {
        ALT_SECT_ERASE_W::new(self, 3)
    }
    ///Bits 16:24 - key index (offset /16 bits) pointing for next swap.
    #[inline(always)]
    pub fn swap_offset(&mut self) -> SWAP_OFFSET_W<SECOBKCFGRrs> {
        SWAP_OFFSET_W::new(self, 16)
    }
}
/**FLASH secure OBK configuration register

You can [`read`](crate::Reg::read) this register and get [`secobkcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secobkcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:SECOBKCFGR)*/
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

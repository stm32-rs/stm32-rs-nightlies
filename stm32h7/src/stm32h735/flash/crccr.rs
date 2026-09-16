///Register `CRCCR` reader
pub type R = crate::R<CRCCRrs>;
///Register `CRCCR` writer
pub type W = crate::W<CRCCRrs>;
///Field `CRC_SECT` reader - CRC sector number
pub type CRC_SECT_R = crate::FieldReader;
///Field `CRC_SECT` writer - CRC sector number
pub type CRC_SECT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `CRC_BY_SECT` reader - CRC sector mode select bit
pub type CRC_BY_SECT_R = crate::BitReader;
///Field `CRC_BY_SECT` writer - CRC sector mode select bit
pub type CRC_BY_SECT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADD_SECT` reader - CRC sector select bit
pub type ADD_SECT_R = crate::BitReader;
///Field `ADD_SECT` writer - CRC sector select bit
pub type ADD_SECT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLEAN_SECT` reader - CRC sector list clear bit
pub type CLEAN_SECT_R = crate::BitReader;
///Field `CLEAN_SECT` writer - CRC sector list clear bit
pub type CLEAN_SECT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `START_CRC` reader - CRC start bit
pub type START_CRC_R = crate::BitReader;
///Field `START_CRC` writer - CRC start bit
pub type START_CRC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLEAN_CRC` reader - CRC clear bit
pub type CLEAN_CRC_R = crate::BitReader;
///Field `CLEAN_CRC` writer - CRC clear bit
pub type CLEAN_CRC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRC_BURST` reader - CRC burst size
pub type CRC_BURST_R = crate::FieldReader;
///Field `CRC_BURST` writer - CRC burst size
pub type CRC_BURST_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ALL_BANK` writer - Bank 1 CRC select bit
pub type ALL_BANK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - CRC sector number
    #[inline(always)]
    pub fn crc_sect(&self) -> CRC_SECT_R {
        CRC_SECT_R::new((self.bits & 7) as u8)
    }
    ///Bit 8 - CRC sector mode select bit
    #[inline(always)]
    pub fn crc_by_sect(&self) -> CRC_BY_SECT_R {
        CRC_BY_SECT_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CRC sector select bit
    #[inline(always)]
    pub fn add_sect(&self) -> ADD_SECT_R {
        ADD_SECT_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - CRC sector list clear bit
    #[inline(always)]
    pub fn clean_sect(&self) -> CLEAN_SECT_R {
        CLEAN_SECT_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 16 - CRC start bit
    #[inline(always)]
    pub fn start_crc(&self) -> START_CRC_R {
        START_CRC_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - CRC clear bit
    #[inline(always)]
    pub fn clean_crc(&self) -> CLEAN_CRC_R {
        CLEAN_CRC_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bits 20:21 - CRC burst size
    #[inline(always)]
    pub fn crc_burst(&self) -> CRC_BURST_R {
        CRC_BURST_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRCCR")
            .field("crc_sect", &self.crc_sect())
            .field("crc_by_sect", &self.crc_by_sect())
            .field("add_sect", &self.add_sect())
            .field("clean_sect", &self.clean_sect())
            .field("start_crc", &self.start_crc())
            .field("clean_crc", &self.clean_crc())
            .field("crc_burst", &self.crc_burst())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - CRC sector number
    #[inline(always)]
    pub fn crc_sect(&mut self) -> CRC_SECT_W<'_, CRCCRrs> {
        CRC_SECT_W::new(self, 0)
    }
    ///Bit 8 - CRC sector mode select bit
    #[inline(always)]
    pub fn crc_by_sect(&mut self) -> CRC_BY_SECT_W<'_, CRCCRrs> {
        CRC_BY_SECT_W::new(self, 8)
    }
    ///Bit 9 - CRC sector select bit
    #[inline(always)]
    pub fn add_sect(&mut self) -> ADD_SECT_W<'_, CRCCRrs> {
        ADD_SECT_W::new(self, 9)
    }
    ///Bit 10 - CRC sector list clear bit
    #[inline(always)]
    pub fn clean_sect(&mut self) -> CLEAN_SECT_W<'_, CRCCRrs> {
        CLEAN_SECT_W::new(self, 10)
    }
    ///Bit 16 - CRC start bit
    #[inline(always)]
    pub fn start_crc(&mut self) -> START_CRC_W<'_, CRCCRrs> {
        START_CRC_W::new(self, 16)
    }
    ///Bit 17 - CRC clear bit
    #[inline(always)]
    pub fn clean_crc(&mut self) -> CLEAN_CRC_W<'_, CRCCRrs> {
        CLEAN_CRC_W::new(self, 17)
    }
    ///Bits 20:21 - CRC burst size
    #[inline(always)]
    pub fn crc_burst(&mut self) -> CRC_BURST_W<'_, CRCCRrs> {
        CRC_BURST_W::new(self, 20)
    }
    ///Bit 22 - Bank 1 CRC select bit
    #[inline(always)]
    pub fn all_bank(&mut self) -> ALL_BANK_W<'_, CRCCRrs> {
        ALL_BANK_W::new(self, 22)
    }
}
/**FLASH CRC control register for bank 1

You can [`read`](crate::Reg::read) this register and get [`crccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FLASH:CRCCR)*/
pub struct CRCCRrs;
impl crate::RegisterSpec for CRCCRrs {
    type Ux = u32;
}
///`read()` method returns [`crccr::R`](R) reader structure
impl crate::Readable for CRCCRrs {}
///`write(|w| ..)` method takes [`crccr::W`](W) writer structure
impl crate::Writable for CRCCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CRCCR to value 0
impl crate::Resettable for CRCCRrs {}

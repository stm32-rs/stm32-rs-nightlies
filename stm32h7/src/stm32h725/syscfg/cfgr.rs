///Register `CFGR` reader
pub type R = crate::R<CFGRrs>;
///Register `CFGR` writer
pub type W = crate::W<CFGRrs>;
///Field `PVDL` reader - Programmable voltage detector lockup bit
pub type PVDL_R = crate::BitReader;
///Field `PVDL` writer - Programmable voltage detector lockup bit
pub type PVDL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLASHL` reader - FLASH double error lockup bit
pub type FLASHL_R = crate::BitReader;
///Field `FLASHL` writer - FLASH double error lockup bit
pub type FLASHL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CM7L` reader - CPU lockup bit
pub type CM7L_R = crate::BitReader;
///Field `CM7L` writer - CPU lockup bit
pub type CM7L_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKRAML` reader - Backup RAM Double error lockup bit
pub type BKRAML_R = crate::BitReader;
///Field `BKRAML` writer - Backup RAM Double error lockup bit
pub type BKRAML_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM4L` reader - SRAM4 Double error lockup bit
pub type SRAM4L_R = crate::BitReader;
///Field `SRAM4L` writer - SRAM4 Double error lockup bit
pub type SRAM4L_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM2L` reader - SRAM2 Double error lockup bit
pub type SRAM2L_R = crate::BitReader;
///Field `SRAM2L` writer - SRAM2 Double error lockup bit
pub type SRAM2L_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM1L` reader - SRAM1 Double error lockup bit
pub type SRAM1L_R = crate::BitReader;
///Field `SRAM1L` writer - SRAM1 Double error lockup bit
pub type SRAM1L_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTCML` reader - DTCM-RAM Double error lockup bit
pub type DTCML_R = crate::BitReader;
///Field `DTCML` writer - DTCM-RAM Double error lockup bit
pub type DTCML_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITCML` reader - ITCM-RAM Double error lockup bit
pub type ITCML_R = crate::BitReader;
///Field `ITCML` writer - ITCM-RAM Double error lockup bit
pub type ITCML_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXIRAML` reader - AXISRAM Double error lockup bit
pub type AXIRAML_R = crate::BitReader;
///Field `AXIRAML` writer - AXISRAM Double error lockup bit
pub type AXIRAML_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 2 - Programmable voltage detector lockup bit
    #[inline(always)]
    pub fn pvdl(&self) -> PVDL_R {
        PVDL_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - FLASH double error lockup bit
    #[inline(always)]
    pub fn flashl(&self) -> FLASHL_R {
        FLASHL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 6 - CPU lockup bit
    #[inline(always)]
    pub fn cm7l(&self) -> CM7L_R {
        CM7L_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Backup RAM Double error lockup bit
    #[inline(always)]
    pub fn bkraml(&self) -> BKRAML_R {
        BKRAML_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - SRAM4 Double error lockup bit
    #[inline(always)]
    pub fn sram4l(&self) -> SRAM4L_R {
        SRAM4L_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - SRAM2 Double error lockup bit
    #[inline(always)]
    pub fn sram2l(&self) -> SRAM2L_R {
        SRAM2L_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SRAM1 Double error lockup bit
    #[inline(always)]
    pub fn sram1l(&self) -> SRAM1L_R {
        SRAM1L_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - DTCM-RAM Double error lockup bit
    #[inline(always)]
    pub fn dtcml(&self) -> DTCML_R {
        DTCML_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - ITCM-RAM Double error lockup bit
    #[inline(always)]
    pub fn itcml(&self) -> ITCML_R {
        ITCML_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - AXISRAM Double error lockup bit
    #[inline(always)]
    pub fn axiraml(&self) -> AXIRAML_R {
        AXIRAML_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR")
            .field("pvdl", &self.pvdl())
            .field("flashl", &self.flashl())
            .field("cm7l", &self.cm7l())
            .field("bkraml", &self.bkraml())
            .field("sram4l", &self.sram4l())
            .field("sram2l", &self.sram2l())
            .field("sram1l", &self.sram1l())
            .field("dtcml", &self.dtcml())
            .field("itcml", &self.itcml())
            .field("axiraml", &self.axiraml())
            .finish()
    }
}
impl W {
    ///Bit 2 - Programmable voltage detector lockup bit
    #[inline(always)]
    pub fn pvdl(&mut self) -> PVDL_W<'_, CFGRrs> {
        PVDL_W::new(self, 2)
    }
    ///Bit 3 - FLASH double error lockup bit
    #[inline(always)]
    pub fn flashl(&mut self) -> FLASHL_W<'_, CFGRrs> {
        FLASHL_W::new(self, 3)
    }
    ///Bit 6 - CPU lockup bit
    #[inline(always)]
    pub fn cm7l(&mut self) -> CM7L_W<'_, CFGRrs> {
        CM7L_W::new(self, 6)
    }
    ///Bit 7 - Backup RAM Double error lockup bit
    #[inline(always)]
    pub fn bkraml(&mut self) -> BKRAML_W<'_, CFGRrs> {
        BKRAML_W::new(self, 7)
    }
    ///Bit 9 - SRAM4 Double error lockup bit
    #[inline(always)]
    pub fn sram4l(&mut self) -> SRAM4L_W<'_, CFGRrs> {
        SRAM4L_W::new(self, 9)
    }
    ///Bit 11 - SRAM2 Double error lockup bit
    #[inline(always)]
    pub fn sram2l(&mut self) -> SRAM2L_W<'_, CFGRrs> {
        SRAM2L_W::new(self, 11)
    }
    ///Bit 12 - SRAM1 Double error lockup bit
    #[inline(always)]
    pub fn sram1l(&mut self) -> SRAM1L_W<'_, CFGRrs> {
        SRAM1L_W::new(self, 12)
    }
    ///Bit 13 - DTCM-RAM Double error lockup bit
    #[inline(always)]
    pub fn dtcml(&mut self) -> DTCML_W<'_, CFGRrs> {
        DTCML_W::new(self, 13)
    }
    ///Bit 14 - ITCM-RAM Double error lockup bit
    #[inline(always)]
    pub fn itcml(&mut self) -> ITCML_W<'_, CFGRrs> {
        ITCML_W::new(self, 14)
    }
    ///Bit 15 - AXISRAM Double error lockup bit
    #[inline(always)]
    pub fn axiraml(&mut self) -> AXIRAML_W<'_, CFGRrs> {
        AXIRAML_W::new(self, 15)
    }
}
/**Timer break lockup register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H725.html#SYSCFG:CFGR)*/
pub struct CFGRrs;
impl crate::RegisterSpec for CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`cfgr::R`](R) reader structure
impl crate::Readable for CFGRrs {}
///`write(|w| ..)` method takes [`cfgr::W`](W) writer structure
impl crate::Writable for CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR to value 0
impl crate::Resettable for CFGRrs {}

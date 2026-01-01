///Register `CFGR` reader
pub type R = crate::R<CFGRrs>;
///Register `CFGR` writer
pub type W = crate::W<CFGRrs>;
///Field `CM4L` reader - CM4L
pub type CM4L_R = crate::BitReader;
///Field `CM4L` writer - CM4L
pub type CM4L_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PVDL` reader - PVDL
pub type PVDL_R = crate::BitReader;
///Field `PVDL` writer - PVDL
pub type PVDL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLASHL` reader - FLASHL
pub type FLASHL_R = crate::BitReader;
///Field `FLASHL` writer - FLASHL
pub type FLASHL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CM7L` reader - CM7L
pub type CM7L_R = crate::BitReader;
///Field `CM7L` writer - CM7L
pub type CM7L_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKRAML` reader - BKRAML
pub type BKRAML_R = crate::BitReader;
///Field `BKRAML` writer - BKRAML
pub type BKRAML_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM4L` reader - SRAM4L
pub type SRAM4L_R = crate::BitReader;
///Field `SRAM4L` writer - SRAM4L
pub type SRAM4L_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM3L` reader - SRAM3L
pub type SRAM3L_R = crate::BitReader;
///Field `SRAM3L` writer - SRAM3L
pub type SRAM3L_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM2L` reader - SRAM2L
pub type SRAM2L_R = crate::BitReader;
///Field `SRAM2L` writer - SRAM2L
pub type SRAM2L_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM1L` reader - SRAM1L
pub type SRAM1L_R = crate::BitReader;
///Field `SRAM1L` writer - SRAM1L
pub type SRAM1L_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTCML` reader - DTCML
pub type DTCML_R = crate::BitReader;
///Field `DTCML` writer - DTCML
pub type DTCML_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITCML` reader - ITCML
pub type ITCML_R = crate::BitReader;
///Field `ITCML` writer - ITCML
pub type ITCML_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAML` reader - AXISRAML
pub type AXISRAML_R = crate::BitReader;
///Field `AXISRAML` writer - AXISRAML
pub type AXISRAML_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - CM4L
    #[inline(always)]
    pub fn cm4l(&self) -> CM4L_R {
        CM4L_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - PVDL
    #[inline(always)]
    pub fn pvdl(&self) -> PVDL_R {
        PVDL_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - FLASHL
    #[inline(always)]
    pub fn flashl(&self) -> FLASHL_R {
        FLASHL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 6 - CM7L
    #[inline(always)]
    pub fn cm7l(&self) -> CM7L_R {
        CM7L_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - BKRAML
    #[inline(always)]
    pub fn bkraml(&self) -> BKRAML_R {
        BKRAML_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - SRAM4L
    #[inline(always)]
    pub fn sram4l(&self) -> SRAM4L_R {
        SRAM4L_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - SRAM3L
    #[inline(always)]
    pub fn sram3l(&self) -> SRAM3L_R {
        SRAM3L_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - SRAM2L
    #[inline(always)]
    pub fn sram2l(&self) -> SRAM2L_R {
        SRAM2L_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SRAM1L
    #[inline(always)]
    pub fn sram1l(&self) -> SRAM1L_R {
        SRAM1L_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - DTCML
    #[inline(always)]
    pub fn dtcml(&self) -> DTCML_R {
        DTCML_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - ITCML
    #[inline(always)]
    pub fn itcml(&self) -> ITCML_R {
        ITCML_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - AXISRAML
    #[inline(always)]
    pub fn axisraml(&self) -> AXISRAML_R {
        AXISRAML_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR")
            .field("cm4l", &self.cm4l())
            .field("pvdl", &self.pvdl())
            .field("flashl", &self.flashl())
            .field("cm7l", &self.cm7l())
            .field("bkraml", &self.bkraml())
            .field("sram4l", &self.sram4l())
            .field("sram3l", &self.sram3l())
            .field("sram2l", &self.sram2l())
            .field("sram1l", &self.sram1l())
            .field("dtcml", &self.dtcml())
            .field("itcml", &self.itcml())
            .field("axisraml", &self.axisraml())
            .finish()
    }
}
impl W {
    ///Bit 0 - CM4L
    #[inline(always)]
    pub fn cm4l(&mut self) -> CM4L_W<'_, CFGRrs> {
        CM4L_W::new(self, 0)
    }
    ///Bit 2 - PVDL
    #[inline(always)]
    pub fn pvdl(&mut self) -> PVDL_W<'_, CFGRrs> {
        PVDL_W::new(self, 2)
    }
    ///Bit 3 - FLASHL
    #[inline(always)]
    pub fn flashl(&mut self) -> FLASHL_W<'_, CFGRrs> {
        FLASHL_W::new(self, 3)
    }
    ///Bit 6 - CM7L
    #[inline(always)]
    pub fn cm7l(&mut self) -> CM7L_W<'_, CFGRrs> {
        CM7L_W::new(self, 6)
    }
    ///Bit 7 - BKRAML
    #[inline(always)]
    pub fn bkraml(&mut self) -> BKRAML_W<'_, CFGRrs> {
        BKRAML_W::new(self, 7)
    }
    ///Bit 9 - SRAM4L
    #[inline(always)]
    pub fn sram4l(&mut self) -> SRAM4L_W<'_, CFGRrs> {
        SRAM4L_W::new(self, 9)
    }
    ///Bit 10 - SRAM3L
    #[inline(always)]
    pub fn sram3l(&mut self) -> SRAM3L_W<'_, CFGRrs> {
        SRAM3L_W::new(self, 10)
    }
    ///Bit 11 - SRAM2L
    #[inline(always)]
    pub fn sram2l(&mut self) -> SRAM2L_W<'_, CFGRrs> {
        SRAM2L_W::new(self, 11)
    }
    ///Bit 12 - SRAM1L
    #[inline(always)]
    pub fn sram1l(&mut self) -> SRAM1L_W<'_, CFGRrs> {
        SRAM1L_W::new(self, 12)
    }
    ///Bit 13 - DTCML
    #[inline(always)]
    pub fn dtcml(&mut self) -> DTCML_W<'_, CFGRrs> {
        DTCML_W::new(self, 13)
    }
    ///Bit 14 - ITCML
    #[inline(always)]
    pub fn itcml(&mut self) -> ITCML_W<'_, CFGRrs> {
        ITCML_W::new(self, 14)
    }
    ///Bit 15 - AXISRAML
    #[inline(always)]
    pub fn axisraml(&mut self) -> AXISRAML_W<'_, CFGRrs> {
        AXISRAML_W::new(self, 15)
    }
}
/**configuration register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#SYSCFG:CFGR)*/
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

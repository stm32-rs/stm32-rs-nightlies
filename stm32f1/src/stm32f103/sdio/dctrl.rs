///Register `DCTRL` reader
pub type R = crate::R<DCTRLrs>;
///Register `DCTRL` writer
pub type W = crate::W<DCTRLrs>;
///Field `DTEN` reader - DTEN
pub type DTEN_R = crate::BitReader;
///Field `DTEN` writer - DTEN
pub type DTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTDIR` reader - DTDIR
pub type DTDIR_R = crate::BitReader;
///Field `DTDIR` writer - DTDIR
pub type DTDIR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTMODE` reader - DTMODE
pub type DTMODE_R = crate::BitReader;
///Field `DTMODE` writer - DTMODE
pub type DTMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMAEN` reader - DMAEN
pub type DMAEN_R = crate::BitReader;
///Field `DMAEN` writer - DMAEN
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBLOCKSIZE` reader - DBLOCKSIZE
pub type DBLOCKSIZE_R = crate::FieldReader;
///Field `DBLOCKSIZE` writer - DBLOCKSIZE
pub type DBLOCKSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PWSTART` reader - PWSTART
pub type PWSTART_R = crate::BitReader;
///Field `PWSTART` writer - PWSTART
pub type PWSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PWSTOP` reader - PWSTOP
pub type PWSTOP_R = crate::BitReader;
///Field `PWSTOP` writer - PWSTOP
pub type PWSTOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RWMOD` reader - RWMOD
pub type RWMOD_R = crate::BitReader;
///Field `RWMOD` writer - RWMOD
pub type RWMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDIOEN` reader - SDIOEN
pub type SDIOEN_R = crate::BitReader;
///Field `SDIOEN` writer - SDIOEN
pub type SDIOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - DTEN
    #[inline(always)]
    pub fn dten(&self) -> DTEN_R {
        DTEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DTDIR
    #[inline(always)]
    pub fn dtdir(&self) -> DTDIR_R {
        DTDIR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DTMODE
    #[inline(always)]
    pub fn dtmode(&self) -> DTMODE_R {
        DTMODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - DMAEN
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:7 - DBLOCKSIZE
    #[inline(always)]
    pub fn dblocksize(&self) -> DBLOCKSIZE_R {
        DBLOCKSIZE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bit 8 - PWSTART
    #[inline(always)]
    pub fn pwstart(&self) -> PWSTART_R {
        PWSTART_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - PWSTOP
    #[inline(always)]
    pub fn pwstop(&self) -> PWSTOP_R {
        PWSTOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - RWMOD
    #[inline(always)]
    pub fn rwmod(&self) -> RWMOD_R {
        RWMOD_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - SDIOEN
    #[inline(always)]
    pub fn sdioen(&self) -> SDIOEN_R {
        SDIOEN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCTRL")
            .field("dten", &self.dten())
            .field("dtdir", &self.dtdir())
            .field("dtmode", &self.dtmode())
            .field("dmaen", &self.dmaen())
            .field("dblocksize", &self.dblocksize())
            .field("pwstart", &self.pwstart())
            .field("pwstop", &self.pwstop())
            .field("rwmod", &self.rwmod())
            .field("sdioen", &self.sdioen())
            .finish()
    }
}
impl W {
    ///Bit 0 - DTEN
    #[inline(always)]
    #[must_use]
    pub fn dten(&mut self) -> DTEN_W<DCTRLrs> {
        DTEN_W::new(self, 0)
    }
    ///Bit 1 - DTDIR
    #[inline(always)]
    #[must_use]
    pub fn dtdir(&mut self) -> DTDIR_W<DCTRLrs> {
        DTDIR_W::new(self, 1)
    }
    ///Bit 2 - DTMODE
    #[inline(always)]
    #[must_use]
    pub fn dtmode(&mut self) -> DTMODE_W<DCTRLrs> {
        DTMODE_W::new(self, 2)
    }
    ///Bit 3 - DMAEN
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<DCTRLrs> {
        DMAEN_W::new(self, 3)
    }
    ///Bits 4:7 - DBLOCKSIZE
    #[inline(always)]
    #[must_use]
    pub fn dblocksize(&mut self) -> DBLOCKSIZE_W<DCTRLrs> {
        DBLOCKSIZE_W::new(self, 4)
    }
    ///Bit 8 - PWSTART
    #[inline(always)]
    #[must_use]
    pub fn pwstart(&mut self) -> PWSTART_W<DCTRLrs> {
        PWSTART_W::new(self, 8)
    }
    ///Bit 9 - PWSTOP
    #[inline(always)]
    #[must_use]
    pub fn pwstop(&mut self) -> PWSTOP_W<DCTRLrs> {
        PWSTOP_W::new(self, 9)
    }
    ///Bit 10 - RWMOD
    #[inline(always)]
    #[must_use]
    pub fn rwmod(&mut self) -> RWMOD_W<DCTRLrs> {
        RWMOD_W::new(self, 10)
    }
    ///Bit 11 - SDIOEN
    #[inline(always)]
    #[must_use]
    pub fn sdioen(&mut self) -> SDIOEN_W<DCTRLrs> {
        SDIOEN_W::new(self, 11)
    }
}
/**SDIO data control register (SDIO_DCTRL)

You can [`read`](crate::Reg::read) this register and get [`dctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F103.html#SDIO:DCTRL)*/
pub struct DCTRLrs;
impl crate::RegisterSpec for DCTRLrs {
    type Ux = u32;
}
///`read()` method returns [`dctrl::R`](R) reader structure
impl crate::Readable for DCTRLrs {}
///`write(|w| ..)` method takes [`dctrl::W`](W) writer structure
impl crate::Writable for DCTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DCTRL to value 0
impl crate::Resettable for DCTRLrs {
    const RESET_VALUE: u32 = 0;
}

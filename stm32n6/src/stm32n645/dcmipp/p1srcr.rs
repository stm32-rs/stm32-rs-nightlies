///Register `P1SRCR` reader
pub type R = crate::R<P1SRCRrs>;
///Register `P1SRCR` writer
pub type W = crate::W<P1SRCRrs>;
///Field `LASTLINE` reader - Amount of following lines to keep when CROPEN = 1. If LASTLINE = 0 all pixels after FIRSTLINEDEL are fed through.
pub type LASTLINE_R = crate::FieldReader<u16>;
///Field `LASTLINE` writer - Amount of following lines to keep when CROPEN = 1. If LASTLINE = 0 all pixels after FIRSTLINEDEL are fed through.
pub type LASTLINE_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `FIRSTLINEDEL` reader - Amount of first lines to delete when CROPEN = 1
pub type FIRSTLINEDEL_R = crate::FieldReader;
///Field `FIRSTLINEDEL` writer - Amount of first lines to delete when CROPEN = 1
pub type FIRSTLINEDEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `CROPEN` reader - Crop line enable
pub type CROPEN_R = crate::BitReader;
///Field `CROPEN` writer - Crop line enable
pub type CROPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:11 - Amount of following lines to keep when CROPEN = 1. If LASTLINE = 0 all pixels after FIRSTLINEDEL are fed through.
    #[inline(always)]
    pub fn lastline(&self) -> LASTLINE_R {
        LASTLINE_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 12:14 - Amount of first lines to delete when CROPEN = 1
    #[inline(always)]
    pub fn firstlinedel(&self) -> FIRSTLINEDEL_R {
        FIRSTLINEDEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bit 15 - Crop line enable
    #[inline(always)]
    pub fn cropen(&self) -> CROPEN_R {
        CROPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1SRCR")
            .field("lastline", &self.lastline())
            .field("firstlinedel", &self.firstlinedel())
            .field("cropen", &self.cropen())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - Amount of following lines to keep when CROPEN = 1. If LASTLINE = 0 all pixels after FIRSTLINEDEL are fed through.
    #[inline(always)]
    pub fn lastline(&mut self) -> LASTLINE_W<'_, P1SRCRrs> {
        LASTLINE_W::new(self, 0)
    }
    ///Bits 12:14 - Amount of first lines to delete when CROPEN = 1
    #[inline(always)]
    pub fn firstlinedel(&mut self) -> FIRSTLINEDEL_W<'_, P1SRCRrs> {
        FIRSTLINEDEL_W::new(self, 12)
    }
    ///Bit 15 - Crop line enable
    #[inline(always)]
    pub fn cropen(&mut self) -> CROPEN_W<'_, P1SRCRrs> {
        CROPEN_W::new(self, 15)
    }
}
/**DCMIPP Pipe1 stat removal configuration register

You can [`read`](crate::Reg::read) this register and get [`p1srcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1srcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#DCMIPP:P1SRCR)*/
pub struct P1SRCRrs;
impl crate::RegisterSpec for P1SRCRrs {
    type Ux = u32;
}
///`read()` method returns [`p1srcr::R`](R) reader structure
impl crate::Readable for P1SRCRrs {}
///`write(|w| ..)` method takes [`p1srcr::W`](W) writer structure
impl crate::Writable for P1SRCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets P1SRCR to value 0
impl crate::Resettable for P1SRCRrs {}

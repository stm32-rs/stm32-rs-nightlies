///Register `P0PPCR` reader
pub type R = crate::R<P0PPCRrs>;
///Register `P0PPCR` writer
pub type W = crate::W<P0PPCRrs>;
///Field `SWAPYUV` reader - Swaps, within a 32-bit word, byte 0-vs-1 and byte 2-vs-3. It corresponds, for YUV422 pixels formats, to swap between UYVY and YUYV.
pub type SWAPYUV_R = crate::BitReader;
///Field `SWAPYUV` writer - Swaps, within a 32-bit word, byte 0-vs-1 and byte 2-vs-3. It corresponds, for YUV422 pixels formats, to swap between UYVY and YUYV.
pub type SWAPYUV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PAD` reader - Pad mode for monochrome and raw Bayer 10/12/14 bpp: MSB vs. LSB alignment.
pub type PAD_R = crate::BitReader;
///Field `PAD` writer - Pad mode for monochrome and raw Bayer 10/12/14 bpp: MSB vs. LSB alignment.
pub type PAD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HEADEREN` reader - CSI header dump enable
pub type HEADEREN_R = crate::BitReader;
///Field `HEADEREN` writer - CSI header dump enable
pub type HEADEREN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BSM` reader - Byte select mode
pub type BSM_R = crate::FieldReader;
///Field `BSM` writer - Byte select mode
pub type BSM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OEBS` reader - Odd/even byte select (byte select start)
pub type OEBS_R = crate::BitReader;
///Field `OEBS` writer - Odd/even byte select (byte select start)
pub type OEBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSM` reader - Line select mode
pub type LSM_R = crate::BitReader;
///Field `LSM` writer - Line select mode
pub type LSM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OELS` reader - Odd/even line select (line select start)
pub type OELS_R = crate::BitReader;
///Field `OELS` writer - Odd/even line select (line select start)
pub type OELS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LINEMULT` reader - Amount of capture completed lines for LINE event and interrupt
pub type LINEMULT_R = crate::FieldReader;
///Field `LINEMULT` writer - Amount of capture completed lines for LINE event and interrupt
pub type LINEMULT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DBM` reader - Double buffer mode
pub type DBM_R = crate::BitReader;
///Field `DBM` writer - Double buffer mode
pub type DBM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Swaps, within a 32-bit word, byte 0-vs-1 and byte 2-vs-3. It corresponds, for YUV422 pixels formats, to swap between UYVY and YUYV.
    #[inline(always)]
    pub fn swapyuv(&self) -> SWAPYUV_R {
        SWAPYUV_R::new((self.bits & 1) != 0)
    }
    ///Bit 5 - Pad mode for monochrome and raw Bayer 10/12/14 bpp: MSB vs. LSB alignment.
    #[inline(always)]
    pub fn pad(&self) -> PAD_R {
        PAD_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CSI header dump enable
    #[inline(always)]
    pub fn headeren(&self) -> HEADEREN_R {
        HEADEREN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 7:8 - Byte select mode
    #[inline(always)]
    pub fn bsm(&self) -> BSM_R {
        BSM_R::new(((self.bits >> 7) & 3) as u8)
    }
    ///Bit 9 - Odd/even byte select (byte select start)
    #[inline(always)]
    pub fn oebs(&self) -> OEBS_R {
        OEBS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Line select mode
    #[inline(always)]
    pub fn lsm(&self) -> LSM_R {
        LSM_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Odd/even line select (line select start)
    #[inline(always)]
    pub fn oels(&self) -> OELS_R {
        OELS_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 13:15 - Amount of capture completed lines for LINE event and interrupt
    #[inline(always)]
    pub fn linemult(&self) -> LINEMULT_R {
        LINEMULT_R::new(((self.bits >> 13) & 7) as u8)
    }
    ///Bit 16 - Double buffer mode
    #[inline(always)]
    pub fn dbm(&self) -> DBM_R {
        DBM_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P0PPCR")
            .field("swapyuv", &self.swapyuv())
            .field("pad", &self.pad())
            .field("headeren", &self.headeren())
            .field("bsm", &self.bsm())
            .field("oebs", &self.oebs())
            .field("lsm", &self.lsm())
            .field("oels", &self.oels())
            .field("linemult", &self.linemult())
            .field("dbm", &self.dbm())
            .finish()
    }
}
impl W {
    ///Bit 0 - Swaps, within a 32-bit word, byte 0-vs-1 and byte 2-vs-3. It corresponds, for YUV422 pixels formats, to swap between UYVY and YUYV.
    #[inline(always)]
    pub fn swapyuv(&mut self) -> SWAPYUV_W<'_, P0PPCRrs> {
        SWAPYUV_W::new(self, 0)
    }
    ///Bit 5 - Pad mode for monochrome and raw Bayer 10/12/14 bpp: MSB vs. LSB alignment.
    #[inline(always)]
    pub fn pad(&mut self) -> PAD_W<'_, P0PPCRrs> {
        PAD_W::new(self, 5)
    }
    ///Bit 6 - CSI header dump enable
    #[inline(always)]
    pub fn headeren(&mut self) -> HEADEREN_W<'_, P0PPCRrs> {
        HEADEREN_W::new(self, 6)
    }
    ///Bits 7:8 - Byte select mode
    #[inline(always)]
    pub fn bsm(&mut self) -> BSM_W<'_, P0PPCRrs> {
        BSM_W::new(self, 7)
    }
    ///Bit 9 - Odd/even byte select (byte select start)
    #[inline(always)]
    pub fn oebs(&mut self) -> OEBS_W<'_, P0PPCRrs> {
        OEBS_W::new(self, 9)
    }
    ///Bit 10 - Line select mode
    #[inline(always)]
    pub fn lsm(&mut self) -> LSM_W<'_, P0PPCRrs> {
        LSM_W::new(self, 10)
    }
    ///Bit 11 - Odd/even line select (line select start)
    #[inline(always)]
    pub fn oels(&mut self) -> OELS_W<'_, P0PPCRrs> {
        OELS_W::new(self, 11)
    }
    ///Bits 13:15 - Amount of capture completed lines for LINE event and interrupt
    #[inline(always)]
    pub fn linemult(&mut self) -> LINEMULT_W<'_, P0PPCRrs> {
        LINEMULT_W::new(self, 13)
    }
    ///Bit 16 - Double buffer mode
    #[inline(always)]
    pub fn dbm(&mut self) -> DBM_W<'_, P0PPCRrs> {
        DBM_W::new(self, 16)
    }
}
/**DCMIPP Pipe0 pixel packer configuration register

You can [`read`](crate::Reg::read) this register and get [`p0ppcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0ppcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DCMIPP:P0PPCR)*/
pub struct P0PPCRrs;
impl crate::RegisterSpec for P0PPCRrs {
    type Ux = u32;
}
///`read()` method returns [`p0ppcr::R`](R) reader structure
impl crate::Readable for P0PPCRrs {}
///`write(|w| ..)` method takes [`p0ppcr::W`](W) writer structure
impl crate::Writable for P0PPCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets P0PPCR to value 0
impl crate::Resettable for P0PPCRrs {}

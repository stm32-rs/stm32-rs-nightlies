///Register `P0CPPCR` reader
pub type R = crate::R<P0CPPCRrs>;
///Field `SWAPYUV` reader - Swaps, within a 32-bit word, byte 0 vs. 1 and byte 2 vs. 3. It corresponds, for YUV422 pixels formats, to swap between UYVY and YUYV.
pub type SWAPYUV_R = crate::BitReader;
///Field `PAD` reader - Current Pad mode for monochrome and raw Bayer 10/12/14 bpp: MSB vs. LSB alignment
pub type PAD_R = crate::BitReader;
///Field `HEADEREN` reader - Current CSI header dump enable
pub type HEADEREN_R = crate::BitReader;
///Field `BSM` reader - Current Byte select mode
pub type BSM_R = crate::FieldReader;
///Field `OEBS` reader - Current odd/even byte select (byte select start)
pub type OEBS_R = crate::BitReader;
///Field `LSM` reader - Current Line select mode
pub type LSM_R = crate::BitReader;
///Field `OELS` reader - Current odd/even line select (ine select start)
pub type OELS_R = crate::BitReader;
///Field `LINEMULT` reader - Current amount of capture completed lines for LINE event and interrupt
pub type LINEMULT_R = crate::FieldReader;
///Field `DBM` reader - Double buffer mode
pub type DBM_R = crate::BitReader;
impl R {
    ///Bit 0 - Swaps, within a 32-bit word, byte 0 vs. 1 and byte 2 vs. 3. It corresponds, for YUV422 pixels formats, to swap between UYVY and YUYV.
    #[inline(always)]
    pub fn swapyuv(&self) -> SWAPYUV_R {
        SWAPYUV_R::new((self.bits & 1) != 0)
    }
    ///Bit 5 - Current Pad mode for monochrome and raw Bayer 10/12/14 bpp: MSB vs. LSB alignment
    #[inline(always)]
    pub fn pad(&self) -> PAD_R {
        PAD_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Current CSI header dump enable
    #[inline(always)]
    pub fn headeren(&self) -> HEADEREN_R {
        HEADEREN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 7:8 - Current Byte select mode
    #[inline(always)]
    pub fn bsm(&self) -> BSM_R {
        BSM_R::new(((self.bits >> 7) & 3) as u8)
    }
    ///Bit 9 - Current odd/even byte select (byte select start)
    #[inline(always)]
    pub fn oebs(&self) -> OEBS_R {
        OEBS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Current Line select mode
    #[inline(always)]
    pub fn lsm(&self) -> LSM_R {
        LSM_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Current odd/even line select (ine select start)
    #[inline(always)]
    pub fn oels(&self) -> OELS_R {
        OELS_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 13:15 - Current amount of capture completed lines for LINE event and interrupt
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
        f.debug_struct("P0CPPCR")
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
/**DCMIPP Pipe0 current pixel packer configuration register

You can [`read`](crate::Reg::read) this register and get [`p0cppcr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P0CPPCR)*/
pub struct P0CPPCRrs;
impl crate::RegisterSpec for P0CPPCRrs {
    type Ux = u32;
}
///`read()` method returns [`p0cppcr::R`](R) reader structure
impl crate::Readable for P0CPPCRrs {}
///`reset()` method sets P0CPPCR to value 0
impl crate::Resettable for P0CPPCRrs {}

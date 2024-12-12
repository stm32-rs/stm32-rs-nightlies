///Register `CTR` reader
pub type R = crate::R<CTRrs>;
///Field `_IminLine` reader - IminLine
pub type _IMIN_LINE_R = crate::FieldReader;
///Field `DMinLine` reader - DMinLine
pub type DMIN_LINE_R = crate::FieldReader;
///Field `ERG` reader - ERG
pub type ERG_R = crate::FieldReader;
///Field `CWG` reader - CWG
pub type CWG_R = crate::FieldReader;
///Field `Format` reader - Format
pub type FORMAT_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - IminLine
    #[inline(always)]
    pub fn _imin_line(&self) -> _IMIN_LINE_R {
        _IMIN_LINE_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 16:19 - DMinLine
    #[inline(always)]
    pub fn dmin_line(&self) -> DMIN_LINE_R {
        DMIN_LINE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - ERG
    #[inline(always)]
    pub fn erg(&self) -> ERG_R {
        ERG_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - CWG
    #[inline(always)]
    pub fn cwg(&self) -> CWG_R {
        CWG_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 29:31 - Format
    #[inline(always)]
    pub fn format(&self) -> FORMAT_R {
        FORMAT_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTR")
            .field("_imin_line", &self._imin_line())
            .field("dmin_line", &self.dmin_line())
            .field("erg", &self.erg())
            .field("cwg", &self.cwg())
            .field("format", &self.format())
            .finish()
    }
}
/**Cache Type register

You can [`read`](crate::Reg::read) this register and get [`ctr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#PF:CTR)*/
pub struct CTRrs;
impl crate::RegisterSpec for CTRrs {
    type Ux = u32;
}
///`read()` method returns [`ctr::R`](R) reader structure
impl crate::Readable for CTRrs {}
///`reset()` method sets CTR to value 0x8303_c003
impl crate::Resettable for CTRrs {
    const RESET_VALUE: u32 = 0x8303_c003;
}

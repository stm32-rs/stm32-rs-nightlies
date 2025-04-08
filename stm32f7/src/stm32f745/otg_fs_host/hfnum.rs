///Register `HFNUM` reader
pub type R = crate::R<HFNUMrs>;
///Field `FRNUM` reader - Frame number
pub type FRNUM_R = crate::FieldReader<u16>;
///Field `FTREM` reader - Frame time remaining
pub type FTREM_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Frame number
    #[inline(always)]
    pub fn frnum(&self) -> FRNUM_R {
        FRNUM_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Frame time remaining
    #[inline(always)]
    pub fn ftrem(&self) -> FTREM_R {
        FTREM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HFNUM")
            .field("frnum", &self.frnum())
            .field("ftrem", &self.ftrem())
            .finish()
    }
}
/**OTG_FS host frame number/frame time remaining register (OTG_FS_HFNUM)

You can [`read`](crate::Reg::read) this register and get [`hfnum::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F745.html#OTG_FS_HOST:HFNUM)*/
pub struct HFNUMrs;
impl crate::RegisterSpec for HFNUMrs {
    type Ux = u32;
}
///`read()` method returns [`hfnum::R`](R) reader structure
impl crate::Readable for HFNUMrs {}
///`reset()` method sets HFNUM to value 0x3fff
impl crate::Resettable for HFNUMrs {
    const RESET_VALUE: u32 = 0x3fff;
}

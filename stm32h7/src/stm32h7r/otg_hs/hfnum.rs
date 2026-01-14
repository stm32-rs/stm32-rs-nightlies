///Register `HFNUM` reader
pub type R = crate::R<HFNUMrs>;
///Field `FRNUM` reader - Frame number This field increments when a new SOF is transmitted on the USB, and is cleared to 0 when it reaches 0x3FFF.
pub type FRNUM_R = crate::FieldReader<u16>;
///Field `FTREM` reader - Frame time remaining Indicates the amount of time remaining in the current frame, in terms of PHY clocks. This field decrements on each PHY clock. When it reaches zero, this field is reloaded with the value in the Frame interval register and a new SOF is transmitted on the USB.
pub type FTREM_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Frame number This field increments when a new SOF is transmitted on the USB, and is cleared to 0 when it reaches 0x3FFF.
    #[inline(always)]
    pub fn frnum(&self) -> FRNUM_R {
        FRNUM_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Frame time remaining Indicates the amount of time remaining in the current frame, in terms of PHY clocks. This field decrements on each PHY clock. When it reaches zero, this field is reloaded with the value in the Frame interval register and a new SOF is transmitted on the USB.
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
/**OTG host frame number/frame time remaining register

You can [`read`](crate::Reg::read) this register and get [`hfnum::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#OTG_HS:HFNUM)*/
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

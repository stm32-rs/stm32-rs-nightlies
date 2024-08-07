///Register `OTG_HFNUM` reader
pub type R = crate::R<OTG_HFNUMrs>;
///Field `FRNUM` reader - FRNUM
pub type FRNUM_R = crate::FieldReader<u16>;
///Field `FTREM` reader - FTREM
pub type FTREM_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - FRNUM
    #[inline(always)]
    pub fn frnum(&self) -> FRNUM_R {
        FRNUM_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - FTREM
    #[inline(always)]
    pub fn ftrem(&self) -> FTREM_R {
        FTREM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTG_HFNUM")
            .field("frnum", &self.frnum())
            .field("ftrem", &self.ftrem())
            .finish()
    }
}
/**This register indicates the current frame number. It also indicates the time remaining (in terms of the number of PHY clocks) in the current frame.

You can [`read`](crate::Reg::read) this register and get [`otg_hfnum::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#OTG:OTG_HFNUM)*/
pub struct OTG_HFNUMrs;
impl crate::RegisterSpec for OTG_HFNUMrs {
    type Ux = u32;
}
///`read()` method returns [`otg_hfnum::R`](R) reader structure
impl crate::Readable for OTG_HFNUMrs {}
///`reset()` method sets OTG_HFNUM to value 0x3fff
impl crate::Resettable for OTG_HFNUMrs {
    const RESET_VALUE: u32 = 0x3fff;
}

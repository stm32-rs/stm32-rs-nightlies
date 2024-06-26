///Register `OTG_HNPTXSTS` reader
pub type R = crate::R<OTG_HNPTXSTSrs>;
///Field `NPTXFSAV` reader - NPTXFSAV
pub type NPTXFSAV_R = crate::FieldReader<u16>;
///Field `NPTQXSAV` reader - NPTQXSAV
pub type NPTQXSAV_R = crate::FieldReader;
///Field `NPTXQTOP` reader - NPTXQTOP
pub type NPTXQTOP_R = crate::FieldReader;
impl R {
    ///Bits 0:15 - NPTXFSAV
    #[inline(always)]
    pub fn nptxfsav(&self) -> NPTXFSAV_R {
        NPTXFSAV_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:23 - NPTQXSAV
    #[inline(always)]
    pub fn nptqxsav(&self) -> NPTQXSAV_R {
        NPTQXSAV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:30 - NPTXQTOP
    #[inline(always)]
    pub fn nptxqtop(&self) -> NPTXQTOP_R {
        NPTXQTOP_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTG_HNPTXSTS")
            .field("nptxfsav", &self.nptxfsav())
            .field("nptqxsav", &self.nptqxsav())
            .field("nptxqtop", &self.nptxqtop())
            .finish()
    }
}
/**In device mode, this register is not valid. This read-only register contains the free space information for the non-periodic Tx FIFO and the non-periodic transmit request queue.

You can [`read`](crate::Reg::read) this register and get [`otg_hnptxsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#OTG:OTG_HNPTXSTS)*/
pub struct OTG_HNPTXSTSrs;
impl crate::RegisterSpec for OTG_HNPTXSTSrs {
    type Ux = u32;
}
///`read()` method returns [`otg_hnptxsts::R`](R) reader structure
impl crate::Readable for OTG_HNPTXSTSrs {}
///`reset()` method sets OTG_HNPTXSTS to value 0x0008_0400
impl crate::Resettable for OTG_HNPTXSTSrs {
    const RESET_VALUE: u32 = 0x0008_0400;
}

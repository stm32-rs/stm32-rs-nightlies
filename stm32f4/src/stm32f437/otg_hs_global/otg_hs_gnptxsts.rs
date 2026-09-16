///Register `OTG_HS_GNPTXSTS` reader
pub type R = crate::R<OTG_HS_GNPTXSTSrs>;
///Field `NPTXFSAV` reader - Nonperiodic TxFIFO space available
pub type NPTXFSAV_R = crate::FieldReader<u16>;
///Field `NPTQXSAV` reader - Nonperiodic transmit request queue space available
pub type NPTQXSAV_R = crate::FieldReader;
///Field `NPTXQTOP` reader - Top of the nonperiodic transmit request queue
pub type NPTXQTOP_R = crate::FieldReader;
impl R {
    ///Bits 0:15 - Nonperiodic TxFIFO space available
    #[inline(always)]
    pub fn nptxfsav(&self) -> NPTXFSAV_R {
        NPTXFSAV_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:23 - Nonperiodic transmit request queue space available
    #[inline(always)]
    pub fn nptqxsav(&self) -> NPTQXSAV_R {
        NPTQXSAV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:30 - Top of the nonperiodic transmit request queue
    #[inline(always)]
    pub fn nptxqtop(&self) -> NPTXQTOP_R {
        NPTXQTOP_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTG_HS_GNPTXSTS")
            .field("nptxfsav", &self.nptxfsav())
            .field("nptqxsav", &self.nptqxsav())
            .field("nptxqtop", &self.nptxqtop())
            .finish()
    }
}
/**OTG_HS nonperiodic transmit FIFO/queue status register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_gnptxsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_GLOBAL:OTG_HS_GNPTXSTS)*/
pub struct OTG_HS_GNPTXSTSrs;
impl crate::RegisterSpec for OTG_HS_GNPTXSTSrs {
    type Ux = u32;
}
///`read()` method returns [`otg_hs_gnptxsts::R`](R) reader structure
impl crate::Readable for OTG_HS_GNPTXSTSrs {}
///`reset()` method sets OTG_HS_GNPTXSTS to value 0x0008_0200
impl crate::Resettable for OTG_HS_GNPTXSTSrs {
    const RESET_VALUE: u32 = 0x0008_0200;
}

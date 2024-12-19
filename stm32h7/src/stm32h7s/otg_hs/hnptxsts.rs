///Register `HNPTXSTS` reader
pub type R = crate::R<HNPTXSTSrs>;
///Field `NPTXFSAV` reader - Non-periodic Tx FIFO space available Indicates the amount of free space available in the non-periodic Tx FIFO. Values are in terms of 32-bit words. n: n words available (where 0 UNDER OR EQUAL n UNDER OR EQUAL 512) Others: Reserved
pub type NPTXFSAV_R = crate::FieldReader<u16>;
///Field `NPTQXSAV` reader - Non-periodic transmit request queue space available Indicates the amount of free space available in the non-periodic transmit request queue. This queue holds both IN and OUT requests. n: n locations available (0 UNDER OR EQUAL n UNDER OR EQUAL 8) Others: Reserved
pub type NPTQXSAV_R = crate::FieldReader;
///Field `NPTXQTOP` reader - Top of the non-periodic transmit request queue Entry in the non-periodic Tx request queue that is currently being processed by the MAC. Bits 30:27: Channel/endpoint number Bits 26:25: XXXX00X: IN/OUT token XXXX01X: Zero-length transmit packet (device IN/host OUT) XXXX11X: Channel halt command Bit 24: Terminate (last entry for selected channel/endpoint)
pub type NPTXQTOP_R = crate::FieldReader;
impl R {
    ///Bits 0:15 - Non-periodic Tx FIFO space available Indicates the amount of free space available in the non-periodic Tx FIFO. Values are in terms of 32-bit words. n: n words available (where 0 UNDER OR EQUAL n UNDER OR EQUAL 512) Others: Reserved
    #[inline(always)]
    pub fn nptxfsav(&self) -> NPTXFSAV_R {
        NPTXFSAV_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:23 - Non-periodic transmit request queue space available Indicates the amount of free space available in the non-periodic transmit request queue. This queue holds both IN and OUT requests. n: n locations available (0 UNDER OR EQUAL n UNDER OR EQUAL 8) Others: Reserved
    #[inline(always)]
    pub fn nptqxsav(&self) -> NPTQXSAV_R {
        NPTQXSAV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:30 - Top of the non-periodic transmit request queue Entry in the non-periodic Tx request queue that is currently being processed by the MAC. Bits 30:27: Channel/endpoint number Bits 26:25: XXXX00X: IN/OUT token XXXX01X: Zero-length transmit packet (device IN/host OUT) XXXX11X: Channel halt command Bit 24: Terminate (last entry for selected channel/endpoint)
    #[inline(always)]
    pub fn nptxqtop(&self) -> NPTXQTOP_R {
        NPTXQTOP_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HNPTXSTS")
            .field("nptxfsav", &self.nptxfsav())
            .field("nptqxsav", &self.nptqxsav())
            .field("nptxqtop", &self.nptxqtop())
            .finish()
    }
}
/**OTG non-periodic transmit FIFO/queue status register

You can [`read`](crate::Reg::read) this register and get [`hnptxsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#OTG_HS:HNPTXSTS)*/
pub struct HNPTXSTSrs;
impl crate::RegisterSpec for HNPTXSTSrs {
    type Ux = u32;
}
///`read()` method returns [`hnptxsts::R`](R) reader structure
impl crate::Readable for HNPTXSTSrs {}
///`reset()` method sets HNPTXSTS to value 0x0008_0400
impl crate::Resettable for HNPTXSTSrs {
    const RESET_VALUE: u32 = 0x0008_0400;
}

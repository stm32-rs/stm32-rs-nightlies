///Register `HPTXSTS` reader
pub type R = crate::R<HPTXSTSrs>;
///Field `PTXFSAVL` reader - Periodic transmit data FIFO space available Indicates the number of free locations available to be written to in the periodic Tx FIFO. Values are in terms of 32-bit words n: n words available (where 0 UNDER OR EQUAL n UNDER OR EQUAL PTXFD) Others: Reserved
pub type PTXFSAVL_R = crate::FieldReader<u16>;
///Field `PTXQSAV` reader - Periodic transmit request queue space available Indicates the number of free locations available to be written in the periodic transmit request queue. This queue holds both IN and OUT requests. n: n locations available (0 UNDER OR EQUAL n UNDER OR EQUAL 8) Others: Reserved
pub type PTXQSAV_R = crate::FieldReader;
///Field `PTXQTOP` reader - Top of the periodic transmit request queue This indicates the entry in the periodic Tx request queue that is currently being processed by the MAC. This register is used for debugging. Bit 31: Odd/Even frame 0XXXXXXX: send in even frame 1XXXXXXX: send in odd frame Bits 30:27: Channel/endpoint number Bits 26:25: Type XXXXX00X: IN/OUT XXXXX01X: Zero-length packet XXXXX11X: Disable channel command Bit 24: Terminate (last entry for the selected channel/endpoint)
pub type PTXQTOP_R = crate::FieldReader;
impl R {
    ///Bits 0:15 - Periodic transmit data FIFO space available Indicates the number of free locations available to be written to in the periodic Tx FIFO. Values are in terms of 32-bit words n: n words available (where 0 UNDER OR EQUAL n UNDER OR EQUAL PTXFD) Others: Reserved
    #[inline(always)]
    pub fn ptxfsavl(&self) -> PTXFSAVL_R {
        PTXFSAVL_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:23 - Periodic transmit request queue space available Indicates the number of free locations available to be written in the periodic transmit request queue. This queue holds both IN and OUT requests. n: n locations available (0 UNDER OR EQUAL n UNDER OR EQUAL 8) Others: Reserved
    #[inline(always)]
    pub fn ptxqsav(&self) -> PTXQSAV_R {
        PTXQSAV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Top of the periodic transmit request queue This indicates the entry in the periodic Tx request queue that is currently being processed by the MAC. This register is used for debugging. Bit 31: Odd/Even frame 0XXXXXXX: send in even frame 1XXXXXXX: send in odd frame Bits 30:27: Channel/endpoint number Bits 26:25: Type XXXXX00X: IN/OUT XXXXX01X: Zero-length packet XXXXX11X: Disable channel command Bit 24: Terminate (last entry for the selected channel/endpoint)
    #[inline(always)]
    pub fn ptxqtop(&self) -> PTXQTOP_R {
        PTXQTOP_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPTXSTS")
            .field("ptxfsavl", &self.ptxfsavl())
            .field("ptxqsav", &self.ptxqsav())
            .field("ptxqtop", &self.ptxqtop())
            .finish()
    }
}
/**OTG_Host periodic transmit FIFO/queue status register

You can [`read`](crate::Reg::read) this register and get [`hptxsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#OTG_HS:HPTXSTS)*/
pub struct HPTXSTSrs;
impl crate::RegisterSpec for HPTXSTSrs {
    type Ux = u32;
}
///`read()` method returns [`hptxsts::R`](R) reader structure
impl crate::Readable for HPTXSTSrs {}
///`reset()` method sets HPTXSTS to value 0x0008_0100
impl crate::Resettable for HPTXSTSrs {
    const RESET_VALUE: u32 = 0x0008_0100;
}

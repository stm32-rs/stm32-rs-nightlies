///Register `HPTXSTS` reader
pub type R = crate::R<HPTXSTSrs>;
///Field `PTXFSAVL` reader - Periodic transmit data FIFO space available
pub type PTXFSAVL_R = crate::FieldReader<u16>;
///Field `PTXQSAV` reader - Periodic transmit request queue space available
pub type PTXQSAV_R = crate::FieldReader;
///Field `PTXQTOP` reader - Top of the periodic transmit request queue
pub type PTXQTOP_R = crate::FieldReader;
impl R {
    ///Bits 0:15 - Periodic transmit data FIFO space available
    #[inline(always)]
    pub fn ptxfsavl(&self) -> PTXFSAVL_R {
        PTXFSAVL_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:23 - Periodic transmit request queue space available
    #[inline(always)]
    pub fn ptxqsav(&self) -> PTXQSAV_R {
        PTXQSAV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Top of the periodic transmit request queue
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

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#OTG1:HPTXSTS)*/
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

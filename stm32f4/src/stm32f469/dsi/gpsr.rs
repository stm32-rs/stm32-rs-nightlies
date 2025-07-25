///Register `GPSR` reader
pub type R = crate::R<GPSRrs>;
///Field `CMDFE` reader - Command FIFO empty This bit indicates the empty status of the generic command FIFO:
pub type CMDFE_R = crate::BitReader;
///Field `CMDFF` reader - Command FIFO full This bit indicates the full status of the generic command FIFO:
pub type CMDFF_R = crate::BitReader;
///Field `PWRFE` reader - Payload write FIFO empty This bit indicates the empty status of the generic write payload FIFO:
pub type PWRFE_R = crate::BitReader;
///Field `PWRFF` reader - Payload write FIFO full This bit indicates the full status of the generic write payload FIFO:
pub type PWRFF_R = crate::BitReader;
///Field `PRDFE` reader - Payload read FIFO empty This bit indicates the empty status of the generic read payload FIFO:
pub type PRDFE_R = crate::BitReader;
///Field `PRDFF` reader - Payload read FIFO full This bit indicates the full status of the generic read payload FIFO:
pub type PRDFF_R = crate::BitReader;
///Field `RCB` reader - Read command busy This bit is set when a read command is issued and cleared when the entire response is stored in the FIFO:
pub type RCB_R = crate::BitReader;
impl R {
    ///Bit 0 - Command FIFO empty This bit indicates the empty status of the generic command FIFO:
    #[inline(always)]
    pub fn cmdfe(&self) -> CMDFE_R {
        CMDFE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Command FIFO full This bit indicates the full status of the generic command FIFO:
    #[inline(always)]
    pub fn cmdff(&self) -> CMDFF_R {
        CMDFF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Payload write FIFO empty This bit indicates the empty status of the generic write payload FIFO:
    #[inline(always)]
    pub fn pwrfe(&self) -> PWRFE_R {
        PWRFE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Payload write FIFO full This bit indicates the full status of the generic write payload FIFO:
    #[inline(always)]
    pub fn pwrff(&self) -> PWRFF_R {
        PWRFF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Payload read FIFO empty This bit indicates the empty status of the generic read payload FIFO:
    #[inline(always)]
    pub fn prdfe(&self) -> PRDFE_R {
        PRDFE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Payload read FIFO full This bit indicates the full status of the generic read payload FIFO:
    #[inline(always)]
    pub fn prdff(&self) -> PRDFF_R {
        PRDFF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Read command busy This bit is set when a read command is issued and cleared when the entire response is stored in the FIFO:
    #[inline(always)]
    pub fn rcb(&self) -> RCB_R {
        RCB_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPSR")
            .field("cmdfe", &self.cmdfe())
            .field("cmdff", &self.cmdff())
            .field("pwrfe", &self.pwrfe())
            .field("pwrff", &self.pwrff())
            .field("prdfe", &self.prdfe())
            .field("prdff", &self.prdff())
            .field("rcb", &self.rcb())
            .finish()
    }
}
/**DSI Host generic packet status register

You can [`read`](crate::Reg::read) this register and get [`gpsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F469.html#DSI:GPSR)*/
pub struct GPSRrs;
impl crate::RegisterSpec for GPSRrs {
    type Ux = u32;
}
///`read()` method returns [`gpsr::R`](R) reader structure
impl crate::Readable for GPSRrs {}
///`reset()` method sets GPSR to value 0x15
impl crate::Resettable for GPSRrs {
    const RESET_VALUE: u32 = 0x15;
}

#[doc = "Register `DSI_GPSR` reader"]
pub type R = crate::R<DSI_GPSRrs>;
#[doc = "Field `CMDFE` reader - Command FIFO empty This bit indicates the empty status of the generic command FIFO:"]
pub type CMDFE_R = crate::BitReader;
#[doc = "Field `CMDFF` reader - Command FIFO full This bit indicates the full status of the generic command FIFO:"]
pub type CMDFF_R = crate::BitReader;
#[doc = "Field `PWRFE` reader - Payload write FIFO empty This bit indicates the empty status of the generic write payload FIFO:"]
pub type PWRFE_R = crate::BitReader;
#[doc = "Field `PWRFF` reader - Payload write FIFO full This bit indicates the full status of the generic write payload FIFO:"]
pub type PWRFF_R = crate::BitReader;
#[doc = "Field `PRDFE` reader - Payload read FIFO empty This bit indicates the empty status of the generic read payload FIFO:"]
pub type PRDFE_R = crate::BitReader;
#[doc = "Field `PRDFF` reader - Payload read FIFO full This bit indicates the full status of the generic read payload FIFO:"]
pub type PRDFF_R = crate::BitReader;
#[doc = "Field `RCB` reader - Read command busy This bit is set when a read command is issued and cleared when the entire response is stored in the FIFO:"]
pub type RCB_R = crate::BitReader;
#[doc = "Field `CMDBE` reader - Command buffer empty This bit indicates the empty status of the generic payload internal buffer:"]
pub type CMDBE_R = crate::BitReader;
#[doc = "Field `CMDBF` reader - Command buffer full This bit indicates the full status of the generic command internal buffer:"]
pub type CMDBF_R = crate::BitReader;
#[doc = "Field `PBE` reader - Payload buffer empty This bit indicates the empty status of the generic payload internal buffer:"]
pub type PBE_R = crate::BitReader;
#[doc = "Field `PBF` reader - Payload buffer full This bit indicates the full status of the generic payload internal buffer:"]
pub type PBF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Command FIFO empty This bit indicates the empty status of the generic command FIFO:"]
    #[inline(always)]
    pub fn cmdfe(&self) -> CMDFE_R {
        CMDFE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Command FIFO full This bit indicates the full status of the generic command FIFO:"]
    #[inline(always)]
    pub fn cmdff(&self) -> CMDFF_R {
        CMDFF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Payload write FIFO empty This bit indicates the empty status of the generic write payload FIFO:"]
    #[inline(always)]
    pub fn pwrfe(&self) -> PWRFE_R {
        PWRFE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Payload write FIFO full This bit indicates the full status of the generic write payload FIFO:"]
    #[inline(always)]
    pub fn pwrff(&self) -> PWRFF_R {
        PWRFF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Payload read FIFO empty This bit indicates the empty status of the generic read payload FIFO:"]
    #[inline(always)]
    pub fn prdfe(&self) -> PRDFE_R {
        PRDFE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Payload read FIFO full This bit indicates the full status of the generic read payload FIFO:"]
    #[inline(always)]
    pub fn prdff(&self) -> PRDFF_R {
        PRDFF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Read command busy This bit is set when a read command is issued and cleared when the entire response is stored in the FIFO:"]
    #[inline(always)]
    pub fn rcb(&self) -> RCB_R {
        RCB_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 16 - Command buffer empty This bit indicates the empty status of the generic payload internal buffer:"]
    #[inline(always)]
    pub fn cmdbe(&self) -> CMDBE_R {
        CMDBE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Command buffer full This bit indicates the full status of the generic command internal buffer:"]
    #[inline(always)]
    pub fn cmdbf(&self) -> CMDBF_R {
        CMDBF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Payload buffer empty This bit indicates the empty status of the generic payload internal buffer:"]
    #[inline(always)]
    pub fn pbe(&self) -> PBE_R {
        PBE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Payload buffer full This bit indicates the full status of the generic payload internal buffer:"]
    #[inline(always)]
    pub fn pbf(&self) -> PBF_R {
        PBF_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[doc = "DSI Host generic packet status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_gpsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_GPSRrs;
impl crate::RegisterSpec for DSI_GPSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_gpsr::R`](R) reader structure"]
impl crate::Readable for DSI_GPSRrs {}
#[doc = "`reset()` method sets DSI_GPSR to value 0x0005_0015"]
impl crate::Resettable for DSI_GPSRrs {
    const RESET_VALUE: u32 = 0x0005_0015;
}

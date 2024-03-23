#[doc = "Register `GPSR` reader"]
pub type R = crate::R<GPSRrs>;
#[doc = "Field `CMDFE` reader - Command FIFO Empty"]
pub type CMDFE_R = crate::BitReader;
#[doc = "Field `CMDFF` reader - Command FIFO Full"]
pub type CMDFF_R = crate::BitReader;
#[doc = "Field `PWRFE` reader - Payload Write FIFO Empty"]
pub type PWRFE_R = crate::BitReader;
#[doc = "Field `PWRFF` reader - Payload Write FIFO Full"]
pub type PWRFF_R = crate::BitReader;
#[doc = "Field `PRDFE` reader - Payload Read FIFO Empty"]
pub type PRDFE_R = crate::BitReader;
#[doc = "Field `PRDFF` reader - Payload Read FIFO Full"]
pub type PRDFF_R = crate::BitReader;
#[doc = "Field `RCB` reader - Read Command Busy"]
pub type RCB_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Command FIFO Empty"]
    #[inline(always)]
    pub fn cmdfe(&self) -> CMDFE_R {
        CMDFE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Command FIFO Full"]
    #[inline(always)]
    pub fn cmdff(&self) -> CMDFF_R {
        CMDFF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Payload Write FIFO Empty"]
    #[inline(always)]
    pub fn pwrfe(&self) -> PWRFE_R {
        PWRFE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Payload Write FIFO Full"]
    #[inline(always)]
    pub fn pwrff(&self) -> PWRFF_R {
        PWRFF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Payload Read FIFO Empty"]
    #[inline(always)]
    pub fn prdfe(&self) -> PRDFE_R {
        PRDFE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Payload Read FIFO Full"]
    #[inline(always)]
    pub fn prdff(&self) -> PRDFF_R {
        PRDFF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Read Command Busy"]
    #[inline(always)]
    pub fn rcb(&self) -> RCB_R {
        RCB_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "DSI Host Generic Packet Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPSRrs;
impl crate::RegisterSpec for GPSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpsr::R`](R) reader structure"]
impl crate::Readable for GPSRrs {}
#[doc = "`reset()` method sets GPSR to value 0x15"]
impl crate::Resettable for GPSRrs {
    const RESET_VALUE: u32 = 0x15;
}

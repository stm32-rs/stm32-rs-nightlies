#[doc = "Register `OTG_DSTS` reader"]
pub type R = crate::R<OTG_DSTSrs>;
#[doc = "Field `SUSPSTS` reader - SUSPSTS"]
pub type SUSPSTS_R = crate::BitReader;
#[doc = "Field `ENUMSPD` reader - ENUMSPD"]
pub type ENUMSPD_R = crate::FieldReader;
#[doc = "Field `EERR` reader - EERR"]
pub type EERR_R = crate::BitReader;
#[doc = "Field `FNSOF` reader - FNSOF"]
pub type FNSOF_R = crate::FieldReader<u16>;
#[doc = "Field `DEVLNSTS` reader - DEVLNSTS"]
pub type DEVLNSTS_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - SUSPSTS"]
    #[inline(always)]
    pub fn suspsts(&self) -> SUSPSTS_R {
        SUSPSTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - ENUMSPD"]
    #[inline(always)]
    pub fn enumspd(&self) -> ENUMSPD_R {
        ENUMSPD_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - EERR"]
    #[inline(always)]
    pub fn eerr(&self) -> EERR_R {
        EERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:21 - FNSOF"]
    #[inline(always)]
    pub fn fnsof(&self) -> FNSOF_R {
        FNSOF_R::new(((self.bits >> 8) & 0x3fff) as u16)
    }
    #[doc = "Bits 22:23 - DEVLNSTS"]
    #[inline(always)]
    pub fn devlnsts(&self) -> DEVLNSTS_R {
        DEVLNSTS_R::new(((self.bits >> 22) & 3) as u8)
    }
}
#[doc = "This register indicates the status of the core with respect to USB-related events. It must be read on interrupts from the device all interrupts (OTG_DAINT) register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_dsts::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_DSTSrs;
impl crate::RegisterSpec for OTG_DSTSrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_dsts::R`](R) reader structure"]
impl crate::Readable for OTG_DSTSrs {}
#[doc = "`reset()` method sets OTG_DSTS to value 0x10"]
impl crate::Resettable for OTG_DSTSrs {
    const RESET_VALUE: u32 = 0x10;
}

#[doc = "Register `SDSR` reader"]
pub type R = crate::R<SDSRrs>;
#[doc = "Field `RE` reader - Refresh error flag An interrupt is generated if REIE = 1 and RE = 1"]
pub type RE_R = crate::BitReader;
#[doc = "Field `MODES1` reader - Status Mode for Bank 1 This bit defines the Status Mode of SDRAM Bank 1."]
pub type MODES1_R = crate::FieldReader;
#[doc = "Field `MODES2` reader - Status Mode for Bank 2 This bit defines the Status Mode of SDRAM Bank 2."]
pub type MODES2_R = crate::FieldReader;
#[doc = "Field `BUSY` reader - Busy status This bit defines the status of the SDRAM controller after a Command Mode request 1; SDRAM Controller is not ready to accept a new request"]
pub type BUSY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Refresh error flag An interrupt is generated if REIE = 1 and RE = 1"]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Status Mode for Bank 1 This bit defines the Status Mode of SDRAM Bank 1."]
    #[inline(always)]
    pub fn modes1(&self) -> MODES1_R {
        MODES1_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - Status Mode for Bank 2 This bit defines the Status Mode of SDRAM Bank 2."]
    #[inline(always)]
    pub fn modes2(&self) -> MODES2_R {
        MODES2_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Busy status This bit defines the status of the SDRAM controller after a Command Mode request 1; SDRAM Controller is not ready to accept a new request"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "SDRAM status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDSRrs;
impl crate::RegisterSpec for SDSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdsr::R`](R) reader structure"]
impl crate::Readable for SDSRrs {}
#[doc = "`reset()` method sets SDSR to value 0"]
impl crate::Resettable for SDSRrs {
    const RESET_VALUE: u32 = 0;
}

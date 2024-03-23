#[doc = "Register `CSR` reader"]
pub type R = crate::R<CSRrs>;
#[doc = "Field `USR` reader - User data information"]
pub type USR_R = crate::FieldReader<u16>;
#[doc = "Field `CS` reader - Channel A status information"]
pub type CS_R = crate::FieldReader;
#[doc = "Field `SOB` reader - Start Of Block"]
pub type SOB_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - User data information"]
    #[inline(always)]
    pub fn usr(&self) -> USR_R {
        USR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Channel A status information"]
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Start Of Block"]
    #[inline(always)]
    pub fn sob(&self) -> SOB_R {
        SOB_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "Channel Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CSRrs {}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CSRrs {
    const RESET_VALUE: u32 = 0;
}

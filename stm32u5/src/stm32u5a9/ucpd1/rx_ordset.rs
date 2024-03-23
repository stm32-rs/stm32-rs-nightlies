#[doc = "Register `RX_ORDSET` reader"]
pub type R = crate::R<RX_ORDSETrs>;
#[doc = "Field `RXORDSET` reader - RXORDSET"]
pub type RXORDSET_R = crate::FieldReader;
#[doc = "Field `RXSOP3OF4` reader - RXSOP3OF4"]
pub type RXSOP3OF4_R = crate::BitReader;
#[doc = "Field `RXSOPKINVALID` reader - RXSOPKINVALID"]
pub type RXSOPKINVALID_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - RXORDSET"]
    #[inline(always)]
    pub fn rxordset(&self) -> RXORDSET_R {
        RXORDSET_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - RXSOP3OF4"]
    #[inline(always)]
    pub fn rxsop3of4(&self) -> RXSOP3OF4_R {
        RXSOP3OF4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - RXSOPKINVALID"]
    #[inline(always)]
    pub fn rxsopkinvalid(&self) -> RXSOPKINVALID_R {
        RXSOPKINVALID_R::new(((self.bits >> 4) & 7) as u8)
    }
}
#[doc = "UCPD Rx Ordered Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_ordset::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_ORDSETrs;
impl crate::RegisterSpec for RX_ORDSETrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_ordset::R`](R) reader structure"]
impl crate::Readable for RX_ORDSETrs {}
#[doc = "`reset()` method sets RX_ORDSET to value 0"]
impl crate::Resettable for RX_ORDSETrs {
    const RESET_VALUE: u32 = 0;
}

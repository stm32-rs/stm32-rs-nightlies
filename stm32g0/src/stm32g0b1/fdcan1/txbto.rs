#[doc = "Register `TXBTO` reader"]
pub type R = crate::R<TXBTOrs>;
#[doc = "Field `TO` reader - Transmission occurred. Each Tx buffer has its own TO bit. The bits are set when the corresponding TXBRP bit is cleared after a successful transmission. The bits are reset when a new transmission is requested by writing a 1 to the corresponding bit of register TXBAR."]
pub type TO_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Transmission occurred. Each Tx buffer has its own TO bit. The bits are set when the corresponding TXBRP bit is cleared after a successful transmission. The bits are reset when a new transmission is requested by writing a 1 to the corresponding bit of register TXBAR."]
    #[inline(always)]
    pub fn to(&self) -> TO_R {
        TO_R::new((self.bits & 7) as u8)
    }
}
#[doc = "FDCAN Tx buffer transmission occurred register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbto::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXBTOrs;
impl crate::RegisterSpec for TXBTOrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbto::R`](R) reader structure"]
impl crate::Readable for TXBTOrs {}
#[doc = "`reset()` method sets TXBTO to value 0"]
impl crate::Resettable for TXBTOrs {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `ETH_MACATSSR` reader"]
pub type R = crate::R<ETH_MACATSSRrs>;
#[doc = "Field `AUXTSHI` reader - AUXTSHI"]
pub type AUXTSHI_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - AUXTSHI"]
    #[inline(always)]
    pub fn auxtshi(&self) -> AUXTSHI_R {
        AUXTSHI_R::new(self.bits)
    }
}
#[doc = "The Auxiliary Timestamp - Seconds register contains the lower 32 bits of the Seconds field of the auxiliary timestamp register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macatssr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MACATSSRrs;
impl crate::RegisterSpec for ETH_MACATSSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_macatssr::R`](R) reader structure"]
impl crate::Readable for ETH_MACATSSRrs {}
#[doc = "`reset()` method sets ETH_MACATSSR to value 0"]
impl crate::Resettable for ETH_MACATSSRrs {
    const RESET_VALUE: u32 = 0;
}

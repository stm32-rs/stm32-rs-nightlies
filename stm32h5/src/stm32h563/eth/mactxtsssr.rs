#[doc = "Register `MACTXTSSSR` reader"]
pub type R = crate::R<MACTXTSSSRrs>;
#[doc = "Field `TXTSSHI` reader - Transmit Timestamp Status High This field contains the lower 32 bits of the Seconds field of Transmit packet's captured timestamp."]
pub type TXTSSHI_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Transmit Timestamp Status High This field contains the lower 32 bits of the Seconds field of Transmit packet's captured timestamp."]
    #[inline(always)]
    pub fn txtsshi(&self) -> TXTSSHI_R {
        TXTSSHI_R::new(self.bits)
    }
}
#[doc = "Tx timestamp status seconds register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mactxtsssr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACTXTSSSRrs;
impl crate::RegisterSpec for MACTXTSSSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mactxtsssr::R`](R) reader structure"]
impl crate::Readable for MACTXTSSSRrs {}
#[doc = "`reset()` method sets MACTXTSSSR to value 0"]
impl crate::Resettable for MACTXTSSSRrs {
    const RESET_VALUE: u32 = 0;
}

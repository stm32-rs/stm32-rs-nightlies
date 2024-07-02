///Register `RX_ORDSET` reader
pub type R = crate::R<RX_ORDSETrs>;
///Field `RXORDSET` reader - RXORDSET
pub type RXORDSET_R = crate::FieldReader;
///Field `RXSOP3OF4` reader - RXSOP3OF4
pub type RXSOP3OF4_R = crate::BitReader;
///Field `RXSOPKINVALID` reader - RXSOPKINVALID
pub type RXSOPKINVALID_R = crate::FieldReader;
impl R {
    ///Bits 0:2 - RXORDSET
    #[inline(always)]
    pub fn rxordset(&self) -> RXORDSET_R {
        RXORDSET_R::new((self.bits & 7) as u8)
    }
    ///Bit 3 - RXSOP3OF4
    #[inline(always)]
    pub fn rxsop3of4(&self) -> RXSOP3OF4_R {
        RXSOP3OF4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - RXSOPKINVALID
    #[inline(always)]
    pub fn rxsopkinvalid(&self) -> RXSOPKINVALID_R {
        RXSOPKINVALID_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_ORDSET")
            .field("rxordset", &self.rxordset())
            .field("rxsop3of4", &self.rxsop3of4())
            .field("rxsopkinvalid", &self.rxsopkinvalid())
            .finish()
    }
}
/**UCPD Rx Ordered Set Register

You can [`read`](crate::Reg::read) this register and get [`rx_ordset::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G071.html#UCPD1:RX_ORDSET)*/
pub struct RX_ORDSETrs;
impl crate::RegisterSpec for RX_ORDSETrs {
    type Ux = u32;
}
///`read()` method returns [`rx_ordset::R`](R) reader structure
impl crate::Readable for RX_ORDSETrs {}
///`reset()` method sets RX_ORDSET to value 0
impl crate::Resettable for RX_ORDSETrs {
    const RESET_VALUE: u32 = 0;
}

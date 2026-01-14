///Register `RX_ORDSETR` reader
pub type R = crate::R<RX_ORDSETRrs>;
///Field `RXORDSET` reader - Rx ordered set code detected
pub type RXORDSET_R = crate::FieldReader;
///Field `RXSOP3OF4` reader - The bit indicates the number of correct K-codes. For debug purposes only.
pub type RXSOP3OF4_R = crate::BitReader;
///Field `RXSOPKINVALID` reader - The bitfield is for debug purposes only.
pub type RXSOPKINVALID_R = crate::FieldReader;
impl R {
    ///Bits 0:2 - Rx ordered set code detected
    #[inline(always)]
    pub fn rxordset(&self) -> RXORDSET_R {
        RXORDSET_R::new((self.bits & 7) as u8)
    }
    ///Bit 3 - The bit indicates the number of correct K-codes. For debug purposes only.
    #[inline(always)]
    pub fn rxsop3of4(&self) -> RXSOP3OF4_R {
        RXSOP3OF4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - The bitfield is for debug purposes only.
    #[inline(always)]
    pub fn rxsopkinvalid(&self) -> RXSOPKINVALID_R {
        RXSOPKINVALID_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_ORDSETR")
            .field("rxordset", &self.rxordset())
            .field("rxsop3of4", &self.rxsop3of4())
            .field("rxsopkinvalid", &self.rxsopkinvalid())
            .finish()
    }
}
/**UCPD Rx ordered set register

You can [`read`](crate::Reg::read) this register and get [`rx_ordsetr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#UCPD:RX_ORDSETR)*/
pub struct RX_ORDSETRrs;
impl crate::RegisterSpec for RX_ORDSETRrs {
    type Ux = u32;
}
///`read()` method returns [`rx_ordsetr::R`](R) reader structure
impl crate::Readable for RX_ORDSETRrs {}
///`reset()` method sets RX_ORDSETR to value 0
impl crate::Resettable for RX_ORDSETRrs {}

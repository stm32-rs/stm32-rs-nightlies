///Register `CSR` reader
pub type R = crate::R<CSRrs>;
///Field `USR` reader - user data information Bit USR\[0\] is the oldest value, and comes from channel A, USR\[1\] comes channel B. So USR\[n\] bits come from channel A is n is even, otherwise they come from channel B.
pub type USR_R = crate::FieldReader<u16>;
///Field `CS` reader - channel A status information Bit CS\[0\] is the oldest value
pub type CS_R = crate::FieldReader;
///Field `SOB` reader - start of block This bit indicates if the bit CS\[0\] corresponds to the first bit of a new block
pub type SOB_R = crate::BitReader;
impl R {
    ///Bits 0:15 - user data information Bit USR\[0\] is the oldest value, and comes from channel A, USR\[1\] comes channel B. So USR\[n\] bits come from channel A is n is even, otherwise they come from channel B.
    #[inline(always)]
    pub fn usr(&self) -> USR_R {
        USR_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:23 - channel A status information Bit CS\[0\] is the oldest value
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bit 24 - start of block This bit indicates if the bit CS\[0\] corresponds to the first bit of a new block
    #[inline(always)]
    pub fn sob(&self) -> SOB_R {
        SOB_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR")
            .field("usr", &self.usr())
            .field("cs", &self.cs())
            .field("sob", &self.sob())
            .finish()
    }
}
/**SPDIFRX channel status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#SPDIFRX:CSR)*/
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
///`read()` method returns [`csr::R`](R) reader structure
impl crate::Readable for CSRrs {}
///`reset()` method sets CSR to value 0
impl crate::Resettable for CSRrs {}

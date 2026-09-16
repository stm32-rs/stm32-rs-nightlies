///Register `CSR` reader
pub type R = crate::R<CSRrs>;
///Field `USR` reader - User data information
pub type USR_R = crate::FieldReader<u16>;
///Field `CS` reader - Channel A status information
pub type CS_R = crate::FieldReader;
///Field `SOB` reader - Start Of Block
pub type SOB_R = crate::BitReader;
impl R {
    ///Bits 0:15 - User data information
    #[inline(always)]
    pub fn usr(&self) -> USR_R {
        USR_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:23 - Channel A status information
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bit 24 - Start Of Block
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
/**Channel Status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H742.html#SPDIFRX:CSR)*/
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
///`read()` method returns [`csr::R`](R) reader structure
impl crate::Readable for CSRrs {}
///`reset()` method sets CSR to value 0
impl crate::Resettable for CSRrs {}

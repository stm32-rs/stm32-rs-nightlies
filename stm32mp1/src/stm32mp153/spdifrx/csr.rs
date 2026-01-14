///Register `CSR` reader
pub type R = crate::R<CSRrs>;
///Field `USR` reader - USR
pub type USR_R = crate::FieldReader<u16>;
///Field `CS` reader - CS
pub type CS_R = crate::FieldReader;
///Field `SOB` reader - SOB
pub type SOB_R = crate::BitReader;
impl R {
    ///Bits 0:15 - USR
    #[inline(always)]
    pub fn usr(&self) -> USR_R {
        USR_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:23 - CS
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bit 24 - SOB
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
/**Channel status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SPDIFRX:CSR)*/
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
///`read()` method returns [`csr::R`](R) reader structure
impl crate::Readable for CSRrs {}
///`reset()` method sets CSR to value 0
impl crate::Resettable for CSRrs {}

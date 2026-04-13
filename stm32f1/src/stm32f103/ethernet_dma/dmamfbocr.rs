///Register `DMAMFBOCR` reader
pub type R = crate::R<DMAMFBOCRrs>;
///Field `MFC` reader - Missed frames by the controller
pub type MFC_R = crate::FieldReader<u16>;
///Field `OMFC` reader - Overflow bit for missed frame counter
pub type OMFC_R = crate::BitReader;
///Field `MFA` reader - Missed frames by the application
pub type MFA_R = crate::FieldReader<u16>;
///Field `OFOC` reader - Overflow bit for FIFO overflow counter
pub type OFOC_R = crate::BitReader;
impl R {
    ///Bits 0:15 - Missed frames by the controller
    #[inline(always)]
    pub fn mfc(&self) -> MFC_R {
        MFC_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 16 - Overflow bit for missed frame counter
    #[inline(always)]
    pub fn omfc(&self) -> OMFC_R {
        OMFC_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:27 - Missed frames by the application
    #[inline(always)]
    pub fn mfa(&self) -> MFA_R {
        MFA_R::new(((self.bits >> 17) & 0x07ff) as u16)
    }
    ///Bit 28 - Overflow bit for FIFO overflow counter
    #[inline(always)]
    pub fn ofoc(&self) -> OFOC_R {
        OFOC_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAMFBOCR")
            .field("mfc", &self.mfc())
            .field("omfc", &self.omfc())
            .field("mfa", &self.mfa())
            .field("ofoc", &self.ofoc())
            .finish()
    }
}
/**Ethernet DMA missed frame and buffer overflow counter register

You can [`read`](crate::Reg::read) this register and get [`dmamfbocr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F103.html#Ethernet_DMA:DMAMFBOCR)*/
pub struct DMAMFBOCRrs;
impl crate::RegisterSpec for DMAMFBOCRrs {
    type Ux = u32;
}
///`read()` method returns [`dmamfbocr::R`](R) reader structure
impl crate::Readable for DMAMFBOCRrs {}
///`reset()` method sets DMAMFBOCR to value 0
impl crate::Resettable for DMAMFBOCRrs {}

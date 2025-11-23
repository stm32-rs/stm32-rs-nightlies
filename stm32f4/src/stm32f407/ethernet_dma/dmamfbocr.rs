///Register `DMAMFBOCR` reader
pub type R = crate::R<DMAMFBOCRrs>;
///Register `DMAMFBOCR` writer
pub type W = crate::W<DMAMFBOCRrs>;
///Field `MFC` reader - Missed frames by the controller
pub type MFC_R = crate::FieldReader<u16>;
///Field `MFC` writer - Missed frames by the controller
pub type MFC_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `OMFC` reader - Overflow bit for missed frame counter
pub type OMFC_R = crate::BitReader;
///Field `OMFC` writer - Overflow bit for missed frame counter
pub type OMFC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MFA` reader - Missed frames by the application
pub type MFA_R = crate::FieldReader<u16>;
///Field `MFA` writer - Missed frames by the application
pub type MFA_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `OFOC` reader - Overflow bit for FIFO overflow counter
pub type OFOC_R = crate::BitReader;
///Field `OFOC` writer - Overflow bit for FIFO overflow counter
pub type OFOC_W<'a, REG> = crate::BitWriter<'a, REG>;
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
impl W {
    ///Bits 0:15 - Missed frames by the controller
    #[inline(always)]
    pub fn mfc(&mut self) -> MFC_W<'_, DMAMFBOCRrs> {
        MFC_W::new(self, 0)
    }
    ///Bit 16 - Overflow bit for missed frame counter
    #[inline(always)]
    pub fn omfc(&mut self) -> OMFC_W<'_, DMAMFBOCRrs> {
        OMFC_W::new(self, 16)
    }
    ///Bits 17:27 - Missed frames by the application
    #[inline(always)]
    pub fn mfa(&mut self) -> MFA_W<'_, DMAMFBOCRrs> {
        MFA_W::new(self, 17)
    }
    ///Bit 28 - Overflow bit for FIFO overflow counter
    #[inline(always)]
    pub fn ofoc(&mut self) -> OFOC_W<'_, DMAMFBOCRrs> {
        OFOC_W::new(self, 28)
    }
}
/**Ethernet DMA missed frame and buffer overflow counter register

You can [`read`](crate::Reg::read) this register and get [`dmamfbocr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmamfbocr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F407.html#Ethernet_DMA:DMAMFBOCR)*/
pub struct DMAMFBOCRrs;
impl crate::RegisterSpec for DMAMFBOCRrs {
    type Ux = u32;
}
///`read()` method returns [`dmamfbocr::R`](R) reader structure
impl crate::Readable for DMAMFBOCRrs {}
///`write(|w| ..)` method takes [`dmamfbocr::W`](W) writer structure
impl crate::Writable for DMAMFBOCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMAMFBOCR to value 0
impl crate::Resettable for DMAMFBOCRrs {}

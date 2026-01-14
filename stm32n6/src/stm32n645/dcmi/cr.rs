///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `CAPTURE` reader - Capture enable
pub type CAPTURE_R = crate::BitReader;
///Field `CAPTURE` writer - Capture enable
pub type CAPTURE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CM` reader - Capture mode
pub type CM_R = crate::BitReader;
///Field `CM` writer - Capture mode
pub type CM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CROP` reader - Crop feature
pub type CROP_R = crate::BitReader;
///Field `CROP` writer - Crop feature
pub type CROP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JPEG` reader - JPEG format
pub type JPEG_R = crate::BitReader;
///Field `JPEG` writer - JPEG format
pub type JPEG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ESS` reader - Embedded synchronization select
pub type ESS_R = crate::BitReader;
///Field `ESS` writer - Embedded synchronization select
pub type ESS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PCKPOL` reader - Pixel clock polarity
pub type PCKPOL_R = crate::BitReader;
///Field `PCKPOL` writer - Pixel clock polarity
pub type PCKPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSPOL` reader - Horizontal synchronization polarity
pub type HSPOL_R = crate::BitReader;
///Field `HSPOL` writer - Horizontal synchronization polarity
pub type HSPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VSPOL` reader - Vertical synchronization polarity
pub type VSPOL_R = crate::BitReader;
///Field `VSPOL` writer - Vertical synchronization polarity
pub type VSPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FCRC` reader - Frame capture rate control
pub type FCRC_R = crate::FieldReader;
///Field `FCRC` writer - Frame capture rate control
pub type FCRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `EDM` reader - Extended data mode
pub type EDM_R = crate::FieldReader;
///Field `EDM` writer - Extended data mode
pub type EDM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ENABLE` reader - DCMI enable
pub type ENABLE_R = crate::BitReader;
///Field `ENABLE` writer - DCMI enable
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BSM` reader - Byte Select mode
pub type BSM_R = crate::FieldReader;
///Field `BSM` writer - Byte Select mode
pub type BSM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OEBS` reader - Odd/Even Byte Select (Byte Select Start)
pub type OEBS_R = crate::BitReader;
///Field `OEBS` writer - Odd/Even Byte Select (Byte Select Start)
pub type OEBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSM` reader - Line Select mode
pub type LSM_R = crate::BitReader;
///Field `LSM` writer - Line Select mode
pub type LSM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OELS` reader - Odd/Even Line Select (Line Select Start)
pub type OELS_R = crate::BitReader;
///Field `OELS` writer - Odd/Even Line Select (Line Select Start)
pub type OELS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Capture enable
    #[inline(always)]
    pub fn capture(&self) -> CAPTURE_R {
        CAPTURE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Capture mode
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Crop feature
    #[inline(always)]
    pub fn crop(&self) -> CROP_R {
        CROP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - JPEG format
    #[inline(always)]
    pub fn jpeg(&self) -> JPEG_R {
        JPEG_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Embedded synchronization select
    #[inline(always)]
    pub fn ess(&self) -> ESS_R {
        ESS_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Pixel clock polarity
    #[inline(always)]
    pub fn pckpol(&self) -> PCKPOL_R {
        PCKPOL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Horizontal synchronization polarity
    #[inline(always)]
    pub fn hspol(&self) -> HSPOL_R {
        HSPOL_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Vertical synchronization polarity
    #[inline(always)]
    pub fn vspol(&self) -> VSPOL_R {
        VSPOL_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - Frame capture rate control
    #[inline(always)]
    pub fn fcrc(&self) -> FCRC_R {
        FCRC_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - Extended data mode
    #[inline(always)]
    pub fn edm(&self) -> EDM_R {
        EDM_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bit 14 - DCMI enable
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bits 16:17 - Byte Select mode
    #[inline(always)]
    pub fn bsm(&self) -> BSM_R {
        BSM_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 18 - Odd/Even Byte Select (Byte Select Start)
    #[inline(always)]
    pub fn oebs(&self) -> OEBS_R {
        OEBS_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Line Select mode
    #[inline(always)]
    pub fn lsm(&self) -> LSM_R {
        LSM_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Odd/Even Line Select (Line Select Start)
    #[inline(always)]
    pub fn oels(&self) -> OELS_R {
        OELS_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("capture", &self.capture())
            .field("cm", &self.cm())
            .field("crop", &self.crop())
            .field("jpeg", &self.jpeg())
            .field("ess", &self.ess())
            .field("pckpol", &self.pckpol())
            .field("hspol", &self.hspol())
            .field("vspol", &self.vspol())
            .field("fcrc", &self.fcrc())
            .field("edm", &self.edm())
            .field("enable", &self.enable())
            .field("bsm", &self.bsm())
            .field("oebs", &self.oebs())
            .field("lsm", &self.lsm())
            .field("oels", &self.oels())
            .finish()
    }
}
impl W {
    ///Bit 0 - Capture enable
    #[inline(always)]
    pub fn capture(&mut self) -> CAPTURE_W<'_, CRrs> {
        CAPTURE_W::new(self, 0)
    }
    ///Bit 1 - Capture mode
    #[inline(always)]
    pub fn cm(&mut self) -> CM_W<'_, CRrs> {
        CM_W::new(self, 1)
    }
    ///Bit 2 - Crop feature
    #[inline(always)]
    pub fn crop(&mut self) -> CROP_W<'_, CRrs> {
        CROP_W::new(self, 2)
    }
    ///Bit 3 - JPEG format
    #[inline(always)]
    pub fn jpeg(&mut self) -> JPEG_W<'_, CRrs> {
        JPEG_W::new(self, 3)
    }
    ///Bit 4 - Embedded synchronization select
    #[inline(always)]
    pub fn ess(&mut self) -> ESS_W<'_, CRrs> {
        ESS_W::new(self, 4)
    }
    ///Bit 5 - Pixel clock polarity
    #[inline(always)]
    pub fn pckpol(&mut self) -> PCKPOL_W<'_, CRrs> {
        PCKPOL_W::new(self, 5)
    }
    ///Bit 6 - Horizontal synchronization polarity
    #[inline(always)]
    pub fn hspol(&mut self) -> HSPOL_W<'_, CRrs> {
        HSPOL_W::new(self, 6)
    }
    ///Bit 7 - Vertical synchronization polarity
    #[inline(always)]
    pub fn vspol(&mut self) -> VSPOL_W<'_, CRrs> {
        VSPOL_W::new(self, 7)
    }
    ///Bits 8:9 - Frame capture rate control
    #[inline(always)]
    pub fn fcrc(&mut self) -> FCRC_W<'_, CRrs> {
        FCRC_W::new(self, 8)
    }
    ///Bits 10:11 - Extended data mode
    #[inline(always)]
    pub fn edm(&mut self) -> EDM_W<'_, CRrs> {
        EDM_W::new(self, 10)
    }
    ///Bit 14 - DCMI enable
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W<'_, CRrs> {
        ENABLE_W::new(self, 14)
    }
    ///Bits 16:17 - Byte Select mode
    #[inline(always)]
    pub fn bsm(&mut self) -> BSM_W<'_, CRrs> {
        BSM_W::new(self, 16)
    }
    ///Bit 18 - Odd/Even Byte Select (Byte Select Start)
    #[inline(always)]
    pub fn oebs(&mut self) -> OEBS_W<'_, CRrs> {
        OEBS_W::new(self, 18)
    }
    ///Bit 19 - Line Select mode
    #[inline(always)]
    pub fn lsm(&mut self) -> LSM_W<'_, CRrs> {
        LSM_W::new(self, 19)
    }
    ///Bit 20 - Odd/Even Line Select (Line Select Start)
    #[inline(always)]
    pub fn oels(&mut self) -> OELS_W<'_, CRrs> {
        OELS_W::new(self, 20)
    }
}
/**DCMI control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#DCMI:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}

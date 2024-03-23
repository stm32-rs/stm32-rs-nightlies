#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Field `CAPTURE` reader - Capture enable The camera interface waits for the first start of frame, then a DMA request is generated to transfer the received data into the destination memory. In snapshot mode, the CAPTURE bit is automatically cleared at the end of the first frame received. In continuous grab mode, if the software clears this bit while a capture is ongoing, the bit is effectively cleared after the frame end. Note: The DMA controller and all DCMI configuration registers must be programmed correctly before enabling this bit."]
pub type CAPTURE_R = crate::BitReader;
#[doc = "Field `CAPTURE` writer - Capture enable The camera interface waits for the first start of frame, then a DMA request is generated to transfer the received data into the destination memory. In snapshot mode, the CAPTURE bit is automatically cleared at the end of the first frame received. In continuous grab mode, if the software clears this bit while a capture is ongoing, the bit is effectively cleared after the frame end. Note: The DMA controller and all DCMI configuration registers must be programmed correctly before enabling this bit."]
pub type CAPTURE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CM` reader - Capture mode"]
pub type CM_R = crate::BitReader;
#[doc = "Field `CM` writer - Capture mode"]
pub type CM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CROP` reader - Crop feature"]
pub type CROP_R = crate::BitReader;
#[doc = "Field `CROP` writer - Crop feature"]
pub type CROP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JPEG` reader - JPEG format"]
pub type JPEG_R = crate::BitReader;
#[doc = "Field `JPEG` writer - JPEG format"]
pub type JPEG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESS` reader - Embedded synchronization select Note: Valid only for 8-bit parallel data. HSPOL/VSPOL are ignored when the ESS bit is set. This bit is disabled in JPEG mode."]
pub type ESS_R = crate::BitReader;
#[doc = "Field `ESS` writer - Embedded synchronization select Note: Valid only for 8-bit parallel data. HSPOL/VSPOL are ignored when the ESS bit is set. This bit is disabled in JPEG mode."]
pub type ESS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCKPOL` reader - Pixel clock polarity This bit configures the capture edge of the pixel clock."]
pub type PCKPOL_R = crate::BitReader;
#[doc = "Field `PCKPOL` writer - Pixel clock polarity This bit configures the capture edge of the pixel clock."]
pub type PCKPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSPOL` reader - Horizontal synchronization polarity This bit indicates the level on the DCMI_HSYNC pin when the data are not valid on the parallel interface."]
pub type HSPOL_R = crate::BitReader;
#[doc = "Field `HSPOL` writer - Horizontal synchronization polarity This bit indicates the level on the DCMI_HSYNC pin when the data are not valid on the parallel interface."]
pub type HSPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSPOL` reader - Vertical synchronization polarity This bit indicates the level on the DCMI_VSYNC pin when the data are not valid on the parallel interface."]
pub type VSPOL_R = crate::BitReader;
#[doc = "Field `VSPOL` writer - Vertical synchronization polarity This bit indicates the level on the DCMI_VSYNC pin when the data are not valid on the parallel interface."]
pub type VSPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCRC` reader - Frame capture rate control These bits define the frequency of frame capture. They are meaningful only in Continuous grab mode. They are ignored in snapshot mode."]
pub type FCRC_R = crate::FieldReader;
#[doc = "Field `FCRC` writer - Frame capture rate control These bits define the frequency of frame capture. They are meaningful only in Continuous grab mode. They are ignored in snapshot mode."]
pub type FCRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EDM` reader - Extended data mode"]
pub type EDM_R = crate::FieldReader;
#[doc = "Field `EDM` writer - Extended data mode"]
pub type EDM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ENABLE` reader - DCMI enable Note: The DCMI configuration registers must be programmed correctly before enabling this bit."]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - DCMI enable Note: The DCMI configuration registers must be programmed correctly before enabling this bit."]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSM` reader - Byte Select mode Note: This mode only works for EDM\\[1:0\\]
= 00. For all other EDM values, this field must be programmed to the reset value."]
pub type BSM_R = crate::FieldReader;
#[doc = "Field `BSM` writer - Byte Select mode Note: This mode only works for EDM\\[1:0\\]
= 00. For all other EDM values, this field must be programmed to the reset value."]
pub type BSM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OEBS` reader - Odd/Even Byte Select (Byte Select Start) This bit works in conjunction with BSM field (BSM ≠ 00)."]
pub type OEBS_R = crate::BitReader;
#[doc = "Field `OEBS` writer - Odd/Even Byte Select (Byte Select Start) This bit works in conjunction with BSM field (BSM ≠ 00)."]
pub type OEBS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSM` reader - Line Select mode"]
pub type LSM_R = crate::BitReader;
#[doc = "Field `LSM` writer - Line Select mode"]
pub type LSM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OELS` reader - Odd/Even Line Select (Line Select Start) This bit works in conjunction with the LSM field (LSM = 1)."]
pub type OELS_R = crate::BitReader;
#[doc = "Field `OELS` writer - Odd/Even Line Select (Line Select Start) This bit works in conjunction with the LSM field (LSM = 1)."]
pub type OELS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Capture enable The camera interface waits for the first start of frame, then a DMA request is generated to transfer the received data into the destination memory. In snapshot mode, the CAPTURE bit is automatically cleared at the end of the first frame received. In continuous grab mode, if the software clears this bit while a capture is ongoing, the bit is effectively cleared after the frame end. Note: The DMA controller and all DCMI configuration registers must be programmed correctly before enabling this bit."]
    #[inline(always)]
    pub fn capture(&self) -> CAPTURE_R {
        CAPTURE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture mode"]
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Crop feature"]
    #[inline(always)]
    pub fn crop(&self) -> CROP_R {
        CROP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - JPEG format"]
    #[inline(always)]
    pub fn jpeg(&self) -> JPEG_R {
        JPEG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Embedded synchronization select Note: Valid only for 8-bit parallel data. HSPOL/VSPOL are ignored when the ESS bit is set. This bit is disabled in JPEG mode."]
    #[inline(always)]
    pub fn ess(&self) -> ESS_R {
        ESS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pixel clock polarity This bit configures the capture edge of the pixel clock."]
    #[inline(always)]
    pub fn pckpol(&self) -> PCKPOL_R {
        PCKPOL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Horizontal synchronization polarity This bit indicates the level on the DCMI_HSYNC pin when the data are not valid on the parallel interface."]
    #[inline(always)]
    pub fn hspol(&self) -> HSPOL_R {
        HSPOL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Vertical synchronization polarity This bit indicates the level on the DCMI_VSYNC pin when the data are not valid on the parallel interface."]
    #[inline(always)]
    pub fn vspol(&self) -> VSPOL_R {
        VSPOL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Frame capture rate control These bits define the frequency of frame capture. They are meaningful only in Continuous grab mode. They are ignored in snapshot mode."]
    #[inline(always)]
    pub fn fcrc(&self) -> FCRC_R {
        FCRC_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Extended data mode"]
    #[inline(always)]
    pub fn edm(&self) -> EDM_R {
        EDM_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 14 - DCMI enable Note: The DCMI configuration registers must be programmed correctly before enabling this bit."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Byte Select mode Note: This mode only works for EDM\\[1:0\\]
= 00. For all other EDM values, this field must be programmed to the reset value."]
    #[inline(always)]
    pub fn bsm(&self) -> BSM_R {
        BSM_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Odd/Even Byte Select (Byte Select Start) This bit works in conjunction with BSM field (BSM ≠ 00)."]
    #[inline(always)]
    pub fn oebs(&self) -> OEBS_R {
        OEBS_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Line Select mode"]
    #[inline(always)]
    pub fn lsm(&self) -> LSM_R {
        LSM_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Odd/Even Line Select (Line Select Start) This bit works in conjunction with the LSM field (LSM = 1)."]
    #[inline(always)]
    pub fn oels(&self) -> OELS_R {
        OELS_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture enable The camera interface waits for the first start of frame, then a DMA request is generated to transfer the received data into the destination memory. In snapshot mode, the CAPTURE bit is automatically cleared at the end of the first frame received. In continuous grab mode, if the software clears this bit while a capture is ongoing, the bit is effectively cleared after the frame end. Note: The DMA controller and all DCMI configuration registers must be programmed correctly before enabling this bit."]
    #[inline(always)]
    #[must_use]
    pub fn capture(&mut self) -> CAPTURE_W<CRrs> {
        CAPTURE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Capture mode"]
    #[inline(always)]
    #[must_use]
    pub fn cm(&mut self) -> CM_W<CRrs> {
        CM_W::new(self, 1)
    }
    #[doc = "Bit 2 - Crop feature"]
    #[inline(always)]
    #[must_use]
    pub fn crop(&mut self) -> CROP_W<CRrs> {
        CROP_W::new(self, 2)
    }
    #[doc = "Bit 3 - JPEG format"]
    #[inline(always)]
    #[must_use]
    pub fn jpeg(&mut self) -> JPEG_W<CRrs> {
        JPEG_W::new(self, 3)
    }
    #[doc = "Bit 4 - Embedded synchronization select Note: Valid only for 8-bit parallel data. HSPOL/VSPOL are ignored when the ESS bit is set. This bit is disabled in JPEG mode."]
    #[inline(always)]
    #[must_use]
    pub fn ess(&mut self) -> ESS_W<CRrs> {
        ESS_W::new(self, 4)
    }
    #[doc = "Bit 5 - Pixel clock polarity This bit configures the capture edge of the pixel clock."]
    #[inline(always)]
    #[must_use]
    pub fn pckpol(&mut self) -> PCKPOL_W<CRrs> {
        PCKPOL_W::new(self, 5)
    }
    #[doc = "Bit 6 - Horizontal synchronization polarity This bit indicates the level on the DCMI_HSYNC pin when the data are not valid on the parallel interface."]
    #[inline(always)]
    #[must_use]
    pub fn hspol(&mut self) -> HSPOL_W<CRrs> {
        HSPOL_W::new(self, 6)
    }
    #[doc = "Bit 7 - Vertical synchronization polarity This bit indicates the level on the DCMI_VSYNC pin when the data are not valid on the parallel interface."]
    #[inline(always)]
    #[must_use]
    pub fn vspol(&mut self) -> VSPOL_W<CRrs> {
        VSPOL_W::new(self, 7)
    }
    #[doc = "Bits 8:9 - Frame capture rate control These bits define the frequency of frame capture. They are meaningful only in Continuous grab mode. They are ignored in snapshot mode."]
    #[inline(always)]
    #[must_use]
    pub fn fcrc(&mut self) -> FCRC_W<CRrs> {
        FCRC_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Extended data mode"]
    #[inline(always)]
    #[must_use]
    pub fn edm(&mut self) -> EDM_W<CRrs> {
        EDM_W::new(self, 10)
    }
    #[doc = "Bit 14 - DCMI enable Note: The DCMI configuration registers must be programmed correctly before enabling this bit."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<CRrs> {
        ENABLE_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Byte Select mode Note: This mode only works for EDM\\[1:0\\]
= 00. For all other EDM values, this field must be programmed to the reset value."]
    #[inline(always)]
    #[must_use]
    pub fn bsm(&mut self) -> BSM_W<CRrs> {
        BSM_W::new(self, 16)
    }
    #[doc = "Bit 18 - Odd/Even Byte Select (Byte Select Start) This bit works in conjunction with BSM field (BSM ≠ 00)."]
    #[inline(always)]
    #[must_use]
    pub fn oebs(&mut self) -> OEBS_W<CRrs> {
        OEBS_W::new(self, 18)
    }
    #[doc = "Bit 19 - Line Select mode"]
    #[inline(always)]
    #[must_use]
    pub fn lsm(&mut self) -> LSM_W<CRrs> {
        LSM_W::new(self, 19)
    }
    #[doc = "Bit 20 - Odd/Even Line Select (Line Select Start) This bit works in conjunction with the LSM field (LSM = 1)."]
    #[inline(always)]
    #[must_use]
    pub fn oels(&mut self) -> OELS_W<CRrs> {
        OELS_W::new(self, 20)
    }
}
#[doc = "DCMI control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CRrs {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}

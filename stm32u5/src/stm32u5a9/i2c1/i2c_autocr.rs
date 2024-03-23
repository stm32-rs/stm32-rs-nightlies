#[doc = "Register `I2C_AUTOCR` reader"]
pub type R = crate::R<I2C_AUTOCRrs>;
#[doc = "Register `I2C_AUTOCR` writer"]
pub type W = crate::W<I2C_AUTOCRrs>;
#[doc = "Field `TCDMAEN` reader - DMA request enable on Transfer Complete event"]
pub type TCDMAEN_R = crate::BitReader;
#[doc = "Field `TCDMAEN` writer - DMA request enable on Transfer Complete event"]
pub type TCDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCRDMAEN` reader - DMA request enable on Transfer Complete Reload event"]
pub type TCRDMAEN_R = crate::BitReader;
#[doc = "Field `TCRDMAEN` writer - DMA request enable on Transfer Complete Reload event"]
pub type TCRDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRIGSEL` reader - Trigger selection"]
pub type TRIGSEL_R = crate::FieldReader;
#[doc = "Field `TRIGSEL` writer - Trigger selection"]
pub type TRIGSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRIGPOL` reader - Trigger polarity"]
pub type TRIGPOL_R = crate::BitReader;
#[doc = "Field `TRIGPOL` writer - Trigger polarity"]
pub type TRIGPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRIGEN` reader - Trigger enable"]
pub type TRIGEN_R = crate::BitReader;
#[doc = "Field `TRIGEN` writer - Trigger enable"]
pub type TRIGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 6 - DMA request enable on Transfer Complete event"]
    #[inline(always)]
    pub fn tcdmaen(&self) -> TCDMAEN_R {
        TCDMAEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA request enable on Transfer Complete Reload event"]
    #[inline(always)]
    pub fn tcrdmaen(&self) -> TCRDMAEN_R {
        TCRDMAEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Trigger selection"]
    #[inline(always)]
    pub fn trigsel(&self) -> TRIGSEL_R {
        TRIGSEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Trigger polarity"]
    #[inline(always)]
    pub fn trigpol(&self) -> TRIGPOL_R {
        TRIGPOL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Trigger enable"]
    #[inline(always)]
    pub fn trigen(&self) -> TRIGEN_R {
        TRIGEN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - DMA request enable on Transfer Complete event"]
    #[inline(always)]
    #[must_use]
    pub fn tcdmaen(&mut self) -> TCDMAEN_W<I2C_AUTOCRrs> {
        TCDMAEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - DMA request enable on Transfer Complete Reload event"]
    #[inline(always)]
    #[must_use]
    pub fn tcrdmaen(&mut self) -> TCRDMAEN_W<I2C_AUTOCRrs> {
        TCRDMAEN_W::new(self, 7)
    }
    #[doc = "Bits 16:19 - Trigger selection"]
    #[inline(always)]
    #[must_use]
    pub fn trigsel(&mut self) -> TRIGSEL_W<I2C_AUTOCRrs> {
        TRIGSEL_W::new(self, 16)
    }
    #[doc = "Bit 20 - Trigger polarity"]
    #[inline(always)]
    #[must_use]
    pub fn trigpol(&mut self) -> TRIGPOL_W<I2C_AUTOCRrs> {
        TRIGPOL_W::new(self, 20)
    }
    #[doc = "Bit 21 - Trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn trigen(&mut self) -> TRIGEN_W<I2C_AUTOCRrs> {
        TRIGEN_W::new(self, 21)
    }
}
#[doc = "I2C Autonomous mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_autocr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_autocr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C_AUTOCRrs;
impl crate::RegisterSpec for I2C_AUTOCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_autocr::R`](R) reader structure"]
impl crate::Readable for I2C_AUTOCRrs {}
#[doc = "`write(|w| ..)` method takes [`i2c_autocr::W`](W) writer structure"]
impl crate::Writable for I2C_AUTOCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2C_AUTOCR to value 0"]
impl crate::Resettable for I2C_AUTOCRrs {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `CR3` reader"]
pub type R = crate::R<CR3rs>;
#[doc = "Register `CR3` writer"]
pub type W = crate::W<CR3rs>;
#[doc = "Field `BYPASS` reader - Power management unit bypass"]
pub type BYPASS_R = crate::BitReader;
#[doc = "Field `BYPASS` writer - Power management unit bypass"]
pub type BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LDOEN` reader - Low drop-out regulator enable"]
pub type LDOEN_R = crate::BitReader;
#[doc = "Field `LDOEN` writer - Low drop-out regulator enable"]
pub type LDOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMPSEN` reader - SMPSEN"]
pub type SMPSEN_R = crate::BitReader;
#[doc = "Field `SMPSEN` writer - SMPSEN"]
pub type SMPSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMPSEXTHP` reader - SMPSEXTHP"]
pub type SMPSEXTHP_R = crate::BitReader;
#[doc = "Field `SMPSEXTHP` writer - SMPSEXTHP"]
pub type SMPSEXTHP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMPSLEVEL` reader - SMPSLEVEL"]
pub type SMPSLEVEL_R = crate::FieldReader;
#[doc = "Field `SMPSLEVEL` writer - SMPSLEVEL"]
pub type SMPSLEVEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VBE` reader - VBAT charging enable"]
pub type VBE_R = crate::BitReader;
#[doc = "Field `VBE` writer - VBAT charging enable"]
pub type VBE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBRS` reader - VBAT charging resistor selection"]
pub type VBRS_R = crate::BitReader;
#[doc = "Field `VBRS` writer - VBAT charging resistor selection"]
pub type VBRS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMPSEXTRDY` reader - SMPSEXTRDY"]
pub type SMPSEXTRDY_R = crate::BitReader;
#[doc = "Field `USB33DEN` reader - VDD33USB voltage level detector enable."]
pub type USB33DEN_R = crate::BitReader;
#[doc = "Field `USB33DEN` writer - VDD33USB voltage level detector enable."]
pub type USB33DEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBREGEN` reader - USB regulator enable."]
pub type USBREGEN_R = crate::BitReader;
#[doc = "Field `USBREGEN` writer - USB regulator enable."]
pub type USBREGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB33RDY` reader - USB supply ready."]
pub type USB33RDY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Power management unit bypass"]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Low drop-out regulator enable"]
    #[inline(always)]
    pub fn ldoen(&self) -> LDOEN_R {
        LDOEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SMPSEN"]
    #[inline(always)]
    pub fn smpsen(&self) -> SMPSEN_R {
        SMPSEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SMPSEXTHP"]
    #[inline(always)]
    pub fn smpsexthp(&self) -> SMPSEXTHP_R {
        SMPSEXTHP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - SMPSLEVEL"]
    #[inline(always)]
    pub fn smpslevel(&self) -> SMPSLEVEL_R {
        SMPSLEVEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - VBAT charging enable"]
    #[inline(always)]
    pub fn vbe(&self) -> VBE_R {
        VBE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - VBAT charging resistor selection"]
    #[inline(always)]
    pub fn vbrs(&self) -> VBRS_R {
        VBRS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - SMPSEXTRDY"]
    #[inline(always)]
    pub fn smpsextrdy(&self) -> SMPSEXTRDY_R {
        SMPSEXTRDY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - VDD33USB voltage level detector enable."]
    #[inline(always)]
    pub fn usb33den(&self) -> USB33DEN_R {
        USB33DEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - USB regulator enable."]
    #[inline(always)]
    pub fn usbregen(&self) -> USBREGEN_R {
        USBREGEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - USB supply ready."]
    #[inline(always)]
    pub fn usb33rdy(&self) -> USB33RDY_R {
        USB33RDY_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power management unit bypass"]
    #[inline(always)]
    #[must_use]
    pub fn bypass(&mut self) -> BYPASS_W<CR3rs> {
        BYPASS_W::new(self, 0)
    }
    #[doc = "Bit 1 - Low drop-out regulator enable"]
    #[inline(always)]
    #[must_use]
    pub fn ldoen(&mut self) -> LDOEN_W<CR3rs> {
        LDOEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - SMPSEN"]
    #[inline(always)]
    #[must_use]
    pub fn smpsen(&mut self) -> SMPSEN_W<CR3rs> {
        SMPSEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - SMPSEXTHP"]
    #[inline(always)]
    #[must_use]
    pub fn smpsexthp(&mut self) -> SMPSEXTHP_W<CR3rs> {
        SMPSEXTHP_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - SMPSLEVEL"]
    #[inline(always)]
    #[must_use]
    pub fn smpslevel(&mut self) -> SMPSLEVEL_W<CR3rs> {
        SMPSLEVEL_W::new(self, 4)
    }
    #[doc = "Bit 8 - VBAT charging enable"]
    #[inline(always)]
    #[must_use]
    pub fn vbe(&mut self) -> VBE_W<CR3rs> {
        VBE_W::new(self, 8)
    }
    #[doc = "Bit 9 - VBAT charging resistor selection"]
    #[inline(always)]
    #[must_use]
    pub fn vbrs(&mut self) -> VBRS_W<CR3rs> {
        VBRS_W::new(self, 9)
    }
    #[doc = "Bit 24 - VDD33USB voltage level detector enable."]
    #[inline(always)]
    #[must_use]
    pub fn usb33den(&mut self) -> USB33DEN_W<CR3rs> {
        USB33DEN_W::new(self, 24)
    }
    #[doc = "Bit 25 - USB regulator enable."]
    #[inline(always)]
    #[must_use]
    pub fn usbregen(&mut self) -> USBREGEN_W<CR3rs> {
        USBREGEN_W::new(self, 25)
    }
}
#[doc = "Reset only by POR only, not reset by wakeup from Standby mode and RESET pad. The lower byte of this register is written once after POR and shall be written before changing VOS level or ck_sys clock frequency. No limitation applies to the upper bytes.Programming data corresponding to an invalid combination of SDLEVEL, SDEXTHP, SDEN, LDOEN and BYPASS bits (see Table9) will be ignored: data will not be written, the written-once mechanism will lock the register and any further write access will be ignored. The default supply configuration will be kept and the ACTVOSRDY bit in PWR control status register 1 (PWR_CSR1) will go on indicating invalid voltage levels. The system shall be power cycled before writing a new value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR3rs;
impl crate::RegisterSpec for CR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr3::R`](R) reader structure"]
impl crate::Readable for CR3rs {}
#[doc = "`write(|w| ..)` method takes [`cr3::W`](W) writer structure"]
impl crate::Writable for CR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR3 to value 0x06"]
impl crate::Resettable for CR3rs {
    const RESET_VALUE: u32 = 0x06;
}

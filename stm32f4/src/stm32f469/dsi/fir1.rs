#[doc = "Register `FIR1` reader"]
pub type R = crate::R<FIR1rs>;
#[doc = "Register `FIR1` writer"]
pub type W = crate::W<FIR1rs>;
#[doc = "Field `FTOHSTX` reader - Force Timeout High-Speed Transmission"]
pub type FTOHSTX_R = crate::BitReader;
#[doc = "Field `FTOHSTX` writer - Force Timeout High-Speed Transmission"]
pub type FTOHSTX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTOLPRX` reader - Force Timeout Low-Power Reception"]
pub type FTOLPRX_R = crate::BitReader;
#[doc = "Field `FTOLPRX` writer - Force Timeout Low-Power Reception"]
pub type FTOLPRX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FECCSE` reader - Force ECC Single-bit Error"]
pub type FECCSE_R = crate::BitReader;
#[doc = "Field `FECCSE` writer - Force ECC Single-bit Error"]
pub type FECCSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FECCME` reader - Force ECC Multi-bit Error"]
pub type FECCME_R = crate::BitReader;
#[doc = "Field `FECCME` writer - Force ECC Multi-bit Error"]
pub type FECCME_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCRCE` reader - Force CRC Error"]
pub type FCRCE_R = crate::BitReader;
#[doc = "Field `FCRCE` writer - Force CRC Error"]
pub type FCRCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPSE` reader - Force Packet Size Error"]
pub type FPSE_R = crate::BitReader;
#[doc = "Field `FPSE` writer - Force Packet Size Error"]
pub type FPSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEOTPE` reader - Force EoTp Error"]
pub type FEOTPE_R = crate::BitReader;
#[doc = "Field `FEOTPE` writer - Force EoTp Error"]
pub type FEOTPE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLPWRE` reader - Force LTDC Payload Write Error"]
pub type FLPWRE_R = crate::BitReader;
#[doc = "Field `FLPWRE` writer - Force LTDC Payload Write Error"]
pub type FLPWRE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FGCWRE` reader - Force Generic Command Write Error"]
pub type FGCWRE_R = crate::BitReader;
#[doc = "Field `FGCWRE` writer - Force Generic Command Write Error"]
pub type FGCWRE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FGPWRE` reader - Force Generic Payload Write Error"]
pub type FGPWRE_R = crate::BitReader;
#[doc = "Field `FGPWRE` writer - Force Generic Payload Write Error"]
pub type FGPWRE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FGPTXE` reader - Force Generic Payload Transmit Error"]
pub type FGPTXE_R = crate::BitReader;
#[doc = "Field `FGPTXE` writer - Force Generic Payload Transmit Error"]
pub type FGPTXE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FGPRDE` reader - Force Generic Payload Read Error"]
pub type FGPRDE_R = crate::BitReader;
#[doc = "Field `FGPRDE` writer - Force Generic Payload Read Error"]
pub type FGPRDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FGPRXE` reader - Force Generic Payload Receive Error"]
pub type FGPRXE_R = crate::BitReader;
#[doc = "Field `FGPRXE` writer - Force Generic Payload Receive Error"]
pub type FGPRXE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Force Timeout High-Speed Transmission"]
    #[inline(always)]
    pub fn ftohstx(&self) -> FTOHSTX_R {
        FTOHSTX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Force Timeout Low-Power Reception"]
    #[inline(always)]
    pub fn ftolprx(&self) -> FTOLPRX_R {
        FTOLPRX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Force ECC Single-bit Error"]
    #[inline(always)]
    pub fn feccse(&self) -> FECCSE_R {
        FECCSE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Force ECC Multi-bit Error"]
    #[inline(always)]
    pub fn feccme(&self) -> FECCME_R {
        FECCME_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Force CRC Error"]
    #[inline(always)]
    pub fn fcrce(&self) -> FCRCE_R {
        FCRCE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Force Packet Size Error"]
    #[inline(always)]
    pub fn fpse(&self) -> FPSE_R {
        FPSE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Force EoTp Error"]
    #[inline(always)]
    pub fn feotpe(&self) -> FEOTPE_R {
        FEOTPE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Force LTDC Payload Write Error"]
    #[inline(always)]
    pub fn flpwre(&self) -> FLPWRE_R {
        FLPWRE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Force Generic Command Write Error"]
    #[inline(always)]
    pub fn fgcwre(&self) -> FGCWRE_R {
        FGCWRE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Force Generic Payload Write Error"]
    #[inline(always)]
    pub fn fgpwre(&self) -> FGPWRE_R {
        FGPWRE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Force Generic Payload Transmit Error"]
    #[inline(always)]
    pub fn fgptxe(&self) -> FGPTXE_R {
        FGPTXE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Force Generic Payload Read Error"]
    #[inline(always)]
    pub fn fgprde(&self) -> FGPRDE_R {
        FGPRDE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Force Generic Payload Receive Error"]
    #[inline(always)]
    pub fn fgprxe(&self) -> FGPRXE_R {
        FGPRXE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Force Timeout High-Speed Transmission"]
    #[inline(always)]
    #[must_use]
    pub fn ftohstx(&mut self) -> FTOHSTX_W<FIR1rs> {
        FTOHSTX_W::new(self, 0)
    }
    #[doc = "Bit 1 - Force Timeout Low-Power Reception"]
    #[inline(always)]
    #[must_use]
    pub fn ftolprx(&mut self) -> FTOLPRX_W<FIR1rs> {
        FTOLPRX_W::new(self, 1)
    }
    #[doc = "Bit 2 - Force ECC Single-bit Error"]
    #[inline(always)]
    #[must_use]
    pub fn feccse(&mut self) -> FECCSE_W<FIR1rs> {
        FECCSE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Force ECC Multi-bit Error"]
    #[inline(always)]
    #[must_use]
    pub fn feccme(&mut self) -> FECCME_W<FIR1rs> {
        FECCME_W::new(self, 3)
    }
    #[doc = "Bit 4 - Force CRC Error"]
    #[inline(always)]
    #[must_use]
    pub fn fcrce(&mut self) -> FCRCE_W<FIR1rs> {
        FCRCE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Force Packet Size Error"]
    #[inline(always)]
    #[must_use]
    pub fn fpse(&mut self) -> FPSE_W<FIR1rs> {
        FPSE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Force EoTp Error"]
    #[inline(always)]
    #[must_use]
    pub fn feotpe(&mut self) -> FEOTPE_W<FIR1rs> {
        FEOTPE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Force LTDC Payload Write Error"]
    #[inline(always)]
    #[must_use]
    pub fn flpwre(&mut self) -> FLPWRE_W<FIR1rs> {
        FLPWRE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Force Generic Command Write Error"]
    #[inline(always)]
    #[must_use]
    pub fn fgcwre(&mut self) -> FGCWRE_W<FIR1rs> {
        FGCWRE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Force Generic Payload Write Error"]
    #[inline(always)]
    #[must_use]
    pub fn fgpwre(&mut self) -> FGPWRE_W<FIR1rs> {
        FGPWRE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Force Generic Payload Transmit Error"]
    #[inline(always)]
    #[must_use]
    pub fn fgptxe(&mut self) -> FGPTXE_W<FIR1rs> {
        FGPTXE_W::new(self, 10)
    }
    #[doc = "Bit 11 - Force Generic Payload Read Error"]
    #[inline(always)]
    #[must_use]
    pub fn fgprde(&mut self) -> FGPRDE_W<FIR1rs> {
        FGPRDE_W::new(self, 11)
    }
    #[doc = "Bit 12 - Force Generic Payload Receive Error"]
    #[inline(always)]
    #[must_use]
    pub fn fgprxe(&mut self) -> FGPRXE_W<FIR1rs> {
        FGPRXE_W::new(self, 12)
    }
}
#[doc = "DSI Host Force Interrupt Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fir1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fir1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIR1rs;
impl crate::RegisterSpec for FIR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fir1::R`](R) reader structure"]
impl crate::Readable for FIR1rs {}
#[doc = "`write(|w| ..)` method takes [`fir1::W`](W) writer structure"]
impl crate::Writable for FIR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIR1 to value 0"]
impl crate::Resettable for FIR1rs {
    const RESET_VALUE: u32 = 0;
}

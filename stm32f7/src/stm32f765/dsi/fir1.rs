#[doc = "Register `FIR1` writer"]
pub type W = crate::W<FIR1rs>;
#[doc = "Field `FTOHSTX` writer - Force Timeout High-Speed Transmission"]
pub type FTOHSTX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTOLPRX` writer - Force Timeout Low-Power Reception"]
pub type FTOLPRX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FECCSE` writer - Force ECC Single-bit Error"]
pub type FECCSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FECCME` writer - Force ECC Multi-bit Error"]
pub type FECCME_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCRCE` writer - Force CRC Error"]
pub type FCRCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPSE` writer - Force Packet Size Error"]
pub type FPSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEOTPE` writer - Force EoTp Error"]
pub type FEOTPE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLPWRE` writer - Force LTDC Payload Write Error"]
pub type FLPWRE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FGCWRE` writer - Force Generic Command Write Error"]
pub type FGCWRE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FGPWRE` writer - Force Generic Payload Write Error"]
pub type FGPWRE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FGPTXE` writer - Force Generic Payload Transmit Error"]
pub type FGPTXE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FGPRDE` writer - Force Generic Payload Read Error"]
pub type FGPRDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FGPRXE` writer - Force Generic Payload Receive Error"]
pub type FGPRXE_W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "DSI Host Force Interrupt Register 1\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fir1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIR1rs;
impl crate::RegisterSpec for FIR1rs {
    type Ux = u32;
}
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

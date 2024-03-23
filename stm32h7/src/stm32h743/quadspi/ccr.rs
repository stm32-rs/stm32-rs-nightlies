#[doc = "Register `CCR` reader"]
pub type R = crate::R<CCRrs>;
#[doc = "Register `CCR` writer"]
pub type W = crate::W<CCRrs>;
#[doc = "Field `INSTRUCTION` reader - Instruction Instruction to be send to the external SPI device. This field can be written only when BUSY = 0."]
pub type INSTRUCTION_R = crate::FieldReader;
#[doc = "Field `INSTRUCTION` writer - Instruction Instruction to be send to the external SPI device. This field can be written only when BUSY = 0."]
pub type INSTRUCTION_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `IMODE` reader - Instruction mode This field defines the instruction phase mode of operation: This field can be written only when BUSY = 0."]
pub type IMODE_R = crate::FieldReader;
#[doc = "Field `IMODE` writer - Instruction mode This field defines the instruction phase mode of operation: This field can be written only when BUSY = 0."]
pub type IMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADMODE` reader - Address mode This field defines the address phase mode of operation: This field can be written only when BUSY = 0."]
pub type ADMODE_R = crate::FieldReader;
#[doc = "Field `ADMODE` writer - Address mode This field defines the address phase mode of operation: This field can be written only when BUSY = 0."]
pub type ADMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADSIZE` reader - Address size This bit defines address size: This field can be written only when BUSY = 0."]
pub type ADSIZE_R = crate::FieldReader;
#[doc = "Field `ADSIZE` writer - Address size This bit defines address size: This field can be written only when BUSY = 0."]
pub type ADSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ABMODE` reader - Alternate bytes mode This field defines the alternate-bytes phase mode of operation: This field can be written only when BUSY = 0."]
pub type ABMODE_R = crate::FieldReader;
#[doc = "Field `ABMODE` writer - Alternate bytes mode This field defines the alternate-bytes phase mode of operation: This field can be written only when BUSY = 0."]
pub type ABMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ABSIZE` reader - Alternate bytes size This bit defines alternate bytes size: This field can be written only when BUSY = 0."]
pub type ABSIZE_R = crate::FieldReader;
#[doc = "Field `ABSIZE` writer - Alternate bytes size This bit defines alternate bytes size: This field can be written only when BUSY = 0."]
pub type ABSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DCYC` reader - Number of dummy cycles This field defines the duration of the dummy phase. In both SDR and DDR modes, it specifies a number of CLK cycles (0-31). This field can be written only when BUSY = 0."]
pub type DCYC_R = crate::FieldReader;
#[doc = "Field `DCYC` writer - Number of dummy cycles This field defines the duration of the dummy phase. In both SDR and DDR modes, it specifies a number of CLK cycles (0-31). This field can be written only when BUSY = 0."]
pub type DCYC_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DMODE` reader - Data mode This field defines the data phases mode of operation: This field also determines the dummy phase mode of operation. This field can be written only when BUSY = 0."]
pub type DMODE_R = crate::FieldReader;
#[doc = "Field `DMODE` writer - Data mode This field defines the data phases mode of operation: This field also determines the dummy phase mode of operation. This field can be written only when BUSY = 0."]
pub type DMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FMODE` reader - Functional mode This field defines the QUADSPI functional mode of operation. If DMAEN = 1 already, then the DMA controller for the corresponding channel must be disabled before changing the FMODE value. This field can be written only when BUSY = 0."]
pub type FMODE_R = crate::FieldReader;
#[doc = "Field `FMODE` writer - Functional mode This field defines the QUADSPI functional mode of operation. If DMAEN = 1 already, then the DMA controller for the corresponding channel must be disabled before changing the FMODE value. This field can be written only when BUSY = 0."]
pub type FMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SIOO` reader - Send instruction only once mode See Section15.3.11: Sending the instruction only once on page13. This bit has no effect when IMODE = 00. This field can be written only when BUSY = 0."]
pub type SIOO_R = crate::BitReader;
#[doc = "Field `SIOO` writer - Send instruction only once mode See Section15.3.11: Sending the instruction only once on page13. This bit has no effect when IMODE = 00. This field can be written only when BUSY = 0."]
pub type SIOO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DHHC` reader - DDR hold Delay the data output by 1/4 of the QUADSPI output clock cycle in DDR mode: This feature is only active in DDR mode. This field can be written only when BUSY = 0."]
pub type DHHC_R = crate::BitReader;
#[doc = "Field `DHHC` writer - DDR hold Delay the data output by 1/4 of the QUADSPI output clock cycle in DDR mode: This feature is only active in DDR mode. This field can be written only when BUSY = 0."]
pub type DHHC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDRM` reader - Double data rate mode This bit sets the DDR mode for the address, alternate byte and data phase: This field can be written only when BUSY = 0."]
pub type DDRM_R = crate::BitReader;
#[doc = "Field `DDRM` writer - Double data rate mode This bit sets the DDR mode for the address, alternate byte and data phase: This field can be written only when BUSY = 0."]
pub type DDRM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Instruction Instruction to be send to the external SPI device. This field can be written only when BUSY = 0."]
    #[inline(always)]
    pub fn instruction(&self) -> INSTRUCTION_R {
        INSTRUCTION_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - Instruction mode This field defines the instruction phase mode of operation: This field can be written only when BUSY = 0."]
    #[inline(always)]
    pub fn imode(&self) -> IMODE_R {
        IMODE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Address mode This field defines the address phase mode of operation: This field can be written only when BUSY = 0."]
    #[inline(always)]
    pub fn admode(&self) -> ADMODE_R {
        ADMODE_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Address size This bit defines address size: This field can be written only when BUSY = 0."]
    #[inline(always)]
    pub fn adsize(&self) -> ADSIZE_R {
        ADSIZE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Alternate bytes mode This field defines the alternate-bytes phase mode of operation: This field can be written only when BUSY = 0."]
    #[inline(always)]
    pub fn abmode(&self) -> ABMODE_R {
        ABMODE_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Alternate bytes size This bit defines alternate bytes size: This field can be written only when BUSY = 0."]
    #[inline(always)]
    pub fn absize(&self) -> ABSIZE_R {
        ABSIZE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:22 - Number of dummy cycles This field defines the duration of the dummy phase. In both SDR and DDR modes, it specifies a number of CLK cycles (0-31). This field can be written only when BUSY = 0."]
    #[inline(always)]
    pub fn dcyc(&self) -> DCYC_R {
        DCYC_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 24:25 - Data mode This field defines the data phases mode of operation: This field also determines the dummy phase mode of operation. This field can be written only when BUSY = 0."]
    #[inline(always)]
    pub fn dmode(&self) -> DMODE_R {
        DMODE_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Functional mode This field defines the QUADSPI functional mode of operation. If DMAEN = 1 already, then the DMA controller for the corresponding channel must be disabled before changing the FMODE value. This field can be written only when BUSY = 0."]
    #[inline(always)]
    pub fn fmode(&self) -> FMODE_R {
        FMODE_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28 - Send instruction only once mode See Section15.3.11: Sending the instruction only once on page13. This bit has no effect when IMODE = 00. This field can be written only when BUSY = 0."]
    #[inline(always)]
    pub fn sioo(&self) -> SIOO_R {
        SIOO_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - DDR hold Delay the data output by 1/4 of the QUADSPI output clock cycle in DDR mode: This feature is only active in DDR mode. This field can be written only when BUSY = 0."]
    #[inline(always)]
    pub fn dhhc(&self) -> DHHC_R {
        DHHC_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Double data rate mode This bit sets the DDR mode for the address, alternate byte and data phase: This field can be written only when BUSY = 0."]
    #[inline(always)]
    pub fn ddrm(&self) -> DDRM_R {
        DDRM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Instruction Instruction to be send to the external SPI device. This field can be written only when BUSY = 0."]
    #[inline(always)]
    #[must_use]
    pub fn instruction(&mut self) -> INSTRUCTION_W<CCRrs> {
        INSTRUCTION_W::new(self, 0)
    }
    #[doc = "Bits 8:9 - Instruction mode This field defines the instruction phase mode of operation: This field can be written only when BUSY = 0."]
    #[inline(always)]
    #[must_use]
    pub fn imode(&mut self) -> IMODE_W<CCRrs> {
        IMODE_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Address mode This field defines the address phase mode of operation: This field can be written only when BUSY = 0."]
    #[inline(always)]
    #[must_use]
    pub fn admode(&mut self) -> ADMODE_W<CCRrs> {
        ADMODE_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Address size This bit defines address size: This field can be written only when BUSY = 0."]
    #[inline(always)]
    #[must_use]
    pub fn adsize(&mut self) -> ADSIZE_W<CCRrs> {
        ADSIZE_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Alternate bytes mode This field defines the alternate-bytes phase mode of operation: This field can be written only when BUSY = 0."]
    #[inline(always)]
    #[must_use]
    pub fn abmode(&mut self) -> ABMODE_W<CCRrs> {
        ABMODE_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Alternate bytes size This bit defines alternate bytes size: This field can be written only when BUSY = 0."]
    #[inline(always)]
    #[must_use]
    pub fn absize(&mut self) -> ABSIZE_W<CCRrs> {
        ABSIZE_W::new(self, 16)
    }
    #[doc = "Bits 18:22 - Number of dummy cycles This field defines the duration of the dummy phase. In both SDR and DDR modes, it specifies a number of CLK cycles (0-31). This field can be written only when BUSY = 0."]
    #[inline(always)]
    #[must_use]
    pub fn dcyc(&mut self) -> DCYC_W<CCRrs> {
        DCYC_W::new(self, 18)
    }
    #[doc = "Bits 24:25 - Data mode This field defines the data phases mode of operation: This field also determines the dummy phase mode of operation. This field can be written only when BUSY = 0."]
    #[inline(always)]
    #[must_use]
    pub fn dmode(&mut self) -> DMODE_W<CCRrs> {
        DMODE_W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Functional mode This field defines the QUADSPI functional mode of operation. If DMAEN = 1 already, then the DMA controller for the corresponding channel must be disabled before changing the FMODE value. This field can be written only when BUSY = 0."]
    #[inline(always)]
    #[must_use]
    pub fn fmode(&mut self) -> FMODE_W<CCRrs> {
        FMODE_W::new(self, 26)
    }
    #[doc = "Bit 28 - Send instruction only once mode See Section15.3.11: Sending the instruction only once on page13. This bit has no effect when IMODE = 00. This field can be written only when BUSY = 0."]
    #[inline(always)]
    #[must_use]
    pub fn sioo(&mut self) -> SIOO_W<CCRrs> {
        SIOO_W::new(self, 28)
    }
    #[doc = "Bit 30 - DDR hold Delay the data output by 1/4 of the QUADSPI output clock cycle in DDR mode: This feature is only active in DDR mode. This field can be written only when BUSY = 0."]
    #[inline(always)]
    #[must_use]
    pub fn dhhc(&mut self) -> DHHC_W<CCRrs> {
        DHHC_W::new(self, 30)
    }
    #[doc = "Bit 31 - Double data rate mode This bit sets the DDR mode for the address, alternate byte and data phase: This field can be written only when BUSY = 0."]
    #[inline(always)]
    #[must_use]
    pub fn ddrm(&mut self) -> DDRM_W<CCRrs> {
        DDRM_W::new(self, 31)
    }
}
#[doc = "QUADSPI communication configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCRrs;
impl crate::RegisterSpec for CCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr::R`](R) reader structure"]
impl crate::Readable for CCRrs {}
#[doc = "`write(|w| ..)` method takes [`ccr::W`](W) writer structure"]
impl crate::Writable for CCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCR to value 0"]
impl crate::Resettable for CCRrs {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Reader of register SDMMC_IDMABASE0R"]
pub type R = crate::R<u32, super::SDMMC_IDMABASE0R>;
#[doc = "Writer for register SDMMC_IDMABASE0R"]
pub type W = crate::W<u32, super::SDMMC_IDMABASE0R>;
#[doc = "Register SDMMC_IDMABASE0R `reset()`'s with value 0"]
impl crate::ResetValue for super::SDMMC_IDMABASE0R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IDMABASE0`"]
pub type IDMABASE0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `IDMABASE0`"]
pub struct IDMABASE0_W<'a> {
    w: &'a mut W,
}
impl<'a> IDMABASE0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Buffer 0 memory base address bits \\[31:2\\], shall be word aligned (bit \\[1:0\\]
are always 0 and read only). This register can be written by firmware when DPSM is inactive (DPSMACT = 0), and can dynamically be written by firmware when DPSM active (DPSMACT = 1) and memory buffer 0 is inactive (IDMABACT = 1)."]
    #[inline(always)]
    pub fn idmabase0(&self) -> IDMABASE0_R {
        IDMABASE0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer 0 memory base address bits \\[31:2\\], shall be word aligned (bit \\[1:0\\]
are always 0 and read only). This register can be written by firmware when DPSM is inactive (DPSMACT = 0), and can dynamically be written by firmware when DPSM active (DPSMACT = 1) and memory buffer 0 is inactive (IDMABACT = 1)."]
    #[inline(always)]
    pub fn idmabase0(&mut self) -> IDMABASE0_W {
        IDMABASE0_W { w: self }
    }
}

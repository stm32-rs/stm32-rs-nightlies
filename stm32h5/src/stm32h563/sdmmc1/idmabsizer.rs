#[doc = "Register `IDMABSIZER` reader"]
pub type R = crate::R<IDMABSIZERrs>;
#[doc = "Register `IDMABSIZER` writer"]
pub type W = crate::W<IDMABSIZERrs>;
#[doc = "Field `IDMABNDT` reader - Number of bytes per buffer This 12-bit value must be multiplied by 8 to get the size of the buffer in 32-bit words and by 32 to get the size of the buffer in bytes. Example: IDMABNDT = 0x001: buffer size = 8 words = 32 bytes. Example: IDMABNDT = 0x800: buffer size = 16384 words = 64 Kbyte. These bits can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
pub type IDMABNDT_R = crate::FieldReader<u16>;
#[doc = "Field `IDMABNDT` writer - Number of bytes per buffer This 12-bit value must be multiplied by 8 to get the size of the buffer in 32-bit words and by 32 to get the size of the buffer in bytes. Example: IDMABNDT = 0x001: buffer size = 8 words = 32 bytes. Example: IDMABNDT = 0x800: buffer size = 16384 words = 64 Kbyte. These bits can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
pub type IDMABNDT_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 5:16 - Number of bytes per buffer This 12-bit value must be multiplied by 8 to get the size of the buffer in 32-bit words and by 32 to get the size of the buffer in bytes. Example: IDMABNDT = 0x001: buffer size = 8 words = 32 bytes. Example: IDMABNDT = 0x800: buffer size = 16384 words = 64 Kbyte. These bits can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
    #[inline(always)]
    pub fn idmabndt(&self) -> IDMABNDT_R {
        IDMABNDT_R::new(((self.bits >> 5) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 5:16 - Number of bytes per buffer This 12-bit value must be multiplied by 8 to get the size of the buffer in 32-bit words and by 32 to get the size of the buffer in bytes. Example: IDMABNDT = 0x001: buffer size = 8 words = 32 bytes. Example: IDMABNDT = 0x800: buffer size = 16384 words = 64 Kbyte. These bits can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn idmabndt(&mut self) -> IDMABNDT_W<IDMABSIZERrs> {
        IDMABNDT_W::new(self, 5)
    }
}
#[doc = "SDMMC IDMA buffer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idmabsizer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idmabsizer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDMABSIZERrs;
impl crate::RegisterSpec for IDMABSIZERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idmabsizer::R`](R) reader structure"]
impl crate::Readable for IDMABSIZERrs {}
#[doc = "`write(|w| ..)` method takes [`idmabsizer::W`](W) writer structure"]
impl crate::Writable for IDMABSIZERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IDMABSIZER to value 0"]
impl crate::Resettable for IDMABSIZERrs {
    const RESET_VALUE: u32 = 0;
}

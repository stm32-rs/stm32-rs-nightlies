#[doc = "Register `IDMALAR` reader"]
pub type R = crate::R<IDMALARrs>;
#[doc = "Register `IDMALAR` writer"]
pub type W = crate::W<IDMALARrs>;
#[doc = "Field `IDMALA` reader - Word aligned linked list item address offset Linked list item offset pointer to the base of the next linked list item structure. Linked list item base address is IDMABA + IDMALA. These bits can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
pub type IDMALA_R = crate::FieldReader<u16>;
#[doc = "Field `IDMALA` writer - Word aligned linked list item address offset Linked list item offset pointer to the base of the next linked list item structure. Linked list item base address is IDMABA + IDMALA. These bits can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
pub type IDMALA_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `ABR` reader - Acknowledge linked list buffer ready This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). This bit is not taken into account when starting the first linked list buffer from the software programmed register information. ABR is only taken into account on subsequent loaded linked list items."]
pub type ABR_R = crate::BitReader;
#[doc = "Field `ABR` writer - Acknowledge linked list buffer ready This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). This bit is not taken into account when starting the first linked list buffer from the software programmed register information. ABR is only taken into account on subsequent loaded linked list items."]
pub type ABR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULS` reader - Update SDMMC_IDMABSIZE from the next linked list when in linked list mode (SDMMC_IDMACTRLR.IDMABMODE select linked list mode and ULA = 1) This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
pub type ULS_R = crate::BitReader;
#[doc = "Field `ULS` writer - Update SDMMC_IDMABSIZE from the next linked list when in linked list mode (SDMMC_IDMACTRLR.IDMABMODE select linked list mode and ULA = 1) This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
pub type ULS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULA` reader - Update SDMMC_IDMALAR from linked list when in linked list mode (SDMMC_IDMACTRLR.IDMABMODE select linked list mode) This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
pub type ULA_R = crate::BitReader;
#[doc = "Field `ULA` writer - Update SDMMC_IDMALAR from linked list when in linked list mode (SDMMC_IDMACTRLR.IDMABMODE select linked list mode) This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
pub type ULA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 2:15 - Word aligned linked list item address offset Linked list item offset pointer to the base of the next linked list item structure. Linked list item base address is IDMABA + IDMALA. These bits can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
    #[inline(always)]
    pub fn idmala(&self) -> IDMALA_R {
        IDMALA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bit 29 - Acknowledge linked list buffer ready This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). This bit is not taken into account when starting the first linked list buffer from the software programmed register information. ABR is only taken into account on subsequent loaded linked list items."]
    #[inline(always)]
    pub fn abr(&self) -> ABR_R {
        ABR_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Update SDMMC_IDMABSIZE from the next linked list when in linked list mode (SDMMC_IDMACTRLR.IDMABMODE select linked list mode and ULA = 1) This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
    #[inline(always)]
    pub fn uls(&self) -> ULS_R {
        ULS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Update SDMMC_IDMALAR from linked list when in linked list mode (SDMMC_IDMACTRLR.IDMABMODE select linked list mode) This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
    #[inline(always)]
    pub fn ula(&self) -> ULA_R {
        ULA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 2:15 - Word aligned linked list item address offset Linked list item offset pointer to the base of the next linked list item structure. Linked list item base address is IDMABA + IDMALA. These bits can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn idmala(&mut self) -> IDMALA_W<IDMALARrs> {
        IDMALA_W::new(self, 2)
    }
    #[doc = "Bit 29 - Acknowledge linked list buffer ready This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). This bit is not taken into account when starting the first linked list buffer from the software programmed register information. ABR is only taken into account on subsequent loaded linked list items."]
    #[inline(always)]
    #[must_use]
    pub fn abr(&mut self) -> ABR_W<IDMALARrs> {
        ABR_W::new(self, 29)
    }
    #[doc = "Bit 30 - Update SDMMC_IDMABSIZE from the next linked list when in linked list mode (SDMMC_IDMACTRLR.IDMABMODE select linked list mode and ULA = 1) This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn uls(&mut self) -> ULS_W<IDMALARrs> {
        ULS_W::new(self, 30)
    }
    #[doc = "Bit 31 - Update SDMMC_IDMALAR from linked list when in linked list mode (SDMMC_IDMACTRLR.IDMABMODE select linked list mode) This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn ula(&mut self) -> ULA_W<IDMALARrs> {
        ULA_W::new(self, 31)
    }
}
#[doc = "SDMMC_IDMALAR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idmalar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idmalar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDMALARrs;
impl crate::RegisterSpec for IDMALARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idmalar::R`](R) reader structure"]
impl crate::Readable for IDMALARrs {}
#[doc = "`write(|w| ..)` method takes [`idmalar::W`](W) writer structure"]
impl crate::Writable for IDMALARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IDMALAR to value 0"]
impl crate::Resettable for IDMALARrs {
    const RESET_VALUE: u32 = 0;
}

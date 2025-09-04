///Register `IDMALAR` reader
pub type R = crate::R<IDMALARrs>;
///Register `IDMALAR` writer
pub type W = crate::W<IDMALARrs>;
///Field `IDMALA` reader - Word aligned linked list item address offset Linked list item offset pointer to the base of the next linked list item structure. Linked list item base address is IDMABA + IDMALA. These bits can only be written by firmware when DPSM is inactive (DPSMACT = 0).
pub type IDMALA_R = crate::FieldReader<u16>;
///Field `IDMALA` writer - Word aligned linked list item address offset Linked list item offset pointer to the base of the next linked list item structure. Linked list item base address is IDMABA + IDMALA. These bits can only be written by firmware when DPSM is inactive (DPSMACT = 0).
pub type IDMALA_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
///Field `ABR` reader - Acknowledge linked list buffer ready This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). This bit is not taken into account when starting the first linked list buffer from the software programmed register information. ABR is only taken into account on subsequent loaded linked list items.
pub type ABR_R = crate::BitReader;
///Field `ABR` writer - Acknowledge linked list buffer ready This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). This bit is not taken into account when starting the first linked list buffer from the software programmed register information. ABR is only taken into account on subsequent loaded linked list items.
pub type ABR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ULS` reader - Update SDMMC_IDMABSIZE from the next linked list when in linked list mode (SDMMC_IDMACTRLR.IDMABMODE select linked list mode and ULA = 1) This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).
pub type ULS_R = crate::BitReader;
///Field `ULS` writer - Update SDMMC_IDMABSIZE from the next linked list when in linked list mode (SDMMC_IDMACTRLR.IDMABMODE select linked list mode and ULA = 1) This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).
pub type ULS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ULA` reader - Update SDMMC_IDMALAR from linked list when in linked list mode (SDMMC_IDMACTRLR.IDMABMODE select linked list mode) This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).
pub type ULA_R = crate::BitReader;
///Field `ULA` writer - Update SDMMC_IDMALAR from linked list when in linked list mode (SDMMC_IDMACTRLR.IDMABMODE select linked list mode) This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).
pub type ULA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 2:15 - Word aligned linked list item address offset Linked list item offset pointer to the base of the next linked list item structure. Linked list item base address is IDMABA + IDMALA. These bits can only be written by firmware when DPSM is inactive (DPSMACT = 0).
    #[inline(always)]
    pub fn idmala(&self) -> IDMALA_R {
        IDMALA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    ///Bit 29 - Acknowledge linked list buffer ready This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). This bit is not taken into account when starting the first linked list buffer from the software programmed register information. ABR is only taken into account on subsequent loaded linked list items.
    #[inline(always)]
    pub fn abr(&self) -> ABR_R {
        ABR_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Update SDMMC_IDMABSIZE from the next linked list when in linked list mode (SDMMC_IDMACTRLR.IDMABMODE select linked list mode and ULA = 1) This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).
    #[inline(always)]
    pub fn uls(&self) -> ULS_R {
        ULS_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Update SDMMC_IDMALAR from linked list when in linked list mode (SDMMC_IDMACTRLR.IDMABMODE select linked list mode) This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).
    #[inline(always)]
    pub fn ula(&self) -> ULA_R {
        ULA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDMALAR")
            .field("idmala", &self.idmala())
            .field("abr", &self.abr())
            .field("uls", &self.uls())
            .field("ula", &self.ula())
            .finish()
    }
}
impl W {
    ///Bits 2:15 - Word aligned linked list item address offset Linked list item offset pointer to the base of the next linked list item structure. Linked list item base address is IDMABA + IDMALA. These bits can only be written by firmware when DPSM is inactive (DPSMACT = 0).
    #[inline(always)]
    pub fn idmala(&mut self) -> IDMALA_W<IDMALARrs> {
        IDMALA_W::new(self, 2)
    }
    ///Bit 29 - Acknowledge linked list buffer ready This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). This bit is not taken into account when starting the first linked list buffer from the software programmed register information. ABR is only taken into account on subsequent loaded linked list items.
    #[inline(always)]
    pub fn abr(&mut self) -> ABR_W<IDMALARrs> {
        ABR_W::new(self, 29)
    }
    ///Bit 30 - Update SDMMC_IDMABSIZE from the next linked list when in linked list mode (SDMMC_IDMACTRLR.IDMABMODE select linked list mode and ULA = 1) This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).
    #[inline(always)]
    pub fn uls(&mut self) -> ULS_W<IDMALARrs> {
        ULS_W::new(self, 30)
    }
    ///Bit 31 - Update SDMMC_IDMALAR from linked list when in linked list mode (SDMMC_IDMACTRLR.IDMABMODE select linked list mode) This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).
    #[inline(always)]
    pub fn ula(&mut self) -> ULA_W<IDMALARrs> {
        ULA_W::new(self, 31)
    }
}
/**SDMMC IDMA linked list address register

You can [`read`](crate::Reg::read) this register and get [`idmalar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idmalar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#SDMMC1:IDMALAR)*/
pub struct IDMALARrs;
impl crate::RegisterSpec for IDMALARrs {
    type Ux = u32;
}
///`read()` method returns [`idmalar::R`](R) reader structure
impl crate::Readable for IDMALARrs {}
///`write(|w| ..)` method takes [`idmalar::W`](W) writer structure
impl crate::Writable for IDMALARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IDMALAR to value 0
impl crate::Resettable for IDMALARrs {}

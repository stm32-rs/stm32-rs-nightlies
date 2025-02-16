///Register `PCROP1BSR` reader
pub type R = crate::R<PCROP1BSRrs>;
///Register `PCROP1BSR` writer
pub type W = crate::W<PCROP1BSRrs>;
///Field `PCROP1B_STRT` reader - PCROP1B area start offset Contains the offset of the first subpage of the PCROP1B area. Note: The number of effective bits depends on the size of the flash memory in the device.
pub type PCROP1B_STRT_R = crate::FieldReader;
///Field `PCROP1B_STRT` writer - PCROP1B area start offset Contains the offset of the first subpage of the PCROP1B area. Note: The number of effective bits depends on the size of the flash memory in the device.
pub type PCROP1B_STRT_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:5 - PCROP1B area start offset Contains the offset of the first subpage of the PCROP1B area. Note: The number of effective bits depends on the size of the flash memory in the device.
    #[inline(always)]
    pub fn pcrop1b_strt(&self) -> PCROP1B_STRT_R {
        PCROP1B_STRT_R::new((self.bits & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCROP1BSR")
            .field("pcrop1b_strt", &self.pcrop1b_strt())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - PCROP1B area start offset Contains the offset of the first subpage of the PCROP1B area. Note: The number of effective bits depends on the size of the flash memory in the device.
    #[inline(always)]
    pub fn pcrop1b_strt(&mut self) -> PCROP1B_STRT_W<PCROP1BSRrs> {
        PCROP1B_STRT_W::new(self, 0)
    }
}
/**FLASH PCROP area B start address register

You can [`read`](crate::Reg::read) this register and get [`pcrop1bsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrop1bsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#FLASH:PCROP1BSR)*/
pub struct PCROP1BSRrs;
impl crate::RegisterSpec for PCROP1BSRrs {
    type Ux = u32;
}
///`read()` method returns [`pcrop1bsr::R`](R) reader structure
impl crate::Readable for PCROP1BSRrs {}
///`write(|w| ..)` method takes [`pcrop1bsr::W`](W) writer structure
impl crate::Writable for PCROP1BSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PCROP1BSR to value 0
impl crate::Resettable for PCROP1BSRrs {
    const RESET_VALUE: u32 = 0;
}

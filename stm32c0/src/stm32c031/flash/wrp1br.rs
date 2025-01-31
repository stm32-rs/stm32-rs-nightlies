///Register `WRP1BR` reader
pub type R = crate::R<WRP1BRrs>;
///Register `WRP1BR` writer
pub type W = crate::W<WRP1BRrs>;
///Field `WRP1B_STRT` reader - WRP area B start offset This bitfield contains the offset of the first page of the WRP area B. Note: The number of effective bits depends on the size of the flash memory in the device.
pub type WRP1B_STRT_R = crate::FieldReader;
///Field `WRP1B_STRT` writer - WRP area B start offset This bitfield contains the offset of the first page of the WRP area B. Note: The number of effective bits depends on the size of the flash memory in the device.
pub type WRP1B_STRT_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `WRP1B_END` reader - WRP area B end offset This bitfield contains the offset of the last page of the WRP area B. Note: The number of effective bits depends on the size of the flash memory in the device.
pub type WRP1B_END_R = crate::FieldReader;
///Field `WRP1B_END` writer - WRP area B end offset This bitfield contains the offset of the last page of the WRP area B. Note: The number of effective bits depends on the size of the flash memory in the device.
pub type WRP1B_END_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:5 - WRP area B start offset This bitfield contains the offset of the first page of the WRP area B. Note: The number of effective bits depends on the size of the flash memory in the device.
    #[inline(always)]
    pub fn wrp1b_strt(&self) -> WRP1B_STRT_R {
        WRP1B_STRT_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 16:21 - WRP area B end offset This bitfield contains the offset of the last page of the WRP area B. Note: The number of effective bits depends on the size of the flash memory in the device.
    #[inline(always)]
    pub fn wrp1b_end(&self) -> WRP1B_END_R {
        WRP1B_END_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WRP1BR")
            .field("wrp1b_strt", &self.wrp1b_strt())
            .field("wrp1b_end", &self.wrp1b_end())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - WRP area B start offset This bitfield contains the offset of the first page of the WRP area B. Note: The number of effective bits depends on the size of the flash memory in the device.
    #[inline(always)]
    pub fn wrp1b_strt(&mut self) -> WRP1B_STRT_W<WRP1BRrs> {
        WRP1B_STRT_W::new(self, 0)
    }
    ///Bits 16:21 - WRP area B end offset This bitfield contains the offset of the last page of the WRP area B. Note: The number of effective bits depends on the size of the flash memory in the device.
    #[inline(always)]
    pub fn wrp1b_end(&mut self) -> WRP1B_END_W<WRP1BRrs> {
        WRP1B_END_W::new(self, 16)
    }
}
/**FLASH WRP area B address register

You can [`read`](crate::Reg::read) this register and get [`wrp1br::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrp1br::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#FLASH:WRP1BR)*/
pub struct WRP1BRrs;
impl crate::RegisterSpec for WRP1BRrs {
    type Ux = u32;
}
///`read()` method returns [`wrp1br::R`](R) reader structure
impl crate::Readable for WRP1BRrs {}
///`write(|w| ..)` method takes [`wrp1br::W`](W) writer structure
impl crate::Writable for WRP1BRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets WRP1BR to value 0
impl crate::Resettable for WRP1BRrs {
    const RESET_VALUE: u32 = 0;
}

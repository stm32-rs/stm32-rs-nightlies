///Register `WRP1AR` reader
pub type R = crate::R<WRP1ARrs>;
///Register `WRP1AR` writer
pub type W = crate::W<WRP1ARrs>;
///Field `WRP1A_STRT` reader - WRP area A start offset This bitfield contains the offset of the first page of the WRP area A. Note: The number of effective bits depends on the size of the flash memory in the device.
pub type WRP1A_STRT_R = crate::FieldReader;
///Field `WRP1A_STRT` writer - WRP area A start offset This bitfield contains the offset of the first page of the WRP area A. Note: The number of effective bits depends on the size of the flash memory in the device.
pub type WRP1A_STRT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `WRP1A_END` reader - WRP area A end offset This bitfield contains the offset of the last page of the WRP area A. Note: The number of effective bits depends on the size of the flash memory in the device.
pub type WRP1A_END_R = crate::FieldReader;
///Field `WRP1A_END` writer - WRP area A end offset This bitfield contains the offset of the last page of the WRP area A. Note: The number of effective bits depends on the size of the flash memory in the device.
pub type WRP1A_END_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - WRP area A start offset This bitfield contains the offset of the first page of the WRP area A. Note: The number of effective bits depends on the size of the flash memory in the device.
    #[inline(always)]
    pub fn wrp1a_strt(&self) -> WRP1A_STRT_R {
        WRP1A_STRT_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 16:19 - WRP area A end offset This bitfield contains the offset of the last page of the WRP area A. Note: The number of effective bits depends on the size of the flash memory in the device.
    #[inline(always)]
    pub fn wrp1a_end(&self) -> WRP1A_END_R {
        WRP1A_END_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WRP1AR")
            .field("wrp1a_strt", &self.wrp1a_strt())
            .field("wrp1a_end", &self.wrp1a_end())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - WRP area A start offset This bitfield contains the offset of the first page of the WRP area A. Note: The number of effective bits depends on the size of the flash memory in the device.
    #[inline(always)]
    pub fn wrp1a_strt(&mut self) -> WRP1A_STRT_W<WRP1ARrs> {
        WRP1A_STRT_W::new(self, 0)
    }
    ///Bits 16:19 - WRP area A end offset This bitfield contains the offset of the last page of the WRP area A. Note: The number of effective bits depends on the size of the flash memory in the device.
    #[inline(always)]
    pub fn wrp1a_end(&mut self) -> WRP1A_END_W<WRP1ARrs> {
        WRP1A_END_W::new(self, 16)
    }
}
/**FLASH WRP area A address register

You can [`read`](crate::Reg::read) this register and get [`wrp1ar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrp1ar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#FLASH:WRP1AR)*/
pub struct WRP1ARrs;
impl crate::RegisterSpec for WRP1ARrs {
    type Ux = u32;
}
///`read()` method returns [`wrp1ar::R`](R) reader structure
impl crate::Readable for WRP1ARrs {}
///`write(|w| ..)` method takes [`wrp1ar::W`](W) writer structure
impl crate::Writable for WRP1ARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets WRP1AR to value 0
impl crate::Resettable for WRP1ARrs {
    const RESET_VALUE: u32 = 0;
}

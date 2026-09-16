///Register `OBKDR5` reader
pub type R = crate::R<OBKDR5rs>;
///Register `OBKDR5` writer
pub type W = crate::W<OBKDR5rs>;
///Field `OBKDATA` reader - option byte key data, bits \[31+x:0+x\] Data register used in conjunction with FLASH_OBKCR register. Reading this register (read value once), or incrementing HDPL value in SBS peripheral automatically clears OBKDATA to 0x0. Writing this register prevents reading OBKDATA until option byte key programming sequence is completed.
pub type OBKDATA_R = crate::FieldReader<u32>;
///Field `OBKDATA` writer - option byte key data, bits \[31+x:0+x\] Data register used in conjunction with FLASH_OBKCR register. Reading this register (read value once), or incrementing HDPL value in SBS peripheral automatically clears OBKDATA to 0x0. Writing this register prevents reading OBKDATA until option byte key programming sequence is completed.
pub type OBKDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - option byte key data, bits \[31+x:0+x\] Data register used in conjunction with FLASH_OBKCR register. Reading this register (read value once), or incrementing HDPL value in SBS peripheral automatically clears OBKDATA to 0x0. Writing this register prevents reading OBKDATA until option byte key programming sequence is completed.
    #[inline(always)]
    pub fn obkdata(&self) -> OBKDATA_R {
        OBKDATA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OBKDR5")
            .field("obkdata", &self.obkdata())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - option byte key data, bits \[31+x:0+x\] Data register used in conjunction with FLASH_OBKCR register. Reading this register (read value once), or incrementing HDPL value in SBS peripheral automatically clears OBKDATA to 0x0. Writing this register prevents reading OBKDATA until option byte key programming sequence is completed.
    #[inline(always)]
    pub fn obkdata(&mut self) -> OBKDATA_W<'_, OBKDR5rs> {
        OBKDATA_W::new(self, 0)
    }
}
/**FLASH option bytes key data register 5

You can [`read`](crate::Reg::read) this register and get [`obkdr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obkdr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FLASH:OBKDR5)*/
pub struct OBKDR5rs;
impl crate::RegisterSpec for OBKDR5rs {
    type Ux = u32;
}
///`read()` method returns [`obkdr5::R`](R) reader structure
impl crate::Readable for OBKDR5rs {}
///`write(|w| ..)` method takes [`obkdr5::W`](W) writer structure
impl crate::Writable for OBKDR5rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OBKDR5 to value 0
impl crate::Resettable for OBKDR5rs {}

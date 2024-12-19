///Register `LCD_RAM4` reader
pub type R = crate::R<LCD_RAM4rs>;
///Register `LCD_RAM4` writer
pub type W = crate::W<LCD_RAM4rs>;
///Field `SEGMENT_DATA` reader - Each bit corresponds to one pixel of the LCD display.
pub type SEGMENT_DATA_R = crate::FieldReader<u32>;
///Field `SEGMENT_DATA` writer - Each bit corresponds to one pixel of the LCD display.
pub type SEGMENT_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Each bit corresponds to one pixel of the LCD display.
    #[inline(always)]
    pub fn segment_data(&self) -> SEGMENT_DATA_R {
        SEGMENT_DATA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCD_RAM4")
            .field("segment_data", &self.segment_data())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Each bit corresponds to one pixel of the LCD display.
    #[inline(always)]
    pub fn segment_data(&mut self) -> SEGMENT_DATA_W<LCD_RAM4rs> {
        SEGMENT_DATA_W::new(self, 0)
    }
}
/**LCD display memory

You can [`read`](crate::Reg::read) this register and get [`lcd_ram4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_ram4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#LCD:LCD_RAM4)*/
pub struct LCD_RAM4rs;
impl crate::RegisterSpec for LCD_RAM4rs {
    type Ux = u32;
}
///`read()` method returns [`lcd_ram4::R`](R) reader structure
impl crate::Readable for LCD_RAM4rs {}
///`write(|w| ..)` method takes [`lcd_ram4::W`](W) writer structure
impl crate::Writable for LCD_RAM4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LCD_RAM4 to value 0
impl crate::Resettable for LCD_RAM4rs {
    const RESET_VALUE: u32 = 0;
}

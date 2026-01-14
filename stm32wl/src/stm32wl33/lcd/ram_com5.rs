///Register `RAM_COM5` reader
pub type R = crate::R<RAM_COM5rs>;
///Register `RAM_COM5` writer
pub type W = crate::W<RAM_COM5rs>;
///Field `SEGMENT_DATA` reader - Each bit corresponds to one pixel of the LCD display.
pub type SEGMENT_DATA_R = crate::FieldReader<u16>;
///Field `SEGMENT_DATA` writer - Each bit corresponds to one pixel of the LCD display.
pub type SEGMENT_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Each bit corresponds to one pixel of the LCD display.
    #[inline(always)]
    pub fn segment_data(&self) -> SEGMENT_DATA_R {
        SEGMENT_DATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RAM_COM5")
            .field("segment_data", &self.segment_data())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Each bit corresponds to one pixel of the LCD display.
    #[inline(always)]
    pub fn segment_data(&mut self) -> SEGMENT_DATA_W<'_, RAM_COM5rs> {
        SEGMENT_DATA_W::new(self, 0)
    }
}
/**LCD_RAM_COMx register

You can [`read`](crate::Reg::read) this register and get [`ram_com5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_com5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LCD:RAM_COM5)*/
pub struct RAM_COM5rs;
impl crate::RegisterSpec for RAM_COM5rs {
    type Ux = u32;
}
///`read()` method returns [`ram_com5::R`](R) reader structure
impl crate::Readable for RAM_COM5rs {}
///`write(|w| ..)` method takes [`ram_com5::W`](W) writer structure
impl crate::Writable for RAM_COM5rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RAM_COM5 to value 0
impl crate::Resettable for RAM_COM5rs {}

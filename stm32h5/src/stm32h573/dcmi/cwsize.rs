///Register `CWSIZE` reader
pub type R = crate::R<CWSIZErs>;
///Register `CWSIZE` writer
pub type W = crate::W<CWSIZErs>;
///Field `CAPCNT` reader - Capture count This value gives the number of pixel clocks to be captured from the starting point on the same line. It value must corresponds to word-aligned data for different widths of parallel interfaces. 0x0000 = 1 pixel 0x0001 = 2 pixels 0x0002 = 3 pixels ....
pub type CAPCNT_R = crate::FieldReader<u16>;
///Field `CAPCNT` writer - Capture count This value gives the number of pixel clocks to be captured from the starting point on the same line. It value must corresponds to word-aligned data for different widths of parallel interfaces. 0x0000 = 1 pixel 0x0001 = 2 pixels 0x0002 = 3 pixels ....
pub type CAPCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16, crate::Safe>;
///Field `VLINE` reader - Vertical line count This value gives the number of lines to be captured from the starting point. ....
pub type VLINE_R = crate::FieldReader<u16>;
///Field `VLINE` writer - Vertical line count This value gives the number of lines to be captured from the starting point. ....
pub type VLINE_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16, crate::Safe>;
impl R {
    ///Bits 0:13 - Capture count This value gives the number of pixel clocks to be captured from the starting point on the same line. It value must corresponds to word-aligned data for different widths of parallel interfaces. 0x0000 = 1 pixel 0x0001 = 2 pixels 0x0002 = 3 pixels ....
    #[inline(always)]
    pub fn capcnt(&self) -> CAPCNT_R {
        CAPCNT_R::new((self.bits & 0x3fff) as u16)
    }
    ///Bits 16:29 - Vertical line count This value gives the number of lines to be captured from the starting point. ....
    #[inline(always)]
    pub fn vline(&self) -> VLINE_R {
        VLINE_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CWSIZE")
            .field("capcnt", &self.capcnt())
            .field("vline", &self.vline())
            .finish()
    }
}
impl W {
    ///Bits 0:13 - Capture count This value gives the number of pixel clocks to be captured from the starting point on the same line. It value must corresponds to word-aligned data for different widths of parallel interfaces. 0x0000 = 1 pixel 0x0001 = 2 pixels 0x0002 = 3 pixels ....
    #[inline(always)]
    pub fn capcnt(&mut self) -> CAPCNT_W<'_, CWSIZErs> {
        CAPCNT_W::new(self, 0)
    }
    ///Bits 16:29 - Vertical line count This value gives the number of lines to be captured from the starting point. ....
    #[inline(always)]
    pub fn vline(&mut self) -> VLINE_W<'_, CWSIZErs> {
        VLINE_W::new(self, 16)
    }
}
/**DCMI crop window size

You can [`read`](crate::Reg::read) this register and get [`cwsize::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cwsize::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#DCMI:CWSIZE)*/
pub struct CWSIZErs;
impl crate::RegisterSpec for CWSIZErs {
    type Ux = u32;
}
///`read()` method returns [`cwsize::R`](R) reader structure
impl crate::Readable for CWSIZErs {}
///`write(|w| ..)` method takes [`cwsize::W`](W) writer structure
impl crate::Writable for CWSIZErs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CWSIZE to value 0
impl crate::Resettable for CWSIZErs {}

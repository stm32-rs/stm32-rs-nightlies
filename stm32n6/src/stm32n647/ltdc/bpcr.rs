///Register `BPCR` reader
pub type R = crate::R<BPCRrs>;
///Register `BPCR` writer
pub type W = crate::W<BPCRrs>;
///Field `AVBP` reader - accumulated Vertical back porch (in units of horizontal scan line)
pub type AVBP_R = crate::FieldReader<u16>;
///Field `AVBP` writer - accumulated Vertical back porch (in units of horizontal scan line)
pub type AVBP_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `AHBP` reader - accumulated horizontal back porch (in units of pixel clock period)
pub type AHBP_R = crate::FieldReader<u16>;
///Field `AHBP` writer - accumulated horizontal back porch (in units of pixel clock period)
pub type AHBP_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - accumulated Vertical back porch (in units of horizontal scan line)
    #[inline(always)]
    pub fn avbp(&self) -> AVBP_R {
        AVBP_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - accumulated horizontal back porch (in units of pixel clock period)
    #[inline(always)]
    pub fn ahbp(&self) -> AHBP_R {
        AHBP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BPCR")
            .field("avbp", &self.avbp())
            .field("ahbp", &self.ahbp())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - accumulated Vertical back porch (in units of horizontal scan line)
    #[inline(always)]
    pub fn avbp(&mut self) -> AVBP_W<'_, BPCRrs> {
        AVBP_W::new(self, 0)
    }
    ///Bits 16:31 - accumulated horizontal back porch (in units of pixel clock period)
    #[inline(always)]
    pub fn ahbp(&mut self) -> AHBP_W<'_, BPCRrs> {
        AHBP_W::new(self, 16)
    }
}
/**LTDC back porch configuration register

You can [`read`](crate::Reg::read) this register and get [`bpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#LTDC:BPCR)*/
pub struct BPCRrs;
impl crate::RegisterSpec for BPCRrs {
    type Ux = u32;
}
///`read()` method returns [`bpcr::R`](R) reader structure
impl crate::Readable for BPCRrs {}
///`write(|w| ..)` method takes [`bpcr::W`](W) writer structure
impl crate::Writable for BPCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BPCR to value 0
impl crate::Resettable for BPCRrs {}

///Register `TWCR` reader
pub type R = crate::R<TWCRrs>;
///Register `TWCR` writer
pub type W = crate::W<TWCRrs>;
///Field `TOTALH` reader - Total Height (in units of horizontal scan line)
pub type TOTALH_R = crate::FieldReader<u16>;
///Field `TOTALH` writer - Total Height (in units of horizontal scan line)
pub type TOTALH_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16, crate::Safe>;
///Field `TOTALW` reader - Total Width (in units of pixel clock period)
pub type TOTALW_R = crate::FieldReader<u16>;
///Field `TOTALW` writer - Total Width (in units of pixel clock period)
pub type TOTALW_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16, crate::Safe>;
impl R {
    ///Bits 0:10 - Total Height (in units of horizontal scan line)
    #[inline(always)]
    pub fn totalh(&self) -> TOTALH_R {
        TOTALH_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:27 - Total Width (in units of pixel clock period)
    #[inline(always)]
    pub fn totalw(&self) -> TOTALW_R {
        TOTALW_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TWCR")
            .field("totalw", &self.totalw())
            .field("totalh", &self.totalh())
            .finish()
    }
}
impl W {
    ///Bits 0:10 - Total Height (in units of horizontal scan line)
    #[inline(always)]
    pub fn totalh(&mut self) -> TOTALH_W<'_, TWCRrs> {
        TOTALH_W::new(self, 0)
    }
    ///Bits 16:27 - Total Width (in units of pixel clock period)
    #[inline(always)]
    pub fn totalw(&mut self) -> TOTALW_W<'_, TWCRrs> {
        TOTALW_W::new(self, 16)
    }
}
/**Total Width Configuration Register

You can [`read`](crate::Reg::read) this register and get [`twcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`twcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#LTDC:TWCR)*/
pub struct TWCRrs;
impl crate::RegisterSpec for TWCRrs {
    type Ux = u32;
}
///`read()` method returns [`twcr::R`](R) reader structure
impl crate::Readable for TWCRrs {}
///`write(|w| ..)` method takes [`twcr::W`](W) writer structure
impl crate::Writable for TWCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TWCR to value 0
impl crate::Resettable for TWCRrs {}

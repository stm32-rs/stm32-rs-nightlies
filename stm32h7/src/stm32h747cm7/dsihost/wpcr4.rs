///Register `WPCR4` reader
pub type R = crate::R<WPCR4rs>;
///Register `WPCR4` writer
pub type W = crate::W<WPCR4rs>;
///Field `TCLKPOST` reader - tCLK-POST
pub type TCLKPOST_R = crate::FieldReader;
///Field `TCLKPOST` writer - tCLK-POST
pub type TCLKPOST_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - tCLK-POST
    #[inline(always)]
    pub fn tclkpost(&self) -> TCLKPOST_R {
        TCLKPOST_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WPCR4")
            .field("tclkpost", &self.tclkpost())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - tCLK-POST
    #[inline(always)]
    pub fn tclkpost(&mut self) -> TCLKPOST_W<'_, WPCR4rs> {
        TCLKPOST_W::new(self, 0)
    }
}
/**DSI wrapper PHY configuration register 4

You can [`read`](crate::Reg::read) this register and get [`wpcr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpcr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#DSIHOST:WPCR4)*/
pub struct WPCR4rs;
impl crate::RegisterSpec for WPCR4rs {
    type Ux = u32;
}
///`read()` method returns [`wpcr4::R`](R) reader structure
impl crate::Readable for WPCR4rs {}
///`write(|w| ..)` method takes [`wpcr4::W`](W) writer structure
impl crate::Writable for WPCR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WPCR4 to value 0
impl crate::Resettable for WPCR4rs {}

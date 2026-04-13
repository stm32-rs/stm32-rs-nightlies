///Register `HNPTXFSIZ_ALTERNATE` reader
pub type R = crate::R<HNPTXFSIZ_ALTERNATErs>;
///Register `HNPTXFSIZ_ALTERNATE` writer
pub type W = crate::W<HNPTXFSIZ_ALTERNATErs>;
///Field `TX0FSA` reader - Endpoint 0 transmit RAM start address
pub type TX0FSA_R = crate::FieldReader<u16>;
///Field `TX0FSA` writer - Endpoint 0 transmit RAM start address
pub type TX0FSA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `TX0FD` reader - Endpoint 0 Tx FIFO depth
pub type TX0FD_R = crate::FieldReader<u16>;
///Field `TX0FD` writer - Endpoint 0 Tx FIFO depth
pub type TX0FD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Endpoint 0 transmit RAM start address
    #[inline(always)]
    pub fn tx0fsa(&self) -> TX0FSA_R {
        TX0FSA_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Endpoint 0 Tx FIFO depth
    #[inline(always)]
    pub fn tx0fd(&self) -> TX0FD_R {
        TX0FD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HNPTXFSIZ_ALTERNATE")
            .field("tx0fsa", &self.tx0fsa())
            .field("tx0fd", &self.tx0fd())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Endpoint 0 transmit RAM start address
    #[inline(always)]
    pub fn tx0fsa(&mut self) -> TX0FSA_W<'_, HNPTXFSIZ_ALTERNATErs> {
        TX0FSA_W::new(self, 0)
    }
    ///Bits 16:31 - Endpoint 0 Tx FIFO depth
    #[inline(always)]
    pub fn tx0fd(&mut self) -> TX0FD_W<'_, HNPTXFSIZ_ALTERNATErs> {
        TX0FD_W::new(self, 16)
    }
}
/**OTG host non-periodic transmit FIFO size register \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`hnptxfsiz_alternate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hnptxfsiz_alternate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#OTG1:HNPTXFSIZ_ALTERNATE)*/
pub struct HNPTXFSIZ_ALTERNATErs;
impl crate::RegisterSpec for HNPTXFSIZ_ALTERNATErs {
    type Ux = u32;
}
///`read()` method returns [`hnptxfsiz_alternate::R`](R) reader structure
impl crate::Readable for HNPTXFSIZ_ALTERNATErs {}
///`write(|w| ..)` method takes [`hnptxfsiz_alternate::W`](W) writer structure
impl crate::Writable for HNPTXFSIZ_ALTERNATErs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HNPTXFSIZ_ALTERNATE to value 0x0200_0200
impl crate::Resettable for HNPTXFSIZ_ALTERNATErs {
    const RESET_VALUE: u32 = 0x0200_0200;
}

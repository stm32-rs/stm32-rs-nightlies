///Register `DIEPTXF0` reader
pub type R = crate::R<DIEPTXF0rs>;
///Register `DIEPTXF0` writer
pub type W = crate::W<DIEPTXF0rs>;
///Field `TX0FSA` reader - Endpoint 0 transmit RAM start address
pub type TX0FSA_R = crate::FieldReader<u16>;
///Field `TX0FSA` writer - Endpoint 0 transmit RAM start address
pub type TX0FSA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `TX0FD` reader - Endpoint 0 TxFIFO depth
pub type TX0FD_R = crate::FieldReader<u16>;
///Field `TX0FD` writer - Endpoint 0 TxFIFO depth
pub type TX0FD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Endpoint 0 transmit RAM start address
    #[inline(always)]
    pub fn tx0fsa(&self) -> TX0FSA_R {
        TX0FSA_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Endpoint 0 TxFIFO depth
    #[inline(always)]
    pub fn tx0fd(&self) -> TX0FD_R {
        TX0FD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPTXF0")
            .field("tx0fsa", &self.tx0fsa())
            .field("tx0fd", &self.tx0fd())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Endpoint 0 transmit RAM start address
    #[inline(always)]
    pub fn tx0fsa(&mut self) -> TX0FSA_W<'_, DIEPTXF0rs> {
        TX0FSA_W::new(self, 0)
    }
    ///Bits 16:31 - Endpoint 0 TxFIFO depth
    #[inline(always)]
    pub fn tx0fd(&mut self) -> TX0FD_W<'_, DIEPTXF0rs> {
        TX0FD_W::new(self, 16)
    }
}
/**Endpoint 0 transmit FIFO size (peripheral mode)

You can [`read`](crate::Reg::read) this register and get [`dieptxf0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F756.html#OTG_HS_GLOBAL:DIEPTXF0)*/
pub struct DIEPTXF0rs;
impl crate::RegisterSpec for DIEPTXF0rs {
    type Ux = u32;
}
///`read()` method returns [`dieptxf0::R`](R) reader structure
impl crate::Readable for DIEPTXF0rs {}
///`write(|w| ..)` method takes [`dieptxf0::W`](W) writer structure
impl crate::Writable for DIEPTXF0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DIEPTXF0 to value 0x0200
impl crate::Resettable for DIEPTXF0rs {
    const RESET_VALUE: u32 = 0x0200;
}
